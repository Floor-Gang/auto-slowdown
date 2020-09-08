use log::warn;
use serenity::{model::channel::Message, prelude::Context, Result as SerenityResult};

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
