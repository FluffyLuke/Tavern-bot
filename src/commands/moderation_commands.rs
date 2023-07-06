use serenity::framework::standard::{
    Args,
    CommandResult
};
use serenity::framework::standard::macros::command;
use serenity::client::Context;
use serenity::model::prelude::Message;
use crate::database::{Database};
use crate::quotes::Quotes;
use crate::guild::{GuildDescription, strip_mention};
use serenity::utils::MessageBuilder;
use serenity::model::prelude::RoleId;



#[command]
#[only_in(guilds)]
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
#[command]
#[only_in(guilds)]
#[allowed_roles("Bar Owner")]
pub async fn add_moderated_role(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let guild_id = msg.guild_id.unwrap().to_string();

    let mut guild_description;
    {
        let data_read = ctx.data.read().await;
        let lock = data_read.get::<Database>().expect("Cannot get the lock");
        let database = &*lock.read().await;
        guild_description = GuildDescription::build(database, &guild_id).await?;
    }

    //If there is a moderated role then return
    if let Some(moderated_role) = guild_description.get_moderated_role_id(){
        let mut response = MessageBuilder::new();
        response.push("Guild already has one role to moderate!: ");
        let data_read = ctx.data.read().await;
        let lock = data_read.get::<Quotes>().expect("Cannot get the lock");
        let quotes = lock.read().await;
        msg.channel_id.say(&ctx.http, format!("Guild already has one role to moderate!: {moderated_role}")).await?;
        msg.channel_id.say(&ctx.http, quotes.random_mean_quote()).await?;
        return Ok(());
    }


    if let Some(moderated_role_id) = args.current() {
        let moderated_role_id = strip_mention(moderated_role_id);
        let parsed_moderated_role_id = moderated_role_id.parse::<u64>();
        if let Err(_) = parsed_moderated_role_id {
            let data_read = ctx.data.read().await;
            let lock = data_read.get::<Quotes>().expect("Cannot get the lock");
            let quotes = lock.read().await;
            msg.channel_id.say(&ctx.http, "You have provided wrong id! Id cannot contain letters. You can also ping the role you want to be moderated.").await?;
            msg.channel_id.say(&ctx.http, quotes.random_mean_quote()).await?;
            return Ok(());
        }
        {
            let data_read = ctx.data.read().await;
            let lock = data_read.get::<Database>().expect("Cannot get the lock");
            let database = &*lock.read().await;
            let str_copy_moderated_role_id = parsed_moderated_role_id.clone().unwrap().to_string();
            guild_description.create_moderated_role(database, str_copy_moderated_role_id).await?;
        }
        let parsed_moderated_role_id: RoleId = RoleId::from(parsed_moderated_role_id.unwrap());
        let response = MessageBuilder::new().push("Role ").mention(&parsed_moderated_role_id).push(" is now under surveilence").build();
        msg.channel_id.say(&ctx.http, response).await?;
    } else {
        let data_read = ctx.data.read().await;
        let lock = data_read.get::<Quotes>().expect("Cannot get the lock");
        let quotes = lock.read().await;
        msg.channel_id.say(&ctx.http, "You should specify role to moderate!").await?;
        msg.channel_id.say(&ctx.http, quotes.random_mean_quote()).await?;
        return Ok(());
    }
    
    {
        let data_read = ctx.data.read().await;
        let lock = data_read.get::<Quotes>().expect("Cannot get the lock");
        let quotes = lock.read().await;
        msg.channel_id.say(&ctx.http, quotes.random_neutral_quote()).await?;
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
#[allowed_roles("Bar Owner")]
async fn delete_moderated_role(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let guild_id = msg.guild_id.unwrap().to_string();
    let guild_description;
    {
        let data_read = ctx.data.read().await;
        let lock = data_read.get::<Database>().expect("Cannot get the lock");
        let database = &*lock.read().await;
        guild_description = GuildDescription::build(database, &guild_id).await?;
        guild_description.delete_moderated_role(database).await?;
    }

    msg.channel_id.say(&ctx.http, "I stop surveilling this server").await?;
    {
        let data_read = ctx.data.read().await;
        let lock = data_read.get::<Quotes>().expect("Cannot get the lock");
        let quotes = lock.read().await;
        msg.channel_id.say(&ctx.http, quotes.random_neutral_quote()).await?;
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
#[allowed_roles("Bar Owner")]
async fn add_words_to_moderate(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let guild_id = msg.guild_id.unwrap().to_string();
    let mut guild_description;
    if args.len() == 0 {
        msg.reply(&ctx.http, "You must provide words remove from moderation, dumbass!").await?;
        return Ok(())
    }

    {
        let data_read = ctx.data.read().await;
        let lock = data_read.get::<Database>().expect("Cannot get the lock");
        let database = &*lock.read().await;
        guild_description = GuildDescription::build(database, &guild_id).await?;
        for arg in args.iter::<String>() {
            if let Ok(word) = arg {
                let word_lower_case = word.to_lowercase();
                guild_description.add_word_to_moderate(database, word_lower_case).await?;
            }
        }
    }
    msg.reply(&ctx.http, "All done. This words will be deleted").await?; 
    {    
        let data_read = ctx.data.read().await;
        let lock = data_read.get::<Quotes>().expect("Cannot get the lock");
        let quotes = lock.read().await;
        msg.channel_id.say(&ctx.http, quotes.random_neutral_quote()).await?; 
    }
    Ok(())
}
#[command]
#[only_in(guilds)]
#[allowed_roles("Bar Owner")]
async fn remove_words_to_moderate(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let guild_id = msg.guild_id.unwrap().to_string();
    let mut guild_description;
    if args.len() == 0 {
        msg.reply(&ctx.http, "You must provide words to moderate, dumbass!").await?; 
        return Ok(());
    }
    {
        let data_read = ctx.data.read().await;
        let lock = data_read.get::<Database>().expect("Cannot get the lock");
        let database = &*lock.read().await;
        guild_description = GuildDescription::build(database, &guild_id).await?;
        for arg in args.iter::<String>() {
            if let Ok(word) = arg {
                let word_lower_case = word.to_lowercase();
                guild_description.remove_word_from_moderation(database, &word_lower_case).await?;
            }
        }  
    }
    msg.channel_id.say(&ctx.http, "All done. Deleted these words from moderation").await?;
    {
        let data_read = ctx.data.read().await;
        let lock = data_read.get::<Quotes>().expect("Cannot get the lock");
        let quotes = lock.read().await;
        msg.channel_id.say(&ctx.http, quotes.random_neutral_quote()).await?;
    }
    Ok(())
}