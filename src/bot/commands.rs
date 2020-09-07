use crate::bot::utils::*;
use crate::bot::Config;
use crate::bot::DataBase;
// use crate::config::Config;
use serenity::{
    framework::standard::{
        macros::{command, group},
        Args, CommandResult, Delimiter,
    },
    model::channel::Message,
    prelude::*,
};

#[group()]
#[commands(exclude, rmexclude, list_excluded)]
pub struct Commands;

#[command]
async fn exclude(ctx: &Context, msg: &Message) -> CommandResult {
    let mut args = Args::new(&msg.content, &[Delimiter::Single(' ')]);
    match args.advance().single::<u64>() {
        Ok(channel_id) => {
            let channel_res = ctx.http.get_channel(channel_id).await;
            match channel_res {
                Ok(_) => {
                    let data = ctx.data.read().await;
                    let db = data.get::<DataBase>().unwrap();

                    let res = db
                        .execute(
                            "INSERT INTO slow_mode.excluded_channels (channel_id) VALUES ($1)",
                            &[&(channel_id as i64)],
                        )
                        .await;

                    match res {
                        Ok(_) => {
                            reply(
                                ctx,
                                msg,
                                &String::from("Channel has been successfully excluded"),
                            )
                            .await;
                            return Ok(());
                        }
                        Err(_) => {
                            reply(
                                ctx,
                                msg,
                                &String::from("Channel already exists in the table"),
                            )
                            .await;
                        }
                    }
                }
                Err(_why) => {
                    reply(ctx, msg, &String::from("Please enter a valid channel id")).await;
                }
            }
        }
        Err(_) => {
            reply(ctx, msg, &String::from("Please enter a valid channel id")).await;
        }
    }

    Ok(())
}

#[command]
async fn rmexclude(ctx: &Context, msg: &Message) -> CommandResult {
    let mut args = Args::new(&msg.content, &[Delimiter::Single(' ')]);
    match args.advance().single::<u64>() {
        Ok(channel_id) => {
            let channel_res = ctx.http.get_channel(channel_id).await;
            match channel_res {
                Ok(_) => {
                    let data = ctx.data.read().await;
                    let db = data.get::<DataBase>().unwrap();

                    let res = db
                        .execute(
                            "DELETE FROM slow_mode.excluded_channels WHERE channel_id = $1 ",
                            &[&(channel_id as i64)],
                        )
                        .await;

                    match res {
                        Ok(outcome) => {
                            if outcome == 1 {
                                reply(
                                    ctx,
                                    msg,
                                    &String::from(
                                        "Channel has been successfully removed from being excluded",
                                    ),
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
                            return Ok(());
                        }
                        Err(why) => panic!(why),
                    }
                }
                Err(_why) => {
                    reply(ctx, msg, &String::from("Please enter a valid channel id")).await;
                }
            }
        }
        Err(_) => {
            reply(ctx, msg, &String::from("Please enter a valid channel id")).await;
        }
    }

    Ok(())
}

#[command]
async fn list_excluded(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let db = data.get::<DataBase>().unwrap();

    let rows = db
        .query("SELECT * FROM slow_mode.excluded_channels", &[])
        .await
        .unwrap();
    let mut output: String = "".to_string();

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

// #[command]
// async fn toggle(ctx: &Context, _msg: &Message) -> CommandResult {
//     let mut data = ctx.data.write().await;
//     let config = data.get_mut::<Config>().unwrap();
//     config.toggle = !config.toggle;
//     reply(ctx, _msg, &format!("changed")).await;
//     Ok(())
// }

// #[command]
// async fn whatis(ctx: &Context, msg: &Message) -> CommandResult {
//     let  data = ctx.data.read().await;
//     let config = data.get::<Config>().unwrap();
//     reply(ctx, msg, &format!("{}",config.toggle)).await;
//     Ok(())
// }
