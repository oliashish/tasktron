use clap::{command, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Add a task
    #[arg(short, long, num_args = 0.. )]
    pub add: Vec<String>,

    /// List all you tasks. This includes todo, done and cancelled tasks
    #[command(subcommand)]
    pub list: Option<Commands>,
}

#[derive(Subcommand, Clone, Debug)]
pub enum Commands {
    /// List all you tasks. This includes todo, done and cancelled tasks
    #[command()]
    List,
}
