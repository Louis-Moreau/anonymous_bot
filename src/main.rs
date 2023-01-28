use std::env;
use std::sync::Arc;

use serenity::async_trait;
use serenity::framework::standard::macros::group;
use serenity::framework::standard::StandardFramework;
use serenity::prelude::*;
mod commands;

use crate::commands::before::*;
use crate::commands::global_data::*;
use crate::commands::message::*;
use crate::commands::vote::*;

#[group]
#[commands(message, newvote, vote)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .before(before)
        .configure(|c| c.prefix("!")) // set the bot's prefix to "!"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("token");
    let anonymous_channel_id = env::var("CHANNEL_ID")
        .expect("channel_id")
        .parse::<u64>()
        .expect("channel_id");
    let intents = GatewayIntents::non_privileged()
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::GUILD_PRESENCES;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<AnonymousChannelId>(Arc::new(RwLock::new(anonymous_channel_id)));
        data.insert::<Vote>(Arc::new(RwLock::new(VoteData::default())));
    }

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
