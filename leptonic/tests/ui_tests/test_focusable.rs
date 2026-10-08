// Upstream: react-aria/test/interactions/Focusable.test.js @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{BrowserTest, async_trait, thirtyfour::prelude::*};
use rootcause::Report;

use crate::pages::{ElementActions, Page, PageActions};

/// `use_focusable`: tab index for disabled and excluded elements, auto focus, keyboard events and
/// the focus handle ("supports isDisabled", "supports excludeFromTabOrder", "supports autoFocus").
/// Every case starts on a fresh page.
pub struct FocusableTests {}

#[async_trait]
impl BrowserTest<str> for FocusableTests {
    fn name(&self) -> Cow<'_, str> {
        "focusable_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        cases!(
            tabindex_attributes(&page),
            keyboard_events(&page),
            tab_skip(&page),
            focus_handle(&page),
            dynamic_disabled_transition(&page),
        );
        Ok(())
    }
}

const PATH: &str = "/hooks/focusable";
const NORMAL: &str = "#test-fcbl-normal";

/// Tab index: normal 0, disabled none, excluded -1. The auto focus element is focused on load.
async fn tabindex_attributes(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let normal = page.element(NORMAL).await?;
    let disabled = page.element("#test-fcbl-disabled").await?;
    let excluded = page.element("#test-fcbl-excluded").await?;
    assert_that!(normal.attr("tabindex").await?)
        .get_some()
        .is_equal_to("0");
    assert_that!(disabled.attr("tabindex").await?).is_none();
    assert_that!(excluded.attr("tabindex").await?)
        .get_some()
        .is_equal_to("-1");

    page.wait_for_focus(&page.element("#test-fcbl-autofocus").await?)
        .await?;
    Ok(())
}

/// Keyboard events reach the element's handlers.
async fn keyboard_events(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let normal = page.element(NORMAL).await?;
    let keydown_count = page.element("#test-fcbl-keydown-count").await?;
    let keyup_count = page.element("#test-fcbl-keyup-count").await?;

    normal.click().await?;
    page.wait_for_focus(&normal).await?;
    assert_that!(keydown_count.inner_text().await?.parse::<u32>()?).is_equal_to(0);
    assert_that!(keyup_count.inner_text().await?.parse::<u32>()?).is_equal_to(0);

    page.send_keys("a").await?;
    keydown_count.wait_for_inner_text("1").await?;
    keyup_count.wait_for_inner_text("1").await?;
    Ok(())
}

/// Tab from the normal element skips the disabled (no tabindex) and the excluded (tabindex -1)
/// one.
async fn tab_skip(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let normal = page.element(NORMAL).await?;
    normal.click().await?;
    page.wait_for_focus(&normal).await?;

    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&page.element("#test-fcbl-tab-target").await?)
        .await?;
    Ok(())
}

/// Programmatic focus through the `FocusHandle`.
async fn focus_handle(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-fcbl-focus-btn").await?.click().await?;
    page.wait_for_focus(&page.element(NORMAL).await?).await?;
    Ok(())
}

/// Toggling `disabled` updates the tab index.
async fn dynamic_disabled_transition(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let dynamic = page.element("#test-fcbl-dynamic").await?;
    let toggle = page.element("#test-fcbl-dynamic-toggle").await?;
    assert_that!(dynamic.attr("tabindex").await?)
        .get_some()
        .is_equal_to("0");

    toggle.click().await?;
    dynamic.wait_for_attr("tabindex", None).await?;

    toggle.click().await?;
    dynamic.wait_for_attr("tabindex", Some("0")).await?;
    Ok(())
}
