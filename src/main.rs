mod commands;

use std::env;
use serenity::{all::Interaction, async_trait, model::{application::Command, gateway::Ready}};
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is online!", ready.user.name);

        let global_command = Command::create_global_command(&ctx.http, commands::ping::register()).await;

        println!("I created the following global slash command: {global_command:#?}");
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            match command.data.name.as_str() {
                "ping" => {
                    commands::ping::execute(&ctx, &command).await;
                }
                _ => {}
            }
        }
    }
}

#[tokio::main]
async fn main() {
    // Login with a bot token from the environment
    dotenv::dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES;

    // Create a new instance of the Client, logging in as a bot.
    let mut client = Client::builder(&token, intents).event_handler(Handler).await.expect("Failed to create client");

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("Failed to initialize client: {why:?}");
    }
}
