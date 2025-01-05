pub enum ProjectStatus {
    Backlog,
    Planning,
    InProgress,
    Paused,
    Done,
    Canceled,
}

pub enum ProjectPriority {
    Low,
    Medium,
    High,
    LongTerm,
}
pub struct Project {
    id: String,
    name: String,
    status: ProjectStatus,
    priority: ProjectPriority,
}

impl Project {
    pub fn new(id: String, name: String, status: ProjectStatus, priority: ProjectPriority) -> Self {
        Self {
            id,
            name,
            status,
            priority,
        }
    }
}
