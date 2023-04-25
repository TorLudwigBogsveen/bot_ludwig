use songbird::input::{cached::Memory, File};

use crate::{sound, Context, Error};

#[poise::command(slash_command, prefix_command)]
pub async fn sb_test(ctx: Context<'_>) -> Result<(), Error> {
    let _t = ctx.defer_or_broadcast().await?;
    let ting_src = Memory::new(File::new("test.wav").into())
        .await
        .expect("These parameters are well-defined.");
    let _ = ting_src.raw.spawn_loader();
    sound::internal_enqueue_source(ctx, ting_src.into()).await?;
    Ok(())
}
