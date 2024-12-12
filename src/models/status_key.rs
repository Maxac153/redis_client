use super::type_key::TypeKey;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct StatusKey {
    key: String,
    type_key: Option<TypeKey>,
    len: i32,
    memory_usage: u32,
    ttl: i32,
}

impl StatusKey {
    pub fn get_key(&self) -> &str {
        &self.key
    }

    pub fn get_type_key(&self) -> Option<&str> {
        match &self.type_key {
            Some(key) => Some(key.as_str()),
            None => None,
        }
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

    pub fn new(
        key: String,
        type_key: Option<TypeKey>,
        len: i32,
        memory_usage: u32,
        ttl: i32,
    ) -> Self {
        StatusKey {
            key,
            type_key,
            len,
            memory_usage,
            ttl,
        }
    }
}
