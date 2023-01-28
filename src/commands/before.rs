use std::collections::HashSet;

use serenity::framework::standard::macros::hook;
use serenity::model::channel::Message;
use serenity::prelude::*;

use super::global_data::*;

#[hook]
pub async fn before(ctx: &Context, msg: &Message, _command_name: &str) -> bool {
    let anonymous_channel_id: u64 = {
        let data_read = ctx.data.read().await;
        data_read
            .get::<AnonymousChannelId>()
            .unwrap()
            .clone()
            .read()
            .await
            .clone()
    };

    let mut members_list: HashSet<u64> = HashSet::new();

    let channel = match ctx.http.get_channel(anonymous_channel_id).await {
        Ok(i) => i,
        Err(e) => {
            eprintln!("Could not send a message : {}", e);
            return false;
        }
    };

    for member in channel.guild().unwrap().members(ctx).await.unwrap() {
        members_list.insert(member.user.id.0);
    }

    if !members_list.contains(&msg.author.id.0) {
        if let Err(e) = msg
            .reply(ctx, "You are not present in the destination channel")
            .await
        {
            eprintln!("Could not send a message : {}", e);
        };
        return false;
    }
    if !msg.is_private() {
        if let Err(e) = msg
            .reply(ctx, "You can only send commands using private messages")
            .await
        {
            eprintln!("Could not send a message : {}", e);
        };

        return false;
    }

    //return true if member in channel
    return true;
}
