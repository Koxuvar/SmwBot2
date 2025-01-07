mod project;
use project::*;

use dotenv::dotenv;
use notion::{ids::DatabaseId, models::{search::NotionSearch, Object, Database}, Error, NotionApi};

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

    pub async fn get_projects(&self, db_id: DatabaseId) -> Result<Vec<Project>, Error> {
        let ns = NotionSearch::Query("So Midwest Creative".to_string());
        let projects = self.n.search(ns).await?;

        let db_list: Vec<Database> = projects.results.into_iter().filter(|&x| x.is_database()).collect();

        if let Err(why) = Ok(db_list.len() == 0){
            info!("Error getting notion projects: {:?}", why);
        }

        let mut project_list: Vec<Project> = Vec::new();
        
        for db in db_list {
            //TODO: Split apart each database into its own struct
        }

        Ok(project_list)
    }
}
