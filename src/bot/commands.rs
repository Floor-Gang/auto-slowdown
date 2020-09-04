use crate::config::Config;
use serenity::{
    prelude::*,
    framework::standard::{
        CommandResult,
        macros::{
            command, group
        }
    },
    model::{
        channel::Message
    }
};
use crate::bot::utils::*;
use crate::bot::DataBase;

#[group()]
#[commands(ping, db_test, prefix)]
pub struct Commands;

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    let test = "fortnite";
    println!("{}", test);

    reply(&ctx, &msg, &String::from("Pong!")).await;
    Ok(())
}

#[command]
async fn db_test(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let db = data.get::<DataBase>().unwrap();

    let row = db.query_one("SELECT channel_id FROM slow_mode.channels", &[]).await.unwrap();
    let channel_id: i64 = row.get(0);


    if let Err(why) = msg.channel_id.send_message(&ctx.http,  |m| {
        m.embed(|embed| {
            embed.title("Channel ID");
            embed.description(format!("This is a sutpid channel id: `{}`", channel_id.to_string()));
            embed.color(0xffa500)
        });
        m

    }).await {
        println!("Failed to send message in #{} because\n{:?}",
                 msg.channel_id, why
        );
    };

    Ok(())
}

#[command]
async fn prefix(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let config = data.get::<Config>().unwrap();

    check_msg(msg.channel_id.send_message(&ctx.http,  |m| {
        m.embed(|embed| {
            embed.title("Prefix");
            embed.description(format!("My prefix is: `{}`", &config.prefix));
            embed.color(0xffa500)
        });
        m

    }).await);

    Ok(())
}