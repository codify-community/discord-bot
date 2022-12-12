use super::Job;
use anyhow::{bail, Context, Error};
use std::env::var;
use thirtyfour::{DesiredCapabilities, WebDriver};
use tokio::{
    select,
    sync::{mpsc, oneshot, watch},
    time::Instant,
};
use typemap_rev::TypeMapKey;

type Message = Vec<u8>;
type Sender = oneshot::Sender<Message>;
type Tx = mpsc::UnboundedSender<Request>;

pub const ENABLE_SAFE_SEARCH: &str = r#"document.querySelector('a[href*="/safesearch"]')
?.parentNode?.parentNode?.parentNode?.querySelector("input")
?.click();
const script = document.createElement("script");
script.textContent = `document.querySelector('a[href*="/safesearch"]')?.parentNode?.parentNode?.parentNode?.querySelector("input")?.click();`;
(document.head || document.documentElement).appendChild(script);
script.remove();"#;

#[derive(Debug)]
pub enum RequestKind {
    Google { query: String },
}

#[derive(Debug)]
pub struct Request {
    pub kind: RequestKind,
    pub sender: Sender,
}

pub struct Browser {
    pub browser: Option<WebDriver>,
    pub rx: mpsc::UnboundedReceiver<Request>,
    pub terminating: watch::Receiver<bool>,
}

impl TypeMapKey for Browser {
    type Value = mpsc::UnboundedSender<Request>;
}

impl Browser {
    /// Creates a new browser instance using geckodriver
    /// # Errors
    /// When Fails to connect to `$GECKODRIVER_ADDRESS`
    pub async fn new(terminating: watch::Receiver<bool>) -> Result<(Tx, Self), Error> {
        let (tx, rx) = mpsc::unbounded_channel();
        let mut caps = DesiredCapabilities::firefox();
        caps.set_headless()?;

        let browser = WebDriver::new(&var("GECKODRIVER_ADDRESS")?, caps).await?;
        browser.set_window_rect(0, 0, 1366, 728).await?;

        let this = Self {
            browser: Some(browser),
            terminating,
            rx,
        };

        Ok((tx, this))
    }
}

#[poise::async_trait]
impl Job for Browser {
    async fn start(mut self) -> anyhow::Result<()> {
        tracing::debug!("Waiting requests.");

        loop {
            let Self {
                rx, terminating, ..
            } = &mut self;

            select! {
                Some(request) = rx.recv() => {
                    self.handle_request(request).await?;
                }

                _ = terminating.changed() => {
                    tracing::info!("Closing geckodriver");
                    let browser = self.browser.take().expect("Browser closed");
                    browser.quit().await?;
                    break;
                }
            }
        }

        bail!("Geckodriver closed")
    }
}

impl Browser {
    #[tracing::instrument(name = "Handle request", skip_all, fields(query = ?request.kind))]
    async fn handle_request(&mut self, request: Request) -> anyhow::Result<()> {
        match request.kind {
            RequestKind::Google { query } => {
                let started = Instant::now();

                tracing::info!("Searching...");
                let uri = format!(
                    "https://www.google.com/search?client=firefox-b-d&q={}",
                    &query
                );

                let Some(browser) = &mut self.browser else {
                    bail!("Browser closed");
                };

                let screenshot = browser
                    .in_new_tab(|| async {
                        browser.goto(uri).await?;
                        browser.execute(ENABLE_SAFE_SEARCH, Vec::new()).await?;

                        let screenshot = browser.screenshot_as_png().await?;
                        Ok(screenshot)
                    })
                    .await?;

                request
                    .sender
                    .send(screenshot)
                    .ok()
                    .context("Failed to screenshot")?;

                tracing::info!(completed_in = ?started.elapsed(), "Done");
            }
        }
        Ok(())
    }
}
