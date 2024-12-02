use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Response {
    status: String,
    message: String,
    data: String,
}

impl Response {
    pub fn get_status(&self) -> &str {
        &self.status
    }

    pub fn get_message(&self) -> &str {
        &self.message
    }

    pub fn get_data(&self) -> &str {
        &self.data
    }

    pub fn status(mut self, status: &str) -> Self {
        self.status = status.to_string();
        self
    }

    pub fn message(mut self, message: &str) -> Self {
        self.message = message.to_string();
        self
    }

    pub fn data(mut self, data: &str) -> Self {
        self.data = data.to_string();
        self
    }

    pub fn ok(message: String, data: String) -> Self {
        Self {
            status: "OK".to_string(),
            message,
            data,
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            status: "KO".to_string(),
            message,
            data: String::new(),
        }
    }
}

impl PartialEq for Response {
    fn eq(&self, other: &Self) -> bool {
        self.status == other.status && self.message == other.message && self.data == other.data
    }
}
