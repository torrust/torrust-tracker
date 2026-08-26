use std::str::FromStr;
use std::time::Duration;

use anyhow::{Context, Result};
use secrecy::SecretString;
use sqlx::mysql::{MySqlConnectOptions, MySqlPoolOptions};
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
/// ready. Belt-and-braces against a brief race between the second
/// `ready for connections` log line and TCP acceptance on port 3306.
const READINESS_PING_RETRIES: usize = 30;
/// Delay between readiness-ping attempts.
const READINESS_PING_INTERVAL: Duration = Duration::from_millis(500);

pub(super) async fn initialize(db_version: &str) -> Result<ActiveDatabase> {
    // The official `mysql` image emits `ready for connections` twice on stderr:
    // first transiently during init on the unix socket, then again once mysqld
    // is actually accepting TCP clients on port 3306. We wait for the second
    // occurrence so the first query (DDL via `initialize_database`) does not
    // race the TCP listener and panic with `UnexpectedEof`. This is the same
    // idiom the Java testcontainers MySQL module uses internally.
    let mysql_container = GenericImage::new("mysql", db_version)
        .with_exposed_port(3306.tcp())
        .with_wait_for(WaitFor::Log(LogWaitStrategy::stderr("ready for connections").with_times(2)))
        .with_env_var("MYSQL_ROOT_PASSWORD", "test")
        .with_env_var("MYSQL_DATABASE", "torrust_tracker_bench")
        .with_env_var("MYSQL_ROOT_HOST", "%")
        .start()
        .await
        .context("failed to start mysql test container")?;

    let host = mysql_container
        .get_host()
        .await
        .context("failed to resolve mysql container host")?;
    let port = mysql_container
        .get_host_port_ipv4(3306)
        .await
        .context("failed to resolve mysql container host port")?;

    let mysql_database_url = format!("mysql://root:test@{host}:{port}/torrust_tracker_bench");

    // Belt-and-braces: even after the readiness log message, the very first TCP
    // connect can still hit `UnexpectedEof` while mysqld finalises bind/accept.
    // Probe with a short connect-and-ping loop so the production
    // `initialize_database` call below sees a steady server. This mirrors what
    // the previous r2d2-based driver did implicitly through pool checkout
    // retries.
    wait_until_mysql_accepts_connections(&mysql_database_url)
        .await
        .context("mysql container did not accept connections in time")?;

    let config = Core {
        database: Some(Database::MySQL(ConnectionInfo {
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
        resource: Some(BenchmarkResource::Mysql(Box::new(mysql_container))),
    })
}

async fn wait_until_mysql_accepts_connections(database_url: &str) -> Result<()> {
    let options = MySqlConnectOptions::from_str(database_url).context("invalid mysql benchmark URL")?;

    let mut last_error: Option<sqlx::Error> = None;

    for _ in 0..READINESS_PING_RETRIES {
        match MySqlPoolOptions::new().max_connections(1).connect_with(options.clone()).await {
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
        "mysql still not accepting connections after {READINESS_PING_RETRIES} attempts; last error: {error}",
        error = last_error.map_or_else(|| "<none>".to_string(), |e| e.to_string())
    ))
}
