#[derive(Debug, Default)]
pub struct StatusKey {
    search_key: String,
}

impl StatusKey {
    pub fn get_search_key(&self) -> &str {
        &self.search_key
    }

    pub fn search_key(mut self, search_key: &str) -> Self {
        self.search_key = search_key.to_string();
        self
    }

    pub fn build(self) -> StatusKey {
        StatusKey {
            search_key: self.search_key,
        }
    }
}