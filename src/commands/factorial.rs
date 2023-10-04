use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
pub async fn factorial(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let mut product = 1;
    let mut num = args.single::<u64>()?;

    while num > 0 {
        product *= num;
        num -= 1;
    }
    
    msg.channel_id.say(&ctx.http, product).await?;

    Ok(())
}
