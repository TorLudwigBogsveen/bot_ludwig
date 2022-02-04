use rspotify::{ClientCredsSpotify, Credentials, clients::BaseClient};
use serenity::{framework::standard::{macros::command, CommandResult, Args}, model::channel::Message, client::Context, };

const ID: &str = "604111fe6a2d4ce880876b857bc6087b";
const SECRET: &str = "4c0be7d9af594303a21f5978d59dca94";

#[command]
pub async fn spotify_test() -> CommandResult {
    let mut spotify = ClientCredsSpotify::new(Credentials::new(ID, SECRET));
    spotify.request_token().await.unwrap();
    println!("{:?}", spotify);
    Ok(())
}

#[command]
pub async fn find_song(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let mut spotify = ClientCredsSpotify::new(Credentials::new(ID, SECRET));
    spotify.request_token().await.unwrap();

    let result = spotify.search(
        &args.single::<String>().unwrap(),
        &rspotify::model::SearchType::Track,
        None,
        None,
        Some(1),
        Some(0),
    ).await.unwrap();

    println!("{:?}", result);

    Ok(())
}