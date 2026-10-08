// Upstream: react-aria-components/test/Link.test.js @ 99e6102368
//! The link atom: `aria-current` by route (prefix or exact), client-side navigation, `replace`,
//! `target`/`rel` for new tabs, a disabled link (no `href`, `aria-disabled`, not followed, no
//! presses), the props of a surrounding trigger, hover/focus/press state, Enter, a disabled
//! `use_link` anchor, and `AnchorLink` (scrolls, sets the hash without a history entry, leaves
//! modified clicks to the browser).
use assertr::prelude::*;
use browser_test::thirtyfour::prelude::*;
use rootcause::Report;

use crate::{
    pages::{ElementActions, Page, PageActions},
    polling::wait_for,
};

const PATH: &str = "/atoms/link";

/// The current page by route: a prefix matches unless the link is exact.
pub async fn current_page(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    assert_that!(
        page.element("#test-link-self")
            .await?
            .attr("aria-current")
            .await?
    )
    .get_some()
    .is_equal_to("page");
    assert_that!(
        page.element("#test-link-prefix")
            .await?
            .attr("aria-current")
            .await?
    )
    .get_some()
    .is_equal_to("page");
    assert_that!(
        page.element("#test-link-exact")
            .await?
            .attr("aria-current")
            .await?
    )
    .is_none();
    Ok(())
}

/// A new tab gets `noopener`.
pub async fn new_tab(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let external = page.element("#test-link-external").await?;
    assert_that!(external.attr("target").await?)
        .get_some()
        .is_equal_to("_blank");
    assert_that!(external.attr("rel").await?)
        .get_some()
        .is_equal_to("noopener");
    Ok(())
}

/// A link as a trigger gets the trigger's props (react-aria's menu triggers:
/// `aria-haspopup="true"`).
pub async fn trigger_props(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let menu = page.element("#test-link-menu").await?;
    assert_that!(menu.attr("aria-haspopup").await?)
        .get_some()
        .is_equal_to("true");
    assert_that!(menu.attr("aria-expanded").await?)
        .get_some()
        .is_equal_to("false");
    Ok(())
}

/// "should support disabled state", "should not navigate if disabled".
pub async fn disabled(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let presses = page.element("#test-link-presses").await?;
    let toggle = page.element("#test-link-toggle-disabled").await?;
    page.element(".test-link-disableable")
        .await?
        .click()
        .await?;
    presses.wait_for_inner_text("1").await?;

    // Disabled, the anchor becomes a `span` with the link role.
    toggle.click().await?;
    let link = page
        .element("span.test-link-disableable[role=link][aria-disabled=true][data-disabled]")
        .await?;
    assert_that!(link.attr("href").await?).is_none();
    link.click().await?;
    presses.inner_text_stays("1").await?;

    toggle.click().await?;
    page.element("a.test-link-disableable[href]").await?;
    Ok(())
}

/// "should support hover", "should support focus ring", "should support press state"; Enter
/// presses a focused link once.
pub async fn state_attributes(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let link = page.element("a.test-link-disableable").await?;
    for name in ["data-hovered", "data-pressed", "data-focus-visible"] {
        assert_that!(link.attr(name).await?)
            .with_detail_message(name)
            .is_none();
    }
    link.hover().await?;
    link.wait_for_attr("data-hovered", Some("true")).await?;
    let presses = page.element("#test-link-presses").await?;
    let count = presses.inner_text().await?.parse::<u32>()?;
    page.driver
        .action_chain()
        .click_and_hold_element(&link)
        .perform()
        .await?;
    link.wait_for_attr("data-pressed", Some("true")).await?;
    page.driver.action_chain().release().perform().await?;
    link.wait_for_attr("data-pressed", None).await?;
    let count = count + 1;
    presses.wait_for_inner_text(&count.to_string()).await?;
    // Pointer focus shows no focus ring.
    link.wait_for_attr("data-focused", Some("true")).await?;
    assert_that!(link.attr("data-focus-visible").await?).is_none();
    page.element("#test-link-toggle-disabled")
        .await?
        .hover()
        .await?;
    link.wait_for_attr("data-hovered", None).await?;

    // Keyboard focus shows it; Enter presses once.
    page.send_keys(Key::Shift + Key::Tab).await?;
    page.send_keys(Key::Tab).await?;
    link.wait_for_attr("data-focus-visible", Some("true"))
        .await?;
    page.send_keys(Key::Enter).await?;
    let count = (count + 1).to_string();
    presses.wait_for_inner_text(&count).await?;
    presses.inner_text_stays(&count).await?;
    page.send_keys(Key::Tab).await?;
    link.wait_for_attr("data-focus-visible", None).await?;
    Ok(())
}

/// A disabled anchor has no `href`, so it keeps the link role explicitly (useLink.test.js
/// "handles isDisabled").
pub async fn disabled_hook_anchor(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let link = page.element("#test-link-hook-disabled").await?;
    assert_that!(link.attr("href").await?).is_none();
    assert_that!(link.attr("role").await?)
        .get_some()
        .is_equal_to("link");
    assert_that!(link.attr("aria-disabled").await?)
        .get_some()
        .is_equal_to("true");
    assert_that!(link.attr("tabindex").await?).is_none();
    Ok(())
}

/// `AnchorLink`: a modified click is the browser's (no scrolling, no hash, the default action not
/// prevented); a press scrolls the target into view and sets the hash, without a history entry.
pub async fn anchor_link(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let hash_before = hash(page).await?;
    let link = page.element("#test-link-anchor").await?;
    let presses = page.element("#test-link-anchor-presses").await?;
    // A listener on the window records whether the click's default was prevented, then prevents
    // it (Ctrl+click would open a tab).
    let prevented: Option<bool> = page
        .eval(
            "let prevented = null;
             const record = e => { prevented = e.defaultPrevented; e.preventDefault(); };
             window.addEventListener('click', record, {once: true});
             arguments[0].dispatchEvent(
                 new MouseEvent('click', {bubbles: true, cancelable: true, ctrlKey: true}));
             return prevented;",
            vec![link.to_json()?],
        )
        .await?;
    assert_that!(prevented).is_equal_to(Some(false));
    presses.wait_for_inner_text("1").await?;
    assert_that!(hash(page).await?).is_equal_to(hash_before);
    assert_that!(scroll_y(page).await?).is_equal_to(0.0);

    let history_before = history_length(page).await?;
    link.click().await?;
    presses.wait_for_inner_text("2").await?;
    assert_that!(hash(page).await?)
        .get_some()
        .is_equal_to("test-link-anchor-target");
    assert_that!(history_length(page).await?).is_equal_to(history_before);
    assert_that!(scroll_y(page).await?).is_greater_than(0.0);
    let target_top = page
        .element("#test-link-anchor-target")
        .await?
        .client_rect()
        .await?
        .top;
    let viewport_height: f64 = page.eval("return window.innerHeight;", vec![]).await?;
    assert_that!(target_top)
        .with_detail_message("the target is in view")
        .is_in_range(0.0..viewport_height);
    page.eval::<()>("window.scrollTo(0, 0);", vec![]).await?;
    Ok(())
}

/// `replace`: no history entry.
pub async fn replace(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let before = history_length(page).await?;
    page.element("#test-link-replace").await?.click().await?;
    wait_for("the query of the URL")
        .observing(|| async { Ok(page.driver.current_url().await?.query().map(str::to_owned)) })
        .to_be_equal_to(Some("replaced".to_owned()))
        .await?;
    page.element("body[data-hydrated]").await?;
    assert_that!(history_length(page).await?).is_equal_to(before);
    Ok(())
}

/// Client-side navigation: the page isn't reloaded.
pub async fn client_side_navigation(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.eval::<()>("window.__testLinkNoReload = true;", vec![])
        .await?;
    page.element("#test-link-toolbar").await?.click().await?;
    page.element("#test-page-atom-toolbar").await?;
    let kept: bool = page
        .eval("return window.__testLinkNoReload === true;", vec![])
        .await?;
    assert_that!(kept)
        .with_detail_message("the link navigated on the client")
        .is_true();
    Ok(())
}

/// The fragment of the current URL.
async fn hash(page: &Page<'_>) -> Result<Option<String>, Report> {
    Ok(page
        .driver
        .current_url()
        .await?
        .fragment()
        .map(str::to_owned))
}

async fn history_length(page: &Page<'_>) -> Result<u64, Report> {
    page.eval("return history.length;", vec![]).await
}

async fn scroll_y(page: &Page<'_>) -> Result<f64, Report> {
    page.eval("return window.scrollY;", vec![]).await
}
