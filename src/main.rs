use crate::{admin::*, apples::*, pokemon::GET_RANDOM_POKEMON_COMMAND};
use serenity::{
    async_trait,
    client::{Client, Context, EventHandler},
    framework::standard::{
        macros::{command, group},
        CommandResult, StandardFramework,
    },
    model::channel::Message,
};

use std::env;

mod admin;
mod apples;
mod pokemon;

#[group]
#[commands(apple_fact, get_random_pokemon, ping, apple_trivia, set_activity)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~"))
        .group(&GENERAL_GROUP);

    let token = env::var("DISCORD_TOKEN").expect("token");
    let application_id = env::var("DISCORD_APPLICATION_ID")
        .expect("application id")
        .parse()
        .unwrap();
    let mut client = Client::builder(token)
        .application_id(application_id)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(e) = client.start().await {
        println!("An error occurred while running the client: {:?}", e);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}
