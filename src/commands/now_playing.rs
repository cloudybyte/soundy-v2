use serenity::{client::Context, model::channel::Message, framework::standard::{CommandResult, macros::command}};

use crate::{Lavalink, check_msg};

#[command]
#[aliases(np)]
pub async fn now_playing(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let lava_client = data.get::<Lavalink>().unwrap().clone();

    if let Some(node) = lava_client.nodes().await.get(&msg.guild_id.unwrap().0) {
        if let Some(track) = &node.now_playing {
            check_msg(
                msg.channel_id
                    .say(
                        &ctx.http,
                        format!("Now Playing: {}", track.track.info.as_ref().unwrap().title),
                    )
                    .await,
            );
        } else {
            check_msg(
                msg.channel_id
                    .say(&ctx.http, "Nothing is playing at the moment.")
                    .await,
            );
        }
    } else {
        check_msg(
            msg.channel_id
                .say(&ctx.http, "Nothing is playing at the moment.")
                .await,
        );
    }

    Ok(())
}