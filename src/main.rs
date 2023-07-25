use clap::{arg, command, Parser, Subcommand};

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

fn main() {
    let args = Args::parse();

    for (idx, task) in args.add.iter().enumerate() {
        println!("{}. {}", idx + 1, task);
    }
}
