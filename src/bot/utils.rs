use serenity::{model::channel::Message, prelude::Context, Result as SerenityResult};
pub async fn reply(ctx: &Context, msg: &Message, content: &String) {
    if let Err(why) = msg.channel_id.say(&ctx.http, &content).await {
        println!(
            "Failed to send message in #{} because\n{:?}",
            msg.channel_id, why
        );
    }
}

#[allow(dead_code)]
pub fn check_msg(result: SerenityResult<Message>) {
    if let Err(why) = result {
        println!("Error sending message: {:?}", why);
    }
}

pub fn between(number: &u64, min: u64, max: u64) -> bool {
    if min <= *number && *number <= max {
        return true;
    }
    return false;
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
