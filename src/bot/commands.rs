use crate::bot::utils::*;
use crate::bot::Config;
use crate::bot::DataBase;

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
#[only_in("guilds")]
#[required_permissions(MANAGE_CHANNELS)]
#[commands(exclude, rmexclude, list_excluded, toggle, help, state)]
pub struct Commands;

#[command]
async fn exclude(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    if args.len() == 0 {
        reply(&ctx, &msg, &"Please provide channels".to_string()).await;
    }

    let data = ctx.data.read().await;
    let db = data.get::<DataBase>().unwrap();

    for c_id in args.iter::<String>() {
        let x = c_id.unwrap();
        let c = parse_channel(&ctx, x).await.unwrap();
        let id: i64 = From::from(c.id());

        let res = db
            .execute(
                "INSERT INTO slow_mode.excluded_channels (channel_id) VALUES ($1)",
                &[&id],
            )
            .await;

        if let Ok(_) = res {
            reply(ctx, msg, &format!("{} has been successfully excluded", c)).await;
        } else {
            reply(ctx, msg, &format!("{} already exists in the table", c)).await;
        }
    }

    Ok(())
}

#[command]
async fn rmexclude(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    if args.len() == 0 {
        reply(&ctx, &msg, &"Please provide channels".to_string()).await;
    }

    let data = ctx.data.read().await;
    let db = data.get::<DataBase>().unwrap();

    for c_id in args.iter::<String>() {
        let x = c_id.unwrap();
        let c = parse_channel(&ctx, x).await.unwrap();
        let id: i64 = From::from(c.id());

        let res = db
            .execute(
                "DELETE FROM slow_mode.excluded_channels WHERE channel_id = $1 ",
                &[&id],
            )
            .await;

        if res.unwrap() == 1 {
            reply(
                ctx,
                msg,
                &format!("{} has been successfully removed from being excluded", c),
            )
            .await;
        } else {
            reply(ctx, msg, &format!("{} is not in the excluded table", c)).await;
        }
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

#[command]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    let mut output = String::new();
    output += "```rust\n";
    output += ";exclude [#channel/s]      // Adds channels to be excluded\n";
    output += ";rmexclude [#channel/s]    // Removes channels from being excluded\n";
    output += ";list_excluded           // Displays all excluded channels\n";
    output += ";toggle                  // Change the bots watching state\n";
    output += ";state                   // View the state of the bot\n";
    output += ";help                    // View this page\n```";

    reply(&ctx, &msg, &output).await;

    Ok(())
}

// WIP

#[command]
async fn toggle(ctx: &Context, msg: &Message) -> CommandResult {
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
        msg,
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

#[command]
async fn state(ctx: &Context, msg: &Message) -> CommandResult {
    let data_read = ctx.data.read().await;
    let db_lock = Arc::clone(&data_read.get::<Config>().unwrap());
    let config = db_lock.read().await;

    reply(
        ctx,
        msg,
        &format!(
            "Slow-mode is now {} channels",
            (if config.toggle {
                "not watching"
            } else {
                "watching"
            })
        ),
    )
    .await;
    Ok(())
}
