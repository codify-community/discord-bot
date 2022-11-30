use anyhow::Context as _;
use tokio::sync::oneshot;

use crate::{
    jobs::browser::{Browser, Request},
    primitives::Context,
};

pub async fn google(ctx: &Context<'_>, query: String) -> anyhow::Result<Vec<u8>> {
    let (sender, result) = oneshot::channel();
    let browser = ctx
        .data()
        .jobs
        .get::<Browser>()
        .context("Can't find browser instance")?;

    browser.send(Request::Google { query, sender })?;

    Ok(result.await?)
}
