use serenity::{
    async_trait,
    model::prelude::*,
    prelude::*
};

use crate::database::DataBase;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        let user = &ready.user;
        println!("Logged in as {}", user.name);
        check_messages(&ctx).await;
    }

    async fn message(&self, ctx: Context, msg: Message) {
    	if msg.is_own(&ctx.cache).await {
    		return;	
    	}
        increment_channel(&ctx, &msg).await;
    }
}

async fn increment_channel(ctx: &Context, msg: &Message) {
    let data = ctx.data.read().await;
    let db = data.get::<DataBase>().unwrap();
    if let Err(why) = db.query("UPDATE slow_mode.channels SET message_count = message_count+1 WHERE channel_id =  $1", &[&(msg.channel_id.0 as i64)]).await {
        println!("Error updating text channels data: {:?}", why);
    }

}

async fn check_messages(ctx: &Context) {
	let data = ctx.data.read().await;
    let db = data.get::<DataBase>().unwrap();

	loop {
        let rows = db.query("SELECT * FROM slow_mode.channels", &[]).await.unwrap();
        for row in rows {
            let x : i64 = row.get(0);
            let z: i64 = row.get(1);
            let channel_id = x as u64;
            let message_count = z as u64;

            if between(&message_count, 51,100) {
                update_slow_mode(&ctx, &channel_id, 60).await;
            } else if between(&message_count, 2,50) {
                update_slow_mode(&ctx, &channel_id, 30).await;
            } else if between(&message_count, 0,1) {
                update_slow_mode(&ctx, &channel_id, 0).await;
            }


            if let Err(why) = db.query("UPDATE slow_mode.channels SET message_count = 0 WHERE channel_id = $1", &[&(channel_id as i64)]).await {
                println!("Error updating text channels data: {:?}", why);
            }
        }

		tokio::time::delay_for(core::time::Duration::from_secs(2)).await;
	}
}

async fn update_slow_mode(ctx: &Context,channel: &u64, seconds:u64) {
    let channel = ctx.http.get_channel(*channel).await.unwrap();
    if let Err(why) = channel.id().edit(&ctx.http, |c| c.slow_mode_rate(seconds)).await {
        println!("Error setting channel's slow mode rate: {:?}", why);
    }
}

fn between(number: &u64, min: u64, max: u64) -> bool {
    if min <= *number && *number <= max {
		return true
	}
	return false
}