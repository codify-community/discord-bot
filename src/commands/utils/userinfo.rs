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

    let user_name = user.tag();
    let user_id = user.id;

    let member = guild.member(cx, user.id).await.unwrap();

    let joined_at_timestamp: i64 = member.joined_at.unwrap().timestamp();
    let joined_at = get_discord_relative_time(joined_at_timestamp);

    let account_age_timestamp: i64 = user.created_at().timestamp();
    let account_age = get_discord_relative_time(account_age_timestamp);

    let description = format!(
        r#"
            -> **Nome do usuário:**     {user_name} 
            -> **ID do usuário:**       {user_id}   
            -> **Entrou no servidor:**  {joined_at}
            -> **Conta criada: **       {account_age}
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
