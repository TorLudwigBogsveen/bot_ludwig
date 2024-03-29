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

use std::env::Args;

use librddit::{url_builder, http};
use serenity::builder::GetMessages;

use crate::{Context, Error};

/*#[command]
pub async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let answers = data.get::<AnswerContainer>().unwrap();

    msg.channel_id.send_message(&ctx.http, |m| {
        let mut help = String::new();
        for kv in answers {
            help.push_str(kv.0);
            help.push_str("\n");
        }
        m.content(format!("
        Phrases:\n{}\n Matte: math/matte ekvation", help,
        )) 
    }).await.unwrap();
    Ok(())
}*/

/*#[poise::command(slash_command, prefix_command)]
pub async fn reddit(ctx: Context<'_>, message: &poise::serenity_prelude::Message, mut args: Args) -> Result<(), Error> {

    message.channel_id.broadcast_typing(&ctx.http).await.unwrap();

    //const LIMIT: i32 = 10;
    //let mut count = 1;
    //let mut key = String::from("hot");
    //let mut sub = String::from("memes");
    //let mut timespan = String::from("day");
    /*for path in read_dir("memes").unwrap() {
        remove_file(path.unwrap().path()).unwrap();
    }*/

    let mut count = 1;
    let mut sub = None;
    let mut timespan = None;
    let mut sorting = None;
    let mut content_type = String::from("image");

    while !args.is_empty() {
        let arg = args.single::<String>().unwrap();
        //println!("{}", &arg);
        match &arg as &str {
            "-i" => {
                content_type = args.single::<String>().unwrap();
            }
            "-s" => {
                sub = Some(args.single::<String>().unwrap());
            }
            "-t" => {
                timespan = Some(args.single::<String>().unwrap());
            }
            "-k" => {
                sorting = Some(args.single::<String>().unwrap());
            }
            "-c" => {
                count = args.single::<u32>().unwrap();
                if count > 10 {
                    count = 10;
                }
            }
            _ => {}
        }
    }

    let limit = 40;
    let sub         =  if let Some(sub) = sub { sub } else { String::from("dankmemes") };
    let sorting         =  if let Some(sorting) = sorting { sorting } else { String::from("new") };
    let timespan    =  if let Some(timespan) = timespan { timespan } else { String::from("day") };
    let ub = url_builder::URLBuilder::new(
        sub,
        Some(url_builder::Sorting::from_string(sorting.to_lowercase())),
        Some(limit),
    );
    let posts = http::fetch(ub.build()).await;
    
    for post in posts {
        if count == 0 {
            return Ok(())
        }

        if &content_type == "image" {
            if post.data.post_hint.as_ref().unwrap() == "image" {
                message.channel_id.send_message(&ctx.http, |m| m.content(format!("{}", post.data.url.clone()))).await.unwrap();
                count -= 1;
            }
        } else if &content_type == "text" {
            let text = post.data.selftext.unwrap().chars().collect::<Vec<char>>();
            for i in 0..(text.len() / 2000) {
                let stop = 2000.min(text.len() - i * 2000);
                let part = text[(i*2000)..stop].iter().collect::<String>();
                message.channel_id.send_message(&ctx.http, |m| m.content(format!("{:?}", part))).await.unwrap();
            }
            //message.channel_id.send_message(&ctx.http, |m| m.content(format!("{}", post.selftext))).await.unwrap();
            count -= 1;
        }
    }
    Ok(())
}*/

/*#[command]
pub async fn math(ctx: &Context, message: &Message, mut args: Args) -> CommandResult {
    let mut equation = String::new();
    for word in args.iter::<String>() {
        //println!("{:?}", word);
        equation.push_str(&word.unwrap());
    }

    equation = equation.replace(" ", "");
    let sum = math::TokenTree::new(&equation).sum();
    message.channel_id.send_message(&ctx.http, |m| m.content(sum.to_string())).await.unwrap();


    Ok(())
}*/

#[poise::command(slash_command, prefix_command)]
pub async fn clear(
    ctx: Context<'_>,
    #[description="Amount of messages to remove, default 1"] amount: Option<u8>,
) -> Result<(), Error> {
    let amount = amount.unwrap_or(1);
    let _t = ctx.defer_or_broadcast().await?;
    let channel = ctx.channel_id();
    let messages = channel.messages(ctx, GetMessages::new().limit(amount+1)).await?;
    let mut messages = messages.into_iter();
    messages.next();
    for message in messages {
        message.delete(ctx).await?;
    }
    ctx.say(format!("Cleared {} messages", amount)).await?;
    Ok(())
}