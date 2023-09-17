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
async fn add_user(ctx: &Context, msg: &Message) -> CommandResult {
    let db_name = env::var("F1DB").expect("No DB");
    db::add_user(&db_name,&msg.author.name.to_string(),&msg.author.id.to_string());
    msg.channel_id.say(&ctx.http, format!("Added user {} {}",msg.author.id,msg.author.name)).await?;
    Ok(())
}