use serde::{Deserialize, Serialize};

use super::status_key::StatusKey;

#[derive(Debug, Default, Serialize, Deserialize)]
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

    pub fn connected_clients(mut self, connected_clients: u32) -> Self {
        self.connected_clients = connected_clients;
        self
    }

    pub fn total_memory_usage(mut self, total_memory_usage: String) -> Self {
        self.total_memory_usage = total_memory_usage;
        self
    }

    pub fn keys(mut self, keys: Vec<String>) -> Self {
        self.keys = keys;
        self
    }

    pub fn statuses(mut self, statuses: Vec<StatusKey>) -> Self {
        self.statuses = statuses;
        self
    }
}
