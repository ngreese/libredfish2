pub mod common;
pub mod manager;
pub mod power;
pub mod storage;
pub mod thermal;

use reqwest::{blocking::Client, header::ACCEPT, header::CONTENT_TYPE, header::HeaderValue};
use serde::de::DeserializeOwned;

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
    /// The endpoint to interact with.
    pub endpoint: String,
    /// Version of the endpoint.
    pub api_version: Option<ApiVersion>,
    /// Password to access the endpoint, if needed.
    pub password: Option<String>,
    /// Point that the endpoint is exposed at.
    pub port: Option<u16>,
}

/// Struct representing a specific host's endpoint to interface with.
pub struct Redfish {
    /// The client to interface with.
    pub client: Client,
    /// The config holding information to access an endpoint.
    pub config: Config,
}

impl Redfish {
    /// Constructor of a Redfish struct.
    pub fn new(client: Client, config: Config) -> Self {
        Redfish { client, config }
    }

    pub fn get<T>(&self, api: &str) -> Result<T, reqwest::Error>
    where
        T: DeserializeOwned + ::std::fmt::Debug,
    {
        let url = match self.config.port {
            Some(p) => {
                    match self.api_version {
                        Some(v) => format!("https://{}:{}/{}/{}", self.config.endpoint, p, v.to_string(), api),
                        None => format!("https://{}:{}/{}", self.config.endpoint, p, api),
                    }
                },
            None => { 
                    match self.api_version {
                        Some(v) => format!("https://{}/{}/{}", self.config.endpoint, v.to_string(), api),
                        None => format!("https://{}/{}", self.config.endpoint, api),
                    }
                },
        };

        let res: T = match &self.config.user {
            Some(user) => self
                .client
                .get(&url)
                .header(ACCEPT, HeaderValue::from_static("application/json"))
                .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
                .basic_auth(user, self.config.password.as_ref())
                .send()?
                .error_for_status()?
                .json()?,
            None => self
                .client
                .get(&url)
                .header(ACCEPT, HeaderValue::from_static("application/json"))
                .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
                .send()?
                .error_for_status()?
                .json()?,
        };
        Ok(res)
    }
    
    pub fn get_array_controller(
        &self,
        controller_id: u64,
    ) -> Result<storage::ArrayController, reqwest::Error> {
        let url = format!("Systems/1/SmartStorage/ArrayControllers/{}/", controller_id);
        let s: storage::ArrayController = self.get(url.as_str())?;
        Ok(s)
    }
    pub fn get_array_controllers(&self) -> Result<storage::ArrayControllers, reqwest::Error> {
        let url = "Systems/1/SmartStorage/ArrayControllers/";
        let s: storage::ArrayControllers = self.get(url)?;
        Ok(s)
    }

    /// Query the manager status from the server
    pub fn get_manager_status(&self) -> Result<manager::Manager, reqwest::Error> {
        let url = "Managers/";
        let m: manager::Manager = self.get(url)?;
        Ok(m)
    }

    /// Query the power status from the server
    pub fn get_power_status(&self) -> Result<power::Power, reqwest::Error> {
        let url = "Chassis/1/Power/";
        let p: power::Power = self.get(url)?;
        Ok(p)
    }

    /// Query the thermal status from the server
    pub fn get_thermal_status(&self) -> Result<thermal::Thermal, reqwest::Error> {
        let url = "Chassis/1/Thermal/";
        let t: thermal::Thermal = self.get(url)?;
        Ok(t)
    }

    /// Query the smart array status from the server
    pub fn get_smart_array_status(
        &self,
        controller_id: u64,
    ) -> Result<storage::SmartArray, reqwest::Error> {
        let url = format!("Systems/1/SmartStorage/ArrayControllers/{}/", controller_id);
        let s: storage::SmartArray = self.get(url.as_str())?;
        Ok(s)
    }

    pub fn get_logical_drives(
        &self,
        controller_id: u64,
    ) -> Result<storage::LogicalDrives, reqwest::Error> {
        let url = format!(
            "Systems/1/SmartStorage/ArrayControllers/{}/LogicalDrives/",
            controller_id
        );
        let s: storage::LogicalDrives = self.get(url.as_str())?;
        Ok(s)
    }

    pub fn get_physical_drive(
        &self,
        drive_id: u64,
        controller_id: u64,
    ) -> Result<storage::DiskDrive, reqwest::Error> {
        let url = format!(
            "Systems/1/SmartStorage/ArrayControllers/{}/DiskDrives/{}/",
            controller_id, drive_id,
        );
        let d: storage::DiskDrive = self.get(url.as_str())?;
        Ok(d)
    }

    pub fn get_physical_drives(
        &self,
        controller_id: u64,
    ) -> Result<storage::DiskDrives, reqwest::Error> {
        let url = format!(
            "Systems/1/SmartStorage/ArrayControllers/{}/DiskDrives/",
            controller_id
        );
        let d: storage::DiskDrives = self.get(url.as_str())?;
        Ok(d)
    }

    pub fn get_storage_enclosures(
        &self,
        controller_id: u64,
    ) -> Result<storage::StorageEnclosures, reqwest::Error> {
        let url = format!(
            "Systems/1/SmartStorage/ArrayControllers/{}/StorageEnclosures/",
            controller_id
        );
        let s: storage::StorageEnclosures = self.get(url.as_str())?;
        Ok(s)
    }
    pub fn get_storage_enclosure(
        &self,
        controller_id: u64,
        enclosure_id: u64,
    ) -> Result<storage::StorageEnclosure, reqwest::Error> {
        let url = format!(
            "Systems/1/SmartStorage/ArrayControllers/{}/StorageEnclosures/{}/",
            controller_id, enclosure_id,
        );
        let s: storage::StorageEnclosure = self.get(url.as_str())?;
        Ok(s)
    }
}
