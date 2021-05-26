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

use std::{fs::{read_dir, remove_file}, process::Command};

use rddit_framework_2::url::Settings;
use serenity::{client::Context, model::channel::Message};
use crate::{handler::Handler, math};

pub fn help(handler: &Handler, message: &mut Message, ctx: &mut Context) {
    message.channel_id.send_message(&ctx.http, |m| {
        let mut help = String::new();
        for kv in &handler.answers {
            help.push_str(kv.0);
            help.push_str("\n");
        }
        m.content(format!("
        Phrases:\n{}\n Matte: math/matte ekvation", help,
        )) 
    }).unwrap();
}

pub fn reddit(_handler: &Handler, args: &[&str], message: &mut Message, ctx: &mut Context) {
    message.channel_id.broadcast_typing(&ctx.http).unwrap();

    //const LIMIT: i32 = 10;
    //let mut count = 1;
    //let mut key = String::from("hot");
    //let mut sub = String::from("memes");
    //let mut timespan = String::from("day");
    /*for path in read_dir("memes").unwrap() {
        remove_file(path.unwrap().path()).unwrap();
    }*/

    let mut count = 1;

    let mut settings = Settings::new();

    for mut i in 0..args.len() {
        let arg = args[i];
        i += 1;
        match arg {
            "-s" => {
                settings.subreddit = String::from(args[i]);
            }
            "-t" => {
                settings.timespan = String::from(args[i]);
            }
            "-k" => {
                settings.sorting = String::from(args[i]);
            }
            "-c" => {
                count = args[i].parse().unwrap();
                if count > 10 {
                    count = 10;
                }
            }
            _ => { i -= 1 }
        }
    }

    let posts = rddit_framework_2::post::data(&mut settings);
    let images = rddit_framework_2::download::img_data(count, &posts);
    
    for img in images {
        message.channel_id.send_message(&ctx.http, |m| m.content(format!("{}", img.url))).unwrap();
    }

    /*let mut c = Command::new("reddit.exe");
    for arg in args {
        println!("{}", arg);
        if *arg != "-f" {
            c.arg(arg);
        }
    }

    c.arg("-f").arg("memes/");
    c.output().unwrap();*/

    /*let mut paths = Vec::new();
    for path in read_dir("memes").unwrap() {
        let path = path.unwrap().path();
        paths.push(path);
    }

    for path in &paths {
        message.channel_id.broadcast_typing(&ctx.http).unwrap();
        match message.channel_id.send_files(&ctx.http, vec![path], |m| m) {
            Ok(_) => {},
            Err(err) => {
                message.channel_id.send_message(&ctx.http, |m| m.content(format!("{}", err))).unwrap();
                println!("{}", err);

            }
        }
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }                
    
    for path in paths {
        remove_file(path).unwrap();
    }*/
}

pub fn math(handler: &Handler, args: &[&str], message: &mut Message, ctx: &mut Context) {
    let mut equation = String::new();
    for word in args {
        equation.push_str(word);
    }

    equation = equation.replace(" ", "");
    let sum = math::TokenTree::new(&equation).sum();
    message.channel_id.send_message(&ctx.http, |m| m.content(sum.to_string())).unwrap();
}

pub fn perms(handler: &Handler, args: &[&str], message: &mut Message, ctx: &mut Context) {
    /*match args[0] {
        "add" => {

        }
        "remove" => {

        }
    }*/
}

pub fn conversations(handler: &Handler, cmd: &str, message: &mut Message, ctx: &mut Context) {
    match handler.answers.get(&cmd.to_lowercase()) {
        Some(answer) => {
            message.channel_id.send_message(&ctx.http, |m| m.content(answer)).unwrap();
        }
        None => {}
    }
}