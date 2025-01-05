use serenity::{
    all::{ActivityData, Context, EventHandler, OnlineStatus, Ready},
    async_trait
};
use tracing::info;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, rdy: Ready) {
        let http = &ctx.http;

        let version = rdy.version;
        let gateway = http.get_bot_gateway().await.unwrap();
        let owner = http.get_current_application_info().await.unwrap().owner.expect("Could not get owner!");
        let total = gateway.session_start_limit.total;
        let remaining = gateway.session_start_limit.remaining;

        info!("Successfully logged into the Doscord API as the following user:");
        info!("Bot Details: {} (User ID: {})", rdy.user.tag(), rdy.user.id);
        info!("Bot owner: {} (User ID: {})", owner.tag(), owner.id.to_string());

        let guilds = rdy.guilds.len();

        info!("Connected to the Discord API (version {version}) with {remaining}/{total} sessions remaining.");
        info!("Connected to and serving a total of {guilds} guild(s).");

        ctx.set_presence(Some(ActivityData::playing(format!("on {guilds} guilds"))), OnlineStatus::Online);
    }
}
