use std::collections::btree_map::Entry::Vacant;
use std::collections::hash_map::RandomState;
use std::collections::{BTreeMap, HashSet};
use std::fs::OpenOptions;
use std::net::{IpAddr, SocketAddr};
use std::path::Path;
use std::str::FromStr;
use std::sync::Arc;

use derive_more::{Deref, DerefMut, Display};
use serde::{Deserialize, Serialize};

use self::old_settings::DatabaseDriversOld;
use crate::apis::settings::ApiTokens;
use crate::errors::settings::{
    CommonSettingsError, GlobalSettingsError, ServiceSettingsError, SettingsError, TlsSettingsError, TrackerSettingsError,
};
use crate::helpers::get_file_at;
use crate::http::{HttpServiceSettings, TlsServiceSettings};
use crate::tracker::mode::Mode;
use crate::tracker::services::common::{Tls, TlsBuilder};
use crate::udp::UdpServiceSettings;
use crate::{apis, databases, Empty};

pub mod manager;
pub mod old_settings;

#[macro_export]
macro_rules! old_to_new {
    ( $( $base_old:expr, $base_new:expr;  $($old:ident: $new:ident),+ )? ) => {
        {
            $( $(
                if let Some(val) = $base_old.$old{
                    $base_new.$new = Some(val)
                }
            )+
        )?
        }
    };
}

#[macro_export]
macro_rules! check_field_is_not_none {
    ( $(  $ctx:expr => $error:ident; $($value:ident),+ )? ) => {
        {
            $( $(
                if $ctx.$value.is_none() {
                    return Err($error::MissingRequiredField {
                        field: format!("{}", stringify!($value)),
                        data: $ctx.into(),
                    })
                };
            )+
            )?
        }
    };
}

#[macro_export]
macro_rules! check_field_is_not_empty {
    ( $( $ctx:expr => $error:ident;$($value:ident : $value_type:ty),+ )? ) => {
        {
            $( $(
                match &$ctx.$value {
                    Some(value) => {
                        if value == &<$value_type>::default(){
                        return Err($error::EmptyRequiredField {
                            field: format!("{}", stringify!($value)),
                            data: $ctx.into()});
                        }
                    },
                    None => {
                        return Err($error::MissingRequiredField {
                            field: format!("{}", stringify!($value)),
                            data: $ctx.into(),
                        });
                    },
                }
            )+
            )?
        }
    };
}

trait Clean {
    fn clean(self) -> Self;
}

trait Fix {
    fn fix(self) -> Self;
    fn empty_fix(self) -> Self;
}

const SETTINGS_NAMESPACE: &str = "org.torrust.tracker.config";
const SETTINGS_NAMESPACE_ERRORED: &str = "org.torrust.tracker.config.errored";
const SETTINGS_VERSION: &str = "1.0.0";

/// Only used to check what is the namespace when deserializing.
#[derive(Deserialize)]
pub struct SettingsNamespace {
    pub namespace: String,
}

/// With an extra 'error' field, used when there are deserializing problems.
#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Ord, Eq, Debug, Clone, Hash)]
pub struct SettingsErrored {
    pub namespace: String,
    pub version: String,
    pub error: String,
    pub tracker: TrackerSettings,
}

impl SettingsErrored {
    pub fn new(tracker: &TrackerSettings, error: &impl std::error::Error) -> Self {
        Self {
            namespace: SETTINGS_NAMESPACE_ERRORED.to_string(),
            version: SETTINGS_VERSION.to_string(),
            error: error.to_string(),
            tracker: tracker.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Ord, Eq, Debug, Clone, Hash)]
pub struct Settings {
    pub namespace: String,
    pub version: String,
    tracker: TrackerSettings,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            namespace: SETTINGS_NAMESPACE.to_string(),
            version: SETTINGS_VERSION.to_string(),
            tracker: TrackerSettings::default(),
        }
    }
}

impl Empty for Settings {
    fn empty() -> Self {
        Self {
            namespace: String::default(),
            version: String::default(),
            tracker: Empty::empty(),
        }
    }
}

impl From<TrackerSettings> for Settings {
    fn from(tracker: TrackerSettings) -> Self {
        Self {
            namespace: SETTINGS_NAMESPACE.to_string(),
            version: SETTINGS_VERSION.to_string(),
            tracker,
        }
    }
}

impl From<Settings> for TrackerSettings {
    fn from(settings: Settings) -> Self {
        settings.tracker
    }
}

impl Settings {
    /// Returns the check of this [`Settings`].
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    pub fn check(&self) -> Result<(), Box<SettingsError>> {
        if self.namespace != *SETTINGS_NAMESPACE {
            return Err(Box::new(SettingsError::NamespaceError {
                message: format!("Actual: \"{}\", Expected: \"{}\"", self.namespace, SETTINGS_NAMESPACE),
                field: "tracker".to_string(),
            }));
        }

        // Todo: Make this Check use Semantic Versioning 2.0.0
        if self.version != *SETTINGS_VERSION {
            return Err(Box::new(SettingsError::VersionError {
                message: format!("Actual: \"{}\", Expected: \"{}\"", self.version, SETTINGS_NAMESPACE),
                field: "version".to_string(),
            }));
        }

        if let Err(err) = self.tracker.check() {
            return Err(Box::new(SettingsError::TrackerSettingsError {
                message: err.to_string(),
                field: err.get_field(),
                source: (Arc::new(err) as Arc<dyn std::error::Error + Send + Sync>).into(),
            }));
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Ord, Eq, Debug, Clone, Hash)]
pub struct TrackerSettings {
    pub global: Option<GlobalSettings>,
    pub common: Option<CommonSettings>,
    pub database: Option<databases::settings::Settings>,
    pub services: Option<Services>,
}

impl Default for TrackerSettings {
    fn default() -> Self {
        Self {
            global: Some(GlobalSettings::default()),
            common: Some(CommonSettings::default()),
            database: Some(databases::settings::SettingsBuilder::default().build().unwrap()),
            services: Some(Services::default()),
        }
    }
}

impl Empty for TrackerSettings {
    fn empty() -> Self {
        Self {
            global: None,
            common: None,
            database: None,
            services: None,
        }
    }
}

impl TrackerSettings {
    fn check(&self) -> Result<(), TrackerSettingsError> {
        check_field_is_not_none!(self.clone() => TrackerSettingsError;
            global, common, database, services
        );
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Hash)]
pub struct TrackerSettingsBuilder {
    tracker_settings: TrackerSettings,
}

impl Empty for TrackerSettingsBuilder {
    fn empty() -> Self {
        Self {
            tracker_settings: Empty::empty(),
        }
    }
}

impl From<TrackerSettings> for TrackerSettingsBuilder {
    fn from(tracker_settings: TrackerSettings) -> Self {
        Self { tracker_settings }
    }
}

impl From<Arc<TrackerSettings>> for TrackerSettingsBuilder {
    fn from(tracker_settings: Arc<TrackerSettings>) -> Self {
        Self {
            tracker_settings: (*tracker_settings).clone(),
        }
    }
}

impl Fix for TrackerSettings {
    /// Replaces with Defaults.
    fn fix(self) -> Self {
        Self {
            global: Some(self.global.filter(|p| p.check().is_ok()).unwrap_or_default()),
            common: Some(self.common.filter(|p| p.check().is_ok()).unwrap_or_default()),
            database: Some(self.database.filter(|p| p.check().is_ok()).unwrap_or_default()),
            services: Some(self.services.filter(|p| p.check().is_ok()).unwrap_or_default()),
        }
    }

    /// Replaces problems, removing everything else, all services are removed.
    fn empty_fix(self) -> Self {
        Self {
            global: self
                .global
                .filter(|p| p.check().is_ok())
                .map_or_else(|| Some(GlobalSettings::default()), |_f| None),
            common: self
                .common
                .filter(|p| p.check().is_ok())
                .map_or_else(|| Some(CommonSettings::default()), |_f| None),
            database: self.database.filter(|p| p.check().is_ok()).map_or_else(
                || Some(databases::settings::SettingsBuilder::default().build().unwrap()),
                |_f| None,
            ),
            services: None,
        }
    }
}

impl Clean for TrackerSettings {
    /// Removes Problems
    fn clean(self) -> Self {
        Self {
            global: self.global.filter(|p| p.check().is_ok()),
            common: self.common.filter(|p| p.check().is_ok()),
            database: self.database.filter(|p| p.check().is_ok()),
            services: self.services.map(Clean::clean),
        }
    }
}

impl TryInto<TrackerSettings> for TrackerSettingsBuilder {
    type Error = SettingsError;

    fn try_into(self) -> Result<TrackerSettings, Self::Error> {
        if let Err(err) = self.tracker_settings.check() {
            return Err(SettingsError::TrackerSettingsError {
                message: err.to_string(),
                field: err.get_field(),
                source: (Arc::new(err) as Arc<dyn std::error::Error + Send + Sync>).into(),
            });
        }

        let settings = TrackerSettings {
            global: Some(GlobalSettingsBuilder::from(self.tracker_settings.global.unwrap()).try_into()?),
            common: Some(CommonSettingsBuilder::from(self.tracker_settings.common.unwrap()).try_into()?),
            database: Some(self.tracker_settings.database.unwrap()),
            services: match self.tracker_settings.services {
                Some(services) => Some(ServicesBuilder::from(services).try_into()?),
                None => None,
            },
        };

        Ok(settings)
    }
}

impl TrackerSettingsBuilder {
    #[must_use]
    pub fn with_global(self, global: &GlobalSettings) -> Self {
        Self {
            tracker_settings: TrackerSettings {
                global: Some(global.clone()),
                common: self.tracker_settings.common,
                database: self.tracker_settings.database,
                services: self.tracker_settings.services,
            },
        }
    }

    #[must_use]
    pub fn with_common(self, common: &CommonSettings) -> Self {
        Self {
            tracker_settings: TrackerSettings {
                global: self.tracker_settings.global,
                common: Some(common.clone()),
                database: self.tracker_settings.database,
                services: self.tracker_settings.services,
            },
        }
    }

    #[must_use]
    pub fn with_database(self, database: &databases::settings::Settings) -> Self {
        Self {
            tracker_settings: TrackerSettings {
                global: self.tracker_settings.global,
                common: self.tracker_settings.common,
                database: Some(database.clone()),
                services: self.tracker_settings.services,
            },
        }
    }

    #[must_use]
    pub fn with_services(self, services: &Services) -> Self {
        Self {
            tracker_settings: TrackerSettings {
                global: self.tracker_settings.global,
                common: self.tracker_settings.common,
                database: self.tracker_settings.database,
                services: Some(services.clone()),
            },
        }
    }

    /// .
    ///
    /// # Panics
    ///
    /// Panics if .
    #[must_use]
    pub fn import_old(mut self, old_settings: &old_settings::Settings) -> Self {
        // Global
        let mut builder = match self.tracker_settings.global.as_ref() {
            Some(settings) => GlobalSettingsBuilder::from(settings.clone()),
            None => GlobalSettingsBuilder::empty(),
        };
        builder = builder.import_old(old_settings);

        self.tracker_settings.global = Some(builder.global_settings);

        // Common
        let mut builder = match self.tracker_settings.common.as_ref() {
            Some(settings) => CommonSettingsBuilder::from(settings.clone()),
            None => CommonSettingsBuilder::empty(),
        };
        builder = builder.import_old(old_settings);

        self.tracker_settings.common = Some(builder.common_settings);

        // Database
        self.tracker_settings.database = old_settings.db_driver.map(|d| match d {
            DatabaseDriversOld::Sqlite3 => databases::settings::SettingsBuilder::default()
                .driver(databases::driver::Driver::Sqlite3)
                .sql_lite_3_db_file_path(old_settings.db_path.as_ref().map(|p| Path::new(p).into()))
                .build()
                .unwrap(),
            DatabaseDriversOld::MySQL => databases::settings::SettingsBuilder::default()
                .driver(databases::driver::Driver::MySQL)
                .my_sql_connection_url(old_settings.db_path.as_ref().cloned())
                .build()
                .unwrap(),
        });

        // Services
        let mut builder = match self.tracker_settings.services.as_ref() {
            Some(settings) => ServicesBuilder::from(settings.clone()),
            None => ServicesBuilder::empty(),
        };
        builder = builder.import_old(old_settings);

        self.tracker_settings.services = Some(builder.services);

        self
    }
}

#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Ord, Eq, Debug, Clone, Hash)]
pub struct GlobalSettings {
    tracker_mode: Option<Mode>,
    log_filter_level: Option<LogFilterLevel>,
    external_ip: Option<IpAddr>,
    on_reverse_proxy: Option<bool>,
}

impl Default for GlobalSettings {
    fn default() -> Self {
        Self {
            tracker_mode: Some(Mode::Listed),
            log_filter_level: Some(LogFilterLevel::Info),
            external_ip: None,
            on_reverse_proxy: Some(false),
        }
    }
}

impl Empty for GlobalSettings {
    fn empty() -> Self {
        Self {
            tracker_mode: None,
            log_filter_level: None,
            external_ip: None,
            on_reverse_proxy: None,
        }
    }
}

impl GlobalSettings {
    fn check(&self) -> Result<(), GlobalSettingsError> {
        self.is_on_reverse_proxy()?;

        Ok(())
    }

    #[must_use]
    pub fn get_tracker_mode(&self) -> Mode {
        self.tracker_mode.unwrap_or_default()
    }

    #[must_use]
    pub fn get_log_filter_level(&self) -> log::LevelFilter {
        match self.log_filter_level.unwrap_or(LogFilterLevel::Info) {
            LogFilterLevel::Off => log::LevelFilter::Off,
            LogFilterLevel::Error => log::LevelFilter::Error,
            LogFilterLevel::Warn => log::LevelFilter::Warn,
            LogFilterLevel::Info => log::LevelFilter::Info,
            LogFilterLevel::Debug => log::LevelFilter::Debug,
            LogFilterLevel::Trace => log::LevelFilter::Trace,
        }
    }

    #[must_use]
    pub fn get_external_ip_opt(&self) -> Option<IpAddr> {
        self.external_ip
    }

    /// Returns the is on reverse proxy of this [`GlobalSettings`].
    ///
    /// # Panics
    ///
    /// Panics if .
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    pub fn is_on_reverse_proxy(&self) -> Result<bool, GlobalSettingsError> {
        check_field_is_not_none!(self.clone() => GlobalSettingsError; on_reverse_proxy);

        Ok(self.on_reverse_proxy.unwrap())
    }
}
#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Ord, Eq, Debug, Clone, Hash, Default)]
pub struct GlobalSettingsBuilder {
    global_settings: GlobalSettings,
}

impl Empty for GlobalSettingsBuilder {
    fn empty() -> Self {
        Self {
            global_settings: Empty::empty(),
        }
    }
}

impl From<GlobalSettings> for GlobalSettingsBuilder {
    fn from(global_settings: GlobalSettings) -> Self {
        Self { global_settings }
    }
}

impl From<Arc<GlobalSettings>> for GlobalSettingsBuilder {
    fn from(global_settings: Arc<GlobalSettings>) -> Self {
        Self {
            global_settings: (*global_settings).clone(),
        }
    }
}

impl TryInto<GlobalSettings> for GlobalSettingsBuilder {
    type Error = SettingsError;

    fn try_into(self) -> Result<GlobalSettings, Self::Error> {
        match self.global_settings.check() {
            Ok(_) => Ok(self.global_settings),
            Err(err) => Err(SettingsError::GlobalSettingsError {
                message: err.to_string(),
                field: err.get_field(),
                source: (Arc::new(err) as Arc<dyn std::error::Error + Send + Sync>).into(),
            }),
        }
    }
}

impl GlobalSettingsBuilder {
    #[must_use]
    pub fn with_external_ip(mut self, external_ip: &IpAddr) -> Self {
        self.global_settings.external_ip = Some(*external_ip);
        self
    }

    #[must_use]
    pub fn with_log_filter(mut self, log_filter: &LogFilterLevel) -> Self {
        self.global_settings.log_filter_level = Some(*log_filter);
        self
    }

    #[must_use]
    pub fn with_mode(mut self, mode: Mode) -> Self {
        self.global_settings.tracker_mode = Some(mode);
        self
    }

    #[must_use]
    pub fn with_reverse_proxy(mut self, reverse_proxy: bool) -> Self {
        self.global_settings.on_reverse_proxy = Some(reverse_proxy);
        self
    }

    #[must_use]
    pub fn import_old(mut self, old_settings: &old_settings::Settings) -> Self {
        if let Some(val) = old_settings.mode.as_ref() {
            self.global_settings.tracker_mode = Some(match val {
                old_settings::TrackerModeOld::Public => Mode::Public,
                old_settings::TrackerModeOld::Listed => Mode::Listed,
                old_settings::TrackerModeOld::Private => Mode::Private,
                old_settings::TrackerModeOld::PrivateListed => Mode::PrivateListed,
            });
        }

        if let Some(val) = old_settings.log_level.as_ref() {
            self.global_settings.log_filter_level = match val.to_lowercase().as_str() {
                "off" => Some(LogFilterLevel::Off),
                "trace" => Some(LogFilterLevel::Trace),
                "debug" => Some(LogFilterLevel::Debug),
                "info" => Some(LogFilterLevel::Info),
                "warn" => Some(LogFilterLevel::Warn),
                "error" => Some(LogFilterLevel::Error),
                _ => None,
            }
        }

        if let Some(val) = old_settings.external_ip.as_ref() {
            if let Ok(ip) = IpAddr::from_str(val) {
                self.global_settings.external_ip = Some(ip);
            };
        }

        if let Some(val) = old_settings.on_reverse_proxy {
            self.global_settings.on_reverse_proxy = Some(val);
        }
        self
    }
}

#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Ord, Eq, Debug, Clone, Hash)]
pub struct CommonSettings {
    pub announce_interval_seconds: Option<u32>,
    pub announce_interval_seconds_minimum: Option<u32>,
    pub peer_timeout_seconds_maximum: Option<u32>,
    pub enable_tracker_usage_statistics: Option<bool>,
    pub enable_persistent_statistics: Option<bool>,
    pub cleanup_inactive_peers_interval_seconds: Option<u64>,
    pub enable_peerless_torrent_pruning: Option<bool>,
}

impl Default for CommonSettings {
    fn default() -> Self {
        Self {
            announce_interval_seconds: Some(120),
            announce_interval_seconds_minimum: Some(120),
            peer_timeout_seconds_maximum: Some(900),
            enable_tracker_usage_statistics: Some(true),
            enable_persistent_statistics: Some(false),
            cleanup_inactive_peers_interval_seconds: Some(600),
            enable_peerless_torrent_pruning: Some(false),
        }
    }
}

impl Empty for CommonSettings {
    fn empty() -> Self {
        Self {
            announce_interval_seconds: None,
            announce_interval_seconds_minimum: None,
            peer_timeout_seconds_maximum: None,
            enable_tracker_usage_statistics: None,
            enable_persistent_statistics: None,
            cleanup_inactive_peers_interval_seconds: None,
            enable_peerless_torrent_pruning: None,
        }
    }
}

impl CommonSettings {
    fn check(&self) -> Result<(), CommonSettingsError> {
        check_field_is_not_none!(self.clone() => CommonSettingsError;
            enable_tracker_usage_statistics,
            enable_persistent_statistics,
            enable_peerless_torrent_pruning
        );

        check_field_is_not_empty!(self.clone() => CommonSettingsError;
            announce_interval_seconds: u32,
            announce_interval_seconds_minimum: u32,
            peer_timeout_seconds_maximum: u32,
            cleanup_inactive_peers_interval_seconds: u64
        );

        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Default)]
pub struct CommonSettingsBuilder {
    common_settings: CommonSettings,
}

impl Empty for CommonSettingsBuilder {
    fn empty() -> Self {
        Self {
            common_settings: Empty::empty(),
        }
    }
}

impl From<CommonSettings> for CommonSettingsBuilder {
    fn from(common_settings: CommonSettings) -> Self {
        Self { common_settings }
    }
}

impl TryInto<CommonSettings> for CommonSettingsBuilder {
    type Error = SettingsError;

    fn try_into(self) -> Result<CommonSettings, Self::Error> {
        match self.common_settings.check() {
            Ok(_) => Ok(self.common_settings),
            Err(err) => Err(SettingsError::CommonSettingsError {
                message: err.to_string(),
                field: err.get_field(),
                source: (Arc::new(err) as Arc<dyn std::error::Error + Send + Sync>).into(),
            }),
        }
    }
}

impl CommonSettingsBuilder {
    #[must_use]
    pub fn import_old(mut self, old_settings: &old_settings::Settings) -> Self {
        old_to_new!(old_settings, self.common_settings;
         announce_interval: announce_interval_seconds,
         max_peer_timeout: peer_timeout_seconds_maximum,
         tracker_usage_statistics: enable_tracker_usage_statistics,
         persistent_torrent_completed_stat: enable_persistent_statistics,
         inactive_peer_cleanup_interval: cleanup_inactive_peers_interval_seconds,
         remove_peerless_torrents: enable_peerless_torrent_pruning
        );
        self
    }
}

/// Special Service Settings with the Private Access Secrets Removed
#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Ord, Eq, Debug, Clone, Hash)]
pub struct ServiceNoSecrets {
    pub enabled: Option<bool>,
    pub display_name: Option<String>,
    pub service: Option<ServiceProtocol>,
    pub socket: Option<SocketAddr>,
    pub tls: Option<TlsSettings>,
    pub access_tokens: Option<ApiTokens>,
}

impl From<&Service> for ServiceNoSecrets {
    fn from(services: &Service) -> Self {
        Self {
            enabled: services.enabled,
            display_name: services.display_name.clone(),
            service: services.service,
            socket: services.socket,
            tls: services.tls.clone(),
            access_tokens: {
                services.api_tokens.as_ref().map(|access_tokens| {
                    access_tokens
                        .iter()
                        .map(|pair| (pair.0.clone(), "SECRET_REMOVED".to_string()))
                        .collect()
                })
            },
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Ord, Eq, Debug, Clone, Hash)]
pub struct Service {
    pub enabled: Option<bool>,
    pub display_name: Option<String>,
    pub service: Option<ServiceProtocol>,
    pub socket: Option<SocketAddr>,
    pub tls: Option<TlsSettings>,
    pub api_tokens: Option<ApiTokens>,
}

impl Empty for Service {
    fn empty() -> Self {
        Self {
            enabled: None,
            display_name: None,
            service: None,
            socket: None,
            tls: None,
            api_tokens: None,
        }
    }
}

impl From<apis::settings::Settings> for Service {
    fn from(service: apis::settings::Settings) -> Self {
        Self {
            enabled: Some(*service.is_enabled()),
            display_name: Some(service.get_display_name().clone()),
            service: Some(ServiceProtocol::Api),
            socket: Some(service.get_socket().expect("socket should be here!")),
            tls: None,
            api_tokens: Some(service.get_access_tokens().clone()),
        }
    }
}

impl From<UdpServiceSettings> for Service {
    fn from(service: UdpServiceSettings) -> Self {
        Self {
            enabled: Some(service.enabled),
            display_name: Some(service.display_name),
            service: Some(ServiceProtocol::Udp),
            socket: Some(service.socket),
            tls: None,
            api_tokens: None,
        }
    }
}

impl From<HttpServiceSettings> for Service {
    fn from(service: HttpServiceSettings) -> Self {
        Self {
            enabled: Some(service.enabled),
            display_name: Some(service.display_name),
            service: Some(ServiceProtocol::Http),
            socket: Some(service.socket),
            tls: None,
            api_tokens: None,
        }
    }
}

impl From<TlsServiceSettings> for Service {
    fn from(service: TlsServiceSettings) -> Self {
        Self {
            enabled: Some(service.enabled),
            display_name: Some(service.display_name),
            service: Some(ServiceProtocol::Tls),
            socket: Some(service.socket),
            tls: Some(TlsSettings {
                certificate_file_path: Some(service.certificate_file_path),
                key_file_path: Some(service.key_file_path),
            }),
            api_tokens: None,
        }
    }
}

impl Service {
    /// .
    ///
    /// # Panics
    ///
    /// Panics if .
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    pub fn check(&self, id: &String) -> Result<(), ServiceSettingsError> {
        check_field_is_not_none!(self => ServiceSettingsError;
        enabled, service, socket);

        check_field_is_not_empty!(self => ServiceSettingsError;
            display_name: String);

        match self.service.unwrap() {
            ServiceProtocol::Api => {
                apis::settings::Settings::try_from((id, self))?;
            }
            ServiceProtocol::Udp => {
                UdpServiceSettings::try_from((id, self))?;
            }
            ServiceProtocol::Http => {
                HttpServiceSettings::try_from((id, self))?;
            }
            ServiceProtocol::Tls => {
                TlsServiceSettings::try_from((id, self))?;
            }
        }

        Ok(())
    }

    /// Returns the get socket of this [`Service`].
    ///
    /// # Panics
    ///
    /// Panics if .
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    pub fn get_socket(&self) -> Result<SocketAddr, ServiceSettingsError> {
        check_field_is_not_none!(self => ServiceSettingsError; socket);

        Ok(self.socket.unwrap())
    }

    /// Returns the get api tokens of this [`Service`].
    ///
    /// # Panics
    ///
    /// Panics if .
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    pub fn get_api_tokens(&self) -> Result<ApiTokens, ServiceSettingsError> {
        check_field_is_not_empty!(self => ServiceSettingsError; api_tokens : ApiTokens);

        Ok(self.api_tokens.clone().unwrap())
    }

    /// Returns the get tls settings of this [`Service`].
    ///
    /// # Panics
    ///
    /// Panics if .
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    pub fn get_tls_settings(&self) -> Result<TlsSettings, ServiceSettingsError> {
        check_field_is_not_empty!(self => ServiceSettingsError; tls : TlsSettings);

        Ok(self.tls.clone().unwrap())
    }

    pub fn get_tls(&self) -> Result<Option<Tls>, ServiceSettingsError> {
        Ok(self.tls.clone().map(|t| -> Tls {
            TlsBuilder::default()
                .certificate_file_path(
                    t.certificate_file_path
                        .expect("if we have a tls, then we should have the certificate"),
                )
                .key_file_path(t.key_file_path.expect("if we have a tls, then we should have the key"))
                .build()
                .expect("failed to build tls settings")
        }))
    }
}

#[derive(Serialize, Deserialize, Ord, PartialOrd, PartialEq, Eq, Debug, Clone, Hash, Deref, DerefMut)]
pub struct Services(BTreeMap<String, Service>);

impl Default for Services {
    fn default() -> Self {
        let api = apis::settings::SettingsBuilder::default()
            .build()
            .expect("defaults should build");
        let udp = UdpServiceSettings::default();
        let http = HttpServiceSettings::default();
        let tls = TlsServiceSettings::default();

        let mut services = Services::empty();

        services.insert(api.get_id().clone(), api.into());
        services.insert(udp.id.clone(), udp.into());
        services.insert(http.id.clone(), http.into());
        services.insert(tls.id.clone(), tls.into());

        services
    }
}

impl Empty for Services {
    fn empty() -> Self {
        Self(BTreeMap::new())
    }
}

/// will remove the services that failed the configuration check, returns removed services.
impl Clean for Services {
    fn clean(self) -> Self {
        Self(
            self.iter()
                .filter(|service| service.1.check(service.0).is_ok())
                .map(|pair| (pair.0.clone(), pair.1.clone()))
                .collect(),
        )
    }
}

impl Services {
    /// Returns the check of this [`Services`].
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    pub fn check(&self) -> Result<(), ServiceSettingsError> {
        for service in self.iter() {
            service.1.check(service.0)?;
        }

        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Default)]
pub struct ServicesBuilder {
    services: Services,
}

impl Empty for ServicesBuilder {
    fn empty() -> Self {
        Self {
            services: Empty::empty(),
        }
    }
}

impl TryInto<Services> for ServicesBuilder {
    type Error = SettingsError;

    fn try_into(self) -> Result<Services, Self::Error> {
        for service in &self.services.0 {
            if let Err(err) = service.1.check(service.0) {
                return Err(SettingsError::ServiceSettingsError {
                    id: service.0.clone(),
                    field: err.get_field(),
                    message: err.to_string(),
                    source: (Arc::new(err) as Arc<dyn std::error::Error + Send + Sync>).into(),
                });
            }
        }

        Ok(self.services)
    }
}

impl From<Services> for ServicesBuilder {
    fn from(services: Services) -> Self {
        Self { services }
    }
}

impl ServicesBuilder {
    #[must_use]
    pub fn import_old(mut self, old_settings: &old_settings::Settings) -> Self {
        let existing_service_map = self.services.clone();
        let existing_services: HashSet<&Service, RandomState> = existing_service_map.0.values().collect::<HashSet<_>>();

        let mut new_values: HashSet<(Service, String)> = HashSet::new();

        if let Some(service) = old_settings.http_api.as_ref() {
            new_values.insert(service.clone().into());
        };

        if let Some(services) = old_settings.udp_trackers.as_ref() {
            for service in services {
                new_values.insert(service.clone().into());
            }
        };

        if let Some(services) = old_settings.http_trackers.as_ref() {
            for service in services {
                new_values.insert(service.clone().into());
            }
        };

        for (value, name) in new_values {
            // Lets not import something we already have...
            if !existing_services.contains(&value) {
                for count in 0.. {
                    let key = format!("{name}_{count}");
                    if let Vacant(e) = self.services.0.entry(key) {
                        e.insert(value.clone());
                        break;
                    }
                }
            }
        }
        self
    }
}

#[derive(Serialize, Deserialize, Default, PartialEq, PartialOrd, Ord, Eq, Debug, Clone, Hash)]
pub struct TlsSettings {
    pub certificate_file_path: Option<Box<Path>>,
    pub key_file_path: Option<Box<Path>>,
}

impl Empty for TlsSettings {
    fn empty() -> Self {
        Self {
            certificate_file_path: None,
            key_file_path: None,
        }
    }
}

impl TlsSettings {
    /// Returns the check of this [`TlsSettings`].
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    pub fn check(&self) -> Result<(), TlsSettingsError> {
        self.get_certificate_file_path()?;
        self.get_key_file_path()?;

        Ok(())
    }

    /// Returns the get certificate file path of this [`TlsSettings`].
    ///
    /// # Panics
    ///
    /// Panics if .
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    pub fn get_certificate_file_path(&self) -> Result<Box<Path>, TlsSettingsError> {
        check_field_is_not_none!(self.clone() => TlsSettingsError; certificate_file_path);

        get_file_at(self.certificate_file_path.as_ref().unwrap(), OpenOptions::new().read(true))
            .map(|at| at.1)
            .map_err(|err| TlsSettingsError::BadCertificateFilePath {
                field: "certificate_file_path".to_string(),
                source: (Arc::new(err) as Arc<dyn std::error::Error + Send + Sync>).into(),
            })
    }

    /// Returns the get key file path of this [`TlsSettings`].
    ///
    /// # Panics
    ///
    /// Panics if .
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    pub fn get_key_file_path(&self) -> Result<Box<Path>, TlsSettingsError> {
        check_field_is_not_none!(self.clone() => TlsSettingsError; key_file_path);

        get_file_at(self.key_file_path.as_ref().unwrap(), OpenOptions::new().read(true))
            .map(|at| at.1)
            .map_err(|err| TlsSettingsError::BadKeyFilePath {
                field: "key_file_path".to_string(),
                source: (Arc::new(err) as Arc<dyn std::error::Error + Send + Sync>).into(),
            })
    }
}

#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Ord, Eq, Debug, Copy, Clone, Hash, Display)]
#[serde(rename_all = "snake_case")]
pub enum LogFilterLevel {
    Off,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl Default for LogFilterLevel {
    fn default() -> Self {
        Self::Info
    }
}

#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Ord, Eq, Debug, Copy, Clone, Hash, Display)]
#[serde(rename_all = "snake_case")]
pub enum ServiceProtocol {
    Udp,
    Http,
    Tls,
    Api,
}
