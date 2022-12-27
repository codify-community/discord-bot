use crate::primitives::Context;
use anyhow::Result;
use poise::serenity_prelude::Member;

/// 「STAFF」Remova um usuário
#[poise::command(
    slash_command,
    prefix_command,
    default_member_permissions = "KICK_MEMBERS"
)]
pub async fn kick(
    ctx: Context<'_>,
    #[description = "Usuário"] user: Member,
    #[description = "Motivo"] reason: Option<String>,
) -> Result<()> {
    if let Some(reason) = reason {
        user.kick_with_reason(&ctx, &reason).await?;
    } else {
        user.kick(&ctx).await?;
    }

    ctx.say(format!("O usuário '{}' foi removido!", user.user.tag()))
        .await?;

    Ok(())
}
