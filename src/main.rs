use core::panic;
use std::{env, /*clone*/};
use std::collections::{HashSet, /*HashMap*/};
use std::sync::Arc;
use std::fs;

use serenity::utils::MessageBuilder;
use serenity::{async_trait};
use serenity::http::Http;
use serenity::framework::standard::macros::{/*check,*/ /*command,*/ group, /*help*/};
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use serenity::model::channel::{/*Channel,*/ Message};
use serenity::framework::standard::StandardFramework;
use sqlx::Row;
use sqlx::sqlite::{SqlitePoolOptions, SqliteConnectOptions};

mod commands;
mod database;
mod hooks;
mod quotes;
use crate::quotes::Quotes;
use crate::commands::{general_commands::*, test_commands::*, moderation_commands::*};
use crate::database::Database;
use crate::hooks::unknown_command::unknown_command;


#[group]
#[owners_only]
#[commands("test_command", "test_quotes")]
struct Owners;

#[group]
#[commands("add_moderated_role", "delete_moderated_role", "add_words_to_moderate", "remove_words_to_moderate")]
#[description = "Commands for server admins/moderators"]
struct Admin;

#[group]
#[commands ("say", "make_sandwich", "see_banned_words")]
struct General;

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env file. You should create .env file with prefix, token and database url variables");
    let token = env::var("DISCORD_BOT_KEY").expect("Bot's key in not defined in environmental variables");
    let prefix = env::var("DISCORD_BOT_PREFIX").expect("Bot's prefix is not defined in environmental variables");
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
        .await.expect("Error when creating a bot instance");


    let mean_quotes: Vec<String> = fs::read_to_string("./barman_quotes/mean_quotes.txt")
            .expect("Cannot read mean quotes")
            .lines()
            .map(String::from)
            .collect();
    let neutral_quotes: Vec<String> = fs::read_to_string("./barman_quotes/neutral_quotes.txt")
        .expect("Cannot read neutral quotes")
        .lines()
        .map(String::from)
        .collect();
    let pleasant_quotes: Vec<String> = fs::read_to_string("./barman_quotes/pleasant_quotes.txt")
            .expect("Cannot read pleasant quotes")
            .lines()
            .map(String::from)
            .collect();

    let quotes = Quotes::new(mean_quotes, neutral_quotes, pleasant_quotes);
    println!("{}", quotes.random_neutral_quote());
    {
        let mut data = client.data.write().await;
        data.insert::<Database>(Arc::new(RwLock::new(database)));
        data.insert::<Quotes>(Arc::new(RwLock::new(quotes)))
    }

    if let Err(err) = client.start().await {
        println!("Client error: {:?}", err);
    }

}
struct Handler;
#[async_trait]
impl EventHandler for Handler {
    //On ready message
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);        
    }

    async fn message(&self, ctx: Context, mut msg: Message) {
        msg.content = msg.content.to_lowercase();
        msg.content = msg.content.replace(" ", "");
        if msg.author.bot {
            return;
        }
        let moderated_role: String;
        let banned_words;

        //Check for banned words
        if let Some(guild_id) = msg.guild_id {
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
                if let Ok(true) = msg.author.has_role(&ctx.http, guild_id, moderated_role.parse::<u64>().expect("Cannot parse the id")).await {
                    for row in rows.iter() {
                        let banned_word: String = row.get("banned_word");
                        if msg.content.contains(&banned_word) {
                            msg.delete(&ctx.http).await.expect("Cannot delete a message");
                            let response = MessageBuilder::new().mention(&msg.author).push(" you cannot say ").push_italic(banned_word).build();
                            msg.author.dm(&ctx.http, |m| m.content(&response) ).await.expect("Cannot send message");
                        }
                    }
                }
            }
        }
    }
}


