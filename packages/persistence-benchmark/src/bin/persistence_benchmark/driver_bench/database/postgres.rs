use std::str::FromStr;
use std::time::Duration;

use anyhow::{Context, Result};
use secrecy::SecretString;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use testcontainers::core::wait::LogWaitStrategy;
use testcontainers::core::{IntoContainerPort, WaitFor};
use testcontainers::runners::AsyncRunner;
use testcontainers::{GenericImage, ImageExt};
use torrust_tracker_configuration::v3_0_0::{
    core::Core,
    database::{ConnectionInfo, Database},
};
use torrust_tracker_core::databases::setup::initialize_database;

use super::{ActiveDatabase, BenchmarkResource};

/// Maximum number of connect-and-ping attempts after the container is reported
/// ready.
const READINESS_PING_RETRIES: usize = 30;
/// Delay between readiness-ping attempts.
const READINESS_PING_INTERVAL: Duration = Duration::from_millis(500);

pub(super) async fn initialize(db_version: &str) -> Result<ActiveDatabase> {
    // The official `postgres` image emits "database system is ready to accept
    // connections" once on stderr when the TCP listener is up. We wait for
    // that single occurrence before probing the connection — this mirrors the
    // two-occurrence strategy used for MySQL where the init cycle emits it
    // twice. PostgreSQL only emits it once.
    let postgres_container = GenericImage::new("postgres", db_version)
        .with_exposed_port(5432.tcp())
        .with_wait_for(WaitFor::Log(LogWaitStrategy::stderr(
            "database system is ready to accept connections",
        )))
        .with_env_var("POSTGRES_PASSWORD", "test")
        .with_env_var("POSTGRES_DB", "torrust_tracker_bench")
        .with_env_var("POSTGRES_USER", "root")
        .start()
        .await
        .context("failed to start postgres test container")?;

    let host = postgres_container
        .get_host()
        .await
        .context("failed to resolve postgres container host")?;
    let port = postgres_container
        .get_host_port_ipv4(5432)
        .await
        .context("failed to resolve postgres container host port")?;

    let postgres_database_url = format!("postgresql://root:test@{host}:{port}/torrust_tracker_bench");

    wait_until_postgres_accepts_connections(&postgres_database_url)
        .await
        .context("postgres container did not accept connections in time")?;

    let config = Core {
        database: Some(Database::PostgreSQL(ConnectionInfo {
            host: host.to_string(),
            port,
            user: "root".to_string(),
            password: SecretString::from("test"),
            database: "torrust_tracker_bench".to_string(),
        })),
        ..Default::default()
    };
    let database = initialize_database(&config).await;

    Ok(ActiveDatabase {
        database: Some(database),
        resource: Some(BenchmarkResource::Postgres(Box::new(postgres_container))),
    })
}

async fn wait_until_postgres_accepts_connections(database_url: &str) -> Result<()> {
    let options = PgConnectOptions::from_str(database_url).context("invalid postgres benchmark URL")?;

    let mut last_error: Option<sqlx::Error> = None;

    for _ in 0..READINESS_PING_RETRIES {
        match PgPoolOptions::new().max_connections(1).connect_with(options.clone()).await {
            Ok(pool) => {
                if let Err(error) = sqlx::query("SELECT 1").execute(&pool).await {
                    last_error = Some(error);
                } else {
                    pool.close().await;
                    return Ok(());
                }
            }
            Err(error) => {
                last_error = Some(error);
            }
        }

        tokio::time::sleep(READINESS_PING_INTERVAL).await;
    }

    Err(anyhow::anyhow!(
        "postgres still not accepting connections after {READINESS_PING_RETRIES} attempts; last error: {error}",
        error = last_error.map_or_else(|| "<none>".to_string(), |e| e.to_string())
    ))
}
