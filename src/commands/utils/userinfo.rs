use crate::{primitives::Context, utils::time::get_relative_time};
use anyhow::Result;
use poise::serenity_prelude as serenity;

/// [🧰 utilidades] Pegue as informações de um usuário
#[poise::command(slash_command, prefix_command)]
pub async fn userinfo(
    ctx: Context<'_>,
    #[description = "Selecione o usuário"] user: Option<serenity::User>,
) -> Result<()> {
    let user = user.as_ref().unwrap_or_else(|| ctx.author());
    let guild = ctx.partial_guild().await.unwrap();

    let user_tag = format!("`{}`", user.tag());
    let user_id = format!("`{}`", user.id);

    let member = guild.member(ctx, user.id).await.unwrap();

    let nick_in_guild = user
        .nick_in(&serenity::CacheHttp::http(&ctx), ctx.guild().unwrap().id)
        .await
        .unwrap_or_else(|| user.name.to_string());

    let joined_at = member.joined_at.unwrap().timestamp();

    let account_age = user.created_at().timestamp();

    let author_avatar = ctx.author().face();

    let avatar = user.face();

    let roles = member.roles(ctx).unwrap_or_default().into_iter();
    let mut roles_str = roles
        .map(|r| format!("<@&{}>", *r.id.as_u64()))
        .collect::<Vec<_>>()
        .join(" ");

    if roles_str.is_empty() {
        roles_str = "O usuário não contem nenhum cargo".to_string();
    }

    ctx.send(|m| {
        m.embed(|e| {
            e.author(|a: &mut serenity::builder::CreateEmbedAuthor| {
                a.icon_url(member.face()).name(nick_in_guild)
            })
            .title("Informações do usuário")
            .fields([
                ("🔖 **Tag do discord:**", user_tag, true),
                ("💻 **Id de usuário:**", user_id, true),
                (
                    "📅 **Conta criada há:**",
                    get_relative_time(account_age as u64),
                    true,
                ),
                (
                    "🌟 **Entrou no servidor há:**",
                    get_relative_time(joined_at as u64),
                    false,
                ),
                ("📚 **Cargos:**", roles_str, false),
            ])
            .thumbnail(avatar)
            .colour(serenity::Colour::DARK_PURPLE)
            .footer(|f| {
                f.icon_url(author_avatar)
                    .text(format!("Pedido por {}", ctx.author().tag()))
            })
        })
    })
    .await?;

    Ok(())
}
