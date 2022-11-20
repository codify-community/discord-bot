use crate::primitives::Context;
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

    let user_name = user.tag();
    let user_id = user.id;

    let member = guild.member(cx, user.id).await.unwrap();

    let joined_at = member.joined_at.unwrap().timestamp();

    let account_age = user.created_at().timestamp();

    let description = format!(
        r#"
            -> **Nome do usuário:**     `{user_name}`
            -> **ID do usuário:**       `{user_id}`
            -> **Entrou no servidor:**  <t:{joined_at}:R>
            -> **Conta criada: **       <t:{account_age}:R>
        "#
    );

    cx.send(|m| {
        m.embed(|e| {
            e.title(format!("Informações do usuário: `{user_name}`"))
                .colour(serenity::Colour::DARK_PURPLE)
                .description(description)
                .footer(|f| f.text(format!("Comando pedido por {}", cx.author().tag())))
        })
    })
    .await?;

    Ok(())
}
