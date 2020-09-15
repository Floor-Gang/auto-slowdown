use log::warn;
use regex::Regex;
use serenity::{
    framework::standard::Args,
    model::channel::{Channel, Message},
    prelude::Context,
    Result as SerenityResult,
};

use crate::bot::Config;
use std::sync::Arc;
pub async fn reply(ctx: &Context, msg: &Message, content: &String) {
    if let Err(why) = msg.channel_id.say(&ctx.http, &content).await {
        warn!(
            "Failed to send message in #{} because\n{:?}",
            msg.channel_id, why,
        );
    }
}

pub async fn resolve_channel(ctx: &Context, args: &mut Args) -> Option<i64> {
    if let Ok(channel_id) = args.single::<u64>() {
        if let Ok(_) = ctx.http.get_channel(channel_id).await {
            return Some(channel_id as i64);
        } else {
            return None;
        }
    } else {
        return None;
    }
}

pub async fn parse_channel(ctx: &Context, channel_name: String) -> Option<Channel> {
    let channel: Channel;
    if let Ok(id) = channel_name.parse::<u64>() {
        let channel = match ctx.http.get_channel(id).await {
            Ok(c) => c,
            Err(_e) => return None,
        };
        Some(channel.to_owned())
    } else if channel_name.starts_with("<#") && channel_name.ends_with(">") {
        let re = Regex::new("[<#!>]").unwrap();
        let channel_id = re.replace_all(&channel_name, "").into_owned();

        channel = match ctx
            .http
            .get_channel(channel_id.parse::<u64>().unwrap())
            .await
        {
            Ok(m) => m,
            Err(_e) => return None,
        };

        Some(channel.to_owned())
    } else {
        None
    }
}

#[allow(dead_code)]
pub fn check_msg(result: SerenityResult<Message>) {
    if let Err(why) = result {
        println!("Error sending message: {:?}", why);
    }
}

pub async fn update_slow_mode(ctx: &Context, channel: &u64, seconds: u64) {
    let channel = ctx.http.get_channel(*channel).await.unwrap();
    if let Err(why) = channel
        .id()
        .edit(&ctx.http, |c| c.slow_mode_rate(seconds))
        .await
    {
        println!("Error setting channel's slow mode rate: {:?}", why);
    }
}

pub async fn toggled(ctx: &Context) -> bool {
    let data_read = ctx.data.read().await;
    let read_lock = Arc::clone(&data_read.get::<Config>().unwrap());
    drop(data_read);
    let config = read_lock.read().await;
    let toggled = config.toggle;
    drop(config);
    return toggled;
}
