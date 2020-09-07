use crate::bot::utils::*;
use serenity::{model::channel::Message,
    prelude::{TypeMapKey, Context}
};

use tokio_postgres::{Client as DBClient, NoTls};


pub struct DataBase(DBClient);


impl TypeMapKey for DataBase {
    type Value = DBClient;
}

pub async fn connect(uri: &String) -> DBClient {
    let (db_client, connection) = tokio_postgres::connect(&uri, NoTls).await.unwrap();

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    db_client
}

pub async fn excluded(ctx: &Context, msg: &Message) -> bool {
    let data = ctx.data.read().await;
    let db = data.get::<DataBase>().unwrap();

    let rows = db
            .query("SELECT * FROM slow_mode.excluded_channels WHERE channel_id = $1", &[&(msg.channel_id.0 as i64)])
            .await
            .unwrap();

    if rows.len() == 1 {
        return true;
    }

    return false;
}

pub async fn increment_channel(ctx: &Context, msg: &Message) {
    let data = ctx.data.read().await;
    let db = data.get::<DataBase>().unwrap();

    let res = db
        .execute(
            "INSERT INTO slow_mode.channels (channel_id, message_count) VALUES ($1, 1)",
            &[&(msg.channel_id.0 as i64)],
        )
        .await;

    match res {
        Ok(_) => return,
        Err(_) => {
            let query_res = db
                .query(
                    "UPDATE slow_mode.channels 
                    SET message_count = message_count+1 
                    WHERE channel_id = $1",
                    &[&(msg.channel_id.0 as i64)],
                )
                .await;
            if let Err(why) = query_res {
                println!("Error updating text channels data: {:?}", why);
            }
        }
    }
}

pub async fn check_messages(ctx: &Context) {
    let data = ctx.data.read().await;
    let db = data.get::<DataBase>().unwrap();

    loop {
        let rows = db
            .query("SELECT * FROM slow_mode.channels", &[])
            .await
            .unwrap();
        for row in rows {
            let x: i64 = row.get(0);
            let z: i64 = row.get(1);
            let channel_id = x as u64;
            let message_count = z as u64;

            if between(&message_count, 51, 100) {
                update_slow_mode(&ctx, &channel_id, 30).await;
            } else if between(&message_count, 11, 50) {
                update_slow_mode(&ctx, &channel_id, 3).await;
            } else if between(&message_count, 0, 10) {
                update_slow_mode(&ctx, &channel_id, 0).await;
            }

            if let Err(why) = db
                .query(
                    "UPDATE slow_mode.channels SET message_count = 0 WHERE channel_id = $1",
                    &[&(channel_id as i64)],
                )
                .await
            {
                println!("Error updating text channels data: {:?}", why);
            }
        }

        tokio::time::delay_for(core::time::Duration::from_secs(2)).await;
    }
}