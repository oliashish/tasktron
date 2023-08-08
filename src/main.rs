use clap::Parser;

mod commands;
mod tasks;
mod utils;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    env_logger::init();
    let args = commands::Args::parse();

    let home_dir = dirs::home_dir().expect("Unable to fetch home directory");

    match &args.command {
        commands::Commands::Add { task } => {
            tasks::add_tasks(task, &home_dir).await?;
        }
        commands::Commands::List => tasks::list_tasks(&home_dir).await?,
        _ => todo!(),
    }

    Ok(())
}
