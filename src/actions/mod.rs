use dotenv::dotenv;
use notion::{ids::DatabaseId, models::{search::NotionSearch, Object, Database}, Error, NotionApi};

use tracing::info;


#[derive(Clone)]
pub struct NotionHandler {
    n: NotionApi,
}

enum ProjectStatus {
    Backlog,
    Planning,
    InProgress,
    Paused,
    Done,
    Canceled,
}

enum ProjectPriority {
    Low,
    Medium,
    High,
    LongTerm,
}

struct Project {
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

impl NotionHandler {
    pub fn new() -> Self {
        dotenv().ok();
        let n = NotionApi::new(dotenv::var("NOTION_SECRET").unwrap()).unwrap();
        Self { n }
    }

    pub async fn get_projects(&self, db_id: DatabaseId) -> Result<Vec<Project>, Error> {
        let ns = NotionSearch::Query("So Midwest Creative".to_string());
        let projects = self.n.search(ns).await?;

        let db_list: Vec<Database> = projects.results.into_iter().filter(|&x| x.is_database()).collect();

        if let Err(why) = Ok(db_list.len() == 0){
            info!("Error getting notion projects: {:?}", why);
        }

        let mut project_list: Vec<Project> = Vec::new();
        
        for db in db_list {
            db.properties.iter().for_each(|x| {
                 project_list.push(Project::new(
                    x.id.to_string(),
                    x.title.to_string(),
                    match x.select.as_ref().unwrap().select {
                        "Backlog" => ProjectStatus::Backlog,
                        "Planning" => ProjectStatus::Planning,
                        "In Progress" => ProjectStatus::InProgress,
                        "Paused" => ProjectStatus::Paused,
                        "Done" => ProjectStatus::Done,
                        "Canceled" => ProjectStatus::Canceled,
                        _ => ProjectStatus::Backlog,
                    },
                    match x.select.as_ref().unwrap().select {
                        "Low" => ProjectPriority::Low,
                        "Medium" => ProjectPriority::Medium,
                        "High" => ProjectPriority::High,
                        "Long Term" => ProjectPriority::LongTerm,
                        _ => ProjectPriority::Low,
                    }
                )
            );
            }
            );
        }

        Ok(project_list)
    }
}
