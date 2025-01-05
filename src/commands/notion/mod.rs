use crate::{Context, Error};
use notion::NotionApi;
use dotenv::dotenv;
use std::env;

#[poise::command(slash_command)]
pub async fn get_projects(ctx: Context<'_>) -> Result<(), Error> {
    dotenv().ok();
    
    let n = NotionApi::new(env::var("NOTION_SECRET").unwrap())?;


    n.get_database("");




    Ok(())
}
