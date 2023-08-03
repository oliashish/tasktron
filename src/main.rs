use std::path::Path;

use chrono::{DateTime, Utc};
use clap::{arg, command, Parser, Subcommand};
use serde::Serialize;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Add a task
    #[arg(short, long, num_args = 0.. )]
    add: Vec<String>,

    /// List all you tasks. This includes todo, done and cancelled tasks
    #[command(subcommand)]
    list: Option<Commands>,
}

#[derive(Subcommand, Clone, Debug)]
pub enum Commands {
    /// List all you tasks. This includes todo, done and cancelled tasks
    #[command()]
    List,
}

#[derive(Debug, Serialize)]
pub struct Task {
    pub name: String,
    pub created: DateTime<Utc>,
    pub completed: bool,
    pub cancelled: bool,
}

pub struct Tasks {
    pub tasks: Vec<Task>,
}

async fn write_tasks_to_home_directory(args: &Args, home_dir: &Path) -> Result<(), anyhow::Error> {
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

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();
    let home_dir = dirs::home_dir().expect("Unable to fetch home directory");
    let res = write_tasks_to_home_directory(&args, &home_dir).await;
    res
}
