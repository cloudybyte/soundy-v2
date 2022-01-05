use serenity::{framework::standard::{macros::command, CommandResult, Args}, client::Context, model::channel::Message};

use crate::{check_msg, util::{send_embed, join_vc, EmbedType}, Lavalink};

#[command]
#[min_args(1)]
pub async fn play(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let query = args.message().to_string();

    let guild_id = match ctx.cache.guild_channel(msg.channel_id).await {
        Some(channel) => channel.guild_id,
        None => {
            check_msg(
                msg.channel_id
                    .say(&ctx.http, "Error finding channel info")
                    .await,
            );

            return Ok(());
        }
    };

    let lava_client = {
        let data = ctx.data.read().await;
        data.get::<Lavalink>().unwrap().clone()
    };

    let manager = songbird::get(ctx).await.unwrap().clone();


    if let Some(_handler) = manager.get(guild_id) {

    } else {

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

        join_vc(guild_id, msg.channel_id, connect_to, ctx).await;
    }


    if let Some(_handler) = manager.get(guild_id) {

    //let connections = lava_client.discord_gateway_connections().await;
    //if connections.contains_key(&guild_id.into()) {
        let query_information = lava_client.auto_search_tracks(&query).await?;

        if query_information.tracks.is_empty() {
            check_msg(
                msg.channel_id
                    .say(&ctx, "Could not find any video of the search query.")
                    .await,
            );
            return Ok(());
        }

        if let Err(why) = &lava_client
            .play(guild_id, query_information.tracks[0].clone())
            // Change this to play() if you want your own custom queue or no queue at all.
            .queue()
            .await
        {
            error!("{}", why);
            return Ok(());
        };
        check_msg(
            msg.channel_id
                .say(
                    &ctx.http,
                    format!(
                        "Added to queue: {}",
                        query_information.tracks[0].info.as_ref().unwrap().title
                    ),
                )
                .await,
        );

        send_embed(msg.channel_id, &ctx.http, Some("Queued".to_string()), format!("Added `{}` to the queue", query_information.tracks[0].info.as_ref().unwrap().title), EmbedType::Info).await;
    } else {
        check_msg(
            msg.channel_id
                .say(
                    &ctx.http,
                    "Use `~join` first, to connect the bot to your current voice channel.",
                )
                .await,
        );
    }

    Ok(())
}