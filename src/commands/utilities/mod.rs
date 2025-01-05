use crate::{Context, Error};



#[poise::command(slash_command)]
pub async fn hello(ctx:Context<'_>) -> Result<(), Error> {
    ctx.say(format!("Hello {}!", ctx.author().name)).await?;
    Ok(())
}
