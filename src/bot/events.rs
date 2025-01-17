use log::info;
use serenity::{async_trait, model::prelude::*, prelude::*};

use crate::database::*;

use crate::bot::utils::toggled;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        let perms = Permissions::from_bits(0).unwrap();
        let user = &ready.user;
        info!(
            "
Ready as {}
 * Serving {} guilds
 * Invite URL: {}",
            user.tag(),
            ready.guilds.len(),
            user.invite_url(&ctx, perms).await.unwrap(),
        );

        check_messages(&ctx).await;
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.is_own(&ctx.cache).await || excluded(&ctx, &msg).await || toggled(&ctx).await {
            return;
        }

        increment_channel(&ctx, &msg).await;
    }
}
