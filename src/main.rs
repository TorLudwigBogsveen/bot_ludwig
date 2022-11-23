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

/*use std::fs::*;
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
use serenity::framework::standard::CommandGroup;
use serenity::framework::standard::macros::group;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use serenity::model::{channel::Message, prelude::{Ready, GuildId, InteractionResponseType}, application::{command::Command, interaction::Interaction}};
use serenity::prelude::*;
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

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        println!("{:?}", interaction);
        
        /*if let Interaction::ApplicationCommand(command) = interaction {
            println!("Received command interaction: {:#?}", command);

            println!("{:?}", command.data.options);
            println!("DDD: {:?}", interaction.message_component().unwrap().message);

            let content = match command.data.name.as_str() {
                _ => "not implemented :(".to_string(),
            };
            /*for c in GENERAL_GROUP.options.commands {
                if (c.options.names[0] == command.data.name.as_str()) {
                    println!("{}\n", c.options.names[0]);
                    (c.fun)(&ctx, Message {});
                }
            }
            for c in MUSIC_GROUP.options.commands {
                if (c.options.names[0] == command.data.name.as_str()) {
                    println!("{}\n", c.options.names[0]);
                }
            }*/

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }*/
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        //let guild_id = GuildId(436444150251126784);//KBM
        let guild_id = GuildId(755492683417518281);

        //let commands = Command::set_global_application_commands(&ctx.http, |commands| {
        let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            let mut commands = commands;
            for c in GENERAL_GROUP.options.commands {
                commands.create_application_command(|command| command.name(c.options.names[0]).description("description"));
            }
            
            for c in MUSIC_GROUP.options.commands {
                commands = commands.create_application_command(|command| command
                    .name(c.options.names[0])
                    .description("description")
                    .create_option(|option| {
                        option
                            .name("input")
                            .description("description")
                            .kind(CommandOptionType::String)
                            .required(true)
                    }
                    ));
            }
            commands
        }).await;

        println!("I now have the following guild slash commands: {:#?}", commands);

        //println!("I created the following global slash command: {:#?}", guild_command);
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

    let token = std::env::var("DISCORD_BOT_TOKEN").unwrap();
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
}*/

mod music;
mod spotify;

use music::*;
use spotify::*;

use poise::{serenity_prelude as serenity, PrefixFrameworkOptions};
use songbird::SerenityInit;


pub struct Handler;

pub struct Data {} // User data, which is stored and accessible in all command invocations
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(prefix_command)]
pub async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let intents = serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT | serenity::GatewayIntents::GUILD_VOICE_STATES;
    let mut prefix = PrefixFrameworkOptions::default();
    prefix.prefix = Some(String::from("-"));

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![register(), join(), play(), skip(), queue(), leave(), find_song(), spotify_test(), spotify_playlist()],
            prefix_options: prefix,
            ..Default::default()
        })
        .token(std::env::var("DISCORD_BOT_TOKEN").expect("missing DISCORD_TOKEN"))
        .intents(intents)
        .client_settings(|builder| builder.register_songbird())
        .user_data_setup(move |_ctx, _ready, _framework| Box::pin(async move { Ok(Data {}) }));

    if let Err(why) = framework.run().await {
        println!("Client error: {:?}", why);
    }
}