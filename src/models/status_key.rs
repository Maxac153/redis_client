use serde::{Deserialize, Serialize};

use super::type_key::TypeKey;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct StatusKey {
    key: String,
    type_key: TypeKey,
    len: i32,
    memory_usage: u32,
    ttl: i32,
}

impl StatusKey {
    pub fn get_key(&self) -> &str {
        &self.key
    }

    pub fn get_type_key(&self) -> &str {
        &self.type_key.as_str()
    }

    pub fn get_len(&self) -> i32 {
        self.len
    }

    pub fn get_memory_usage(&self) -> u32 {
        self.memory_usage
    }

    pub fn get_ttl(&self) -> i32 {
        self.ttl
    }

    pub fn key(mut self, key: &str) -> Self {
        self.key = key.to_string();
        self
    }

    pub fn type_key(mut self, type_key: TypeKey) -> Self {
        self.type_key = type_key;
        self
    }

    pub fn len(mut self, len: i32) -> Self {
        self.len = len;
        self
    }

    pub fn memory_usage(mut self, memory_usage: u32) -> Self {
        self.memory_usage = memory_usage;
        self
    }

    pub fn ttl(mut self, ttl: i32) -> Self {
        self.ttl = ttl;
        self
    }
}
