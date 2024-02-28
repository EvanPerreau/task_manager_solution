use std::fmt::{Display, Formatter};

pub enum TaskStatus {
    Todo,
    InProgress,
    Done,
}

impl Display for TaskStatus {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            TaskStatus::Todo => write!(f, "Todo"),
            TaskStatus::InProgress => write!(f, "InProgress"),
            TaskStatus::Done => write!(f, "Done"),
        }
    }
}

pub enum TaskImportance {
    Low,
    Medium,
    High,
}

impl Display for TaskImportance {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            TaskImportance::Low => write!(f, "Low"),
            TaskImportance::Medium => write!(f, "Medium"),
            TaskImportance::High => write!(f, "High"),
        }
    }
}

pub struct Task {
    pub title: String,
    pub description: String,
    pub status: TaskStatus,
    pub importance: TaskImportance,
}

pub type Tasks = Vec<Task>;