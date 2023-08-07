use clap::Parser;

mod commands;
mod tasks;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    env_logger::init();
    let args = commands::Args::parse();
    let home_dir = dirs::home_dir().expect("Unable to fetch home directory");

    tasks::write_tasks_to_home_directory(&args, &home_dir).await
}
