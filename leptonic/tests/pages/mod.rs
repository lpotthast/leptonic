pub mod button;
pub mod focus;
pub mod focus_manager;
pub mod focus_ring;
pub mod focus_scope;
pub mod focus_visible;
pub mod focus_within;
pub mod focusable;
pub mod has_tabbable_child;

use std::time::{Duration, Instant};

use browser_test::thirtyfour::{By, Key, TypingData, WebDriver, WebElement};
use leptos_browser_test::{Report, ResultExt, bail};

/// A page object without page-specific helpers. Tests that only need [`BaseActions`] use this
/// instead of defining their own page type.
pub struct Page<'d> {
    pub driver: &'d WebDriver,
    pub base_url: &'d str,
}

impl BaseActions for Page<'_> {
    fn driver(&self) -> &WebDriver {
        self.driver
    }

    fn base_url(&self) -> &str {
        self.base_url
    }
}

const POLL_TIMEOUT: Duration = Duration::from_secs(10);
const POLL_INTERVAL: Duration = Duration::from_millis(50);

/// Shared page-object actions. Page objects only need to provide the driver and base URL.
#[allow(dead_code)] // Not every test binary uses every helper.
pub trait BaseActions {
    fn driver(&self) -> &WebDriver;
    fn base_url(&self) -> &str;

    /// Navigate to `path` and wait until the test-app finished hydrating, so that event handlers
    /// are attached before the test starts interacting with the page.
    async fn goto_path(&self, path: &str) -> Result<(), Report> {
        let url = format!("{}{path}", self.base_url());
        self.driver()
            .goto(&url)
            .await
            .context_with(|| format!("failed to go to {url}"))?;
        self.wait_for_selector("body[data-hydrated]")
            .await
            .context_with(|| format!("{url} did not finish hydrating"))?;
        Ok(())
    }

    async fn element(&self, id: &str) -> Result<WebElement, Report> {
        Ok(self
            .driver()
            .find(By::Id(id))
            .await
            .context_with(|| format!("failed to find #{id}"))?)
    }

    async fn click_element_with_id(&self, id: &str) -> Result<(), Report> {
        tracing::info!("Click element with id '{id}'.");
        self.element(id)
            .await?
            .click()
            .await
            .context_with(|| format!("failed to click #{id}"))?;
        Ok(())
    }

    async fn read_text_of(&self, id: &str) -> Result<String, Report> {
        Ok(self
            .element(id)
            .await?
            .text()
            .await
            .context_with(|| format!("failed to read text of #{id}"))?)
    }

    /// Read the text of `id` and interpret `"true"` / `"false"`.
    async fn read_bool(&self, id: &str) -> Result<bool, Report> {
        let text = self.read_text_of(id).await?;
        match text.trim() {
            "true" => Ok(true),
            "false" => Ok(false),
            other => bail!("expected #{id} to contain true/false, got {other:?}"),
        }
    }

    /// Read the text of `id` and parse it as a number.
    async fn read_u32(&self, id: &str) -> Result<u32, Report> {
        let text = self.read_text_of(id).await?;
        Ok(text
            .trim()
            .parse()
            .context_with(|| format!("expected #{id} to contain a number, got {text:?}"))?)
    }

    async fn attr_of(&self, id: &str, name: &str) -> Result<Option<String>, Report> {
        Ok(self
            .element(id)
            .await?
            .attr(name)
            .await
            .context_with(|| format!("failed to read attribute {name:?} of #{id}"))?)
    }

    /// The id of `document.activeElement`, or `None` if it has no id (e.g. `<body>`).
    async fn active_element_id(&self) -> Result<Option<String>, Report> {
        let active = self
            .driver()
            .active_element()
            .await
            .context("failed to get the active element")?;
        let id = active
            .id()
            .await
            .context("failed to read active element id")?;
        Ok(id.filter(|id| !id.is_empty()))
    }

    /// Send keys to whatever element currently has focus.
    async fn send_keys_to_active(&self, keys: impl Into<TypingData> + Send) -> Result<(), Report> {
        self.driver()
            .active_element()
            .await
            .context("failed to get the active element")?
            .send_keys(keys)
            .await
            .context("failed to send keys to the active element")?;
        Ok(())
    }

    async fn press_tab(&self) -> Result<(), Report> {
        self.send_keys_to_active(Key::Tab).await
    }

    async fn press_shift_tab(&self) -> Result<(), Report> {
        self.send_keys_to_active(Key::Shift + Key::Tab).await
    }

    /// Wait until at least one element matches the CSS `selector`.
    async fn wait_for_selector(&self, selector: &str) -> Result<(), Report> {
        let deadline = Instant::now() + POLL_TIMEOUT;
        loop {
            let found = self
                .driver()
                .find_all(By::Css(selector))
                .await
                .context_with(|| format!("failed to query {selector:?}"))?;
            if !found.is_empty() {
                return Ok(());
            }
            if Instant::now() >= deadline {
                bail!("no element matched {selector:?} within {POLL_TIMEOUT:?}");
            }
            tokio::time::sleep(POLL_INTERVAL).await;
        }
    }

    /// Wait until the text of `id` equals `expected`. Use this after interactions whose effects
    /// are applied asynchronously (timers, effects, animation frames).
    async fn wait_for_text(&self, id: &str, expected: &str) -> Result<(), Report> {
        let deadline = Instant::now() + POLL_TIMEOUT;
        loop {
            let actual = self.read_text_of(id).await?;
            if actual == expected {
                return Ok(());
            }
            if Instant::now() >= deadline {
                bail!(
                    "#{id} did not become {expected:?} within {POLL_TIMEOUT:?}; last seen {actual:?}"
                );
            }
            tokio::time::sleep(POLL_INTERVAL).await;
        }
    }

    /// The element matching the CSS `selector`.
    async fn css(&self, selector: &str) -> Result<WebElement, Report> {
        Ok(self
            .driver()
            .find(By::Css(selector))
            .await
            .context_with(|| format!("failed to find {selector:?}"))?)
    }

    /// The element with ARIA `role` whose visible text is `text`.
    async fn by_role_and_text(&self, role: &str, text: &str) -> Result<WebElement, Report> {
        let xpath = format!("//*[@role='{role}'][normalize-space(.)='{text}']");
        Ok(self
            .driver()
            .find(By::XPath(xpath))
            .await
            .context_with(|| format!("failed to find role={role} with text {text:?}"))?)
    }

    /// The visible text of `document.activeElement`.
    async fn active_element_text(&self) -> Result<String, Report> {
        let active = self
            .driver()
            .active_element()
            .await
            .context("failed to get the active element")?;
        Ok(active.text().await?.trim().to_owned())
    }

    /// Wait until the focused element's visible text is `expected`. Focus often moves in an
    /// effect after the triggering event, so assert it by waiting rather than sampling once.
    async fn wait_for_active_text(&self, expected: &str) -> Result<(), Report> {
        let deadline = Instant::now() + POLL_TIMEOUT;
        loop {
            let actual = self.active_element_text().await?;
            if actual == expected {
                return Ok(());
            }
            if Instant::now() >= deadline {
                bail!(
                    "focus did not move to {expected:?} within {POLL_TIMEOUT:?}; it is on {actual:?}"
                );
            }
            tokio::time::sleep(POLL_INTERVAL).await;
        }
    }
}
