use std::{collections::HashMap, hash::Hash, time::Duration};

use poise::{CreateReply, serenity_prelude::{CreateButton, CreateActionRow, MessageId}, futures_util::StreamExt};
use serde_json::{Value, json, to_writer_pretty};
use songbird::input::{cached::Memory, File, Input, YoutubeDl, Compose};

use crate::{sound::{self, SoundError, internal_enqueue_source, download_sound, download_sound_metadata}, Context, Error};

#[derive(Debug)]
pub enum SoundBoardError {
    SoundWithNameAlreadyExists
}

impl std::fmt::Display for SoundBoardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}

impl std::error::Error for SoundBoardError {}


pub struct Sound {
    name: String,
    path: String,
    src: Memory,
}

impl Sound {
    async fn load(name: String, path: &str) -> Sound {
        //println!("sounds/{}", path);
        let ting_src = Memory::new(File::new(format!("sounds/{}", path)).into())
                .await
                .expect("These parameters are well-defined.");
        let _ = ting_src.raw.spawn_loader();

        Sound {
            name,
            path: path.to_string(),
            src: ting_src
        }
    }
}

pub struct SoundBoard(HashMap<String, Sound>);

impl SoundBoard {
    pub async fn load() -> Result<SoundBoard, Error>  {
        let json = serde_json::from_str::<Value>(&std::fs::read_to_string("soundboard.json")?)?;

        let mut sounds = HashMap::new();

        for sound in json.as_array().ok_or(SoundError::SoundCouldNotBeLoaded)? {
            let path = sound["path"].as_str().ok_or(SoundError::SoundCouldNotBeLoaded)?;
            let name = sound["name"].as_str().ok_or(SoundError::SoundCouldNotBeLoaded)?.to_string();

            sounds.insert(name.to_owned(), Sound::load(name, path).await);
        }

        Ok(SoundBoard(sounds))
    }

    pub fn save(&self) -> Result<(), Error> {
        let mut json = Vec::new();

        for sound in self.0.values() {
            json.push(json!({
                "path": sound.path,
                "name": sound.name,
            }));
        }

        let json = Value::Array(json);

        let file = std::fs::File::create("soundboard.json")?;

        to_writer_pretty(file, &json)?;

        Ok(())
    }
}

#[poise::command(slash_command, prefix_command)]
pub async fn create_soundboard(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let t = ctx.defer_or_broadcast().await?;
    let soundboard = ctx.data().soundboard.lock().await;

    let mut rows = Vec::new();

    let mut buttons = Vec::new();

    for (name, _sound) in &soundboard.0 {
        buttons.push(
            CreateButton::new(name)
                .label(name)
        );

        if buttons.len() >= 4 {
            rows.push(CreateActionRow::Buttons(buttons));
            buttons = Vec::new();
        }
    }

    rows.push(CreateActionRow::Buttons(buttons));

    let reply = CreateReply::new()
    .components(rows);

    let handle = ctx.send(reply
    ).await.unwrap();

    drop(t);

    // Wait for multiple interactions
    let mut interaction_stream =
    handle.message().await?.await_component_interactions(&ctx.discord()).timeout(Duration::from_secs(60 * 3)).stream();

    while let Some(interaction) = interaction_stream.next().await {
        interaction.defer(ctx.cache_and_http()).await?;
        let id = interaction.data.custom_id;
        let handle = internal_enqueue_source(ctx, soundboard.0.get(&id).unwrap().src.new_handle().into()).await?;
        handle.play()?;
    }
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn add_sound(
    ctx: Context<'_>,
    name: String,
    song: String,
) -> Result<(), Error> {
    let t = ctx.defer_or_broadcast().await?;

    let url = if song.starts_with("http") || song.starts_with("https") {
        song.to_string()
    } else {
        format!("ytsearch:{song}")
    };

    //let meta_data = download_sound_metadata(&url).await?;

    download_sound(&url, &name).await?;

    let mut soundboard = ctx.data().soundboard.lock().await;
    
    if soundboard.0.contains_key(&song) {
        Err(Box::new(SoundBoardError::SoundWithNameAlreadyExists))?
    }

    soundboard.0.insert(name.clone(), Sound::load(name.clone(), &format!("{}.mp3", &name)).await);
    soundboard.save()?;

    ctx.say("Successfully added to the soundboard").await?;
    drop(t);

    Ok(())
}