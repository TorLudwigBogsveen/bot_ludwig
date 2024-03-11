/*
 *   Copyright (c) 2020 Ludwig Bogsveen
 *   All rights reserved.

 *   Permission is hereby granted, free of charge, to any person obtaining a copy
 *   of this software and associated documentation files (the "Software"), to deal
 *   in the Software without restriction, including without limitation the rights
 *   to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 *   copies of the Software, and to permit persons to whom the Software is
 *   furnished to do so, subject to the following conditions:
 
 *   The above copyright notice and this permission notice shall be included in all
 *   copies or substantial portions of the Software.
 
 *   THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 *   IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 *   FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 *   AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 *   LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 *   OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 *   SOFTWARE.
 */

mod music;
mod spotify;
mod soundboard;
mod sound;
mod cmds;

use std::sync::Arc;

use cmds::clear;
use music::*;
use sound::{join, leave};
use soundboard::{create_soundboard, SoundBoard, add_sound};
use spotify::*;

use poise::{futures_util::lock::Mutex, serenity_prelude::{self as serenity, FullEvent}, Framework, FrameworkContext, FrameworkOptions, PrefixFrameworkOptions};
use songbird::SerenityInit;


pub struct Handler;

pub struct Data {
    soundboard: Arc<Mutex<SoundBoard>>,
} // User data, which is stored and accessible in all command invocations
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(prefix_command)]
pub async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}

pub async fn listener(
    	ctx: &serenity::Context,
        event: &FullEvent,
        _framework: poise::FrameworkContext<'_, Data, Error>,
        _data: &Data
    ) -> Result<(), Error> {
        match event {
            FullEvent::Ready { data_about_bot } => {
                println!("{} is connected!", data_about_bot.user.name);
            },
            FullEvent::VoiceStateUpdate { old: _, new } => {
                if new.channel_id.is_none() {
                    let sb = songbird::get(ctx).await.expect("No songbird initialised").clone();
                    match sb.get(new.guild_id.unwrap()) {
                        Some(c) => {
                            //TODO: FIX
                            //let mut call = c.lock().await;
                            //call.queue().stop();
                            //call.leave().await?;
                        },
                        None => {
                            println!("No call on dc");
                        }
                    }
                }
            }
            _ => {}
        }
        //println!("{:?}",event);
        Ok(())
    }
    

#[tokio::main]
async fn main() {
    let intents = serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT | serenity::GatewayIntents::GUILD_VOICE_STATES | serenity::GatewayIntents::GUILD_EMOJIS_AND_STICKERS;
    let mut prefix = PrefixFrameworkOptions::default();
    prefix.prefix = Some(String::from("-"));

    let soundboard = Arc::new(Mutex::new(SoundBoard::load().await.unwrap()));

    let framework = Framework::new(
        FrameworkOptions {
            commands: vec![register(), clear(), join(), play(), skip(), queue(), leave(), find_song(), spotify_test(), spotify_playlist(), create_soundboard(), add_sound()],
            event_handler: |ctx, event, framework, data| {
                Box::pin(listener(ctx, event, framework, data))
            },
            prefix_options: prefix,
            ..Default::default()
        },
        move |_ctx, _ready, _framework| Box::pin(async move { Ok(Data { soundboard }) })
    );
    let token = std::env::args().next().unwrap();
    let mut client = serenity::Client::builder(token, intents)
        .framework(framework)
        .register_songbird()
        .await
        .unwrap();
    client.start_autosharded().await.unwrap();
}