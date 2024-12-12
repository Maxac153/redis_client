use super::status_key::StatusKey;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct StatusJson {
    connected_clients: u32,
    total_memory_usage: String,
    keys: Vec<String>,
    statuses: Vec<StatusKey>,
}

impl StatusJson {
    pub fn get_connected_clients(&self) -> u32 {
        self.connected_clients
    }

    pub fn get_total_memory_usage(&self) -> &str {
        &self.total_memory_usage
    }

    pub fn get_keys(&self) -> &Vec<String> {
        &self.keys
    }

    pub fn get_status(&self) -> &Vec<StatusKey> {
        &self.statuses
    }

    pub fn new(
        connected_clients: u32,
        total_memory_usage: String,
        keys: Vec<String>,
        statuses: Vec<StatusKey>,
    ) -> Self {
        StatusJson {
            connected_clients,
            total_memory_usage,
            keys,
            statuses,
        }
    }
}
