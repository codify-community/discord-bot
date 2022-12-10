use crate::primitives::Context;
use anyhow::{Context as _, Result};
use poise::{
    futures_util::{stream, Stream, StreamExt},
    serenity_prelude::UserId,
};
use std::future::ready;

async fn autocomplete_bans<'a>(
    ctx: Context<'_>,
    partial: &'a str,
) -> impl Stream<Item = poise::AutocompleteChoice<UserId>> + 'a {
    stream::iter(ctx.guild().unwrap().bans(ctx).await.unwrap())
        .filter(move |ban| ready(ban.user.name.contains(partial)))
        .map(|ban| poise::AutocompleteChoice {
            name: ban.user.name,
            value: ban.user.id,
        })
}

/// 「STAFF」Bane um membro
#[poise::command(
    slash_command,
    prefix_command,
    default_member_permissions = "BAN_MEMBERS"
)]
pub async fn unban(
    ctx: Context<'_>,
    #[description = "Usuário"]
    #[autocomplete = "autocomplete_bans"]
    user: UserId,
) -> Result<()> {
    let guild = ctx.guild().context("No Guild!")?;
    guild.unban(ctx, user).await?;

    ctx.say("Usuário desbanido!").await?;

    Ok(())
}
