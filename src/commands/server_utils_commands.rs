use serenity::framework::standard::{
    Args,
    CommandResult
};
use serenity::framework::standard::macros::command;
use serenity::client::Context;
use serenity::model::prelude::Message;


use crate::database::{Database};
use crate::guild::{GuildDescription};
use serenity::utils::MessageBuilder;

#[command]
#[sub_commands(describe_server)]
pub async fn server(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let mut response = MessageBuilder::new();
    response.push_line("You need to specify what you want to do, dumbass!")
        .push_line("You can use:")
        .push("> server ").push_bold_line("describe")
        .build();
    msg.channel_id.say(&ctx.http, response).await?;
    Ok(())
}

#[command]
#[aliases(describe)]
pub async fn describe_server(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let guild_id = msg.guild_id.unwrap().to_string();

    let guild_description;
    {
        let data_read = ctx.data.read().await;
        let lock = data_read.get::<Database>().expect("Cannot get the lock");
        let database = lock.read().await;
        guild_description = GuildDescription::build(&*database, &guild_id).await?;
    }
    msg.channel_id.say(&ctx.http, guild_description.guild_description_msg()).await?;
    Ok(())
}

