use crate::bot::utils::*;
use crate::bot::Config;
use crate::bot::DataBase;
// use crate::config::Config;
use serenity::{
    framework::standard::{
        macros::{command, group},
        Args, CommandResult,
    },
    model::prelude::*,
    prelude::*,
};

use std::sync::Arc;

#[group()]
#[commands(exclude, rmexclude, list_excluded, toggle)]
pub struct Commands;

#[command]
async fn exclude(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let channel_id;
    if let Some(_channel_id) = resolve_channel(&ctx, &mut args).await {
        channel_id = _channel_id;
    } else {
        reply(ctx, msg, &String::from("Please enter a valid channel id")).await;
        return Ok(());
    }

    let data = ctx.data.read().await;
    let db = data.get::<DataBase>().unwrap();

    let res = db
        .execute(
            "INSERT INTO slow_mode.excluded_channels (channel_id) VALUES ($1)",
            &[&channel_id],
        )
        .await;

    if let Ok(_) = res {
        reply(
            ctx,
            msg,
            &String::from("Channel has been successfully excluded"),
        )
        .await;
    } else {
        reply(
            ctx,
            msg,
            &String::from("Channel already exists in the table"),
        )
        .await;
    }

    Ok(())
}

async fn resolve_channel(ctx: &Context, args: &mut Args) -> Option<i64> {
    if let Ok(channel_id) = args.advance().single::<u64>() {
        if let Ok(_) = ctx.http.get_channel(channel_id).await {
            return Some(channel_id as i64);
        } else {
            return None;
        }
    } else {
        return None;
    }
}

#[command]
async fn rmexclude(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let channel_id;
    if let Some(_channel_id) = resolve_channel(&ctx, &mut args).await {
        channel_id = _channel_id;
    } else {
        reply(ctx, msg, &String::from("Please enter a valid channel id")).await;
        return Ok(());
    }

    let data = ctx.data.read().await;
    let db = data.get::<DataBase>().unwrap();

    let res = db
        .execute(
            "DELETE FROM slow_mode.excluded_channels WHERE channel_id = $1 ",
            &[&(channel_id as i64)],
        )
        .await;

    if res.unwrap() == 1 {
        reply(
            ctx,
            msg,
            &String::from("Channel has been successfully removed from being excluded"),
        )
        .await;
    } else {
        reply(
            ctx,
            msg,
            &String::from("Channel is not in the excluded table"),
        )
        .await;
    }

    Ok(())
}

#[command]
async fn list_excluded(ctx: &Context, msg: &Message) -> CommandResult {
    let mut output: String = "".to_string();

    let data = ctx.data.read().await;
    let db = data.get::<DataBase>().unwrap();

    let rows = db
        .query("SELECT * FROM slow_mode.excluded_channels", &[])
        .await
        .unwrap();

    for row in rows {
        let channel_id: i64 = row.get(0);
        output += &format!("<#{}>\n", channel_id);
    }

    check_msg(
        msg.channel_id
            .send_message(&ctx.http, |m| {
                m.embed(|embed| {
                    embed.title("Excluded channels");
                    embed.description(output);
                    embed.color(0xffa500)
                });
                m
            })
            .await,
    );

    Ok(())
}

// WIP

#[command]
async fn toggle(ctx: &Context, _msg: &Message) -> CommandResult {
    let toggle_value;
    {
        let data_read = ctx.data.read().await;
        let db_lock = Arc::clone(&data_read.get::<Config>().unwrap());
        let mut config = db_lock.write().await;
        config.toggle = !config.toggle;
        toggle_value = config.toggle;
    }
    reply(
        ctx,
        _msg,
        &format!(
            "Slow-mode is now {} channels",
            (if toggle_value {
                "not watching"
            } else {
                "watching"
            })
        ),
    )
    .await;
    Ok(())
}
