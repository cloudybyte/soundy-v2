use std::convert::TryInto;
use serenity::{framework::standard::{macros::command, CommandResult}, model::channel::Message, client::Context};
use serenity::framework::standard::{Args, CommandError};
use crate::Lavalink;
use crate::util::{EmbedType, send_embed};

#[command]
#[aliases(vol)]
#[min_args(1)]
pub async fn volume(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let manager = songbird::get(ctx).await.unwrap().clone();


    let data = ctx.data.read().await;
    let lava_client = data.get::<Lavalink>().unwrap().clone();

    if let Some(x) = args.message().to_string().split_whitespace().next() {
        if let Ok(new_vol) = x.parse::<u16>() {
            lava_client.volume(msg.guild_id.unwrap(), new_vol).await?;
        } else {
            send_embed(msg.channel_id, &ctx.http, None, "New volume must be a valid number".to_string(), EmbedType::Error).await;
            return Ok(());
        }

        match manager.get(msg.guild_id.unwrap()) {
            Some(m) => m,
            None => {
                send_embed(msg.channel_id, &ctx.http, None, "The bot is currently not connected to any channel!".to_string(), EmbedType::Error).await;
                return Ok(());
            }
        };
    } else {
        send_embed(msg.channel_id, &ctx.http, None, "Missing arguments".to_string(), EmbedType::Error).await;
    }

    Ok(())
}