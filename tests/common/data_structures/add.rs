#[derive(Debug, Default)]
pub struct AddKey {
    key: String,
    add_mod: String,
    field: String,
    body_data: String,
}

impl AddKey {
    pub fn get_key(&self) -> &str {
        &self.key
    }

    pub fn get_add_mod(&self) -> &str {
        &self.add_mod
    }

    pub fn get_field(&self) -> &str {
        &self.field
    }

    pub fn get_body_data(&self) -> &str {
        &self.body_data
    }

    pub fn key(mut self, key: &str) -> Self {
        self.key = key.to_string();
        self
    }

    pub fn add_mod(mut self, add_mod: &str) -> Self {
        self.add_mod = add_mod.to_string();
        self
    }

    pub fn field(mut self, field: &str) -> Self {
        self.field = field.to_string();
        self
    }

    pub fn body_data(mut self, body_data: &str) -> Self {
        self.body_data = body_data.to_string();
        self
    }

    pub fn build(self) -> AddKey {
        AddKey {
            key: self.key,
            add_mod: self.add_mod,
            field: self.field,
            body_data: self.body_data,
        }
    }
}
