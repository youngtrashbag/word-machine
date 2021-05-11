pub mod word;
pub mod language;

use serde::{Serialize, Deserialize};

#[derive(Serialize,Deserialize)]
pub struct Message {
    message: String,
}

impl Message {
    /// new message, where parameter should be typed as a static string
    /// ## Example:
    /// ```
    /// Message::new("Hello World");
    /// ```
    pub fn new(message: &str) -> Self {
        Message {
            message: message.to_string(),
        }
    }

    /// Quick Conversion to JSON
    /// consumes self
    /// ## Example:
    /// ```
    /// Message::new("Hello World").to_json();
    /// // output:
    /// // {"message": "Hello World"}
    /// ```
    pub fn to_json(self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}
