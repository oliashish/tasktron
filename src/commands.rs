use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Clone, Debug)]
pub enum Commands {
    /// Add a task
    Add { task: Vec<String> },
    /// List all tasks
    List,
    /// Mark task as done
    Done { id: u32 },
    /// Ignore a task
    Ignore { id: u32 },
}
