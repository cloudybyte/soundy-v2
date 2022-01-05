use serenity::{client::Context, model::channel::Message, framework::standard::{CommandResult, macros::command}};

use crate::{Lavalink, check_msg};
use crate::util::{EmbedType, send_embed};

#[command]
pub async fn skip(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let lava_client = data.get::<Lavalink>().unwrap().clone();

    if let Some(track) = lava_client.skip(msg.guild_id.unwrap()).await {
        send_embed(msg.channel_id, &ctx.http, Some("Skipped".to_string()), format!("Skipped: {}", track.track.info.as_ref().unwrap().title),EmbedType::Info).await;
    } else {
        send_embed(msg.channel_id, &ctx.http, None, "Nothing to skip.".to_string(), EmbedType::Error).await;
    }

    Ok(())
}