use serenity::framework::standard::{
    Args,
    CommandResult
};
use serenity::framework::standard::macros::command;
use serenity::client::Context;
use serenity::model::prelude::Message;
use crate::quotes::Quotes;

#[command]
async fn test_command(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Better test deez nutz").await?;
    Ok(())
}

#[command]
async fn test_quotes(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    {
        let data_read = ctx.data.read().await;
        let quote_lock = data_read.get::<Quotes>().expect("Cannot get quote lock");
        let quotes = quote_lock.read().await;

        msg.channel_id.say(&ctx.http, quotes.random_mean_quote()).await?;
        msg.channel_id.say(&ctx.http, quotes.random_neutral_quote()).await?;
        msg.channel_id.say(&ctx.http, quotes.random_pleasant_quote()).await?;
    }
    Ok(())
}