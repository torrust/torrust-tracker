use thiserror::Error;

use crate::located_error::LocatedError;
use crate::settings::{CommonSettings, GlobalSettings, ServiceNoSecrets, ServiceProtocol, TlsSettings, TrackerSettings};

#[derive(Error, Clone, Debug)]
pub enum SettingsError {
    #[error("Bad Namespace: \".{field}\" {message}")]
    NamespaceError { message: String, field: String },

    // Todo: Expand this for Semantic Versioning 2.0.0
    #[error("Bad Version: \".{field}\" {message}")]
    VersionError { message: String, field: String },

    #[error("Tracker Settings Error: \".tracker.{field}\": {message}")]
    TrackerSettingsError {
        message: String,
        field: String,
        source: LocatedError<'static, dyn std::error::Error + Send + Sync>,
    },

    #[error("Global Settings Error: \".tracker.global.{field}\": {message}")]
    GlobalSettingsError {
        message: String,
        field: String,
        source: LocatedError<'static, dyn std::error::Error + Send + Sync>,
    },

    #[error("Common Settings Error: \".tracker.common.{field}\": {message}")]
    CommonSettingsError {
        message: String,
        field: String,
        source: LocatedError<'static, dyn std::error::Error + Send + Sync>,
    },

    #[error("Service Settings Error: \".tracker.service.{id}.{field}\": {message}")]
    ServiceSettingsError {
        message: String,
        field: String,
        id: String,
        source: LocatedError<'static, dyn std::error::Error + Send + Sync>,
    },
}

#[derive(Error, Clone, Debug, Eq, Hash, PartialEq)]
pub enum TrackerSettingsError {
    #[error("Required Field is missing (null)!")]
    MissingRequiredField { field: String, data: TrackerSettings },
}

impl TrackerSettingsError {
    #[must_use]
    pub fn get_field(&self) -> String {
        match self {
            Self::MissingRequiredField { field, data: _ } => field,
        }
        .clone()
    }
}

#[derive(Error, Clone, Debug, Eq, Hash, PartialEq)]
pub enum GlobalSettingsError {
    #[error("Required Field is missing (null)!")]
    MissingRequiredField { field: String, data: GlobalSettings },

    #[error("Bad Socket String: \"{input}\", {message}")]
    ExternalIpBadSyntax {
        field: String,
        input: String,
        message: String,
        data: GlobalSettings,
    },
}

impl GlobalSettingsError {
    #[must_use]
    pub fn get_field(&self) -> String {
        match self {
            Self::MissingRequiredField { field, data: _ }
            | Self::ExternalIpBadSyntax {
                field,
                input: _,
                message: _,
                data: _,
            } => field,
        }
        .clone()
    }
}

#[derive(Error, Clone, Debug, Eq, Hash, PartialEq)]
pub enum CommonSettingsError {
    #[error("Required Field is missing (null)!")]
    MissingRequiredField { field: String, data: CommonSettings },

    #[error("Required Field is empty (0 or \"\")!")]
    EmptyRequiredField { field: String, data: CommonSettings },
}

impl CommonSettingsError {
    #[must_use]
    pub fn get_field(&self) -> String {
        match self {
            Self::MissingRequiredField { field, data: _ } | Self::EmptyRequiredField { field, data: _ } => field,
        }
        .clone()
    }
}

#[derive(Error, Clone, Debug)]
pub enum ServiceSettingsError {
    #[error("Required Field is missing (null)!")]
    MissingRequiredField { field: String, data: ServiceNoSecrets },

    #[error("Required Field is empty (0 or \"\")!")]
    EmptyRequiredField { field: String, data: ServiceNoSecrets },

    #[error("Api Services Requires at least one Access Token!")]
    ApiRequiresAccessToken { field: String, data: ServiceNoSecrets },

    #[error("TLS Services Requires TLS Settings!")]
    TlsRequiresTlsConfig { field: String, data: ServiceNoSecrets },

    #[error("Bad TLS Configuration: {source}.")]
    TlsSettingsError {
        field: String,
        source: LocatedError<'static, dyn std::error::Error + Send + Sync>,
        data: ServiceNoSecrets,
    },

    #[error("Bad Socket String: \"{input}\".")]
    BindingAddressBadSyntax {
        field: String,
        input: String,
        message: String,
        data: ServiceNoSecrets,
    },
    #[error("Unexpected Service. Expected: {expected}, Found {found}.")]
    WrongService {
        field: String,
        expected: ServiceProtocol,
        found: ServiceProtocol,
        data: ServiceNoSecrets,
    },
}

impl ServiceSettingsError {
    #[must_use]
    pub fn get_field(&self) -> String {
        match self {
            Self::MissingRequiredField { field, data: _ }
            | Self::EmptyRequiredField { field, data: _ }
            | Self::ApiRequiresAccessToken { field, data: _ }
            | Self::TlsRequiresTlsConfig { field, data: _ }
            | Self::TlsSettingsError {
                field,
                source: _,
                data: _,
            }
            | Self::BindingAddressBadSyntax {
                field,
                input: _,
                message: _,
                data: _,
            }
            | Self::WrongService {
                field,
                expected: _,
                found: _,
                data: _,
            } => field,
        }
        .clone()
    }
}

#[derive(Error, Clone, Debug)]
pub enum TlsSettingsError {
    #[error("Required Field is missing (null)!")]
    MissingRequiredField { field: String, data: TlsSettings },

    #[error("Required Field is empty (0 or \"\")!")]
    EmptyRequiredField { field: String, data: TlsSettings },

    #[error("Unable to find TLS Certificate File: {source}")]
    BadCertificateFilePath {
        field: String,
        source: LocatedError<'static, dyn std::error::Error + Send + Sync>,
    },

    #[error("Unable to find TLS Key File: {source}")]
    BadKeyFilePath {
        field: String,
        source: LocatedError<'static, dyn std::error::Error + Send + Sync>,
    },
}

impl TlsSettingsError {
    #[must_use]
    pub fn get_field(&self) -> String {
        match self {
            Self::BadKeyFilePath { field, source: _ }
            | Self::BadCertificateFilePath { field, source: _ }
            | Self::EmptyRequiredField { field, data: _ }
            | Self::MissingRequiredField { field, data: _ } => field,
        }
        .clone()
    }
}
