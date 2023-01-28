use regex::Regex;

use serenity::builder::CreateEmbed;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;
use serenity::model::prelude::ChannelId;
use serenity::prelude::*;

use super::global_data::*;

#[command]
pub async fn vote(ctx: &Context, msg: &Message) -> CommandResult {
    let ano_message = msg.content.trim_start_matches("!vote").trim_start().trim_end();
    let ano_vote = match ano_message.parse::<usize>() {
        Ok(i) => i,
        Err(_) => {
            msg.reply(&ctx, "With 'n' your vote , vote by typing \"!vote n\"")
                .await?;
            return Ok(());
        }
    };

    let lock = {
        let data_read = ctx.data.read().await;
        data_read.get::<Vote>().unwrap().clone()
    };
    let mut vote_data = lock.write().await;

    if vote_data.message_id == 0 {
        msg.reply(&ctx, "There is no vote currently").await?;
        return Ok(());
    }

    if ano_vote >= vote_data.vote_options.len() {
        msg.reply(&ctx, "This vote option does not exist").await?;
        return Ok(());
    }

    vote_data.votes.insert(msg.author.id.0, ano_vote);

    let mut vote_embed = CreateEmbed::default();
    update_vote_embed(&vote_data, &mut vote_embed);
    {
        let anonymous_channel_id = {
            let data_read = ctx.data.read().await;
            data_read.get::<AnonymousChannelId>().unwrap().clone()
        };
        ChannelId(anonymous_channel_id.read().await.clone())
            .edit_message(&ctx, vote_data.message_id, |m| m.set_embed(vote_embed))
            .await?;
    }

    msg.reply(
        &ctx,
        format!(
            "You voted for : {}",
            &vote_data.vote_options.get(ano_vote).unwrap()
        ),
    )
    .await?;
    Ok(())
}

#[command]
pub async fn newvote(ctx: &Context, msg: &Message) -> CommandResult {
    let ano_message = msg.content.trim_start_matches("!newvote").trim_start();
    let re = Regex::new(r#"(?x) "([^"]*)" "#).unwrap();
    let args: Vec<String> = re
        .captures_iter(ano_message)
        .map(|c| c[1].to_string())
        .collect();

    if args.len() <= 2 || args.len() > 6 {
        msg.reply(
            &ctx,
            "Please provide a correct number of vote options (2-5)",
        )
        .await?;
        return Ok(());
    }

    let data_read = ctx.data.read().await;

    let mut vote_embed = CreateEmbed::default();
    {
        let lock_vote = data_read.get::<Vote>().unwrap().clone();
        let mut vote = lock_vote.write().await;
        vote.question = args.get(0).unwrap().to_string();
        vote.vote_options.clear();
        vote.votes.clear();
        for vote_option in args.iter().skip(1) {
            vote.vote_options.push(vote_option.to_string());
        }
        update_vote_embed(&vote, &mut vote_embed);
    }


    let message = {
        let anonymous_channel_id: u64 = data_read
            .get::<AnonymousChannelId>()
            .unwrap()
            .clone()
            .read()
            .await
            .clone();
        ChannelId(anonymous_channel_id)
            .send_message(&ctx, |m| {m.set_embed(vote_embed)})
            .await?
    };
    {
        let lock_vote = data_read.get::<Vote>().unwrap().clone();
        let mut vote = lock_vote.write().await;
        vote.message_id = message.id.0;
    }

    msg.reply(ctx, "Vote created!").await?;
    Ok(())
}

fn update_vote_embed(vote_data: &VoteData, vote_embed: &mut CreateEmbed) {
    vote_embed.title(&vote_data.question);

    //Showing vote options
    let mut description = String::new();
    for (i, option) in vote_data.vote_options.iter().enumerate() {
        let emoji = match i {
            0 => ":zero:",
            1 => ":one:",
            2 => ":two:",
            3 => ":three:",
            4 => ":four:",
            _ => ":regional_indicator_n:",
        };
        description.push_str(&format!("{} : {}\n\n", emoji, &option));
    }

    //Showing vote results
    description.push_str("**Results :**\n");
    let mut votes_result: Vec<usize> = vec![0; vote_data.vote_options.len()];
    for v in &vote_data.votes {
        votes_result[*v.1] += 1;
    }
    for (i, v) in votes_result.iter().enumerate() {
        description.push_str(&format!(
            "{} : **{}**\n",
            vote_data.vote_options.get(i).unwrap(),
            v
        ));
    }

    vote_embed.description(description);
}
