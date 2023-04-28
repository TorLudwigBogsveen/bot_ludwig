/*
 *   Copyright (c) 2023 Ludwig Bogsveen
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

use std::{io::ErrorKind};

use serde_json::from_slice;
use songbird::{input::{Input, AudioStreamError, YoutubeDl, Compose, AuxMetadata}, tracks::TrackHandle};
use tokio::process::Command;

use crate::{Context, Error};


#[derive(Debug)]
pub enum SoundError {
    UserNotInChannel(String),
    SoundCouldNotBeLoaded,
}

impl std::fmt::Display for SoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}

impl std::error::Error for SoundError {}

pub async fn download_sound_metadata(url: &str) -> Result<AuxMetadata, AudioStreamError>{
    let mut source = YoutubeDl::new_ytdl_like("yt-dlp", reqwest::Client::new(), url.to_owned());
    source.aux_metadata().await
}

pub async fn download_sound(url: &str, name: &str) -> Result<(), AudioStreamError> {
    let program = "yt-dlp";

    let ytdl_args = [
        url,
        "-f",
        //"'ba[abr>0][vcodec=none]/best'",
        "ba",
        "--no-playlist",
        "-x",
        "-P",
        "sounds",
        "-o",
        name,
        "--audio-format",
        "mp3",
    ];

    /*let a = ytdl_args.concat();
    println!("{program} {a}");*/

    let _output = Command::new(program)
        .args(ytdl_args)
        .output()
        .await
        .map_err(|e| {
            AudioStreamError::Fail(if e.kind() == ErrorKind::NotFound {
                format!("could not find executable '{}' on path", program).into()
            } else {
                Box::new(e)
            })
        })?;
    /*println!("stderr of ls: {:?}", std::str::from_utf8(&output.stderr[..]));
    println!("stdout of ls: {:?}", std::str::from_utf8(&output.stdout[..]));
    println!("status of ls: {:?}", output.status);*/
    
    Ok(())
}


pub async fn internal_join(
    ctx: Context<'_>,
) -> Result<(), Error> {
    println!("{} used join", ctx.author().name);
    
    let channel_id = {
        let guild = ctx.guild().unwrap();
        guild
        .voice_states.get(&ctx.author().id)
        .and_then(|voice_state| voice_state.channel_id)
    };

    let guild_id = ctx.guild_id().unwrap();

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            Err(Box::new(SoundError::UserNotInChannel(ctx.author().name.clone())))?
        }
    };

    let manager = songbird::get(ctx.discord()).await
        .expect("Songbird Voice client placed in at initialisation.").clone();

    let _handler = manager.join(guild_id, connect_to).await;

    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn join(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let t = ctx.defer_or_broadcast().await?;
    match internal_join(ctx).await {
        Ok(_) => { ctx.say("Joined voice channel").await?; }
        Err(err) => {
            if let Some(err) = err.downcast_ref::<SoundError>() {
                match err {
                    SoundError::UserNotInChannel(_) => {
                        ctx.say("You need to be in channel to use this Command").await?;
                    }
                    _ => panic!()
                }
            } else {
                Err(err)?
            }
        }
    }
    drop(t);
    Ok(())
}
    

#[poise::command(slash_command, prefix_command)]
pub async fn leave(
    ctx: Context<'_>,
) -> Result<(), Error> {
    println!("{} used leave", ctx.author().name);

    let guild_id = ctx.guild_id().unwrap();

    let manager = songbird::get(ctx.discord()).await
        .expect("Songbird Voice client placed in at initialisation.").clone();
    let has_handler = manager.get(guild_id).is_some();

    if has_handler {
        if let Err(e) = manager.remove(guild_id).await {
            ctx.say(format!("Failed: {:?}", e)).await.unwrap();
        }
        
        ctx.say("Left voice channel").await.unwrap();
    } else {
        ctx.say("Not in a voice channel").await.unwrap();
    }
    Ok(())
}

pub async fn internal_enqueue_source(
    ctx: Context<'_>,
    source: Input,
) -> Result<TrackHandle, Error> {
    let guild_id = ctx.guild_id().unwrap();

    let manager = songbird::get(ctx.discord()).await
        .expect("Songbird Voice client placed in at initialisation.").clone();
    let has_handler = manager.get(guild_id).is_some();
    if !has_handler {
        internal_join(ctx).await?;
    }

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;
        Ok(handler.enqueue_input(source).await)
    } else {
        Err(Box::new(SoundError::UserNotInChannel(ctx.author().name.clone())))
    }
}