use rand::{thread_rng, Rng};
use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};

use std::fmt::{Display, Formatter, Result};

const APPLE_FACTS: [&str; 3] = [
    "Apples are apples.",
    "You can eat apples.",
    "Apples are not oranges.",
];

const APPLE_TRIVIA: [AppleTrivia; 2] = [
    AppleTrivia::TrueOrFalse(("Are there yellow apples?", true)),
    AppleTrivia::Question(("What is the most common type of apple?", "Red Delicious")),
];

#[derive(Copy, Clone)]
enum AppleTrivia {
    MultipleChoice((&'static str, [&'static str; 4], u32)),
    TrueOrFalse((&'static str, bool)),
    Question((&'static str, &'static str)),
}

enum Apples {
    Red,
    Green,
}

impl Apples {
    pub fn generate_apple() -> Self {
        let mut rng = thread_rng();
        match rng.gen_range(0..1000000) {
            0..=899999 => Apples::Red,
            _ => Apples::Green,
        }
    }
}

impl Display for Apples {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            match self {
                Apples::Red => ":apple:",
                Apples::Green => ":green_apple:",
            }
        )
    }
}

#[command]
async fn apple(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, format!("{}", Apples::generate_apple()))
        .await?;
    Ok(())
}

#[command]
async fn apple_fact(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, format!("{}", random_apple_fact())).await?;
    Ok(())
}

#[command]
async fn apple_trivia(ctx: &Context, msg: &Message) -> CommandResult {
    match random_apple_trivia() {
        AppleTrivia::MultipleChoice((question, _responses, _correct)) => {
            msg.reply(ctx, format!("{}", question)).await?;
        }
        AppleTrivia::TrueOrFalse((question, is_true)) => {
            msg.reply(ctx, format!("{}", question)).await?;

            let responses = if is_true {
                ["true", "yes"]
            } else {
                ["false", "no"]
            };

            match msg
                .channel_id
                .await_reply(ctx)
                .author_id(msg.author.id)
                .timeout(std::time::Duration::new(10, 0))
                .await
            {
                Some(message) => {
                    if responses.contains(&&message.content.to_lowercase()[..]) {
                        msg.reply(
                            ctx,
                            format!("Correct, have a {}!", Apples::generate_apple()),
                        )
                        .await?;
                    } else {
                        msg.reply(ctx, format!("Incorrect, it was {}", responses[0]))
                            .await?;
                    }
                }
                None => {
                    msg.reply(ctx, format!("You ran out of time, it was {}", responses[0]))
                        .await?;
                }
            }
        }
        AppleTrivia::Question((question, response)) => {
            msg.reply(ctx, format!("{}", question)).await?;

            match msg
                .channel_id
                .await_reply(ctx)
                .author_id(msg.author.id)
                .timeout(std::time::Duration::new(10, 0))
                .await
            {
                Some(message) => {
                    if &message.content.to_lowercase()[..] == response.to_lowercase() {
                        msg.reply(
                            ctx,
                            format!("Correct, have a {}!", Apples::generate_apple()),
                        )
                        .await?;
                    } else {
                        msg.reply(
                            ctx,
                            format!("Incorrect, the correct response was: {}", response),
                        )
                        .await?;
                    }
                }
                None => {
                    msg.reply(
                        ctx,
                        format!(
                            "You ran out of time, the correct response was: {}",
                            response
                        ),
                    )
                    .await?;
                }
            }
        }
    }
    Ok(())
}

fn random_apple_fact() -> String {
    let mut rng = thread_rng();
    APPLE_FACTS[rng.gen_range(0..APPLE_FACTS.len()) as usize].to_string()
}

fn random_apple_trivia() -> AppleTrivia {
    let mut rng = thread_rng();
    APPLE_TRIVIA[rng.gen_range(0..APPLE_TRIVIA.len()) as usize]
}
