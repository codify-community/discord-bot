use crate::primitives::Context;
use anyhow::Result;
use poise::serenity_prelude::Member;

///〔🛡️ Administração〕 Bana um usuário
#[poise::command(slash_command, prefix_command)]
pub async fn ban(
    cx: Context<'_>,
    #[description = "Usuário"] user: Member,
    #[description = "Motivo"] reason: Option<String>,
) -> Result<()> {
    if let Some(reason) = reason {
        user.ban_with_reason(&cx, 0, reason).await?;
    } else {
        user.ban(&cx, 0).await?;
    }

    let msg = format!("Usuário {} banido!", user.display_name());
    cx.say(msg).await?;

    Ok(())
}
