use songbird::input;

use crate::{Context, Error, sound};

#[poise::command(slash_command, prefix_command)]
pub async fn sb_test(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let _t = ctx.defer_or_broadcast().await?;
    let source = input::ffmpeg("test.wav").await.expect("File should be in root folder.");
    sound::internal_enqueue_source(ctx, source).await?;
    Ok(())
}