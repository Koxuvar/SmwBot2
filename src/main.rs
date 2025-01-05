#![allow(dead_code)]

mod config;
mod utils;
mod commands;
mod listeners;
mod constants;

use listeners::handler::Handler;
use constants::REQWEST_USER_AGENT;
use config::ConfigurationData;
use dotenv::dotenv;
use std::env;

use poise::{serenity_prelude as serenity, Framework, FrameworkOptions};
use reqwest::{redirect::Policy, Client};
use serenity::GatewayIntents;
use tracing::{info, Level};
use tracing_log::LogTracer;
use tracing_subscriber::{FmtSubscriber, EnvFilter};
use utils::read_config;

type Error = anyhow::Error;
type Context<'a> = poise::Context<'a, Data, Error>;


struct Data {
    config: ConfigurationData, 
    reqwest_container: Client,
}

#[tokio::main]
async fn main() -> Result<(), Error>{

    let configuration = read_config("config.toml");

    LogTracer::init()?;

    let mut level: Level = Level::INFO;

    if configuration.bot.logging.enabled {
        level = match configuration.bot.logging.level.as_str() {
            "error" => Level::ERROR,
            "warn"  => Level::WARN,
            "info"  => Level::INFO,
            "debug" => Level::DEBUG,
            _       => Level::TRACE,
        };
    }

    let subscriber = FmtSubscriber::builder()
        .with_target(false)
        .with_max_level(level)
        .with_env_filter(EnvFilter::from_default_env())
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    info!("Tracing initialized with logging level set to {}.", "info".to_string());
    

    dotenv().ok();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    
    let framework = Framework::builder()
        .options(FrameworkOptions {
            commands: vec![
                commands::utilities::hello(),
                //TODO: put commands here
            ],
        ..Default::default()
    })
    .setup(move |context, _ready, framework| {
        Box::pin(async move {
            poise::builtins::register_globally(context, &framework.options().commands).await?;
            Ok(Data {
                config: read_config("config.toml"),
                reqwest_container: Client::builder().user_agent(REQWEST_USER_AGENT).redirect(Policy::none()).build()?
            })
        })
    })
    .build();


    let command_count = &framework.options().commands.len();
    let commands_str: String = framework.options().commands.iter().map(|c| &c.name).cloned().collect::<Vec<String>>().join(", ");
    info!("Initialized {} commands: {}", command_count, commands_str);

    let mut client = serenity::Client::builder(token, GatewayIntents::all()).event_handler(Handler).framework(framework).await?;
    if let Err(why) = client.start_autosharded().await {
        eprintln!("An error occurred while running the client: {why:?}");
    }

    Ok(())
    
}
