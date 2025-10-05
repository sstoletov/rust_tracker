use clap::{Parser, Subcommand};
use anyhow::Context;
use crate::models::Task;
use crate::db::{create_task, get_all_tasks, mark_task_completed, delete_task_by_id};

#[derive(Parser)]
#[command(author, version, about = "TaskTracker — CLI + REST API", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run HTTP server
    Server {
        #[arg(short, long, default_value = "127.0.0.1")]
        host: String,
        #[arg(short, long, default_value_t = 8080)]
        port: u16,
    },

    /// Add task
    Add {
        title: String,
        #[arg(short, long)]
        description: Option<String>,
    },

    /// List tasks
    List,

    /// Mark task completed by id
    Complete {
        id: String,
    },

    /// Delete task by id
    Delete {
        id: String,
    },
}

pub async fn add_task(database_url: &str, title: String, description: Option<String>) -> anyhow::Result<()> {
    let task = Task::new(title, description);
    create_task(database_url, &task)
        .await
        .context("failed to create task")?;
    println!("Added task: {}", task.id);
    Ok(())
}

pub async fn list_tasks(database_url: &str) -> anyhow::Result<()> {
    let tasks = get_all_tasks(database_url)
        .await
        .context("failed to fetch tasks")?;
    for t in tasks {
        println!("- {} | {} | completed: {} | created_at: {}", t.id, t.title, t.completed, t.created_at);
    }
    Ok(())
}

pub async fn complete_task(database_url: &str, id: String) -> anyhow::Result<()> {
    mark_task_completed(database_url, &id)
        .await
        .context("failed to mark completed")?;
    println!("Task {} marked completed", id);
    Ok(())
}

pub async fn delete_task(database_url: &str, id: String) -> anyhow::Result<()> {
    delete_task_by_id(database_url, &id)
        .await
        .context("failed to delete")?;
    println!("Task {} deleted", id);
    Ok(())
}

