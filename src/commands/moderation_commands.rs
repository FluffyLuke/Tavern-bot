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
#[sub_commands(add_banned_words, remove_banned_words, see_banned_words)]
pub async fn banned_words(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let mut response = MessageBuilder::new();
    response.push_line("You need to specify what you want to do, dumbass!")
        .push_line("You can use:")
        .push("> banned_words ").push_bold_line("add {words}")
        .push("> banned_words ").push_bold_line("remove {words}")
        .push("> banned_words ").push_bold_line("show")
        .build();
    msg.channel_id.say(&ctx.http, response).await?;
    Ok(())
}

#[command]
#[aliases("add")]
#[allowed_roles("Bar Owner")]
async fn add_banned_words(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
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
#[aliases(show)]
async fn see_banned_words(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let mut response = MessageBuilder::new();
    let banned_words;
    let guild_id = msg.guild_id.unwrap().to_string();
    let guild_description;
    {
        let data_read = ctx.data.write().await;
        let database_lock = data_read.get::<Database>().expect("Cannot find database in TypeMap").clone();
        let database = &*database_lock.write().await;
        guild_description = GuildDescription::build(database, &guild_id).await?;
        banned_words = guild_description.get_banned_words();
    }
    response.push_line("List of banned words: ");
    for word in banned_words.iter() {
        response.push("> ").push_bold_line(word);
    }
    response.build();
    msg.channel_id.say(&ctx.http, &response).await?;
    Ok(())
}

#[command]
#[aliases(remove)]
#[allowed_roles("Bar Owner")]
async fn remove_banned_words(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
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

#[command]
#[sub_commands(add_moderated_role, remove_moderated_role, show_moderated_role)]
pub async fn moderated_role(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let mut response = MessageBuilder::new();
    response.push_line("You need to specify what to do, dumbass!")
        .push_line("You can use:")
        .push("> moderated_role ").push_bold_line("add {role id/role ping}")
        .push("> moderated_role ").push_bold_line("remove {role id/role ping}")
        .build();
    msg.channel_id.say(&ctx.http, response).await?;
    Ok(())
}

#[command]
#[allowed_roles("Bar Owner")]
#[aliases("add")]
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
        response.push("Guild already has one role to moderate!: ").push_bold(moderated_role).build();
        let data_read = ctx.data.read().await;
        let lock = data_read.get::<Quotes>().expect("Cannot get the lock");
        let quotes = lock.read().await;
        msg.channel_id.say(&ctx.http, response).await?;
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
#[aliases("remove")]
#[allowed_roles("Bar Owner")]
async fn remove_moderated_role(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let guild_id = msg.guild_id.unwrap().to_string();
    let mut guild_description;
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
#[aliases("show")]
pub async fn show_moderated_role(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let guild_id = msg.guild_id.unwrap().to_string();
    let mut guild_description;
    {
        let data_read = ctx.data.read().await;
        let lock = data_read.get::<Database>().expect("Cannot get the lock");
        let database = &*lock.read().await;
        guild_description = GuildDescription::build(database, &guild_id).await?;
        guild_description.delete_basic_role(database).await?;
    }
    let mut response = MessageBuilder::new();
    response.push("Moderated role: ");
    if let Some(id) = guild_description.get_moderated_role_id() {
        response.push_bold(id);
    } else {
        response.push_italic("None");
    }
    msg.channel_id.say(&ctx.http, response).await?;
    Ok(())
}

#[command]
#[allowed_roles("Bar Owner")]
#[sub_commands("add_basic_role", "remove_basic_role", "show_basic_role")]
pub async fn basic_role(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let mut response = MessageBuilder::new();
    response.push_line("You need to specify what to do, dumbass!")
        .push_line("You can use:")
        .push("> basic_role ").push_bold_line("add {role id/role ping}")
        .push("> basic_role ").push_bold_line("remove {role id/role ping}")
        .push("> basic_role ").push_bold_line("show {role id/role ping}")
        .build();
    msg.channel_id.say(&ctx.http, response).await?;
    Ok(())
}

#[command]
#[aliases("add")]
pub async fn add_basic_role(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let guild_id = msg.guild_id.unwrap().to_string();
    let mut guild_description;
    {
        let data_read = ctx.data.read().await;
        let lock = data_read.get::<Database>().expect("Cannot get the lock");
        let database = &*lock.read().await;
        guild_description = GuildDescription::build(database, &guild_id).await?;
    }

    if let Some(basic_role) = guild_description.get_basic_role_id(){
        let mut response = MessageBuilder::new();
        response.push("Guild already has basic role: ").push_bold(basic_role).build();
        msg.channel_id.say(&ctx.http, response).await?;
        let data_read = ctx.data.read().await;
        let lock = data_read.get::<Quotes>().expect("Cannot get the lock");
        let quotes = lock.read().await;
        msg.channel_id.say(&ctx.http, quotes.random_mean_quote()).await?;
        return Ok(());
    }


    if let Some(basic_role_id) = args.current() {
        let basic_role_id = strip_mention(basic_role_id);
        let parsed_basic_role_id = basic_role_id.parse::<u64>();
        if let Err(_) = parsed_basic_role_id {
            let data_read = ctx.data.read().await;
            let lock = data_read.get::<Quotes>().expect("Cannot get the lock");
            let quotes = lock.read().await;
            msg.channel_id.say(&ctx.http, "You have provided wrong id! Id cannot contain letters. You can also ping the role you want to make a basic role.").await?;
            msg.channel_id.say(&ctx.http, quotes.random_mean_quote()).await?;
            return Ok(());
        }
        {
            let data_read = ctx.data.read().await;
            let lock = data_read.get::<Database>().expect("Cannot get the lock");
            let database = &*lock.read().await;
            let str_copy_basic_role_id = parsed_basic_role_id.clone().unwrap().to_string();
            guild_description.create_basic_role(database, str_copy_basic_role_id).await?;
        }
        let parsed_basic_role_id: RoleId = RoleId::from(parsed_basic_role_id.unwrap());
        let response = MessageBuilder::new().push("Role ").mention(&parsed_basic_role_id).push(" is now a basic role.").build();
        msg.channel_id.say(&ctx.http, response).await?;
    } else {
        let data_read = ctx.data.read().await;
        let lock = data_read.get::<Quotes>().expect("Cannot get the lock");
        let quotes = lock.read().await;
        msg.channel_id.say(&ctx.http, "You should specify the basic role!").await?;
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
#[aliases("remove")]
pub async fn remove_basic_role(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let guild_id = msg.guild_id.unwrap().to_string();
    let mut guild_description;
    {
        let data_read = ctx.data.read().await;
        let lock = data_read.get::<Database>().expect("Cannot get the lock");
        let database = &*lock.read().await;
        guild_description = GuildDescription::build(database, &guild_id).await?;
        guild_description.delete_moderated_role(database).await?;
    }

    msg.channel_id.say(&ctx.http, "I removed the basic role").await?;
    {
        let data_read = ctx.data.read().await;
        let lock = data_read.get::<Quotes>().expect("Cannot get the lock");
        let quotes = lock.read().await;
        msg.channel_id.say(&ctx.http, quotes.random_neutral_quote()).await?;
    }

    Ok(())
}

#[command]
#[aliases("show")]
pub async fn show_basic_role(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let guild_id = msg.guild_id.unwrap().to_string();
    let mut guild_description;
    {
        let data_read = ctx.data.read().await;
        let lock = data_read.get::<Database>().expect("Cannot get the lock");
        let database = &*lock.read().await;
        guild_description = GuildDescription::build(database, &guild_id).await?;
        guild_description.delete_basic_role(database).await?;
    }
    let mut response = MessageBuilder::new();
    response.push("Basic role: ");
    if let Some(id) = guild_description.get_basic_role_id() {
        response.push_bold(id);
    } else {
        response.push_italic("None");
    }
    msg.channel_id.say(&ctx.http, response).await?;
    Ok(())
}