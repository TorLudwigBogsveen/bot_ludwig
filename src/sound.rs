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

use songbird::{input::Input, tracks::TrackHandle};

use crate::{Context, Error};


#[derive(Debug)]
enum SoundError {
    UserNotInChannel(String),
}

impl std::fmt::Display for SoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}

impl std::error::Error for SoundError {}


pub async fn internal_join(
    ctx: Context<'_>,
) -> Result<(), Error> {
    println!("{} used join", ctx.author().name);
    
    let guild = ctx.guild().unwrap();
    let guild_id = guild.id;

    let channel_id = guild
        .voice_states.get(&ctx.author().id)
        .and_then(|voice_state| voice_state.channel_id);

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
    let _t = ctx.defer_or_broadcast().await?;
    match internal_join(ctx).await {
        Ok(_) => { ctx.say("Joined voice channel").await?; }
        Err(err) => {
            if let Some(err) = err.downcast_ref::<SoundError>() {
                match err {
                    SoundError::UserNotInChannel(_) => {
                        ctx.say("You need to be in channel to use this Command").await?;
                    }
                }
            } else {
                Err(err)?
            }
        }
    }
    
    Ok(())
}
    

#[poise::command(slash_command, prefix_command)]
pub async fn leave(
    ctx: Context<'_>,
) -> Result<(), Error> {
    println!("{} used leave", ctx.author().name);

    let guild = ctx.guild().unwrap();
    let guild_id = guild.id;

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

pub async fn internal_enqueue_sources(
    ctx: Context<'_>,
    sources: Vec<Input>,
) -> Result<Vec<TrackHandle>, Error> {
    let guild = ctx.guild().unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx.discord()).await
        .expect("Songbird Voice client placed in at initialisation.").clone();
    let has_handler = manager.get(guild_id).is_some();
    if !has_handler {
        internal_join(ctx).await?;
    }

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;
        
        Ok(sources.into_iter().map(|source| handler.enqueue_source(source)).collect())
    } else {
        Err(Box::new(SoundError::UserNotInChannel(ctx.author().name.clone())))
    }
}

pub async fn internal_enqueue_source(
    ctx: Context<'_>,
    source: Input,
) -> Result<TrackHandle, Error> {
    let t = internal_enqueue_sources(ctx, vec![source]).await.map(|mut tracks| tracks.remove(0))?;
    t.set_volume(1.0)?;
    Ok(t)
}