use songbird::input::{cached::Memory, self};

use crate::{Context, Error, music::internal_join};

#[poise::command(slash_command, prefix_command)]
pub async fn sb_test(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let guild = ctx.guild().unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx.discord()).await
        .expect("Songbird Voice client placed in at initialisation.").clone();
    let has_handler = manager.get(guild_id).is_some();
    if !has_handler {
        internal_join(ctx).await.unwrap();
    }

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;
        let source = input::ffmpeg("test.wav").await.expect("File should be in root folder.");
        handler.enqueue_source(source);


    } else {
        ctx.say("Not in a voice channel to play in").await.unwrap();
    }
    Ok(())
}