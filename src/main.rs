mod cli;
mod api;
mod db;
mod handlers;
mod models;
mod utils;


use crate::db::init_db;
use clap::Parser;
use tracing_subscriber::{fmt, EnvFilter};


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
    .with_env_filter(EnvFilter::from_default_env())
    .init();

    let cli = cli::Cli::parse();


    // ensure DB exists / run migrations (simple)
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:./task_tracker.db".to_string());
    init_db(&database_url).await?;


    match cli.command {
        cli::Commands::Server { host, port } => {
            api::run_server(&database_url, &host, port).await?;
        }
        cli::Commands::Add { title, description } => {
            cli::add_task(&database_url, title, description).await?;
        }
        cli::Commands::List => {
            cli::list_tasks(&database_url).await?;
        }
        cli::Commands::Complete { id } => {
            cli::complete_task(&database_url, id).await?;
        }
        cli::Commands::Delete { id } => {
            cli::delete_task(&database_url, id).await?;
        }
    }

Ok(())
}
