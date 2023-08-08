use std::path::Path;

use anyhow::anyhow;
use chrono::{NaiveDate, Utc};
use log::error;
use serde::{Deserialize, Serialize};

use tabled::{
    settings::{Color, Style},
    Table, Tabled,
};

use crate::utils::exit_code;

#[derive(Debug, Serialize, Deserialize, Tabled)]
pub struct Task {
    pub task: String,
    pub created: NaiveDate,
    pub completed: bool,
    pub cancelled: bool,
}

pub async fn add_tasks(args: &Vec<String>, home_dir: &Path) -> Result<(), anyhow::Error> {
    let mut tasks: Vec<Task> = Vec::new();

    for task in args {
        let task = Task {
            task: task.to_owned(),
            created: Utc::now().date_naive(),
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
    .map_err(|err| {
        error!("Error while adding task: {}", err);
        anyhow!("exiting with code: {}", exit_code::UNABLE_TO_WRITE)
    })
}

pub async fn list_tasks(home_dir: &Path) -> Result<(), anyhow::Error> {
    let tasks = std::fs::read_to_string(format!("{}/.tasktron/tasks.json", home_dir.display()))
        .map_err(|err| {
            error!("Error while reading tasks: {}", err);
            anyhow!("Exiting with: {}", exit_code::NOT_FOUND)
        })?;

    let json_task: Vec<Task> = serde_json::from_str(&tasks).map_err(|err| {
        error!("Error while parsing the tasks: {}", err);
        anyhow!("")
    })?;

    let task_table = Table::builder(json_task)
        .index()
        .build()
        .with(Style::modern())
        .with(Color::FG_BRIGHT_GREEN)
        .to_string();

    println!("{}", task_table);

    Ok(())
}
