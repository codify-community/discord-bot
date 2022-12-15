pub mod join;
pub mod leave;
pub mod next;
pub mod np;
pub mod play;
pub mod queue;
pub mod remove;

use crate::primitives::Context;
use anyhow::Result;

#[poise::command(
    prefix_command,
    slash_command,
    aliases("music", "m"),
    subcommands(
        "play::tocar",
        "join::entrar",
        "leave::sair",
        "queue::fila",
        "np::tocando",
        "next::proximo",
        "remove::remover"
    )
)]
#[allow(clippy::unused_async)]
pub async fn musica(_ctx: Context<'_>) -> Result<()> {
    Ok(())
}
