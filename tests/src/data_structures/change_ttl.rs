#[derive(Debug, Default)]
pub struct ChangeTtl {
    key: String,
    ttl: i64,
}

impl ChangeTtl {
    pub fn get_key(&self) -> &str {
        &self.key
    }

    pub fn get_ttl(&self) -> i64 {
        self.ttl
    }

    pub fn key(mut self, key: &str) -> Self {
        self.key = key.to_string();
        self
    }

    pub fn ttl(mut self, new_ttl: i64) -> Self {
        self.ttl = new_ttl;
        self
    }
}
