use std::time::{Duration, Instant};

use browser_test::{StepExt, thirtyfour::WebDriver};
use leptos_browser_test::{Report, ResultExt, bail};

/// A page of the book. Tests read the page through [`BookPage`]'s helpers and assert in Rust.
///
/// The helpers run as [`step`]s: each is debug-logged with its duration, a slow one is logged as a warning, and the
/// run summary lists the step kinds that took the most time.
pub struct BookPage<'d> {
    pub driver: &'d WebDriver,
    pub base_url: &'d str,
}

const POLL_TIMEOUT: Duration = Duration::from_secs(10);
const POLL_INTERVAL: Duration = Duration::from_millis(50);

impl BookPage<'_> {
    /// The documentation pages to check: every page of the navigation, or those whose path contains
    /// `BOOK_TEST_PAGES`.
    pub fn paths() -> Vec<String> {
        let filter = std::env::var("BOOK_TEST_PAGES").ok();
        book_ssr::nav::nav()
            .pages()
            .filter(|path| filter.as_deref().is_none_or(|filter| path.contains(filter)))
            .map(str::to_owned)
            .collect()
    }

    /// Emulates a viewport of `width` x `height` CSS pixels (also when the browser window is shown, whose size has a
    /// minimum). The book switches to its mobile layout at 800px and below.
    pub async fn set_viewport(&self, width: u32, height: u32) -> Result<(), Report> {
        self.driver.cdp().send_raw(
                "Emulation.setDeviceMetricsOverride",
                serde_json::json!({ "width": width, "height": height, "deviceScaleFactor": 1, "mobile": false }),
            ).step("set_viewport").detail(format!("{width}x{height}"))
        .await
        .context_with(|| format!("failed to emulate a {width}x{height} viewport"))?;
        Ok(())
    }

    /// Navigate to `path` and wait until the book finished hydrating, so that the page shows its final state.
    pub async fn goto(&self, path: &str) -> Result<(), Report> {
        let url = format!("{}{path}", self.base_url);
        self.driver
            .goto(&url)
            .step("navigate")
            .detail(path)
            .await
            .context_with(|| format!("failed to go to {url}"))?;
        self.wait_until(
            &format!("{url} finished hydrating"),
            "return document.body.hasAttribute('data-hydrated');",
        )
        .step("wait_for_hydration")
        .detail(path)
        .await
    }

    /// Moves the mouse pointer to the viewport coordinates `x`, `y` (CSS pixels), changing the hovered element.
    pub async fn move_pointer_to(&self, x: u32, y: u32) -> Result<(), Report> {
        self.driver
            .cdp()
            .send_raw(
                "Input.dispatchMouseEvent",
                serde_json::json!({ "type": "mouseMoved", "x": x, "y": y }),
            )
            .step("move_pointer")
            .detail(format!("{x},{y}"))
            .await
            .context_with(|| format!("failed to move the pointer to {x},{y}"))?;
        Ok(())
    }

    /// Uncaught errors (and Rust panics) the page reported since it loaded.
    pub async fn page_errors(&self) -> Result<Vec<String>, Report> {
        self.strings("return window.__pageErrors || [];").await
    }

    /// Runs `script` (which returns an array of strings) and returns the strings.
    pub async fn strings(&self, script: &str) -> Result<Vec<String>, Report> {
        let value = self
            .driver
            .execute(script, vec![])
            .step("script")
            .detail(script_summary(script))
            .await
            .context_with(|| format!("failed to run {script}"))?;
        Ok(value
            .json()
            .as_array()
            .map(|items| {
                items
                    .iter()
                    .map(|item| item.as_str().unwrap_or_default().to_owned())
                    .collect()
            })
            .unwrap_or_default())
    }

    /// Runs `script` (which returns a number).
    pub async fn number(&self, script: &str) -> Result<f64, Report> {
        let value = self
            .driver
            .execute(script, vec![])
            .step("script")
            .detail(script_summary(script))
            .await
            .context_with(|| format!("failed to run {script}"))?;
        match value.json().as_f64() {
            Some(number) => Ok(number),
            None => bail!("{script} returned {}, not a number", value.json()),
        }
    }

    /// Polls `script` (which returns a boolean) until it returns `true`.
    pub async fn wait_until(&self, what: &str, script: &str) -> Result<(), Report> {
        self.wait_until_within(what, script, POLL_TIMEOUT).await
    }

    /// [`Self::wait_until`] with its own `timeout`, for steps that wait on slow server work.
    pub async fn wait_until_within(
        &self,
        what: &str,
        script: &str,
        timeout: Duration,
    ) -> Result<(), Report> {
        self.poll_until(what, script, timeout)
            .step("wait_until")
            .detail(what)
            .await
    }

    async fn poll_until(&self, what: &str, script: &str, timeout: Duration) -> Result<(), Report> {
        let start = Instant::now();
        loop {
            let value = self
                .driver
                .execute(script, vec![])
                .await
                .context_with(|| format!("failed to check that {what}"))?;
            if value.json().as_bool() == Some(true) {
                return Ok(());
            }
            if start.elapsed() > timeout {
                bail!("timed out waiting until {what}");
            }
            tokio::time::sleep(POLL_INTERVAL).await;
        }
    }
}

/// The start of `script`, to name it in logs.
fn script_summary(script: &str) -> String {
    let script = script.split_whitespace().collect::<Vec<_>>().join(" ");
    match script.char_indices().nth(60) {
        Some((end, _)) => format!("{}...", &script[..end]),
        None => script,
    }
}
