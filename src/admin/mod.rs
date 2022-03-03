use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::{channel::Message, gateway::Activity},
};

#[command]
async fn set_activity(ctx: &Context, msg: &Message) -> CommandResult {
    let message = msg.content.clone();
    if message.len() < 15 {
        return Ok(());
    }
    let activity = Activity::competing(&message[14..]);
    ctx.set_activity(activity).await;
    Ok(())
}
