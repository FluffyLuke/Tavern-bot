use core::panic;
use std::{env, /*clone*/};
use std::collections::{HashSet, /*HashMap*/};
use std::sync::Arc;

use serenity::utils::MessageBuilder;
use serenity::{async_trait};
use serenity::http::Http;
use serenity::framework::standard::macros::{/*check,*/ command, group, /*help,*/ hook};
use serenity::model::gateway::Ready;
use serenity::prelude::*;
//use tracing::{error, info};
use serenity::model::channel::{/*Channel,*/ Message};
//use serenity::utils::{content_safe, ContentSafeOptions};
use serenity::framework::standard::{
    //help_commands,
    Args,
    //CommandGroup,
    //CommandOptions,
    CommandResult,
    //DispatchError,
    //HelpOptions,
    //Reason,
    StandardFramework,
};

use sqlx::sqlite::{SqlitePoolOptions, SqliteConnectOptions};
use sqlx::{SqlitePool, Row};



#[group]
#[owners_only]
#[commands("test_command")]
struct Owners;

#[command]
async fn test_command(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Better test deez nutz").await?;
    Ok(())
}

#[group]
#[commands("add_moderated_role", "delete_moderated_role", "add_words_to_moderate", "remove_words_to_moderate")]
#[description = "Commands for server admins/moderators"]
struct Admin;

#[command]
#[allowed_roles("Bar Owner")]
async fn add_moderated_role(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let guild_id;

    if let Some(id) = msg.guild_id {
        guild_id = id.to_string();
    } else {
        msg.channel_id.say(&ctx.http, "This command must be used in guild").await?;
        return Ok(());
    }

    //Why is this a thing?
    {
        let data_read = ctx.data.write().await;
        let database_lock = data_read.get::<Database>().expect("Cannot find database in TypeMap").clone();
        let database = database_lock.write().await;
        
        let if_guild_role_exists = sqlx::query(&format!("SELECT * FROM guilds where guild_id = '{}'", &guild_id))
            .fetch_one(&*database)
            .await;
        if let Ok(_) = if_guild_role_exists {
            msg.channel_id.say(&ctx.http, "Guild already has one role to moderate!").await?;
            return Ok(());
        }

        if let Some(role_id) = args.current() {
            if let Err(_) = role_id.parse::<u64>() {
                msg.channel_id.say(&ctx.http, "You have provided wrong id! Id cannot contain letters.").await?;
                return Ok(());
            }

            sqlx::query!("INSERT INTO guilds (guild_id, role_id) VALUES (?, ?)", guild_id, role_id)
                .execute(&*database)
                .await?;
            msg.channel_id.say(&ctx.http, format!("Role <@{}> is now under surveilnce", role_id)).await?;
        } else {
            msg.channel_id.say(&ctx.http, "You should specify a role to moderate!").await?;
            return Ok(());
        }
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
        msg.channel_id.say(&ctx.http, "This command must be used in guild").await?;
        return Ok(());
    }
    {
        let data_read = ctx.data.write().await;
        let database_lock = data_read.get::<Database>().expect("Cannot find database in TypeMap").clone();
        let database = database_lock.write().await;
        sqlx::query(&format!("DELETE FROM guilds where guild_id = '{}'", &guild_id))
            .execute(&*database)
            .await?;
        msg.channel_id.say(&ctx.http, "I stop surveilling this server").await?;
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
        msg.channel_id.say(&ctx.http, "This command must be used in guild").await?;
        return Ok(());
    }
    if args.len() == 0 {
        msg.reply(&ctx.http, "You must provide words remove from moderation, dumbass!").await?; 
    }
    {
        let data_read = ctx.data.write().await;
        let database_lock = data_read.get::<Database>().expect("Cannot find database in TypeMap").clone();
        let database = database_lock.write().await;

        for arg in args.iter::<String>() {
            if let Ok(word) = arg {
                let word_lower_case = word.to_lowercase();
                sqlx::query!("INSERT INTO banned_words (guild_id, banned_word) VALUES (?, ?)", guild_id, word_lower_case)
                    .execute(&*database)
                    .await?;
            }
        }
    }
    msg.reply(&ctx.http, "All done. This words will be deleted").await.expect("Cannot reply to a message"); 
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
        let data_read = ctx.data.write().await;
        let database_lock = data_read.get::<Database>().expect("Cannot find database in TypeMap").clone();
        let database = database_lock.write().await;

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
    Ok(())
}

#[group]
#[commands ("say", "make_sandwich", "see_banned_words")]
struct General;

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

#[hook]
async fn unknown_command(ctx: &Context, msg: &Message, _unknown_command_name: &str) {
    let response = MessageBuilder::new()
        .mention(&msg.author)
        .push(" stop mumbling. I don't understand.")
        .build();
    if let Err(err) = msg.reply(&ctx.http, &response).await {
        println!("Error replying to user {}: {}", msg.author.name, err);
    };
}


#[tokio::main]
async fn main() {

    dotenv::dotenv().expect("Failed to load .env file: ");
    let token = env::var("DISCORD_BOT_KEY").expect("Error accured while retrieving bots key: ");
    let prefix = env::var("DISCORD_BOT_PREFIX").expect("Error accured while retrieving bots prefix: ");
    let intents = GatewayIntents::all();
    tracing_subscriber::fmt::init();

    let database = SqlitePoolOptions::new()
        .max_connections(10)
        .connect_with(
            SqliteConnectOptions::new()
                .filename("naughtylist.sqlite")
                .create_if_missing(true),
        ).await.expect("Error connecting a database");
    sqlx::migrate!("./migrations").run(&database).await.expect("Couldn't run database migrations");

    let http = Http::new(&token);

    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);
            (owners, info.id)
        }
        Err(err) => {
            panic!("Error getting applications info: {}", err);
        }
    };
    let framework = StandardFramework::new()
        .configure(|c| 
            c.owners(owners)
            .prefix(prefix)
            .with_whitespace(true)
        ).group(&OWNERS_GROUP).group(&GENERAL_GROUP).group(&ADMIN_GROUP)
        .unrecognised_command(unknown_command);
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await.expect("Error accured why creating a bot instance");

    {
        let mut data = client.data.write().await;
        data.insert::<Database>(Arc::new(RwLock::new(database)));
    }

    if let Err(err) = client.start().await {
        println!("Client error: {:?}", err);
    }

}

struct Database;

impl TypeMapKey for Database {
    type Value = Arc<RwLock<SqlitePool>>;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    //On ready message
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);        
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }

        let guild_id;
        let moderated_role: String;
        let banned_words;

        if let Some(id) = msg.guild_id {
            guild_id = id;
        } else {
            panic!("Cannot retrieve Guild it ")
        }

        //Check for banned words
        {
            let data_read = ctx.data.write().await;
            let database_lock = data_read.get::<Database>().expect("Cannot find database in TypeMap").clone();
            let database = database_lock.write().await;

            let if_guild_role_exists = sqlx::query(&format!("SELECT role_id FROM guilds where guild_id = '{}'", &guild_id))
                .fetch_one(&*database)
                .await;
            match if_guild_role_exists {
                Ok(row) => { 
                    moderated_role = row.get("role_id");
                    banned_words = sqlx::query(&format!("SELECT banned_word FROM banned_words where guild_id = '{}'", &guild_id))
                        .fetch_all(&*database)
                        .await;
                }
                Err(err) => { 
                    banned_words = Err(err);
                    moderated_role = "".to_string();
                }
            }
        }

        if let Ok(rows) = banned_words {
            if let Ok(_) = msg.author.has_role(&ctx.http, guild_id, moderated_role.parse::<u64>().expect("cannot parse the id")).await {
                for row in rows.iter() {
                    let word: String = row.get("banned_word");
                    if msg.content.contains(&word) {
                        msg.delete(&ctx.http).await.expect("Cannot delete a message");
                        let response = MessageBuilder::new().mention(&msg.author).push(" you cannot say ").push_italic(word).build();
                        msg.author.dm(&ctx.http, |m| m.content(&response) ).await.expect("Cannot send message");
                    }
                }
            }
        }
    }
}


