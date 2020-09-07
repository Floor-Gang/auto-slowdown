use serenity::{async_trait, model::prelude::*, prelude::*};

use crate::database::*;

use crate::bot::utils::toggled;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        let user = &ready.user;
        println!("Logged in as {}", user.name);
        check_messages(&ctx).await;
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.is_own(&ctx.cache).await || excluded(&ctx, &msg).await || toggled(&ctx).await {
            return;
        }

        increment_channel(&ctx, &msg).await;
    }
}
