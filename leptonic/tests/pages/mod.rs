pub mod button;
pub mod dnd;
pub mod focus;
pub mod focus_manager;
pub mod focus_ring;
pub mod focus_scope;
pub mod focus_visible;
pub mod focus_within;
pub mod focusable;
pub mod has_tabbable_child;

use std::time::{Duration, Instant};

use browser_test::{
    StepExt,
    thirtyfour::{By, Key, TypingData, WebDriver, WebElement},
};
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
///
/// Navigations, waits and element lookups run as [`step`]s: each is debug-logged with its duration,
/// a slow one is logged as a warning, and the run summary lists the step kinds that took the most
/// time.
#[allow(dead_code)] // Not every test binary uses every helper.
pub trait BaseActions {
    fn driver(&self) -> &WebDriver;
    fn base_url(&self) -> &str;

    /// Navigate to `path` and wait until the test-app finished hydrating, so that event handlers
    /// are attached before the test starts interacting with the page.
    async fn goto_path(&self, path: &str) -> Result<(), Report> {
        // The page we leave must not have reported errors.
        self.expect_no_page_errors().await?;
        let url = format!("{}{path}", self.base_url());
        self.driver()
            .goto(&url)
            .step("navigate")
            .detail(path)
            .await
            .context_with(|| format!("failed to go to {url}"))?;
        if let Err(error) = self
            .wait_for_selector("body[data-hydrated]")
            .step("wait_for_hydration")
            .detail(path)
            .await
        {
            // Usually a panic while hydrating: report what the page caught.
            let page_errors = self
                .driver()
                .execute("return window.__pageErrors || [];", vec![])
                .await
                .map(|errors| errors.json().to_string())
                .unwrap_or_default();
            return Err(error
                .context(format!(
                    "{url} did not finish hydrating; page errors: {page_errors}"
                ))
                .into_dynamic());
        }
        Ok(())
    }

    /// Fail if the page reported uncaught errors (collected by the test-app since the page
    /// loaded), e.g. wasm-bindgen errors thrown from event handlers, or has elements with literal
    /// `attr:` attributes.
    async fn expect_no_page_errors(&self) -> Result<(), Report> {
        async {
            let errors = self
                .driver()
                .execute("return window.__pageErrors || [];", vec![])
                .await
                .context("failed to read the page errors")?;
            let errors: Vec<String> = errors
                .json()
                .as_array()
                .map(|errors| {
                    errors
                        .iter()
                        .map(|e| e.as_str().unwrap_or_default().to_owned())
                        .collect()
                })
                .unwrap_or_default();
            if !errors.is_empty() {
                bail!("the page reported errors: {errors:?}");
            }
            // `attr:` only means something on components; on elements it becomes a literal attribute.
            let literal = self
                .driver()
                .execute(
                    "return [...document.querySelectorAll('*')].flatMap(e => [...e.attributes].map(a => a.name)).filter(n => n.startsWith('attr:'));",
                    vec![],
                )
                .await
                .context("failed to read the attribute names")?;
            if literal
                .json()
                .as_array()
                .is_some_and(|names| !names.is_empty())
            {
                bail!(
                    "elements have literal `attr:` attributes: {}",
                    literal.json()
                );
            }
            Ok(())
        }.step("check_page_errors")
        .await
    }

    async fn element(&self, id: &str) -> Result<WebElement, Report> {
        async {
            Ok(self
                .driver()
                .find(By::Id(id))
                .await
                .context_with(|| format!("failed to find #{id}"))?)
        }
        .step("find")
        .detail(format!("#{id}"))
        .await
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

    /// The number of elements matching the CSS `selector`, counted in the page. Unlike
    /// `find_all`, this never blocks for the session's implicit wait when nothing matches.
    async fn count_matching(&self, selector: &str) -> Result<u64, Report> {
        let count = self
            .driver()
            .execute(
                "return document.querySelectorAll(arguments[0]).length;",
                vec![serde_json::Value::from(selector)],
            )
            .await
            .context_with(|| format!("failed to query {selector:?}"))?;
        match count.json().as_u64() {
            Some(count) => Ok(count),
            None => bail!("counting {selector:?} returned {}", count.json()),
        }
    }

    /// Wait until at least one element matches the CSS `selector`.
    async fn wait_for_selector(&self, selector: &str) -> Result<(), Report> {
        async {
            let deadline = Instant::now() + POLL_TIMEOUT;
            loop {
                if self.count_matching(selector).await? > 0 {
                    return Ok(());
                }
                if Instant::now() >= deadline {
                    bail!("no element matched {selector:?} within {POLL_TIMEOUT:?}");
                }
                tokio::time::sleep(POLL_INTERVAL).await;
            }
        }
        .step("wait_for_selector")
        .detail(selector)
        .await
    }

    /// Wait until the text of `id` equals `expected`. Use this after interactions whose effects
    /// are applied asynchronously (timers, effects, animation frames).
    async fn wait_for_text(&self, id: &str, expected: &str) -> Result<(), Report> {
        async {
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
        }.step("wait_for_text").detail(format!("#{id} = {expected:?}"))
        .await
    }

    /// The element matching the CSS `selector`.
    async fn css(&self, selector: &str) -> Result<WebElement, Report> {
        async {
            Ok(self
                .driver()
                .find(By::Css(selector))
                .await
                .context_with(|| format!("failed to find {selector:?}"))?)
        }
        .step("find")
        .detail(selector)
        .await
    }

    /// The element with ARIA `role` (explicit, or implicit for links and buttons) whose visible
    /// text is `text`.
    async fn by_role_and_text(&self, role: &str, text: &str) -> Result<WebElement, Report> {
        async {
            let has_role = match role {
                "button" => "(self::button or @role='button')".to_owned(),
                "link" => "((self::a and @href) or @role='link')".to_owned(),
                role => format!("@role='{role}'"),
            };
            let xpath = format!("//*[{has_role}][normalize-space(.)='{text}']");
            Ok(self
                .driver()
                .find(By::XPath(xpath))
                .await
                .context_with(|| format!("failed to find role={role} with text {text:?}"))?)
        }
        .step("find")
        .detail(format!("role={role} {text:?}"))
        .await
    }

    /// The visible text of `document.activeElement`.
    async fn active_element_text(&self) -> Result<String, Report> {
        let active = self
            .driver()
            .active_element()
            .await
            .context("failed to get the active element")?;
        Ok(rendered_text(&active).await?.trim().to_owned())
    }

    /// Wait until no element matches the CSS `selector` (e.g. an overlay closed).
    async fn wait_for_no_selector(&self, selector: &str) -> Result<(), Report> {
        async {
            let deadline = Instant::now() + POLL_TIMEOUT;
            loop {
                let found = self.count_matching(selector).await?;
                if found == 0 {
                    return Ok(());
                }
                if Instant::now() >= deadline {
                    bail!("{found} element(s) still matched {selector:?} after {POLL_TIMEOUT:?}");
                }
                tokio::time::sleep(POLL_INTERVAL).await;
            }
        }
        .step("wait_for_no_selector")
        .detail(selector)
        .await
    }

    /// Wait until exactly `expected` elements match `selector` (e.g. after a closing overlay's exit
    /// animation, while it is still rendered).
    async fn wait_for_count(&self, selector: &str, expected: u64) -> Result<(), Report> {
        async {
            let deadline = Instant::now() + POLL_TIMEOUT;
            loop {
                let found = self.count_matching(selector).await?;
                if found == expected {
                    return Ok(());
                }
                if Instant::now() >= deadline {
                    bail!("{found} element(s) matched {selector:?}, not {expected}, after {POLL_TIMEOUT:?}");
                }
                tokio::time::sleep(POLL_INTERVAL).await;
            }
        }
        .step("wait_for_count")
        .detail(format!("{selector} = {expected}"))
        .await
    }

    /// Wait until the first element matching `selector` shows the visible text `expected` (e.g. an
    /// overlay fading in: invisible text doesn't count).
    async fn wait_for_selector_text(&self, selector: &str, expected: &str) -> Result<(), Report> {
        async {
            let deadline = Instant::now() + POLL_TIMEOUT;
            loop {
                let actual = match self
                    .driver()
                    .find(browser_test::thirtyfour::By::Css(selector))
                    .await
                {
                    Ok(element) => rendered_text(&element).await.unwrap_or_default(),
                    Err(_) => String::new(),
                };
                if actual == expected {
                    return Ok(());
                }
                if Instant::now() >= deadline {
                    bail!(
                        "{selector:?} did not show {expected:?} within {POLL_TIMEOUT:?}; last seen {actual:?}"
                    );
                }
                tokio::time::sleep(POLL_INTERVAL).await;
            }
        }
        .step("wait_for_selector_text")
        .detail(format!("{selector} = {expected:?}"))
        .await
    }

    /// The ARIA role and visible text of `document.activeElement`, e.g. `role=row "Inbox"`.
    async fn describe_active_element(&self) -> Result<String, Report> {
        let active = self
            .driver()
            .active_element()
            .await
            .context("failed to get the active element")?;
        let role = active.attr("role").await?;
        let label = active.attr("aria-label").await?;
        let tag = active.tag_name().await?;
        let text = rendered_text(&active).await?;
        Ok(format!(
            "<{tag}> role={} {:?}{}",
            role.as_deref().unwrap_or("(none)"),
            text.trim(),
            label
                .map(|l| format!(" aria-label={l:?}"))
                .unwrap_or_default()
        ))
    }

    /// Wait until `document.activeElement` has the ARIA `role` and, if given, the visible `text`.
    async fn wait_for_focus(&self, role: &str, text: Option<&str>) -> Result<(), Report> {
        async {
            let deadline = Instant::now() + POLL_TIMEOUT;
            loop {
                let active = self
                    .driver()
                    .active_element()
                    .await
                    .context("failed to get the active element")?;
                let role_matches = active.attr("role").await?.as_deref() == Some(role);
                let text_matches = match text {
                    Some(text) => rendered_text(&active).await?.trim() == text,
                    None => true,
                };
                if role_matches && text_matches {
                    return Ok(());
                }
                if Instant::now() >= deadline {
                    let actual = self.describe_active_element().await?;
                    bail!(
                        "focus did not move to role={role} {text:?} within {POLL_TIMEOUT:?}; it is on {actual}"
                    );
                }
                tokio::time::sleep(POLL_INTERVAL).await;
            }
        }.step("wait_for_focus").detail(format!("role={role} {text:?}"))
        .await
    }

    /// Wait until the attribute `name` of `element` is `expected` (`None`: absent). State
    /// attributes often update in an effect after the triggering event.
    async fn wait_for_attr(
        &self,
        element: &WebElement,
        name: &str,
        expected: Option<&str>,
    ) -> Result<(), Report> {
        async {
            let deadline = Instant::now() + POLL_TIMEOUT;
            loop {
                let actual = element
                    .attr(name)
                    .await
                    .context_with(|| format!("failed to read attribute {name}"))?;
                if actual.as_deref() == expected {
                    return Ok(());
                }
                if Instant::now() >= deadline {
                    bail!(
                        "attribute {name} did not become {expected:?} within {POLL_TIMEOUT:?}; last seen {actual:?}"
                    );
                }
                tokio::time::sleep(POLL_INTERVAL).await;
            }
        }.step("wait_for_attr").detail(format!("{name} = {expected:?}"))
        .await
    }

    /// Wait until `element` is `document.activeElement`. `what` names it in the error.
    async fn wait_for_focus_on(&self, element: &WebElement, what: &str) -> Result<(), Report> {
        async {
            let deadline = Instant::now() + POLL_TIMEOUT;
            loop {
                let active = self
                    .driver()
                    .active_element()
                    .await
                    .context("failed to get the active element")?;
                if &active == element {
                    return Ok(());
                }
                if Instant::now() >= deadline {
                    let actual = self.describe_active_element().await?;
                    bail!(
                        "focus did not move to {what} within {POLL_TIMEOUT:?}; it is on {actual}"
                    );
                }
                tokio::time::sleep(POLL_INTERVAL).await;
            }
        }
        .step("wait_for_focus")
        .detail(what)
        .await
    }

    /// Wait until the element with the id `expected` has focus.
    async fn wait_for_active_id(&self, expected: &str) -> Result<(), Report> {
        async {
            let deadline = Instant::now() + POLL_TIMEOUT;
            loop {
                let actual = self.active_element_id().await?;
                if actual.as_deref() == Some(expected) {
                    return Ok(());
                }
                if Instant::now() >= deadline {
                    bail!(
                        "focus did not move to #{expected} within {POLL_TIMEOUT:?}; it is on {actual:?}"
                    );
                }
                tokio::time::sleep(POLL_INTERVAL).await;
            }
        }.step("wait_for_focus").detail(format!("#{expected}"))
        .await
    }

    /// Wait until the focused element's visible text is `expected`. Focus often moves in an
    /// effect after the triggering event, so assert it by waiting rather than sampling once.
    async fn wait_for_active_text(&self, expected: &str) -> Result<(), Report> {
        async {
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
        }.step("wait_for_focus").detail(format!("{expected:?}"))
        .await
    }

    /// Wait until `observe` returns `expected`: the generic form of the `wait_for_*` helpers, for
    /// anything a test reads (a list of option texts, a segment's text, an active descendant).
    /// `what` names the observed thing in the error, which also shows the last value seen.
    ///
    /// ```ignore
    /// page.wait_for_value("the options", vec!["Banana".to_owned()], || option_texts(page)).await?;
    /// ```
    async fn wait_for_value<T, Fut>(
        &self,
        what: &str,
        expected: T,
        observe: impl FnMut() -> Fut,
    ) -> Result<(), Report>
    where
        T: PartialEq + std::fmt::Debug,
        Fut: Future<Output = Result<T, Report>>,
    {
        async {
            if let Err(actual) = poll_until(&expected, POLL_TIMEOUT, observe).await? {
                bail!(
                    "{what} did not become {expected:?} within {POLL_TIMEOUT:?}; last seen {actual:?}"
                );
            }
            Ok(())
        }
        .step("wait_for_value")
        .detail(format!("{what} = {expected:?}"))
        .await
    }

    /// Wait until `condition` holds (e.g. a section's text contains a message). Prefer
    /// [`wait_for_value`](Self::wait_for_value) where there's an expected value: its error shows
    /// what was seen instead.
    async fn wait_until<Fut>(
        &self,
        what: &str,
        condition: impl FnMut() -> Fut,
    ) -> Result<(), Report>
    where
        Fut: Future<Output = Result<bool, Report>>,
    {
        async {
            if poll_until(&true, POLL_TIMEOUT, condition).await?.is_err() {
                bail!("{what} didn't happen within {POLL_TIMEOUT:?}");
            }
            Ok(())
        }
        .step("wait_until")
        .detail(what)
        .await
    }

    /// Wait until the DOM property `name` of `element` is `expected`, e.g. an input's `value`
    /// (which the `value` attribute doesn't follow once the user typed) or `checked`.
    async fn wait_for_prop(
        &self,
        element: &WebElement,
        name: &str,
        expected: &str,
    ) -> Result<(), Report> {
        self.wait_for_value(
            &format!("property {name}"),
            Some(expected.to_owned()),
            || async {
                Ok(element
                    .prop(name)
                    .await
                    .context_with(|| format!("failed to read property {name}"))?)
            },
        )
        .await
    }

    /// Negative check ("nothing happens"): `observe` returns `expected` now and keeps returning it
    /// for [`SETTLE`], re-checked every 50 ms. A single read right after an interaction passes
    /// before the effects it would trigger ran; this lets them run (and catches a value that
    /// changes and changes back). For changes after a longer timer, see
    /// [`assert_stays_for`](Self::assert_stays_for).
    async fn assert_stays<T, Fut>(
        &self,
        what: &str,
        expected: T,
        observe: impl FnMut() -> Fut,
    ) -> Result<(), Report>
    where
        T: PartialEq + std::fmt::Debug,
        Fut: Future<Output = Result<T, Report>>,
    {
        self.assert_stays_for(what, SETTLE, expected, observe).await
    }

    /// [`assert_stays`](Self::assert_stays) over `window` (e.g. past a 500 ms long-press or a
    /// tooltip delay).
    async fn assert_stays_for<T, Fut>(
        &self,
        what: &str,
        window: Duration,
        expected: T,
        mut observe: impl FnMut() -> Fut,
    ) -> Result<(), Report>
    where
        T: PartialEq + std::fmt::Debug,
        Fut: Future<Output = Result<T, Report>>,
    {
        async {
            let deadline = Instant::now() + window;
            loop {
                let actual = observe().await?;
                if actual != expected {
                    bail!("{what} changed to {actual:?}; expected it to stay {expected:?}");
                }
                if Instant::now() >= deadline {
                    return Ok(());
                }
                tokio::time::sleep(POLL_INTERVAL).await;
            }
        }
        .step("assert_stays")
        .detail(format!("{what} = {expected:?}"))
        .await
    }
}

/// How long [`BaseActions::assert_stays`] watches a value: long enough for effects, animation
/// frames and short timers to run after an interaction.
pub const SETTLE: Duration = Duration::from_millis(300);

/// Polls `observe` every [`POLL_INTERVAL`] until it returns `expected` (`Ok(Ok(()))`) or `timeout`
/// passed (`Ok(Err(last value seen))`). Errors of `observe` end the poll.
async fn poll_until<T, Fut>(
    expected: &T,
    timeout: Duration,
    mut observe: impl FnMut() -> Fut,
) -> Result<Result<(), T>, Report>
where
    T: PartialEq,
    Fut: Future<Output = Result<T, Report>>,
{
    let deadline = Instant::now() + timeout;
    loop {
        let actual = observe().await?;
        if actual == *expected {
            return Ok(Ok(()));
        }
        if Instant::now() >= deadline {
            return Ok(Err(actual));
        }
        tokio::time::sleep(POLL_INTERVAL).await;
    }
}

/// Summarize the checks of a `*KnownIssues` test: each `(issue, result)` pair is one independent
/// check of a known bug. Fails listing every still-broken issue, and warns about issues that
/// pass now, so that they get moved into the regular test.
// Used by `*KnownIssues` tests; there are none at the moment.
#[allow(dead_code)]
pub fn known_issues_outcome(results: Vec<(&str, Result<(), Report>)>) -> Result<(), Report> {
    let mut broken = Vec::new();
    for (issue, result) in results {
        match result {
            Ok(()) => tracing::warn!(
                "Known issue '{issue}' passes now; move its check into the regular test."
            ),
            Err(err) => broken.push(format!("- {issue}: {err}")),
        }
    }
    if broken.is_empty() {
        return Ok(());
    }
    bail!(
        "{} known issue(s) still broken:\n{}",
        broken.len(),
        broken.join("\n")
    )
}

/// The text the browser renders for `element` (`innerText`). WebDriver's own text algorithm skips
/// the children of `display: contents` elements in some layouts (e.g. a grid list row's cell in a
/// flex row), which the atoms use as react-aria-components does.
pub async fn rendered_text(element: &WebElement) -> Result<String, Report> {
    Ok(element
        .prop("innerText")
        .await
        .context("failed to read innerText")?
        .unwrap_or_default())
}
