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

use std::fs::*;
use std::io::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

mod math;
mod perms;
mod cmds;
mod user;
mod reddit;
mod music;
mod spotify;

use serenity::client::{EventHandler, Context};
use serenity::framework::standard::macros::group;
use serenity::model::channel::Message;
use serenity::model::prelude::Ready;
use serenity::prelude::{TypeMapKey, GatewayIntents};
use songbird::SerenityInit;

use serenity::{Client, async_trait};
use serenity::framework::{
    StandardFramework,
};

use crate::perms::PermsContainer;
use crate::user::{load_users, save_users};
use crate::music::{
    PLAY_COMMAND,
    LEAVE_COMMAND,
    JOIN_COMMAND,
    QUEUE_COMMAND,
    SKIP_COMMAND
};

use crate::cmds::{
    REDDIT_COMMAND,
    HELP_COMMAND,
    MATH_COMMAND,
    CONVERSATIONS_COMMAND,
    PERMS_COMMAND
};

use crate::spotify::{
    SPOTIFY_TEST_COMMAND,
    FIND_SONG_COMMAND,
};

fn load_answers() -> HashMap<String, String> {
    let mut answers = HashMap::new();

    let mut content = Vec::new();
    let mut file = File::open("answers.yaml").unwrap();
    file.read_to_end(&mut content).unwrap();
    let content = &String::from_utf8(content).unwrap();

    //println!("{}",content);

    let yaml = yaml_rust::YamlLoader::load_from_str(content).unwrap();
    let yaml = yaml[0].as_hash().unwrap();
    for item in yaml {
        answers.insert(String::from(item.0.as_str().unwrap()), String::from(item.1.as_str().unwrap()));
    }

    answers
}

/*const LUDWIGS_SERVER: &str = "755492683417518281";
const KBM_SERVER:     &str = "436444150251126784";
const KRABBANS_SERVER:&str = "220112575801589760";*/

pub struct AnswerContainer;

impl TypeMapKey for AnswerContainer {
    type Value = HashMap<String, String>;
}

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, _ctx: Context, _message: Message) {}

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[group]
#[commands(
    join, leave, play, queue, skip, spotify_test, find_song
)]
struct Music;

#[group]
#[commands(
    reddit, perms, conversations, math, help
)]
struct General;

#[tokio::main]
async fn main() {
    let answers = load_answers();
    let users = load_users();
    save_users(&users);
    println!("\n\n{:?}\n\n", users);
/* 
    answers,
    users: Arc::new(Mutex::new(users)),*/
    let handler = Handler {};

    let framework = StandardFramework::new()
    .configure(|c| c.prefix("-"))
    .group(&GENERAL_GROUP)
    .group(&MUSIC_GROUP);

    let token = "NzU1NDM5ODA3MDM1MDgwODM1.X2DUJQ.r3xpo09dyOWXaqQUpifuzgcy_18";
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT | GatewayIntents::GUILD_VOICE_STATES;
    let mut client = Client::builder(&token, intents)
    .event_handler(handler)
    .framework(framework)
    .register_songbird()
    .await.expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<AnswerContainer>(answers);
        data.insert::<PermsContainer>(Arc::new(Mutex::new(users)));
    }

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}