use crate::models::Task;
use sqlx::SqlitePool;
use anyhow::Result;
use sqlx::Row;

pub async fn init_db(database_url: &str) -> Result<()> {
    let pool = SqlitePool::connect(database_url).await?;
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS tasks (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            description TEXT,
            completed INTEGER NOT NULL DEFAULT 0,
            created_at INTEGER NOT NULL
        );
        "#
    )
    .execute(&pool)
    .await?;
    Ok(())
}

async fn get_pool(database_url: &str) -> Result<SqlitePool> {
    let pool = SqlitePool::connect(database_url).await?;
    Ok(pool)
}

pub async fn create_task(database_url: &str, task: &Task) -> Result<()> {
    let pool = get_pool(database_url).await?;
    sqlx::query(
        "INSERT INTO tasks (id, title, description, completed, created_at) VALUES (?, ?, ?, ?, ?)"
    )
    .bind(&task.id)
    .bind(&task.title)
    .bind(&task.description)
    .bind(task.completed as i32)
    .bind(task.created_at)
    .execute(&pool)
    .await?;
    Ok(())
}

pub async fn get_all_tasks(database_url: &str) -> Result<Vec<Task>> {
    let pool = get_pool(database_url).await?;
    let rows = sqlx::query(
        "SELECT id, title, description, completed, created_at FROM tasks ORDER BY created_at DESC"
    )
    .fetch_all(&pool)
    .await?;

    let tasks = rows
        .into_iter()
        .map(|r| Task {
            id: r.get::<String, _>(0),
            title: r.get::<String, _>(1),
            description: r.get::<Option<String>, _>(2),
            completed: r.get::<i64, _>(3) != 0,
            created_at: r.get::<i64, _>(4),
        })
        .collect();

    Ok(tasks)
}

pub async fn mark_task_completed(database_url: &str, id: &str) -> Result<()> {
    let pool = get_pool(database_url).await?;
    sqlx::query("UPDATE tasks SET completed = 1 WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await?;
    Ok(())
}

pub async fn delete_task_by_id(database_url: &str, id: &str) -> Result<()> {
    let pool = get_pool(database_url).await?;
    sqlx::query("DELETE FROM tasks WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await?;
    Ok(())
}
