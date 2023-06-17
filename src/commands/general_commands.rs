use serenity::framework::standard::{
    Args,
    CommandResult
};
use serenity::framework::standard::macros::command;
use serenity::client::Context;
use serenity::model::prelude::Message;
use serenity::utils::MessageBuilder;
use crate::database::Database;

#[command]
async fn make_sandwich(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let response = MessageBuilder::new()
        .mention(&msg.author)
        .push_bold(" Here is your sandwich")
        .build();
    msg.channel_id.say(&ctx.http, response).await?;
    msg.channel_id.say(&ctx.http, "https://cdn.discordapp.com/attachments/744887025907400828/1118238901870543028/iu.png").await?;
    Ok(())
}


#[command]
async fn say(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let mut response: String = String::from("");
    if args.len() == 0 {
        msg.reply(&ctx.http, "Say what you dumbass?").await.expect("Cannot reply to a message: "); 
    }
    for arg in args.iter::<String>() {
        match arg {
            Ok(x) => { response.push_str(" "); response.push_str(&x) }
            Err(err) => { println!("Error while parsing arguments: {}", err) }
        }
    }
    msg.reply(&ctx.http, response).await.expect("Cannot reply to a message: "); 
    Ok(())
}

#[command]
async fn see_banned_words(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let mut response = MessageBuilder::new();
    let query;
    let guild_id;

    if let Some(id) = msg.guild_id {
        guild_id = id.to_string()
    } else {
        msg.channel_id.say(&ctx.http, "This command must be used in guild").await?;
        return Ok(());
    }

    {
        let data_read = ctx.data.write().await;
        let database_lock = data_read.get::<Database>().expect("Cannot find database in TypeMap").clone();
        let database = database_lock.write().await;

        query = sqlx::query!("SELECT * FROM banned_words where guild_id = ?", guild_id)
            .fetch_all(&*database)
            .await?;
    }
    response.push("List of banned words: ");
    for word in query.iter() {
        response.push_bold(word.banned_word.as_str());
        response.push(", ");
    }
    response.build();
    msg.channel_id.say(&ctx.http, &response).await?;
    Ok(())
}