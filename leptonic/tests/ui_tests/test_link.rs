// Upstream: react-aria-components/test/Link.test.js @ 99e6102368
//! The link atom: `aria-current` by route (prefix or exact), client-side navigation, `replace`,
//! `target`/`rel` for new tabs, a disabled link (no `href`, `aria-disabled`, not followed, no
//! presses), the props of a surrounding trigger, hover/focus/press state, Enter, a disabled
//! `use_link` anchor, and `AnchorLink` (scrolls, sets the hash without a history entry, leaves
//! modified clicks to the browser).
use assertr::{matchers::eq, prelude::*};
use browser_test::{browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::pages::{ElementActions, Page};

const PATH: &str = "/atoms/link";

/// A link to the current route or a prefix of it gets `aria-current="page"`, unless an exact link
/// only matches a prefix.
#[browser_test]
pub async fn current_page(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    assert_that!(page.element("#test-link-self").await?)
        .has_attribute("aria-current")
        .await
        .is_equal_to("page");
    assert_that!(page.element("#test-link-prefix").await?)
        .has_attribute("aria-current")
        .await
        .is_equal_to("page");
    assert_that!(page.element("#test-link-exact").await?)
        .attribute("aria-current")
        .await
        .is_none();
    Ok(())
}

/// A link opening a new tab gets `target="_blank"` and `rel="noopener"`.
#[browser_test]
pub async fn new_tab(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let external = page.element("#test-link-external").await?;
    assert_that!(external)
        .has_attribute("target")
        .await
        .is_equal_to("_blank");
    assert_that!(external)
        .has_attribute("rel")
        .await
        .is_equal_to("noopener");
    Ok(())
}

/// A link used as a menu trigger gets the trigger's props: `aria-haspopup="true"` and
/// `aria-expanded="false"`.
#[browser_test]
pub async fn trigger_props(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let menu = page.element("#test-link-menu").await?;
    assert_that!(menu)
        .has_attribute("aria-haspopup")
        .await
        .is_equal_to("true");
    assert_that!(menu)
        .has_attribute("aria-expanded")
        .await
        .is_equal_to("false");
    Ok(())
}

/// A disabled link renders as a `span` with the link role, `aria-disabled` and no `href`, and
/// clicks don't press it ("should support disabled state", "should not navigate if disabled").
#[browser_test]
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
    assert_that!(link).attribute("href").await.is_none();
    link.click().await?;
    presses
        .inner_text_stays("1", std::time::Duration::from_millis(100))
        .await?;

    toggle.click().await?;
    page.element("a.test-link-disableable[href]").await?;
    Ok(())
}

/// A link shows hover, press and keyboard-only focus-ring state in data attributes, and Enter
/// presses a focused link once ("should support hover", "should support focus ring", "should
/// support press state").
#[browser_test]
pub async fn state_attributes(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let link = page.element("a.test-link-disableable").await?;
    for name in ["data-hovered", "data-pressed", "data-focus-visible"] {
        assert_that!(link)
            .attribute(name)
            .await
            .with_detail_message(name)
            .is_none();
    }
    link.hover().await?;
    link.wait_for_attr("data-hovered", Some("true")).await?;
    let presses = page.element("#test-link-presses").await?;
    let count = presses.inner_text().await?.parse::<u32>()?;
    let held = link.press_and_hold().await?;
    link.wait_for_attr("data-pressed", Some("true")).await?;
    held.release().await?;
    link.wait_for_attr("data-pressed", None).await?;
    let count = count + 1;
    presses.wait_for_inner_text(&count.to_string()).await?;
    // Pointer focus shows no focus ring.
    link.wait_for_attr("data-focused", Some("true")).await?;
    assert_that!(link)
        .attribute("data-focus-visible")
        .await
        .is_none();
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
    presses
        .inner_text_stays(&count, std::time::Duration::from_millis(100))
        .await?;
    page.send_keys(Key::Tab).await?;
    link.wait_for_attr("data-focus-visible", None).await?;
    Ok(())
}

/// A disabled `use_link` anchor has no `href` and no `tabindex`, but an explicit link role and
/// `aria-disabled` (useLink.test.js "handles isDisabled").
#[browser_test]
pub async fn disabled_hook_anchor(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let link = page.element("#test-link-hook-disabled").await?;
    assert_that!(link).attribute("href").await.is_none();
    assert_that!(link)
        .has_attribute("role")
        .await
        .is_equal_to("link");
    assert_that!(link)
        .has_attribute("aria-disabled")
        .await
        .is_equal_to("true");
    assert_that!(link).attribute("tabindex").await.is_none();
    Ok(())
}

/// An `AnchorLink` leaves a modified click to the browser, while a press scrolls its target into
/// view and sets the hash without adding a history entry.
#[browser_test]
pub async fn anchor_link(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let hash_before = hash(page).await?;
    let link = page.element("#test-link-anchor").await?;
    let presses = page.element("#test-link-anchor-presses").await?;
    // A listener on the window records whether the click's default was prevented, then prevents
    // it (Ctrl+click would open a tab).
    let prevented: Option<bool> = page
        .low_level()
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
        .some()
        .is_equal_to("test-link-anchor-target");
    assert_that!(history_length(page).await?).is_equal_to(history_before);
    assert_that!(scroll_y(page).await?).is_greater_than(0.0);
    let target_top = page
        .element("#test-link-anchor-target")
        .await?
        .client_rect()
        .await?
        .top;
    let viewport_height: f64 = page
        .low_level()
        .eval("return window.innerHeight;", vec![])
        .await?;
    assert_that!(target_top)
        .with_detail_message("the target is in view")
        .is_in_range(0.0..viewport_height);
    page.low_level()
        .eval::<()>("window.scrollTo(0, 0);", vec![])
        .await?;
    Ok(())
}

/// A link with `replace` navigates without adding a history entry.
#[browser_test]
pub async fn replace(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let before = history_length(page).await?;
    page.element("#test-link-replace").await?.click().await?;
    assert_that!(|| async {
        Ok::<_, Report>(
            page.low_level()
                .driver()
                .current_url()
                .await?
                .query()
                .map(str::to_owned),
        )
    })
    .eventually_ok()
    .matches(eq(Some("replaced".to_owned())))
    .await;
    page.element("body[data-hydrated]").await?;
    assert_that!(history_length(page).await?).is_equal_to(before);
    Ok(())
}

/// Clicking a link to another route navigates on the client without reloading the page ("should
/// work with RouterProvider").
#[browser_test]
pub async fn client_side_navigation(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.low_level()
        .eval::<()>("window.__testLinkNoReload = true;", vec![])
        .await?;
    page.element("#test-link-toolbar").await?.click().await?;
    page.element("#test-page-atom-toolbar").await?;
    let kept: bool = page
        .low_level()
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
        .low_level()
        .driver()
        .current_url()
        .await?
        .fragment()
        .map(str::to_owned))
}

async fn history_length(page: &Page<'_>) -> Result<u64, Report> {
    page.low_level()
        .eval("return history.length;", vec![])
        .await
}

async fn scroll_y(page: &Page<'_>) -> Result<f64, Report> {
    page.low_level()
        .eval("return window.scrollY;", vec![])
        .await
}
