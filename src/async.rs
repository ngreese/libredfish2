//! Module defining async Redfish functionality when the `async` feature is used.

use crate::{Config, manager, power, storage, thermal};
use reqwest::Client;
use reqwest::header::ACCEPT;
use reqwest::header::CONTENT_TYPE;
use reqwest::header::HeaderValue;
use serde::de::DeserializeOwned;

/// Struct representing a specific host's endpoint to interface with.
#[derive(Debug)]
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

    /// Utility function used to send an async request to Redfish endpoint.
    ///
    /// This should not normally be used to pull from endpoints. If you *must*, call `redfish.get::<serde_json::Value>(api).await?` to return a generic JSON object.
    pub async fn get<T>(&self, api: &str) -> Result<T, reqwest::Error>
    where
        T: DeserializeOwned + ::std::fmt::Debug,
    {
        let uri = super::build_uri(
            &self.config.host,
            self.config.port,
            self.config.api_version,
            api,
        );

        let res: T = match &self.config.user {
            Some(user) => {
                self.client
                    .get(&uri)
                    .header(ACCEPT, HeaderValue::from_static("application/json"))
                    .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
                    .basic_auth(user, self.config.password.as_ref())
                    .send()
                    .await?
                    .error_for_status()?
                    .json()
                    .await?
            }
            None => {
                self.client
                    .get(&uri)
                    .header(ACCEPT, HeaderValue::from_static("application/json"))
                    .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
                    .send()
                    .await?
                    .error_for_status()?
                    .json()
                    .await?
            }
        };
        Ok(res)
    }

    /// Pulls array controller information.
    ///
    /// Uses the `Systems/1/SmartStorage/ArrayControllers/{controller_id}/` endpoint,
    /// where `controller_id` is a specified ID of the array controller.
    pub async fn get_array_controller(
        &self,
        controller_id: u64,
    ) -> Result<storage::ArrayController, reqwest::Error> {
        let uri = format!("Systems/1/SmartStorage/ArrayControllers/{}/", controller_id);
        let s: storage::ArrayController = self.get(uri.as_str()).await?;
        Ok(s)
    }

    /// Gets all of the array controllers for a LOM host.
    pub async fn get_array_controllers(&self) -> Result<storage::ArrayControllers, reqwest::Error> {
        let uri = "Systems/1/SmartStorage/ArrayControllers/";
        let s: storage::ArrayControllers = self.get(uri).await?;
        Ok(s)
    }

    /// Query the manager status from the server
    pub async fn get_manager_status(&self) -> Result<manager::Manager, reqwest::Error> {
        let uri = "Managers/";
        let m: manager::Manager = self.get(uri).await?;
        Ok(m)
    }

    /// Query the power status from the server
    pub async fn get_power_status(&self) -> Result<power::Power, reqwest::Error> {
        let uri = "Chassis/1/Power/";
        let p: power::Power = self.get(uri).await?;
        Ok(p)
    }

    /// Query the thermal status from the server
    pub async fn get_thermal_status(&self) -> Result<thermal::Thermal, reqwest::Error> {
        let uri = "Chassis/1/Thermal/";
        let t: thermal::Thermal = self.get(uri).await?;
        Ok(t)
    }

    /// Query the smart array status from the server for a specified `controller_id`
    ///
    /// Uses the `Systems/1/SmartStorage/ArrayControllers/{controller_id}/` API endpoint
    pub async fn get_smart_array_status(
        &self,
        controller_id: u64,
    ) -> Result<storage::SmartArray, reqwest::Error> {
        let uri = format!("Systems/1/SmartStorage/ArrayControllers/{}/", controller_id);
        let s: storage::SmartArray = self.get(uri.as_str()).await?;
        Ok(s)
    }

    /// Gets all of the LUN's configured for a host for a specified `controller_id`.
    ///
    /// Uses the `Systems/1/SmartStorage/ArrayControllers/{controller_id}/LogicalDrives/` API endpoint.
    pub async fn get_logical_drives(
        &self,
        controller_id: u64,
    ) -> Result<storage::LogicalDrives, reqwest::Error> {
        let uri = format!(
            "Systems/1/SmartStorage/ArrayControllers/{}/LogicalDrives/",
            controller_id
        );
        let s: storage::LogicalDrives = self.get(uri.as_str()).await?;
        Ok(s)
    }

    /// Gets a single physical drive for a host.
    ///
    /// Uses the `Systems/1/SmartStorage/ArrayControllers/{controller_id}/DiskDrives/{drive_id}/` API endpoint
    /// where `drive_id` is the identifier for the drive controlled by the controller of `controller_id`.
    pub async fn get_physical_drive(
        &self,
        drive_id: u64,
        controller_id: u64,
    ) -> Result<storage::DiskDrive, reqwest::Error> {
        let uri = format!(
            "Systems/1/SmartStorage/ArrayControllers/{}/DiskDrives/{}/",
            controller_id, drive_id,
        );
        let d: storage::DiskDrive = self.get(uri.as_str()).await?;
        Ok(d)
    }

    /// Gets all of the physical drives for a host.
    ///
    /// Uses the `Systems/1/SmartStorage/ArrayControllers/{controller_id}/DiskDrives/` API endpoint
    /// where `drive_id` is the identifier for the drive controlled by the controller of `controller_id`.
    pub async fn get_physical_drives(
        &self,
        controller_id: u64,
    ) -> Result<storage::DiskDrives, reqwest::Error> {
        let uri = format!(
            "Systems/1/SmartStorage/ArrayControllers/{}/DiskDrives/",
            controller_id
        );
        let d: storage::DiskDrives = self.get(uri.as_str()).await?;
        Ok(d)
    }

    /// Gets all of the storage enclosures for a specific `controller_id`
    ///
    /// Uses the `Systems/1/SmartStorage/ArrayControllers/{controller_id}/StorageEnclosures/` API endpoint
    pub async fn get_storage_enclosures(
        &self,
        controller_id: u64,
    ) -> Result<storage::StorageEnclosures, reqwest::Error> {
        let uri = format!(
            "Systems/1/SmartStorage/ArrayControllers/{}/StorageEnclosures/",
            controller_id
        );
        let s: storage::StorageEnclosures = self.get(uri.as_str()).await?;
        Ok(s)
    }

    /// Gets a single storage enclosure for a specific `controller_id`
    ///
    /// Uses the `Systems/1/SmartStorage/ArrayControllers/{controller_id}/StorageEnclosures/{enclosure_id}/` API endpoint
    pub async fn get_storage_enclosure(
        &self,
        controller_id: u64,
        enclosure_id: u64,
    ) -> Result<storage::StorageEnclosure, reqwest::Error> {
        let uri = format!(
            "Systems/1/SmartStorage/ArrayControllers/{}/StorageEnclosures/{}/",
            controller_id, enclosure_id,
        );
        let s: storage::StorageEnclosure = self.get(uri.as_str()).await?;
        Ok(s)
    }
}
