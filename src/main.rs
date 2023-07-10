use core::panic;
use std::{env, /*clone*/};
use std::collections::HashSet;
use std::sync::Arc;
use std::fs;

use guild::GuildDescription;
use serenity::utils::MessageBuilder;
use serenity::async_trait;
use serenity::http::Http;
use serenity::framework::standard::macros::{/*check,*/ /*command,*/ group, /*help*/};
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use serenity::model::channel::{/*Channel,*/ Message};
use serenity::framework::standard::StandardFramework;
use sqlx::sqlite::{SqlitePoolOptions, SqliteConnectOptions};
// use sqlx::Row;
use serenity::model::prelude::{Member, ChannelId};

mod commands;
mod database;
mod hooks;
mod quotes;
mod guild;
use crate::quotes::Quotes;
use crate::commands::{general_commands::*, test_commands::*, moderation_commands::*, server_utils_commands::*};
use crate::database::{Database, split_at, CommandDescriptions};
use crate::hooks::unknown_command::unknown_command;

#[group]
#[owners_only]
#[commands(test_command, test_quotes)]
struct Owners;

#[group]
#[only_in(guilds)]
#[commands(banned_words, moderated_role, basic_role, logs)]
#[description = "Commands for server admins/moderators"]
struct Admin;

#[group]
#[only_in(guilds)]
#[commands (server)]
struct ServerUtils;

#[group]
#[commands (say, make_sandwich, help)]
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
                .filename("tavern-database.sqlite")
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
        ).group(&OWNERS_GROUP).group(&GENERAL_GROUP).group(&ADMIN_GROUP).group(&SERVERUTILS_GROUP)
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

    let command_descriptions = split_at(';', "command_descriptions/command_descriptions.txt")
        .unwrap_or_else(|err| panic!("Cannot initialize command descriptions: {err}"));
    let command_descriptions = CommandDescriptions::new(command_descriptions);

    {
        let mut data = client.data.write().await;
        data.insert::<CommandDescriptions>(Arc::new(RwLock::new(command_descriptions)));
        data.insert::<Database>(Arc::new(RwLock::new(database)));
        data.insert::<Quotes>(Arc::new(RwLock::new(quotes)))
    }

    if let Err(err) = client.start().await {
        println!("Client error: {:?}", err);
    }

}

//TODO Add auto removal of data from old guilds.
struct Handler;
#[async_trait]
impl EventHandler for Handler {
    //On ready message
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);        
    }

    async fn guild_member_addition(&self, ctx: Context, mut new_member: Member) {
        let guild_description;
        {
            let data_read = ctx.data.read().await;
            let lock = data_read.get::<Database>().expect("Cannot get the lock");
            let database = &*lock.read().await;
            guild_description = GuildDescription::build(database, &new_member.guild_id.to_string()).await.unwrap();
        }
        if let Some(id) = guild_description.get_basic_role_id() {
            new_member.add_role(&ctx.http, id.parse::<u64>()
                .expect("Error parsing id for basic role"))
                .await
                .expect("Error adding basic role");
        }
    }

    async fn message(&self, ctx: Context, mut msg: Message) {
        if msg.author.bot {
            return;
        }
        msg.content = msg.content.to_lowercase();
        msg.content = msg.content.replace(" ", "");

        if let Some(guild_id) = msg.guild_id {
            let guild_description;
            {
                let data_read = ctx.data.read().await;
                let lock = data_read.get::<Database>().expect("Cannot get the lock");
                let database = &*lock.read().await;
                guild_description = GuildDescription::build(database, &guild_id.to_string()).await.unwrap();
            }
            if let Some(moderated_role) = guild_description.get_moderated_role_id() {
                if let Ok(true) = msg.author.has_role(&ctx.http, guild_id, moderated_role.parse::<u64>().unwrap()).await {
                    for word in guild_description.get_banned_words().iter() {
                        if msg.content.contains(word) {
                            msg.delete(&ctx.http).await.expect("Cannot delete a message");
                            let response_to_user = MessageBuilder::new()
                                .mention(&msg.author)
                                .push(" you cannot say ")
                                .push_italic(word).build();
                            let response_to_log_channel = MessageBuilder::new()
                            .mention(&msg.author)
                            .push_line(" said bad things:")
                            .push_bold(&msg.content).build();
                            msg.author.dm(&ctx.http, |m| m.content(&response_to_user) ).await.expect("Cannot send message");
                            if let Some(id) = guild_description.get_log_channel_id() {
                                let id = id.parse::<u64>().expect("Cannot parse id of a log channel");
                                let id = ChannelId::from(id);
                                id.say(&ctx.http, response_to_log_channel).await.unwrap();
                            }
                        }
                    }
                }
            }
        }
    }
}

