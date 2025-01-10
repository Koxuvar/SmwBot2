
#[derive(Clone)]
pub enum ProjectStatus {
    Backlog,
    Planning,
    InProgress,
    Paused,
    Done,
    Canceled,
}

#[derive(Clone)]
pub enum ProjectPriority {
    Low,
    Medium,
    High,
    LongTerm,
}

#[derive(Clone)]
pub struct Project {
    id: String,
    pub name: String,
    status: ProjectStatus,
    priority: ProjectPriority,
}

impl Project {
    pub fn create(id: String, name: String, status: ProjectStatus, priority: ProjectPriority) -> Self {
        Self {
            id,
            name,
            status,
            priority,
        }
    }
}
