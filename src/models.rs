use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
    pub created_at: i64,
}

impl Task {
    pub fn new(title: String, description: Option<String>) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            title,
            description,
            completed: false,
            created_at: chrono::Utc::now().timestamp(),
        }
    }
}

