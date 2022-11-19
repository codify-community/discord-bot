use crate::{primitives::Context, utils::time::get_discord_relative_time};
use anyhow::Result;
use poise::serenity_prelude as serenity;

/// [🧰 utilidades] Pegue as informações de um usuário
#[poise::command(slash_command, prefix_command)]
pub async fn userinfo(
    cx: Context<'_>,
    #[description = "Selecione o usuário"] user: Option<serenity::User>,
) -> Result<()> {
    let user = user.as_ref().unwrap_or_else(|| cx.author());
    let guild = cx.partial_guild().await.unwrap();

    let user_name = format!("`{}`", user.name);
    let user_tag = format!("`{}`", user.tag());
    let user_id = format!("`{}`", user.id);

    let member = guild.member(cx, user.id).await.unwrap();

    let joined_at_timestamp: i64 = member.joined_at.unwrap().timestamp();
    let joined_at = get_discord_relative_time(joined_at_timestamp);

    let account_age_timestamp: i64 = user.created_at().timestamp();
    let account_age = get_discord_relative_time(account_age_timestamp);

    let avatar = user.avatar_url().unwrap();
    let roles = member.roles(cx).unwrap().into_iter();
    let roles_str = format!("`{}`", roles.map(|r| r.name).collect::<Vec<_>>().join(", "));

    cx.send(|m| {
        m.embed(|e| {
            e.title(format!("Informações do usuário: {user_name}"));
            e.field("🔖 **Tag do discord:**", user_tag, true);
            e.field("💻 **Id de usuário:**", user_id, true);
            e.field("📅 **Conta criada há:**", account_age, true);
            e.field("🌟 **Entrou no servidor há:**", joined_at, false);
            e.field("📚 **Cargos:**", roles_str, false);
            e.thumbnail(avatar)
                .colour(serenity::Colour::DARK_PURPLE)
                .footer(|f| f.text(format!("Comando pedido por {}", cx.author().tag())))
        })
    })
    .await?;

    Ok(())
}
