use crate::primitives::Context;
use anyhow::Result;
use poise::serenity_prelude::{Colour, Mentionable, Message};

fn description(mention: &str) -> String {
    format!(
        "Olá {mention}! Me desculpe incomodar, mas você **não precisa perguntar para perguntar**.\n\n\
        - Ué, como assim?\n\
        Simples, você pode mandar a pergunta direto, no lugar de \"Alguém sabe Node.JS?\", \
        você pode mandar no canal de pergunta adequado\n"
    )
}

#[poise::command(context_menu_command = "Enviar NPPP", prefix_command)]
pub async fn nppp(ctx: Context<'_>, message: Message) -> Result<()> {
    ctx.channel_id()
        .send_message(ctx, |m| {
            m.reference_message(&message).embed(|e| {
                e.colour(Colour::TEAL)
                    .description(description(&message.author.mention().to_string()))
                    .fields([
                        (
                            "Titulo",
                            "Como resolver `Cannot find type definition file for 'node'`",
                            true,
                        ),
                        ("Tags", "Typescript", true),
                        (
                            "Corpo",
                            "Olá gente! eu to com esse probleminha faz algum tempo. \
                                   É o seguinte, quando eu tento usar `process.exit(0)`, \
                                   dá esse erro do titulo.\n\
                                   Alguém sabe resolver?",
                            false,
                        ),
                    ])
            })
        })
        .await?;

    ctx.send(|m| m.ephemeral(true).content("✅ Feito.")).await?;
    Ok(())
}
