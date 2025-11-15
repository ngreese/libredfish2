//! Library to interface with Redfish endpoints. A continuation of libredfish.

#[cfg(feature = "async")]
pub mod r#async;
#[cfg(feature = "blocking")]
pub mod blocking;
pub mod common;
pub mod manager;
pub mod power;
pub mod storage;
pub mod thermal;

#[cfg(feature = "async")]
pub use r#async::Redfish;
#[cfg(feature = "blocking")]
pub use blocking::Redfish;

/// Enumerator to represent the API version information.
#[derive(Default, Copy, Clone, Debug)]
pub enum ApiVersion {
    /// Version 1
    #[default]
    V1,
    /// Version 2
    V2,
}

impl std::fmt::Display for ApiVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::V1 => write!(f, "redfish/v1"),
            Self::V2 => write!(f, "redfish/v2"),
        }
    }
}

/// Struct holding information to interact with a specified endpoint.
#[derive(Debug)]
pub struct Config {
    /// User with access to the endpoint.
    pub user: Option<String>,
    /// The host that is exposing the Redfish endpoint.
    ///
    /// > Note: *Only* use either the IP address or FQDN of the LOM host.
    pub host: String,
    /// Version of the endpoint.
    pub api_version: Option<ApiVersion>,
    /// Password to access the endpoint, if needed.
    pub password: Option<String>,
    /// Point that the endpoint is exposed at.
    pub port: Option<u16>,
}

/// Utility function to build a URI based on port, api version, etc.
fn build_uri(
    host: &String,
    port: Option<u16>,
    api_version: Option<ApiVersion>,
    api: &str,
) -> String {
    match port {
        Some(p) => match api_version {
            Some(v) => format!("https://{}:{}/{}/{}", host, p, v, api),
            None => format!("https://{}:{}/{}", host, p, api),
        },
        None => match api_version {
            Some(v) => format!("https://{}/{}/{}", host, v, api),
            None => format!("https://{}/{}", host, api),
        },
    }
}
