use crate::bot::utils::*;
use serenity::{
    model::channel::Message,
    prelude::{Context, TypeMapKey},
};

use std::sync::Arc;
use tokio::sync::RwLock;

use tokio_postgres::{Client as DBClient, NoTls};

pub struct DataBase(DBClient);

impl TypeMapKey for DataBase {
    type Value = Arc<RwLock<DBClient>>;
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
    let data_read = ctx.data.read().await;
    let db_lock = Arc::clone(&data_read.get::<DataBase>().unwrap());
    drop(data_read);
    let db = db_lock.read().await;

    let rows = db
        .query(
            "SELECT * FROM slow_mode.excluded_channels WHERE channel_id = $1",
            &[&(msg.channel_id.0 as i64)],
        )
        .await
        .unwrap();

    drop(db);
    if rows.len() == 1 {
        return true;
    }

    return false;
}

pub async fn increment_channel(ctx: &Context, msg: &Message) {
    let data_read = ctx.data.read().await;
    let db_lock = Arc::clone(&data_read.get::<DataBase>().unwrap());
    drop(data_read);
    let db = db_lock.read().await;

    let res = db
        .execute(
            "INSERT INTO slow_mode.channels (channel_id, message_count) VALUES ($1, 1)",
            &[&(msg.channel_id.0 as i64)],
        )
        .await;

    match res {
        Ok(_) => {
            drop(db);
            return;
        }
        Err(_) => {
            let query_res = db
                .query(
                    "UPDATE slow_mode.channels 
                    SET message_count = message_count+1 
                    WHERE channel_id = $1",
                    &[&(msg.channel_id.0 as i64)],
                )
                .await;
            drop(db);
            if let Err(why) = query_res {
                println!("Error updating text channels data: {:?}", why);
            }
        }
    }
}

pub async fn check_messages(ctx: &Context) {
    loop {
        let data_read = ctx.data.read().await;
        let db_lock = Arc::clone(&data_read.get::<DataBase>().unwrap());
        drop(data_read);
        let db = db_lock.read().await;
        let rows = db
            .query("SELECT * FROM slow_mode.channels", &[])
            .await
            .unwrap();
        for row in rows {
            let x: i64 = row.get(0);
            let z: i64 = row.get(1);
            let channel_id = x as u64;
            let message_count = z as u64;

            match message_count {
                51..=100 => {
                    update_slow_mode(&ctx, &channel_id, 30).await;
                }
                15..=50 => {
                    update_slow_mode(&ctx, &channel_id, 3).await;
                }

                0..=5 => {
                    update_slow_mode(&ctx, &channel_id, 0).await;
                }
                _ => {
                    return;
                }
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
        drop(db);
        tokio::time::delay_for(core::time::Duration::from_secs(15)).await;
    }
}
