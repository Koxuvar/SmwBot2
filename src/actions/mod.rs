mod project;
use project::Project;
use project::ProjectStatus;
use project::ProjectPriority;

use dotenv::dotenv;
use notion::{ids::DatabaseId, models::{search::{FilterProperty, FilterValue, NotionSearch}, Database, Object}, Error, NotionApi};

use tracing::info;


#[derive(Clone)]
pub struct NotionHandler {
    n: NotionApi,
}


impl NotionHandler {
    pub fn new() -> Self {
        dotenv().ok();
        let n = NotionApi::new(dotenv::var("NOTION_SECRET").unwrap()).unwrap();
        Self { n }
    }

    pub async fn get_database(&self) -> Result<Vec<Project>, Error> {

        let other_ns = NotionSearch::Filter{
            property: FilterProperty::Object,
            value: FilterValue::Database,
        };

        let projects = self.n.search(other_ns).await?;

        let db:Vec<&Database> = projects
            .results()
            .iter()
            .filter_map(|o| match o {
                Object::Database {database} => Some(database),
                _=> None,
            })
            .collect();

        let mut project_list: Vec<Project> = Vec::new();
        

        info!("Found {:?} databases!", db.len());

        for d in db {
            info!("Adding {} to the database list!", d.title_plain_text());

            let p = Project::create(
                d.id.to_string(),
                d.title_plain_text(),
                ProjectStatus::Planning,
                ProjectPriority::Medium,
            );

            project_list.push(p);
        }

        Ok(project_list)
    }



    
}
