use serenity::{framework::standard::{macros::command, CommandResult}, client::Context, model::channel::Message, prelude::Mentionable};

use crate::{check_msg, Lavalink, util::join_vc};

#[command]
pub async fn join(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guild_id = guild.id;

    let channel_id = guild
        .voice_states
        .get(&msg.author.id)
        .and_then(|voice_state| voice_state.channel_id);

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            check_msg(msg.reply(&ctx.http, "Join a voice channel first.").await);

            return Ok(());
        }
    };



    join_vc(guild_id, msg.channel_id, connect_to, ctx).await?;

    Ok(())
}