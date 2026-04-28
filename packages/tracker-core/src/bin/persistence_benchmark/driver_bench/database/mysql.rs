use anyhow::{Context, Result};
use bittorrent_tracker_core::databases::setup::initialize_database;
use testcontainers::core::IntoContainerPort;
use testcontainers::runners::AsyncRunner;
use testcontainers::{GenericImage, ImageExt};
use torrust_tracker_configuration as configuration;

use super::{ActiveDatabase, BenchmarkResource};

pub(super) async fn initialize(db_version: &str) -> Result<ActiveDatabase> {
    let mysql_container = GenericImage::new("mysql", db_version)
        .with_exposed_port(3306.tcp())
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
    let mut config = configuration::Core::default();
    config.database.driver = configuration::Driver::MySQL;
    config.database.path = mysql_database_url;
    let database = initialize_database(&config);

    Ok(ActiveDatabase {
        database,
        resource: Some(BenchmarkResource::Mysql(Box::new(mysql_container))),
    })
}
