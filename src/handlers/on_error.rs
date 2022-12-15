use std::process;

use crate::{primitives::State, utils::log_channel};
use anyhow::Error;
use poise::{
    serenity_prelude::{Colour, CreateEmbed, Mentionable, Timestamp},
    FrameworkError,
};

#[allow(clippy::needless_pass_by_value)]
pub fn build_embed(e: &mut CreateEmbed, error: Error) -> &mut CreateEmbed {
    if let Some(msg) = error.downcast_ref::<&str>() {
        e.description(msg).colour(Colour::ROSEWATER)
    } else {
        e.title("UH!").colour(Colour::ROSEWATER).description(format!(r#"Me desculpe! ocorreu um erro não corretamente tratado aqui então eu tive que parar o interação!
                                               Esse erro foi automaticamente reportado para a staff. Me desculpe pelo incomodo.

                                               **DESCRIÇÂO DO ERRO**: `{error:?}`"#).trim_start())
    }
}

pub async fn on_error(error: FrameworkError<'_, State, Error>) {
    match error {
        FrameworkError::Setup { error, .. } => {
            tracing::error!("Setup error: {error}");
            process::abort();
        }

        FrameworkError::EventHandler {
            error, ctx, event, ..
        } => {
            tracing::error!(event = ?event, "{error}");

            log_channel::send(
                ctx,
                CreateEmbed::default()
                    .colour(Colour::ROHRKATZE_BLUE)
                    .title("Erro em um evento")
                    .description(format!(
                        r#"Evento: {}
                           Erro: {error:?}"#,
                        event.name(),
                    ))
                    .timestamp(Timestamp::now())
                    .clone(),
            )
            .await
            .take();
        }

        FrameworkError::Command { error, ctx } => {
            tracing::error!(command_name = ctx.command().name, "{error}");
            log_channel::send(
                ctx.serenity_context(),
                CreateEmbed::default()
                    .title("Erro em um comando")
                    .colour(Colour::ROSEWATER)
                    .description(
                        format!(
                            r#"Comando: `{}` (`{}`)
                            Canal: `{}` ({})
                            Executado por: `{}` ({})
                            Erro: `{error:?}`"#,
                            ctx.command().name,
                            ctx.invoked_command_name(),
                            ctx.channel_id().0,
                            ctx.channel_id().mention(),
                            ctx.author().tag(),
                            ctx.author().mention()
                        )
                        .trim_start(),
                    )
                    .timestamp(Timestamp::now())
                    .clone(),
            )
            .await
            .take();

            ctx.send(|m| m.ephemeral(true).embed(|e| build_embed(e, error)))
                .await
                .ok();
        }

        _ => {
            tracing::warn!("Not handling a error.");
        }
    }
}
