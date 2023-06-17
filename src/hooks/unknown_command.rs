use serenity::client::Context;
use serenity::model::prelude::Message;
use serenity::utils::MessageBuilder;
use serenity::framework::standard::macros::hook;

#[hook]
pub async fn unknown_command(ctx: &Context, msg: &Message, _unknown_command_name: &str) {
    let response = MessageBuilder::new()
        .mention(&msg.author)
        .push(" stop mumbling. I don't understand.")
        .build();
    if let Err(err) = msg.reply(&ctx.http, &response).await {
        println!("Error replying to user {}: {}", msg.author.name, err);
    };
}