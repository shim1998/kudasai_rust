use std::{env, format};

use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use crate::formulabot::*;

#[command]
async fn fetch_data(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let table_name = args.single::<String>()?;
    let db_name = env::var("F1DB").expect("No DB");
    db::fetch_data(&db_name,&table_name);
    msg.channel_id.say(&ctx.http, "Logged DB").await?;
    Ok(())
}

#[command]
async fn user(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let db_name = env::var("F1DB").expect("No DB");
    let action = args.single::<String>()?;
    let author_name = msg.author.name.to_string();
    let author_id = &msg.author.id.to_string();
    let res = db::modify_user(&action, &db_name,&author_name,&author_id);
    match res {
        Ok(res) => msg.channel_id.say(&ctx.http, format!("{} user {}",res,msg.author)).await?,
        Err(e) => msg.channel_id.say(&ctx.http, format!("{}",e)).await?,
    };
    Ok(())
}