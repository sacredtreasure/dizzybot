use std::env;
use tokio;

use serenity::{ async_trait, model::{channel::Message, gateway::Ready}, prelude::*,};

const HELP_MESSAGE: &str = "
    Hello there, fellow degen!

    I'm here to help you to get up to speed in this server and also help you get what you need.

    ? Need Alpha and insights? 
    - Post in the <#CHANNEL_ID> channel and our top degen specialists will assist you.

    ? Need the next 100x? 
    - This should help - <https://dexscreener.com>

    ? Something wrong? 
    - Dont be shy to dm a mod!

    ALL OF THESE SHOULD HELP

    -- DizzyBot 2023 

";

const HELP_COMMAND: &str = "!degen";

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content = HELP_COMMAND {
            if let Err(why) = msg.channel_id.say(&ctx.http, HELP_MESSAGE).await {
                println!("Error sending message: {:?}" why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!" ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN")
    .expect("Expected a token in the environment...");

    let mut client = Client::new(&token)
        .event_handler(Handler)
        .await
        .expect("Error creating client...");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}


