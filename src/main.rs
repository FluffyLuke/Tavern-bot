use std::env;


use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        //Lipti
        if msg.author.id == 452396917373272075 {
            msg.channel_id.say(&ctx.http,"Our lord has spoken").await.expect("Error while sending a message");
        }

        if msg.content == "Fuck u" {
            msg.channel_id.say(&ctx.http,"Suck my balls").await.expect("Error while sending a message");
        }

        if msg.content == "test" {
            println!("Shard {}", ctx.shard_id);
            msg.channel_id.say(&ctx.http,"I test deez nuts").await.expect("Error while sending a message");

            let dm = msg.author.dm(&ctx, |m| m.content("I will slaugher you family")).await;

            if let Err(why) = dm {
                println!("Error when direct messaging user: {:?}", why);
            }
        }
    }

    // async fn ready(&self, _: Context, ready: Ready) {
    //     let channel_id = ChannelId(1012788440875667467);
    //     let _ = channel_id.send_message(&http, |message| {
    //         message.content("BARMAN PRZYBYŁ").tts(true)
    //     });


    //     println!("{} is connected!", ready.user.name);
    // }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    
    let token = "MTExNDk2NzE1NTY1ODcyMzQzMA.GHT-y5.6GLFIL-5ngz1SrhgAzWNbfDAlyI3Ou_cYCYOSg";

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::DIRECT_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .await.expect("Error accured why creating a bot instance");

    if let Err(err) = client.start_shards(2).await {
        println!("Client error: {:?}", err);
    }
}


