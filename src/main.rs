///! Commented lines of code use the built-in discord-gateway instead of songbird.

#[macro_use]
extern crate tracing;

use std::env;

use serenity::{
    async_trait,
    client::{Client, Context, EventHandler},
    framework::{
        standard::{
            macros::{command, group, hook},
            Args, CommandResult,
        },
        StandardFramework,
    },
    http::Http,
    model::{channel::Message, gateway::Ready, id::GuildId, misc::Mentionable},
    Result as SerenityResult,
};

use lavalink_rs::{gateway::*, model::*, LavalinkClient};
use serenity::prelude::*;
use songbird::SerenityInit;
use crate::commands::{play::PLAY_COMMAND, disconnect::DC_COMMAND, now_playing::NOW_PLAYING_COMMAND, join::JOIN_COMMAND, skip::SKIP_COMMAND, volume::VOLUME_COMMAND};

struct Lavalink;

impl TypeMapKey for Lavalink {
    type Value = LavalinkClient;
}

struct Handler;
struct LavalinkHandler;

mod util;
mod commands;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }

    async fn cache_ready(&self, _: Context, _guilds: Vec<GuildId>) {
        info!("cache is ready!");
    }
}

#[async_trait]
impl LavalinkEventHandler for LavalinkHandler {
    async fn track_start(&self, _client: LavalinkClient, event: TrackStart) {
        info!("Track started!\nGuild: {}", event.guild_id);
    }
    async fn track_finish(&self, _client: LavalinkClient, event: TrackFinish) {
        info!("Track finished!\nGuild: {}", event.guild_id);
    }
}

#[hook]
async fn after(_ctx: &Context, _msg: &Message, command_name: &str, command_result: CommandResult) {
    match command_result {
        Err(why) => info!(
            "Command '{}' returned error {:?} => {}",
            command_name, why, why
        ),
        _ => (),
    }
}

#[group]
#[only_in(guilds)]
#[commands(join, dc, play, now_playing, skip, ping, volume)]
struct General;

#[tokio::main]
//#[tracing::instrument]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env::set_var("RUST_LOG", "info,lavalink_rs=debug");
    tracing_subscriber::fmt::init();
    info!("Tracing initialized");

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let http = Http::new_with_token(&token);

    let bot_id = match http.get_current_application_info().await {
        Ok(info) => info.id,
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("<"))
        .after(after)
        .group(&GENERAL_GROUP);

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .framework(framework)
        .register_songbird()
        .await
        .expect("Err creating client");


    let lava_client = LavalinkClient::builder(bot_id)
        .set_host("lavalink") // the lavalink server should be available under the `lavalink` hostname
        .set_password(
            env::var("LAVALINK_PASSWORD").unwrap_or_else(|_| "youshallnotpass".to_string()),
        )
        .build(LavalinkHandler)
        .await?;

    {
        let mut data = client.data.write().await;
        data.insert::<Lavalink>(lava_client);
    }

    let _ = client
        .start()
        .await
        .map_err(|why| warn!("Client ended: {:?}", why));

    Ok(())
}

#[command]
async fn ping(context: &Context, msg: &Message) -> CommandResult {
    check_msg(msg.channel_id.say(&context.http, "Pong!").await);

    Ok(())
}

fn check_msg(result: SerenityResult<Message>) {
    if let Err(why) = result {
        error!("Error sending message: {:?}", why);
    }
}
