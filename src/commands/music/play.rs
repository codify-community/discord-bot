use crate::primitives::Context;
use anyhow::{Context as _, Result};
use std::process::Command;

#[poise::command(prefix_command, slash_command)]
/// Toca uma música
pub async fn play(
    ctx: Context<'_>,
    #[description = "URL do youtube ou nome"] song: String,
) -> Result<()> {
    let reply = ctx.say(format!("Tentando tocar `{song}`...")).await?;

    let json = Command::new("yt-dlp")
        .args(["--default-search", "ytsearch", &song, "--dump-json"])
        .output()?
        .stdout;

    let json = String::from_utf8(json)?;
    let json: serde_json::Value = serde_json::from_str(&json)?;

    let video_json = json["formats"]
        .as_array()
        .into_iter()
        .flatten()
        .find(|item| {
            item["format"]
                .as_str()
                .map_or(false, |str| str.contains("audio only"))
        })
        .context("Couldn't find the video's audio!")?;

    let url = video_json["url"]
        .as_str()
        .context("Couldn't find video url!")?;

    let guild = ctx.guild().context("No Guild!")?;

    let client = songbird::get(ctx.serenity_context())
        .await
        .context("Couldn't start songbird client")?;

    let handler = client
        .get(guild.id)
        .context("Must be in a voice channel to play music!")?;

    let mut handler = handler.lock().await;

    let mut input = songbird::ytdl(&url).await?;

    let title = json["title"].as_str().unwrap_or(&song);

    input.metadata.title = Some(title.to_string());

    handler.enqueue_source(input);

    reply
        .edit(ctx, |e| e.content(format!("Tocando `{title}`")))
        .await?;

    Ok(())
}
