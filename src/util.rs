use std::sync::Arc;
use chrono::Utc;

use serenity::client::Context;
use serenity::framework::standard::CommandResult;
use serenity::model::id::GuildId;
use serenity::prelude::Mentionable;
use serenity::{model::id::ChannelId, http::Http};
use serenity::builder::Timestamp;
use serenity::utils::Colour;
use tracing_subscriber::fmt::time::ChronoUtc;

use crate::Lavalink;

pub enum EmbedType {
    Info,
    Success,
    Warn,
    Error,
    Special,
}

pub async fn send_embed(channel_id: ChannelId, http: &Arc<Http>, title: Option<String>, content: String, embed_type: EmbedType) {
    let _ = channel_id.send_message(http, |m| {
        m.embed(|mut e| {
            e.description(content);
            match embed_type {
                EmbedType::Info => {
                    e.color(Colour::from_rgb(32, 34, 37));
                }
                EmbedType::Success => {
                    e.color(Colour::from_rgb(0, 255, 0));
                    e.title(title.unwrap_or("Success".to_string()));
                }
                EmbedType::Warn => {
                    e.color(Colour::from_rgb(255, 200, 0));
                    e.title(title.unwrap_or("Warning".to_string()));
                }
                EmbedType::Error => {
                    e.color(Colour::from_rgb(255, 0, 0));
                    e.title(title.unwrap_or("Error".to_string()));
                }
                EmbedType::Special => { e.color(Colour::from_rgb(0, 0, 255)); },
            };
            e.footer(|f| {
                f.text("cloudybyte.net");
                f
            });
            /*e.timestamp(|t| {
                Timestamp::from(Utc::now().to_string())
            });*/
            e
        });
        m
    }).await;
}


pub async fn join_vc(guild_id: GuildId, text_channel: ChannelId, connect_to: ChannelId, ctx: &Context) -> CommandResult {
    let manager = songbird::get(ctx).await.unwrap().clone();

    let (_, handler) = manager.join_gateway(guild_id, connect_to).await;

    match handler {
        Ok(connection_info) => {
            let data = ctx.data.read().await;
            let lava_client = data.get::<Lavalink>().unwrap().clone();
            lava_client.create_session_with_songbird(&connection_info).await?;

            send_embed(text_channel, &ctx.http, Some("Connected".to_string()), format!("Joined `{}`", connect_to.mention()), EmbedType::Info).await;
        }
        Err(why) => send_embed(text_channel, &ctx.http, None, format!("Error joining the channel: {}", why), EmbedType::Error).await,
    }
    Ok(())
}