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

use songbird::input::{YoutubeDl, Compose};

use crate::{Context, Error, sound::internal_enqueue_source};

pub async fn internal_play(
    ctx: Context<'_>,
    song: String,
) -> Result<(), Error> {
    let t = ctx.defer_or_broadcast().await?;

    let url = song;
    let mut source = if url.starts_with("http") || url.starts_with("https") {
        YoutubeDl::new_ytdl_like("yt-dlp", reqwest::Client::new(), url) 
    } else {
        YoutubeDl::new_ytdl_like("yt-dlp", reqwest::Client::new(), format!("ytsearch:{url}"))
    };
    let meta: songbird::input::AuxMetadata = source.aux_metadata().await?;
    let url = meta.source_url.unwrap();
    let title = meta.title.as_ref().unwrap().clone();
    internal_enqueue_source(ctx, source.into()).await?;
    println!("{} added \"{}\" to the queue{}", ctx.author().name, title, url);
    ctx.say(&format!("Added \"{}\" to the queue.\n{}", title, url)).await.unwrap();
    drop(t);
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn play(
    ctx: Context<'_>,
    #[description = "Song title or yt-link"] song: String,
) -> Result<(), Error> {
   internal_play(ctx, song).await
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

    let guild_id = ctx.guild_id().unwrap();

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
    let guild_id = ctx.guild_id().unwrap();

    let manager = songbird::get(ctx.discord()).await
        .expect("Songbird Voice client placed in at initialisation.").clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        
        let tracks = handler.queue().current_queue()
        .iter()
        .enumerate()
        .map(|(i, track)| 
            {
                format!("{}\t:\t\"{}\"\n", i+1, todo!())
            }
        )
        .collect::<Vec<String>>().concat();

        println!("{} used queue\n```Nth\t:\tTitle\n{}```", ctx.author().name, tracks);
        ctx.say(&format!("```Nth\t:\tTitle\n{}```", tracks)).await.unwrap();
    } else {
        ctx.say("Not playing any songs").await.unwrap();
    }
    Ok(())
}