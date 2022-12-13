/*
 *   Copyright (c) 2021 Ludwig Bogsveen
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
use songbird::{input::ytdl_search};

use crate::{Context, Error};

async fn internal_join(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let guild = ctx.guild().unwrap();
    let guild_id = guild.id;

    let channel_id = guild
        .voice_states.get(&ctx.author().id)
        .and_then(|voice_state| voice_state.channel_id);

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            ctx.say("Not in a voice channel").await.unwrap();
            return Ok(());
        }
    };

    let manager = songbird::get(ctx.discord()).await
        .expect("Songbird Voice client placed in at initialisation.").clone();

    let _handler = manager.join(guild_id, connect_to).await;

    println!("{} used join", ctx.author().name);
    ctx.say(&format!("Joined voice channel")).await.unwrap();
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn join(
    ctx: Context<'_>,
) -> Result<(), Error> {
    internal_join(ctx).await
}
    

#[poise::command(slash_command, prefix_command)]
pub async fn leave(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let guild = ctx.guild().unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx.discord()).await
        .expect("Songbird Voice client placed in at initialisation.").clone();
    let has_handler = manager.get(guild_id).is_some();

    if has_handler {
        if let Err(e) = manager.remove(guild_id).await {
            ctx.say(format!("Failed: {:?}", e)).await.unwrap();
        }

        println!("{} used leave", ctx.author().name);
        ctx.say("Left voice channel").await.unwrap();
    } else {
        ctx.say("Not in a voice channel").await.unwrap();
    }
    Ok(())
}

pub async fn internal_play_many(
    ctx: Context<'_>,
    songs: Vec<String>,
) -> Result<(), Error> {
    let guild = ctx.guild().unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx.discord()).await
        .expect("Songbird Voice client placed in at initialisation.").clone();
    let has_handler = manager.get(guild_id).is_some();
    if !has_handler {
        internal_join(ctx).await.unwrap();
    }

    let manager = songbird::get(ctx.discord()).await
        .expect("Songbird Voice client placed in at initialisation.").clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;
        for url in &songs {
            let source = if url.starts_with("http") || url.starts_with("https") {
                match songbird::ytdl(&url).await {
                    Ok(source) => source,
                    Err(why) => {
                        println!("Err starting source: {:?}", why);

                        ctx.say("Error sourcing ffmpeg").await.unwrap();

                        return Ok(());
                    },
                }
            } else {
                //println!("url:{}", &url);
                match ytdl_search(&url).await {
                    Ok(source) => source,
                    Err(why) => {
                        println!("Err starting source: {:?}", why);

                        ctx.say("Error sourcing ffmpeg").await.unwrap();

                        return Ok(());
                    },
                }
            };
            handler.enqueue_source(source);

            //ctx.say(&format!("Added \"{}\" to the queue.\n{}", title, url)).await.unwrap();
        }
    } else {
        ctx.say("Not in a voice channel to play in").await.unwrap();
    }
    ctx.say(&format!("Added {} songs to the queue.", songs.len())).await.unwrap();
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn play(
    ctx: Context<'_>,
    #[description = "Song title or yt-link"] song: String,
) -> Result<(), Error> {
    let url = song;

    let guild = ctx.guild().unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx.discord()).await
        .expect("Songbird Voice client placed in at initialisation.").clone();
    let has_handler = manager.get(guild_id).is_some();
    if !has_handler {
        internal_join(ctx).await.unwrap();
    }

    let manager = songbird::get(ctx.discord()).await
        .expect("Songbird Voice client placed in at initialisation.").clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;
        let source = if url.starts_with("http") || url.starts_with("https") {
            match songbird::ytdl(&url).await {
                Ok(source) => source,
                Err(why) => {
                    println!("Err starting source, url: \"{}\", reason: {:?}", &url, why);

                    ctx.say("Error sourcing ffmpeg").await.unwrap();

                    return Ok(());
                },
            }
        } else {
            match ytdl_search(&url).await {
                Ok(source) => source,
                Err(why) => {
                    println!("Err starting source, url: \"{}\", reason: {:?}", &url, why);

                    ctx.say("Error sourcing ffmpeg").await.unwrap();

                    return Ok(());
                },
            }
        };
        let url = source.metadata.source_url.as_ref().unwrap().clone();
        let title = source.metadata.title.as_ref().unwrap().clone();
        handler.enqueue_source(source);

        println!("{} added \"{}\" to the queue{}", ctx.author().name, title, url);
        ctx.say(&format!("Added \"{}\" to the queue.\n{}", title, url)).await.unwrap();
    } else {
        ctx.say("Not in a voice channel to play in").await.unwrap();
    }
    Ok(())
}


#[poise::command(slash_command, prefix_command)]
pub async fn skip(
    ctx: Context<'_>,
    #[description="Amount of songs to skip"] amount: Option<usize>,
) -> Result<(), Error> {
    let amount = if let Some(num) = amount {
       num
    } else {
        1
    };

    let guild = ctx.guild().unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx.discord()).await
        .expect("Songbird Voice client placed in at initialisation.").clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        let num_skipped_tracks = amount.min(handler.queue().len());

        for _i in 0..num_skipped_tracks { 
           handler.queue().skip().unwrap();
        }

        println!("{} skipped {} songs", ctx.author().name, num_skipped_tracks);
        ctx.say(&format!("Skipping {} songs", num_skipped_tracks)).await.unwrap();
    } else {
        ctx.say("Not playing any songs able to skip").await.unwrap();
    }

    Ok(())
}


#[poise::command(slash_command, prefix_command)]
pub async fn queue(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let guild = ctx.guild().unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx.discord()).await
        .expect("Songbird Voice client placed in at initialisation.").clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        
        let tracks = handler.queue().current_queue()
        .iter()
        .enumerate()
        .map(|(i, track)| 
            format!("{}\t:\t\"{}\"\n", i+1, track.metadata().title.as_ref().unwrap())
        )
        .collect::<Vec<String>>().concat();

        println!("{} used queue\n```Nth\t:\tTitle\n{}```", ctx.author().name, tracks);
        ctx.say(&format!("```Nth\t:\tTitle\n{}```", tracks)).await.unwrap();
    } else {
        ctx.say("Not playing any songs").await.unwrap();
    }
    Ok(())
}