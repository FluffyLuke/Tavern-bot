use serenity::framework::standard::{
    Args,
    CommandResult
};
use serenity::framework::standard::macros::command;
use serenity::client::Context;
use serenity::model::prelude::Message;
use crate::database::Database;
use crate::quotes::Quotes;
use crate::ctx_get_lock;
#[command]
#[allowed_roles("Bar Owner")]
pub async fn create_server_profile(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    
    Ok(())
}

#[command]
#[allowed_roles("Bar Owner")]
pub async fn add_moderated_role(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let guild_id;

    if let Some(id) = msg.guild_id {
        guild_id = id.to_string();
    } else {
        let quotes = ctx_get_lock!(&ctx, Quotes, Mode::Read);
        msg.channel_id.say(&ctx.http, "This command must be used in guild").await?;
        msg.channel_id.say(&ctx.http, quotes.random_mean_quote()).await?;
        return Ok(());
    }

    let if_guild_role_exists;
    {
        let database = ctx_get_lock!(&ctx, Database, Mode::Write);
        if_guild_role_exists = sqlx::query!("SELECT * FROM guild where guild_id = ?", &guild_id)
            .fetch_one(&*database)
            .await;
    }
    if let Ok(_) = if_guild_role_exists {
        let quotes = ctx_get_lock!(&ctx, Quotes, Mode::Read);
        msg.channel_id.say(&ctx.http, "Guild already has one role to moderate!").await?;
        msg.channel_id.say(&ctx.http, quotes.random_mean_quote()).await?;
        return Ok(());
    }

    if let Some(role_id) = args.current() {
        if let Err(_) = role_id.parse::<u64>() {
            let quotes = ctx_get_lock!(&ctx, Quotes, Mode::Read);
            msg.channel_id.say(&ctx.http, "You have provided wrong id! Id cannot contain letters.").await?;
            msg.channel_id.say(&ctx.http, quotes.random_mean_quote()).await?;
            return Ok(());
        }
        {
            let database = ctx_get_lock!(&ctx, Database, Mode::Write);
            sqlx::query!("INSERT INTO guild (guild_id, role_id) VALUES (?, ?)", guild_id, role_id)
            .execute(&*database)
            .await?;
        }
        msg.channel_id.say(&ctx.http, format!("Role <@{}> is now under surveilnce", role_id)).await?;
    } else {
        let quotes = ctx_get_lock!(&ctx, Quotes, Mode::Read);
        msg.channel_id.say(&ctx.http, "You should specify a role to moderate!").await?;
        msg.channel_id.say(&ctx.http, quotes.random_mean_quote()).await?;
        return Ok(());
    }
    
    {
        let quotes = ctx_get_lock!(&ctx, Quotes, Mode::Read);
        msg.channel_id.say(&ctx.http, quotes.random_neutral_quote()).await?;
    }

    Ok(())
}

#[command]
#[allowed_roles("Bar Owner")]
async fn delete_moderated_role(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let guild_id;

    if let Some(id) = msg.guild_id {
        guild_id = id.to_string();
    } else {
        let quotes = ctx_get_lock!(&ctx, Quotes, Mode::Read);
        msg.channel_id.say(&ctx.http, "This command must be used in guild").await?;
        msg.channel_id.say(&ctx.http, quotes.random_mean_quote()).await?;
        return Ok(());
    }
    
    {
        let database = ctx_get_lock!(&ctx, Database, Mode::Write);
        sqlx::query(&format!("DELETE FROM moderated_role where guild_id = '{}'", &guild_id))
            .execute(&*database)
            .await?;
    }

    msg.channel_id.say(&ctx.http, "I stop surveilling this server").await?;
    {
        let quotes = ctx_get_lock!(&ctx, Quotes, Mode::Read);
        msg.channel_id.say(&ctx.http, quotes.random_neutral_quote()).await?;
    }

    Ok(())
}

#[command]
#[allowed_roles("Bar Owner")]
async fn add_words_to_moderate(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let guild_id;
    if let Some(id) = msg.guild_id {
        guild_id = id.to_string();
    } else {
        let quotes = ctx_get_lock!(&ctx, Quotes, Mode::Read);
        msg.channel_id.say(&ctx.http, "This command must be used in guild").await?;
        msg.channel_id.say(&ctx.http, quotes.random_neutral_quote()).await?;
        return Ok(());
    }

    if args.len() == 0 {
        msg.reply(&ctx.http, "You must provide words remove from moderation, dumbass!").await?;
        return Ok(())
    }

    {
        let database = ctx_get_lock!(&ctx, Database, Mode::Write);
        for arg in args.iter::<String>() {
            if let Ok(word) = arg {
                let word_lower_case = word.to_lowercase();
                sqlx::query!("INSERT INTO banned_words (guild_id, banned_word) VALUES (?, ?)", guild_id, word_lower_case)
                    .execute(&*database)
                    .await?;
            }
        }
    }
    msg.reply(&ctx.http, "All done. This words will be deleted").await?; 
    {    
        let quotes = ctx_get_lock!(&ctx, Quotes, Mode::Read);
        msg.channel_id.say(&ctx.http, quotes.random_neutral_quote()).await?; 
    }
    Ok(())
}
#[command]
#[allowed_roles("Bar Owner")]
async fn remove_words_to_moderate(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let guild_id;
    if let Some(id) = msg.guild_id {
        guild_id = id.to_string();
    } else {
        msg.channel_id.say(&ctx.http, "This command must be used in guild").await?;
        return Ok(());
    }
    if args.len() == 0 {
        msg.reply(&ctx.http, "You must provide words to moderate, dumbass!").await?; 
        return Ok(());
    }
    {
        let database = ctx_get_lock!(&ctx, Database, Mode::Write);
        for arg in args.iter::<String>() {
            if let Ok(word) = arg {
                let word_lower_case = word.to_lowercase();
                sqlx::query!("DELETE FROM banned_words WHERE guild_id = ? and banned_word = ?", guild_id, word_lower_case)
                    .execute(&*database)
                    .await?;
            }
        }  
    }
    msg.channel_id.say(&ctx.http, "All done. Deleted these words from moderation").await?;
    {
        let quotes = ctx_get_lock!(&ctx, Quotes, Mode::Read);
        msg.channel_id.say(&ctx.http, quotes.random_neutral_quote()).await?;
    }
    Ok(())
}