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

use serenity::{
    client::{Context},
    framework::{
        standard::{
            macros::{command},
            CommandResult, Args,
        },
    },
    model::{channel::Message},
};

use songbird::{input::ytdl_search};

#[command]
pub async fn join(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let channel_id = guild
        .voice_states.get(&msg.author.id)
        .and_then(|voice_state| voice_state.channel_id);

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            msg.reply(ctx, "Not in a voice channel").await.unwrap();
            return Ok(());
        }
    };

    let manager = songbird::get(ctx).await
        .expect("Songbird Voice client placed in at initialisation.").clone();

    let _handler = manager.join(guild_id, connect_to).await;
    Ok(())
}

#[command]
pub async fn leave(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx).await
        .expect("Songbird Voice client placed in at initialisation.").clone();
    let has_handler = manager.get(guild_id).is_some();

    if has_handler {
        if let Err(e) = manager.remove(guild_id).await {
            msg.channel_id.say(&ctx.http, format!("Failed: {:?}", e)).await.unwrap();
        }

        msg.channel_id.say(&ctx.http, "Left voice channel").await.unwrap();
    } else {
        msg.reply(ctx, "Not in a voice channel").await.unwrap();
    }
    Ok(())
}

#[command]
pub async fn play(ctx: &Context, msg: &Message, args: Args) -> CommandResult {    
    let url = args.rest();

    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx).await
        .expect("Songbird Voice client placed in at initialisation.").clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;
        let source = if url.starts_with("http") {
            match songbird::ytdl(&url).await {
                Ok(source) => source,
                Err(why) => {
                    println!("Err starting source: {:?}", why);

                    msg.channel_id.say(&ctx.http, "Error sourcing ffmpeg").await.unwrap();

                    return Ok(());
                },
            }
        } else {
            //println!("url:{}", &url);
            match ytdl_search(&url).await {
                Ok(source) => source,
                Err(why) => {
                    println!("Err starting source: {:?}", why);

                    msg.channel_id.say(&ctx.http, "Error sourcing ffmpeg").await.unwrap();

                    return Ok(());
                },
            }
        };
        handler.enqueue_source(source);

        msg.channel_id.say(&ctx.http, "Playing song").await.unwrap();
    } else {
        msg.channel_id.say(&ctx.http, "Not in a voice channel to play in").await.unwrap();
    }
    Ok(())
}

#[command]
pub async fn skip(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let amount = if let Ok(num) = args.single::<usize>() {
       num
    } else {
        1
    };

    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx).await
        .expect("Songbird Voice client placed in at initialisation.").clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        let num_skipped_tracks = if amount < handler.queue().len() {
            amount
        } else {
            handler.queue().len()
        };


        for _i in 0..amount { 
           handler.queue().skip().unwrap();
        }


        msg.channel_id.say(&ctx.http, &format!("Skipping {} songs", num_skipped_tracks)).await.unwrap();
    } else {
        msg.channel_id.say(&ctx.http, "Not playing any songs able to skip").await.unwrap();
    }

    Ok(())
}

#[command]
pub async fn queue(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx).await
        .expect("Songbird Voice client placed in at initialisation.").clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        
        let tracks = handler.queue().current_queue()
        .iter()
        .map(|track| 
            format!("{}\n", track.metadata().track.as_ref().unwrap())
        )
        .collect::<Vec<String>>().concat();


        msg.channel_id.say(&ctx.http, &tracks).await.unwrap();
    } else {
        msg.channel_id.say(&ctx.http, "Not playing any songs").await.unwrap();
    }
    Ok(())
}