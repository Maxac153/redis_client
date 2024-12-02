#[derive(Debug, Default)]
pub struct RenameKey {
    old_name_key: String,
    new_name_key: String,
}

impl RenameKey {
    pub fn get_old_name_key(&self) -> &str {
        &self.old_name_key
    }

    pub fn get_new_name_key(&self) -> &str {
        &self.new_name_key
    }

    pub fn old_name_key(mut self, old_name_key: &str) -> Self {
        self.old_name_key = old_name_key.to_string();
        self
    }

    pub fn new_name_key(mut self, new_name_key: &str) -> Self {
        self.new_name_key = new_name_key.to_string();
        self
    }
}
