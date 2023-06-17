use serenity::framework::standard::{
    Args,
    CommandResult
};
use serenity::framework::standard::macros::command;
use serenity::client::Context;
use serenity::model::prelude::Message;

#[command]
async fn test_command(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Better test deez nutz").await?;
    Ok(())
}