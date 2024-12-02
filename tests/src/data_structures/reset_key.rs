#[derive(Debug, Default)]
pub struct ResetKey {
    key: String,
}

impl ResetKey {
    pub fn get_key(&self) -> &str {
        &self.key
    }

    pub fn key(mut self, key: &str) -> Self {
        self.key = key.to_string();
        self
    }
}
