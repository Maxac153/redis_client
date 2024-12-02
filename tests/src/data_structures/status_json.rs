#[derive(Debug, Default)]
pub struct StatusJson {
    search_key: String,
    type_key: String,
    lower_limit: usize,
    upper_limit: usize,
}

impl StatusJson {
    pub fn get_search_key(&self) -> &str {
        &self.search_key
    }

    pub fn get_type_key(&self) -> &str {
        &self.type_key
    }

    pub fn get_lower_limit(&self) -> usize {
        self.lower_limit
    }

    pub fn get_upper_limit(&self) -> usize {
        self.upper_limit
    }

    pub fn search_key(mut self, search_key: &str) -> Self {
        self.search_key = search_key.to_string();
        self
    }

    pub fn type_key(mut self, type_key: &str) -> Self {
        self.type_key = type_key.to_string();
        self
    }

    pub fn lower_limit(mut self, lower_limit: usize) -> Self {
        self.lower_limit = lower_limit;
        self
    }

    pub fn upper_limit(mut self, upper_limit: usize) -> Self {
        self.upper_limit = upper_limit;
        self
    }
}
