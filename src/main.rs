mod contest;

use std::env;
use std::process;

use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::model::prelude::ChannelId;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let channel_id_number: u64 = env::var("CHANNEL_ID").unwrap().parse().unwrap();
        let channel_id = ChannelId(channel_id_number);

        if contest::is_friday_9pm() {
            if let Err(why) = channel_id.say(&ctx.http, contest::get_contest_info()).await {
                println!("Error sending message: {why:?}");
            }
        } else {
            println!("Not Friday 9pm");
        }

        process::exit(0);
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
