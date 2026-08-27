//! Database configuration for schema v3.
use secrecy::{ExposeSecret, SecretString};
use serde::de::{self, Deserializer};
use serde::ser::{SerializeStruct, Serializer};
use serde::{Deserialize, Serialize};
use torrust_tracker_primitives::Driver;
use url::Url;

/// Network database connection settings.
#[derive(Serialize, Debug, Clone)]
pub struct ConnectionInfo {
    /// Database server host name or IP address.
    pub host: String,
    /// Database server port.
    pub port: u16,
    /// Database user name.
    pub user: String,
    /// Database user password.
    #[serde(serialize_with = "serialize_secret_for_redacted_output")]
    pub password: SecretString,
    /// Database name.
    pub database: String,
}

impl PartialEq for ConnectionInfo {
    fn eq(&self, other: &Self) -> bool {
        self.host == other.host
            && self.port == other.port
            && self.user == other.user
            && self.password.expose_secret() == other.password.expose_secret()
            && self.database == other.database
    }
}

impl Eq for ConnectionInfo {}

/// Database configuration.
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Database {
    /// SQLite database stored at a filesystem path.
    Sqlite3 {
        /// SQLite database file path.
        path: String,
    },
    /// MySQL database connection.
    MySQL(ConnectionInfo),
    /// PostgreSQL database connection.
    PostgreSQL(ConnectionInfo),
}

impl Default for Database {
    fn default() -> Self {
        Self::Sqlite3 {
            path: Self::default_path(),
        }
    }
}

impl Database {
    fn default_path() -> String {
        String::from("./storage/tracker/lib/database/sqlite3.db")
    }

    /// Returns the connection string required by the persistence driver.
    #[must_use]
    pub fn connection_url(&self) -> String {
        match self {
            Self::Sqlite3 { path } => path.clone(),
            Self::MySQL(connection) => Self::network_connection_url("mysql", connection),
            Self::PostgreSQL(connection) => Self::network_connection_url("postgresql", connection),
        }
    }

    fn network_connection_url(scheme: &str, connection: &ConnectionInfo) -> String {
        let mut url = Url::parse(&format!("{scheme}://localhost")).expect("database URL scheme must be valid");
        url.set_username(&connection.user)
            .expect("database user names must be representable in a URL");
        url.set_password(Some(connection.password.expose_secret()))
            .expect("database passwords must be representable in a URL");
        url.set_host(Some(&connection.host))
            .expect("database hosts must be representable in a URL");
        url.set_port(Some(connection.port))
            .expect("database ports must be representable in a URL");
        url.path_segments_mut()
            .expect("database URLs must support path segments")
            .push(&connection.database);
        url.into()
    }

    /// Serializes the database configuration for the authorized persistence boundary.
    #[must_use]
    pub(crate) fn serialize_for_persistence(&self) -> toml::Table {
        let mut table = toml::Table::new();

        match self {
            Self::Sqlite3 { path } => {
                table.insert("driver".to_string(), toml::Value::String("sqlite3".to_string()));
                table.insert("path".to_string(), toml::Value::String(path.clone()));
            }
            Self::MySQL(connection) => Self::insert_network_connection_for_persistence(&mut table, "mysql", connection),
            Self::PostgreSQL(connection) => {
                Self::insert_network_connection_for_persistence(&mut table, "postgresql", connection);
            }
        }

        table
    }

    fn insert_network_connection_for_persistence(table: &mut toml::Table, driver: &str, connection: &ConnectionInfo) {
        table.insert("driver".to_string(), toml::Value::String(driver.to_string()));
        table.insert("host".to_string(), toml::Value::String(connection.host.clone()));
        table.insert("port".to_string(), toml::Value::Integer(i64::from(connection.port)));
        table.insert("user".to_string(), toml::Value::String(connection.user.clone()));
        table.insert(
            "password".to_string(),
            toml::Value::String(connection.password.expose_secret().to_string()),
        );
        table.insert("database".to_string(), toml::Value::String(connection.database.clone()));
    }
}

impl Serialize for Database {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Sqlite3 { path } => {
                let mut state = serializer.serialize_struct("Database", 2)?;
                state.serialize_field("driver", "sqlite3")?;
                state.serialize_field("path", path)?;
                state.end()
            }
            Self::MySQL(connection) => serialize_network_database(serializer, "mysql", connection),
            Self::PostgreSQL(connection) => serialize_network_database(serializer, "postgresql", connection),
        }
    }
}

fn serialize_network_database<S>(serializer: S, driver: &str, connection: &ConnectionInfo) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut state = serializer.serialize_struct("Database", 6)?;
    state.serialize_field("driver", driver)?;
    state.serialize_field("host", &connection.host)?;
    state.serialize_field("port", &connection.port)?;
    state.serialize_field("user", &connection.user)?;
    state.serialize_field("password", "***")?;
    state.serialize_field("database", &connection.database)?;
    state.end()
}

fn serialize_secret_for_redacted_output<S>(_password: &SecretString, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str("***")
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawDatabase {
    #[serde(default)]
    driver: Option<Driver>,
    path: Option<String>,
    host: Option<String>,
    port: Option<u16>,
    user: Option<String>,
    password: Option<SecretString>,
    database: Option<String>,
}

impl<'de> Deserialize<'de> for Database {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = RawDatabase::deserialize(deserializer)?;

        match raw.driver.clone().unwrap_or(Driver::Sqlite3) {
            Driver::Sqlite3 => {
                reject_network_fields(&raw).map_err(de::Error::custom)?;
                Ok(Self::Sqlite3 {
                    path: raw.path.unwrap_or_else(Self::default_path),
                })
            }
            Driver::MySQL => build_network_database(raw, &Driver::MySQL, 3306).map_err(de::Error::custom),
            Driver::PostgreSQL => build_network_database(raw, &Driver::PostgreSQL, 5432).map_err(de::Error::custom),
        }
    }
}

fn reject_network_fields(raw: &RawDatabase) -> Result<(), &'static str> {
    if raw.host.is_some() || raw.port.is_some() || raw.user.is_some() || raw.password.is_some() || raw.database.is_some() {
        return Err("SQLite database configuration only accepts the `path` field");
    }

    Ok(())
}

fn build_network_database(raw: RawDatabase, driver: &Driver, default_port: u16) -> Result<Database, &'static str> {
    if raw.path.is_some() {
        return Err("network database configuration does not accept the `path` field");
    }

    let password = raw
        .password
        .ok_or("network database configuration requires a `password` field")?;
    if password.expose_secret().trim().is_empty() {
        return Err("network database configuration requires a non-empty `password` field");
    }

    let connection = ConnectionInfo {
        host: raw.host.ok_or("network database configuration requires a `host` field")?,
        port: raw.port.unwrap_or(default_port),
        user: raw.user.ok_or("network database configuration requires a `user` field")?,
        password,
        database: raw
            .database
            .ok_or("network database configuration requires a `database` field")?,
    };

    match driver {
        Driver::MySQL => Ok(Database::MySQL(connection)),
        Driver::PostgreSQL => Ok(Database::PostgreSQL(connection)),
        Driver::Sqlite3 => unreachable!("SQLite is not a network database"),
    }
}

#[cfg(test)]
mod tests {
    use secrecy::{ExposeSecret, SecretString};

    use super::{ConnectionInfo, Database};

    #[test]
    fn it_should_deserialize_mysql_configuration_with_a_default_port() {
        // Arrange
        let config = r#"
            driver = "mysql"
            host = "mysql"
            user = "db_user"
            password = "db_password"
            database = "torrust_tracker"
        "#;

        // Act
        let database: Database = toml::from_str(config).expect("database configuration should deserialize");

        // Assert
        assert_eq!(
            database,
            Database::MySQL(ConnectionInfo {
                host: "mysql".to_string(),
                port: 3306,
                user: "db_user".to_string(),
                password: SecretString::from("db_password"),
                database: "torrust_tracker".to_string(),
            })
        );
    }

    #[test]
    fn it_should_deserialize_postgresql_configuration_with_a_default_port() {
        // Arrange
        let config = r#"
            driver = "postgresql"
            host = "postgres"
            user = "db_user"
            password = "db_password"
            database = "torrust_tracker"
        "#;

        // Act
        let database: Database = toml::from_str(config).expect("database configuration should deserialize");

        // Assert
        let Database::PostgreSQL(connection) = database else {
            panic!("database configuration should be PostgreSQL");
        };
        assert_eq!(connection.port, 5432);
        assert_eq!(connection.password.expose_secret(), "db_password");
    }

    #[test]
    fn sqlite_database_path_should_be_publicly_constructible_and_readable() {
        // Arrange
        let path = "database.db".to_string();

        // Act
        let database = Database::Sqlite3 { path: path.clone() };

        // Assert
        let Database::Sqlite3 { path: configured_path } = database else {
            panic!("database configuration should be SQLite");
        };
        assert_eq!(configured_path, path);
    }

    #[test]
    fn it_should_percent_encode_network_database_connection_components() {
        // Arrange
        let connection = ConnectionInfo {
            host: "database.example".to_string(),
            port: 3306,
            user: "user@example".to_string(),
            password: SecretString::from("pass:word/@+"),
            database: "tracker/name?tenant=one".to_string(),
        };

        // Act
        let url = Database::MySQL(connection).connection_url();

        // Assert
        // cspell:disable
        assert_eq!(
            url,
            "mysql://user%40example:pass%3Aword%2F%40+@database.example:3306/tracker%2Fname%3Ftenant=one"
        );
        // cspell:enable
    }

    #[test]
    fn it_should_reject_missing_or_empty_network_database_password() {
        // Arrange
        let missing_password = "driver = \"mysql\"\nhost = \"mysql\"\nuser = \"user\"\ndatabase = \"tracker\"";
        let empty_password = "driver = \"mysql\"\nhost = \"mysql\"\nuser = \"user\"\npassword = \"  \"\ndatabase = \"tracker\"";

        // Act and assert
        assert!(toml::from_str::<Database>(missing_password).is_err());
        assert!(toml::from_str::<Database>(empty_password).is_err());
    }

    #[test]
    fn it_should_reject_fields_for_another_database_driver() {
        // Arrange
        let config = "driver = \"sqlite3\"\npath = \"database.db\"\nhost = \"mysql\"";

        // Act
        let result = toml::from_str::<Database>(config);

        // Assert
        assert!(result.is_err());
    }

    #[test]
    fn it_should_reject_network_only_and_unknown_fields_for_sqlite() {
        // Arrange
        let network_only_fields = [
            "host = \"mysql\"",
            "port = 3306",
            "user = \"db_user\"",
            "password = \"db_password\"",
            "database = \"torrust_tracker\"",
        ];
        let unknown_field = "driver = \"sqlite3\"\npath = \"database.db\"\nunknown = \"value\"";

        // Act and assert
        for field in network_only_fields {
            let config = format!("driver = \"sqlite3\"\npath = \"database.db\"\n{field}");
            assert!(
                toml::from_str::<Database>(&config).is_err(),
                "field should be rejected: {field}"
            );
        }
        assert!(toml::from_str::<Database>(unknown_field).is_err());
    }

    #[test]
    fn it_should_reject_a_path_for_network_database_drivers() {
        // Arrange
        let connection =
            "host = \"database\"\nuser = \"db_user\"\npassword = \"db_password\"\ndatabase = \"tracker\"\npath = \"database.db\"";

        // Act and assert
        for driver in ["mysql", "postgresql"] {
            let config = format!("driver = \"{driver}\"\n{connection}");
            assert!(
                toml::from_str::<Database>(&config).is_err(),
                "driver should reject path: {driver}"
            );
        }
    }

    #[test]
    fn it_should_redact_password_when_serializing_a_network_database() {
        // Arrange
        let password = "database-password-for-redaction";
        let database = Database::MySQL(ConnectionInfo {
            host: "mysql".to_string(),
            port: 3306,
            user: "db_user".to_string(),
            password: SecretString::from(password),
            database: "torrust_tracker".to_string(),
        });

        // Act
        let serialized = serde_json::to_string(&database).expect("database configuration should serialize");

        // Assert
        assert!(serialized.contains("***"));
        assert!(!serialized.contains(password));
    }
}
