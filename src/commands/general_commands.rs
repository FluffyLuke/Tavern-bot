use serenity::framework::standard::{
    Args,
    CommandResult
};
use serenity::framework::standard::macros::command;
use serenity::client::Context;
use serenity::model::prelude::Message;
use serenity::utils::MessageBuilder;
use crate::quotes::Quotes;
use crate::database::CommandDescriptions;

#[command]
async fn make_sandwich(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let response = MessageBuilder::new()
        .mention(&msg.author)
        .push_bold(" Here is your sandwich")
        .build();
    msg.channel_id.say(&ctx.http, response).await?;
    msg.channel_id.say(&ctx.http, "https://cdn.discordapp.com/attachments/744887025907400828/1118238901870543028/iu.png").await?;
    {
        let data_read = ctx.data.read().await;
        let quote_lock = data_read.get::<Quotes>().expect("Cannot get quote lock");
        let quotes = quote_lock.read().await;
        msg.channel_id.say(&ctx.http, quotes.random_pleasant_quote()).await?;
    }
    Ok(())
}


#[command]
async fn say(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let mut response: String = String::from("");
    if args.len() == 0 {
        let data_read = ctx.data.read().await;
        let quote_lock = data_read.get::<Quotes>().expect("Cannot get quote lock");
        let quotes = quote_lock.read().await;
        msg.reply(&ctx.http, "Say what you dumbass?").await?;
        msg.channel_id.say(&ctx.http, quotes.random_mean_quote()).await?;
    }
    for arg in args.iter::<String>() {
        match arg {
            Ok(x) => { response.push_str(" "); response.push_str(&x) }
            Err(err) => { println!("Error while parsing arguments: {}", err) }
        }
    }
    msg.reply(&ctx.http, response).await?; 
    Ok(())
}

#[command]
async fn help(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let mut response = MessageBuilder::new();
    response.push_line("These are command descriptions: ");
    {
        let data_read = ctx.data.read().await;
        let descriptions_lock = data_read.get::<CommandDescriptions>().expect("Cannot get command descriptions lock");
        let command_descriptions = &*descriptions_lock.read().await;
        for (key, value) in &command_descriptions.descriptions {
            response.push(key).push(": ").push_bold_line(value);
        }
    }
    let response = response.build();
    msg.channel_id.say(&ctx.http, response).await?;
    Ok(())
}
