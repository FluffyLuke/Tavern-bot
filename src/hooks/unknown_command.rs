use serenity::client::Context;
use serenity::model::prelude::Message;
use serenity::utils::MessageBuilder;
use serenity::framework::standard::macros::hook;
use crate::quotes::Quotes;
#[hook]
pub async fn unknown_command(ctx: &Context, msg: &Message, _unknown_command_name: &str) {
    let response = MessageBuilder::new()
        .mention(&msg.author)
        .push(" stop mumbling. I don't understand.")
        .build();
    let data_read = ctx.data.read().await;
    let quote_lock = data_read.get::<Quotes>().expect("Cannot get quote lock");
    let quotes = quote_lock.read().await;
    if let Err(err) = msg.reply(&ctx.http, &response).await {
        println!("{}",err)
    } 
    if let Err(err) = msg.channel_id.say(&ctx.http, quotes.random_mean_quote()).await {
        println!("{}", err)
    }
}