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

use music::*;
use soundboard::sb_test;
use spotify::*;

use poise::{serenity_prelude as serenity, PrefixFrameworkOptions};
use songbird::SerenityInit;


pub struct Handler;

pub struct Data {} // User data, which is stored and accessible in all command invocations
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(prefix_command)]
pub async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let intents = serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT | serenity::GatewayIntents::GUILD_VOICE_STATES | serenity::GatewayIntents::GUILD_EMOJIS_AND_STICKERS;
    let mut prefix = PrefixFrameworkOptions::default();
    prefix.prefix = Some(String::from("-"));

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![register(), join(), play(), skip(), queue(), leave(), find_song(), spotify_test(), spotify_playlist(), sb_test()],
            prefix_options: prefix,
            ..Default::default()
        })
        //.token(std::env::var("DISCORD_BOT_TOKEN").expect("missing DISCORD_TOKEN"))
        .token("NzU1NDM5ODA3MDM1MDgwODM1.GOVTgS.LOd6awZfm61S4GkH5SNgZFrn2czuz7Mx_McuiA")
        .intents(intents)
        .client_settings(|builder| builder.register_songbird())
        .user_data_setup(move |_ctx, _ready, _framework| Box::pin(async move { Ok(Data {}) }));

    if let Err(why) = framework.run().await {
        println!("Client error: {:?}", why);
    }
}