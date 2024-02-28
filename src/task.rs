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

impl TaskStatus {
    pub fn from_str(s: &str) -> TaskStatus {
        match s {
            "todo" => TaskStatus::Todo,
            "inprogress" => TaskStatus::InProgress,
            "done" => TaskStatus::Done,
            _ => TaskStatus::Todo,
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

impl TaskImportance {
    pub fn from_str(s: &str) -> TaskImportance {
        match s {
            "low" => TaskImportance::Low,
            "medium" => TaskImportance::Medium,
            "high" => TaskImportance::High,
            _ => TaskImportance::Low,
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