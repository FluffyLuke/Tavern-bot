use std::env;
use std::collections::{HashSet, HashMap};


use serenity::utils::MessageBuilder;
use serenity::{async_trait};
use serenity::http::Http;
use serenity::framework::standard::macros::{check, command, group, help, hook};
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use tracing::{error, info};
use serenity::model::channel::{Channel, Message};

use serenity::framework::standard::{
    help_commands,
    Args,
    CommandGroup,
    CommandOptions,
    CommandResult,
    DispatchError,
    HelpOptions,
    Reason,
    StandardFramework,
};



#[group]
#[commands("test_command")]
struct Owners;

#[command]
async fn test_command(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Better test deez nutz").await?;
    Ok(())
}

#[hook]
async fn unknown_command(ctx: &Context, msg: &Message, unknown_command_name: &str) {
    let response = MessageBuilder::new()
        .mention(&msg.author)
        .push(", stop mumbling. I don't understand.")
        .build();
    if let Err(err) = msg.reply(&ctx.http, &response).await {
        println!("Error replying to user {}: {}", msg.author.name, err);
    };
}

struct CommandCounter;

impl TypeMapKey for CommandCounter {
    type Value = HashMap<String, u64>;
}


#[tokio::main]
async fn main() {

    dotenv::dotenv().expect("Failed to load .env file: ");
    let token = env::var("DISCORD_BOT_KEY").expect("Error accured while retrieving bots key: ");
    let prefix = env::var("DISCORD_BOT_PREFIX").expect("Error accured while retrieving bots prefix: ");
    let intents = GatewayIntents::all();
    tracing_subscriber::fmt::init();

    let http = Http::new(&token);

    let (owners, bot_id) = match http.get_current_application_info().await {
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
        ).group(&OWNERS_GROUP).unrecognised_command(unknown_command);

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .framework(framework)
        .type_map_insert::<CommandCounter>(HashMap::default())
        .await.expect("Error accured why creating a bot instance");

    if let Err(err) = client.start().await {
        println!("Client error: {:?}", err);
    }

}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        //Test message
        if msg.content.contains("test") && !msg.author.bot {
            msg.channel_id.say(ctx.http, "Handling test deez nutz")
                .await
                .expect("Cannot display test message: ");
        }
    }

    //On ready message
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}


