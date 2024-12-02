#[derive(Debug, Default)]
pub struct UploadDump {
    file_path: String,
    file_name: String,
}

impl UploadDump {
    pub fn get_file_path(&self) -> String {
        self.file_path.to_string()
    }

    pub fn get_file_name(&self) -> String {
        self.file_name.to_string()
    }

    pub fn file_path(mut self, file_path: &str) -> Self {
        self.file_path = file_path.to_string();
        self
    }

    pub fn file_name(mut self, file_name: &str) -> Self {
        self.file_name = file_name.to_string();
        self
    }
}
