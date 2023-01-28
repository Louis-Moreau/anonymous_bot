use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;
use serenity::model::prelude::ChannelId;
use serenity::prelude::*;

use super::global_data::*;

#[command]
pub async fn message(ctx: &Context, msg: &Message) -> CommandResult {
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

    let ano_message = msg.content.trim_start_matches("!message").trim_start();
    let attachments: Vec<&str> = msg.attachments.iter().map(|a| a.url.as_str()).collect();

    if ano_message.is_empty() && attachments.len() < 1 {
        msg.reply(&ctx, "Cannot send an empty message").await?;
        return Ok(());
    }

    ChannelId(anonymous_channel_id)
        .send_files(&ctx, attachments.clone(), |m| m.content(ano_message))
        .await?;

    msg.reply(ctx, "Message sent!").await?;

    println!(
        "Sent message with {} attachments : \"{}\"",
        attachments.len(),
        ano_message
    );

    Ok(())
}
