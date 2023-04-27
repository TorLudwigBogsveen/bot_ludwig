use poise::futures_util::{TryStreamExt, StreamExt};
use rspotify::{ClientCredsSpotify, Credentials, clients::BaseClient, model::{PlaylistItem, PlaylistId}, prelude::Id};

use crate::{Context, Error, sound::internal_enqueue_source, music::internal_play};

const ID: &str = "604111fe6a2d4ce880876b857bc6087b";
const SECRET: &str = "4c0be7d9af594303a21f5978d59dca94";

#[poise::command(slash_command, prefix_command)]
pub async fn spotify_test(
    _ctx: Context<'_>,
) -> Result<(), Error> {
    let mut spotify = ClientCredsSpotify::new(Credentials::new(ID, SECRET));
    spotify.request_token().await.unwrap();
    println!("{:?}", spotify);
    Ok(())
}


#[poise::command(slash_command, prefix_command)]
pub async fn find_song(
    _ctx: Context<'_>,
    #[description = "Song title"] song: String,
) -> Result<(), Error> {
    let mut spotify = ClientCredsSpotify::new(Credentials::new(ID, SECRET));
    spotify.request_token().await?;

    let result = spotify.search(
        &song,
        rspotify::model::SearchType::Track,
        None,
        None,
        Some(1),
        Some(0),
    ).await?;

    println!("{:?}", result);

    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn spotify_playlist(
    ctx: Context<'_>,
    #[description = "Playlist link"] playlist: String,
) -> Result<(), Error> {
    let mut spotify = ClientCredsSpotify::new(Credentials::new(ID, SECRET));
    spotify.request_token().await?;

    println!("[{}]", playlist);

    let id = PlaylistId::from_id(&playlist)?;
    let playlist = spotify.playlist(id, None, None).await?;
    let mut tracks = vec![];
    for track in playlist.tracks.items {
        match track.track.unwrap() {
            rspotify::model::PlayableItem::Track(track) => {
                //println!("{}", track.name);
                tracks.push(track.name);
            },
            rspotify::model::PlayableItem::Episode(_) => todo!(),
        }
    }

    for track in tracks {
        internal_play(ctx, track).await?;
    }
    Ok(())
}