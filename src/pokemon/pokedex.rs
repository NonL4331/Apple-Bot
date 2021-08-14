
use serenity::client::{Context};
use serenity::model::channel::Message;
use serenity::framework::standard::{
    CommandResult,
    macros::{
        command
    }
};

use rand::seq::IteratorRandom;

use regex::Regex;
#[command]
async fn get_random_pokemon(ctx: &Context, msg: &Message) -> CommandResult {
    let pokemon_html = reqwest::get("https://www.pokemon.com/us/pokedex/")
        .await.unwrap()
        .text()
        .await.unwrap();

    let re = Regex::new(r#"    <li><a href="/us/pokedex/.+">([0-9]+) - (.+)</a></li>"#).unwrap();


    let poke_iter = re.captures_iter(&pokemon_html);

    let pokemon = poke_iter.choose(&mut rand::thread_rng()).unwrap();

    msg.reply(ctx, format!("https://assets.pokemon.com/assets/cms2/img/pokedex/full/{}.png", {&pokemon[1]})).await?;
    
    
    Ok(())
}

/*#[command]
async fn guess_random_pokemon(ctx: &Context, msg: &Message) -> CommandResult {
    let channel = match msg.channel().unwrap() {
        Guild(channel) => { channel },
        Private(_) => { msg.reply(ctx, "This command can only be used in guild channels!"); return Ok(()); },
        Category(_) => { msg.reply(ctx, "This command can only be used in guild channels!"); return Ok(()); }
    };


    let pokemon_html = reqwest::get("https://www.pokemon.com/us/pokedex/")
        .await.unwrap()
        .text()
        .await.unwrap();

    let re = Regex::new(r#"    <li><a href="/us/pokedex/.+">([0-9]+) - (.+)</a></li>"#).unwrap();

    let poke_iter = re.captures_iter(&pokemon_html);

    let pokemon = poke_iter.choose(&mut rand::thread_rng()).unwrap();

    msg.reply(ctx, format!("https://assets.pokemon.com/assets/cms2/img/pokedex/full/{}.png", {&pokemon[1]})).await?;

    let timestamp = msg.timestamp.timestamp();

    let message_id = msg.id;

    while msg.id - channel.last_message_id.unwrap() <= 30 {

    }
}*/