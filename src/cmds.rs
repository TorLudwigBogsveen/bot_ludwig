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

use lib_rddit_v3::{url_builder, http};
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

use crate::{math, reddit, user::{User, save_users}, perms::{Permission, PermsAccount, PermsContainer}, AnswerContainer};

#[command]
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
}

#[command]
pub async fn reddit(ctx: &Context, message: &Message, mut args: Args) -> CommandResult {

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
}

#[command]
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
}

#[command]
pub async fn perms(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let data = ctx.data.read().await;
    let users = data.get::<PermsContainer>().unwrap();
    match &args.single::<String>().unwrap() as &str {
        "add" => {
            let name = args.single::<String>().unwrap();
            let perm = Permission::from(&args.single::<String>().unwrap() as &str);
            let mut user = {
                let lock = users.lock().unwrap();
                let user = lock.get(&name);
                let user = if let Some(user) = user {
                    user.clone()
                } else {
                    User::new()
                };
                user
            };
            if let Some(server) = msg.guild_id {
                let server = server.0.to_string();
                if let Some(perms) = user.server_perms_mut(&server) {
                    perms.add(perm);
                } else {
                    let mut perms = PermsAccount::new();
                    perms.add(perm);
                    user.add_perms(server, perms);
                }
            } else {
                msg.channel_id.send_message(&ctx.http, |m| m.content("You have to be in a guild chat for this function to work")).await.unwrap();
                return Ok(());
            };
            //println!("d{:?}", user);
            {
                let mut lock = users.lock().unwrap();
                lock.insert(String::from(name), user);
                save_users(&lock);
            }
            //lock.insert(String::from(name), user);

            //save_users(&lock);
        }
        "remove" => {

        },
        _=>{}
    }
    Ok(())
}

#[command]
pub async fn conversations(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let cmd = args.single::<String>().unwrap();
    let data = ctx.data.read().await;
    let answers = data.get::<AnswerContainer>().unwrap();
    match answers.get(&cmd.to_lowercase()) {
        Some(answer) => {
            msg.channel_id.send_message(&ctx.http, |m| m.content(answer)).await.unwrap();
        }
        None => {}
    }
    Ok(())
}