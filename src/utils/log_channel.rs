use std::env;

use poise::serenity_prelude::Context;
use poise::serenity_prelude::{CacheHttp, CreateEmbed, Message};

#[inline]
pub async fn send(ctx: &Context, embed: CreateEmbed) -> Option<Message> {
    ctx.cache()?
        .channel(
            env::var("CODIFY_LOG_CHANNEL_ID")
                .ok()?
                .parse::<u64>()
                .ok()?,
        )?
        .id()
        .send_message(ctx.http(), |m| m.set_embed(embed))
        .await
        .ok()
}
