//@supermaven how to import functions from actions?

use crate::{Context, Error};
use crate::actions::NotionHandler;



#[poise::command(slash_command)]
pub async fn get_projects(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Sure thing! Here is a list of projects that are active on Notion.").await?;
    let n = NotionHandler::new();
    let db_list = n.get_database().await?;


    for (i, db) in db_list.iter().enumerate() {
        ctx.say(format!("{}. {}", i, db.name)).await?;
    }

    Ok(())
}
