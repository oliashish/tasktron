use std::path::Path;

use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::commands;

#[derive(Debug, Serialize)]
pub struct Task {
    pub name: String,
    pub created: DateTime<Utc>,
    pub completed: bool,
    pub cancelled: bool,
}

pub async fn write_tasks_to_home_directory(
    args: &commands::Args,
    home_dir: &Path,
) -> Result<(), anyhow::Error> {
    let mut tasks: Vec<Task> = Vec::new();

    for task in args.add.iter() {
        let task = Task {
            name: task.to_owned(),
            created: Utc::now(),
            completed: false,
            cancelled: false,
        };

        tasks.push(task)
    }

    // Check if tasktron directory exits or not
    let file_dir = format!("{}/.tasktron", home_dir.to_str().unwrap());
    let file_path = Path::new(&file_dir);
    if !file_path.exists() {
        std::fs::create_dir_all(file_path)?
    }

    std::fs::write(
        format!("{}/.tasktron/tasks.json", home_dir.display()),
        serde_json::to_string_pretty(&tasks).expect("unable to parse tasks"),
    )
    .expect("Unable to write data to desinated path");

    Ok(())
}
