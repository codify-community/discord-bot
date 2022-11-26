use super::Job;
use anyhow::{Context, Error};
use std::env::var;
use thirtyfour::{DesiredCapabilities, WebDriver};
use tokio::{
    process::Command,
    sync::{mpsc, oneshot},
    time::Instant,
};
use typemap_rev::TypeMapKey;
use urlencoding::encode;

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
pub enum Request {
    Google { query: String, sender: Sender },
}

pub struct Browser {
    pub browser: WebDriver,
    pub rx: mpsc::UnboundedReceiver<Request>,
}

impl TypeMapKey for Browser {
    type Value = mpsc::UnboundedSender<Request>;
}

impl Browser {
	/// Creates a new browser instance using geckodriver
	/// # Errors
	/// When `$GECKODRIVER_CMDLINE` does not exists.
	/// Fails to connect to `$GECKODRIVER_ADDRESS`
    pub async fn new() -> Result<(Tx, Self), Error> {
        let cmdline = var("GECKODRIVER_CMDLINE")?;
        let mut args = cmdline.split_whitespace();

        Command::new(args.next().context("No executable name")?)
            .args(args)
            .spawn()?;

        let (tx, rx) = mpsc::unbounded_channel();
        let mut caps = DesiredCapabilities::firefox();
        caps.set_headless()?;

        let browser = WebDriver::new(&var("GECKODRIVER_ADDRESS")?, caps).await?;
        browser.set_window_rect(0, 0, 1366, 728).await?;

        let this = Self { browser, rx };

        Ok((tx, this))
    }
}

#[poise::async_trait]
impl Job for Browser {
    async fn start(mut self) -> anyhow::Result<()> {
        let Self { browser, rx } = &mut self;
        tracing::debug!("Waiting requests.");

        while let Some(request) = rx.recv().await {
            match request {
                Request::Google { query, sender } => {
                    let started = Instant::now();

                    tracing::info!("Searching {query}...");
                    let uri = format!(
                        "https://www.google.com/search?client=firefox-b-d&q={}",
                        encode(&query)
                    );

                    let screenshot = browser
                        .in_new_tab(|| async {
                            browser.goto(uri).await?;
                            browser.execute(ENABLE_SAFE_SEARCH, Vec::new()).await?;

                            let screenshot = browser.screenshot_as_png().await?;
                            Ok(screenshot)
                        })
                        .await?;

                    sender
                        .send(screenshot)
                        .ok()
                        .context("Failed to screenshot")?;

                    tracing::info!(completed_in = ?started.elapsed(), query = ?query, "Done");
                }
            }
        }

        Ok(())
    }
}
