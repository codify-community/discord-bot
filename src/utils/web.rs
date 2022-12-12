use anyhow::Context as _;
use tokio::sync::oneshot;

use crate::{
    jobs::browser::{Browser, Request, RequestKind},
    primitives::Context,
};

pub async fn request(ctx: &Context<'_>, kind: RequestKind) -> anyhow::Result<Vec<u8>> {
    let (sender, result) = oneshot::channel();
    let browser = ctx
        .data()
        .jobs
        .get::<Browser>()
        .context("Can't find browser instance")?;

    browser.send(Request { kind, sender })?;

    Ok(result.await?)
}
