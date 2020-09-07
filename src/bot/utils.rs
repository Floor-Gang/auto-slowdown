use serenity::{model::channel::Message, prelude::Context, Result as SerenityResult};

pub(crate) async fn reply(ctx: &Context, msg: &Message, content: &String) {
    if let Err(why) = msg.channel_id.say(&ctx.http, &content).await {
        println!(
            "Failed to send message in #{} because\n{:?}",
            msg.channel_id, why
        );
    }
}

pub(crate) fn check_msg(result: SerenityResult<Message>) {
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
/*
pub(crate) async fn reply_embed<T>(ctx: &Context, msg: &Message, embed: T) {
    if let Err(why) = msg.channel_id.send_message(&ctx.http, &embed).await {
        println!("Failed to send message in #{} because\n{:?}",
                 msg.channel_id, why
        );
    }
}
*/
