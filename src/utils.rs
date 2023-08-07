use serde::{Deserialize, Serialize};
use tabled::Tabled;

// Task related colored icons
pub const _TASK_DONE: &str = "✅";
pub const _TASK_IGNORED: &str = "❌";

// Error messages
pub mod exit_code {
    pub const _SUCCESS: i32 = 0;
    pub const _NOT_FOUND: i32 = 0; // Will get back to this with linux error code
    pub const _UNABLE_TO_WRITE: i32 = 0; // Will get back to this with linux error code
}

#[derive(Tabled, Serialize, Deserialize)]
pub struct Tasks {
    #[serde(rename = "#")]
    pub serial_number: u32,
    pub task: String,
    pub done: bool,
    pub ignored: bool,
}
