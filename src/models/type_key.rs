use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub enum TypeKey {
    List,
    Hash,
}

impl Default for TypeKey {
    fn default() -> Self {
        TypeKey::List
    }
}

impl TypeKey {
    pub fn as_str(&self) -> &str {
        match self {
            TypeKey::List => "list",
            TypeKey::Hash => "hash",
        }
    }
}

impl From<&str> for TypeKey {
    fn from(s: &str) -> Self {
        match s {
            "list" => TypeKey::List,
            "hash" => TypeKey::Hash,
            _ => TypeKey::default(),
        }
    }
}
