#[derive(Debug, Default)]
pub struct ReadKey {
    key: String,
    read_mod: String,
}

impl ReadKey {
    pub fn get_key(&self) -> &str {
        &self.key
    }

    pub fn get_read_mod(&self) -> &str {
        &self.read_mod
    }

    pub fn key(mut self, key: &str) -> Self {
        self.key = key.to_string();
        self
    }

    pub fn read_mod(mut self, read_mod: &str) -> Self {
        self.read_mod = read_mod.to_string();
        self
    }
}
