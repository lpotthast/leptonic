use std::time::Duration;

use assertr::{matchers::eq, prelude::*};
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
    ///
    /// The page it leaves must not have reported errors. Runs as a `page_load` step (navigation and hydration), so the
    /// run summary shows what loading pages costs.
    pub async fn goto(&self, path: &str) -> Result<(), Report> {
        self.expect_no_page_errors().await?;
        let url = format!("{}{path}", self.base_url);
        async {
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
        .step("page_load")
        .detail(path)
        .await
    }

    /// Makes `theme` (`"light"` or `"dark"`) the reader's theme, as if they had chosen it with the app bar's toggle
    /// before: the book keeps it in a cookie, so the server renders the pages loaded afterwards in it.
    pub async fn set_theme(&self, theme: &str) -> Result<(), Report> {
        self.driver
            .cdp()
            .send_raw(
                "Network.setCookie",
                serde_json::json!({ "name": "theme", "value": theme, "url": self.base_url }),
            )
            .step("set_theme")
            .detail(theme)
            .await
            .context_with(|| format!("failed to set the theme cookie to {theme}"))?;
        Ok(())
    }

    /// Allows the page to read the clipboard back (writing during a press needs no permission). Granted for the
    /// browser context of this tab: browser-test runs every test in a context of its own.
    pub async fn allow_clipboard_read(&self) -> Result<(), Report> {
        let tab = self
            .driver
            .cdp()
            .send_raw("Target.getTargetInfo", serde_json::json!({}))
            .await?;
        self.driver
            .cdp()
            .send_raw(
                "Browser.grantPermissions",
                serde_json::json!({
                    "origin": self.base_url.trim_end_matches('/'),
                    "permissions": ["clipboardReadWrite", "clipboardSanitizedWrite"],
                    "browserContextId": tab["targetInfo"]["browserContextId"],
                }),
            )
            .await
            .context("failed to allow reading the clipboard")?;
        Ok(())
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

    /// Fails if the page reported errors (see [`Self::page_errors`]). Every test checks this after it ran
    /// (`ui_tests::CheckPageErrors`), and [`Self::goto`] for the page it leaves.
    pub async fn expect_no_page_errors(&self) -> Result<(), Report> {
        let errors = self.page_errors().await?;
        if !errors.is_empty() {
            bail!("the page reported errors: {errors:#?}");
        }
        Ok(())
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

    /// Whether the browser runs on a Mac, iPhone or iPad, where shortcuts use Meta (Command) instead of Control, as
    /// leptonic's `platform::is_apple_device` detects it.
    pub async fn is_apple_device(&self) -> Result<bool, Report> {
        // Case-insensitive: Chromium's `userAgentData.platform` is "macOS".
        let script = "return [String(/^(Mac|iPhone|iPad)/i.test(navigator.userAgentData?.platform || navigator.platform))];";
        Ok(self.strings(script).await? == ["true"])
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

    /// Polls `script` (which returns a boolean) until it returns `true`: an eventual assertion
    /// (assertr) on the script's result, named by `what`. A failing script fails at once.
    ///
    /// `#[track_caller]`: the assertion is built when called, so that a failure names the test's
    /// line.
    #[track_caller]
    pub fn wait_until<'a>(
        &'a self,
        what: &'a str,
        script: &'a str,
    ) -> impl Future<Output = Result<(), Report>> + 'a {
        self.wait_until_within(what, script, POLL_TIMEOUT)
    }

    /// [`Self::wait_until`] with its own `timeout`, for steps that wait on slow server work.
    #[track_caller]
    pub fn wait_until_within<'a>(
        &'a self,
        what: &'a str,
        script: &'a str,
        timeout: Duration,
    ) -> impl Future<Output = Result<(), Report>> + 'a {
        let check = assert_that_owned!(move || self.returns_true(script))
            .with_subject_name(what)
            .eventually_ok()
            .within(timeout)
            .polling_every(POLL_INTERVAL)
            .giving_up_on_any_error()
            .matches(eq(true));
        async move {
            check.await;
            Ok(())
        }
        .step("wait_until")
        .detail(what)
    }

    /// Whether `script` returns `true`.
    async fn returns_true(&self, script: &str) -> Result<bool, Report> {
        let value = self
            .driver
            .execute(script, vec![])
            .await
            .context_with(|| format!("failed to run {}", script_summary(script)))?;
        Ok(value.json().as_bool() == Some(true))
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
