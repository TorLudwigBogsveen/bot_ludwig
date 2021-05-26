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

use std::collections::HashMap;

use serenity::{client::{Context, EventHandler}, model::{channel::Message, prelude::Ready}, prelude::RwLock};

use crate::{cmds::{self, conversations, help, math, perms, reddit}, math, perms::{Permission, PermsAccount}, user::User};
pub struct Handler {
    pub answers: HashMap<String, String>,
    pub users: HashMap<String, User>,
}

impl EventHandler for Handler {
    fn message(&self, ctx: Context, message: Message) {
        let mut ctx = ctx;
        let mut message = message;
        
        let words = message.content.clone();//.to_lowercase();
        let words: Vec<&str> = words.split_whitespace().collect();

        //println!("G:{}", message.guild_id.unwrap().to_string());

        if words.len() == 0 {
            return;
        }

        if words[0] != "cmd" {
            return;
        }

        let user = self.users.get(&message.author.tag());

        if user.is_none() {
            message.channel_id.send_message(&ctx.http, |m| { m.content("You do not have permission to use that command on this server!") }).unwrap();
            return;
        }

        let user = user.unwrap();
        

        let mut perm_acc = user.server_perms(&message.guild_id.unwrap().to_string());

        match perm_acc {
            Some(_) => {},
            None => {
                perm_acc = user.server_perms("*");
            }
        };

        if perm_acc.is_none() {
            message.channel_id.send_message(&ctx.http, |m| { m.content("You do not have permission to use that command on this server!") }).unwrap();
            return;
        }

        let perm_acc = perm_acc.unwrap();
        
        match words[1] {
            "hjälp" | "help" => {
                if perm_acc.has(Permission::Owner) || perm_acc.has(Permission::Admin) || perm_acc.has(Permission::Help) {
                    help(self, &mut message, &mut ctx);
                } else {
                    message.channel_id.send_message(&ctx.http, |m| m.content("You do not have permission to use that command on this server!")).unwrap();
                }
            }
            "reddit" => {
                if perm_acc.has(Permission::Owner) || perm_acc.has(Permission::Admin) || perm_acc.has(Permission::Reddit) {
                    reddit(self, &words[2..], &mut message, &mut ctx);
                } else {
                    message.channel_id.send_message(&ctx.http, |m| m.content("You do not have permission to use that command on this server!")).unwrap();
                }
            }
            "matte" => {
                if perm_acc.has(Permission::Owner) || perm_acc.has(Permission::Admin) || perm_acc.has(Permission::Math) {
                    math(self, &words[2..], &mut message, &mut ctx);
                } else {
                    message.channel_id.send_message(&ctx.http, |m| m.content("You do not have permission to use that command on this server!")).unwrap();
                }
            }
            "perms" => {
                if perm_acc.has(Permission::Owner) || perm_acc.has(Permission::Admin) {
                    perms(self, &words[2..], &mut message, &mut ctx);
                } else {
                    message.channel_id.send_message(&ctx.http, |m| m.content("You do not have permission to use that command on this server!")).unwrap();
                }
            }
            _=> {
                if perm_acc.has(Permission::Owner) || perm_acc.has(Permission::Admin) || perm_acc.has(Permission::Conversation) {
                    conversations(self, words[1], &mut message, &mut ctx);
                } else {
                    message.channel_id.send_message(&ctx.http, |m| m.content("You do not have permission to use that command on this server!")).unwrap();
                }
            }
        }

        //println!("Message: {}", message.content);
    }

    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}