// Upstream: react-aria-components/test/Toolbar.test.tsx @ 99e6102368
//! The toolbar atom ("supports keyboard navigation"): one tab stop, arrow keys along its
//! orientation across nested toolbars and dividers without wrapping, Tab leaving and re-entering
//! at the control focused last; nested toolbars are groups; vertical and right-to-left toolbars;
//! toolbars of toggle buttons, checkboxes and links.
use assertr::prelude::*;
use browser_test::thirtyfour::prelude::*;
use rootcause::Report;

use crate::pages::{Page, PageActions, role, xpath};

const PATH: &str = "/atoms/toolbar";

/// The button with the text `text`.
async fn button(page: &Page<'_>, text: &str) -> Result<WebElement, Report> {
    page.element(role("button").text(text)).await
}

/// Clicks the button with the text `text` and waits until it has the focus.
async fn focus_button(page: &Page<'_>, text: &str) -> Result<(), Report> {
    let button = button(page, text).await?;
    button.click().await?;
    page.wait_for_focus(&button).await?;
    Ok(())
}

/// A horizontal toolbar; nested toolbars are groups.
pub async fn structure(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let tools = page.element("[aria-label=Tools]").await?;
    assert_that!(tools.attr("role").await?)
        .get_some()
        .is_equal_to("toolbar");
    assert_that!(tools.attr("aria-orientation").await?)
        .get_some()
        .is_equal_to("horizontal");
    let align = page.element("[aria-label='Align text']").await?;
    assert_that!(align.attr("role").await?)
        .get_some()
        .is_equal_to("group");
    Ok(())
}

/// One tab stop; the arrow keys along the orientation move across nested toolbars and dividers.
pub async fn keyboard_navigation(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    button(page, "Before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&button(page, "Align left").await?)
        .await?;
    page.send_keys(Key::Right).await?;
    page.wait_for_focus(&button(page, "Align center").await?)
        .await?;
    // Down does nothing in a horizontal toolbar.
    page.send_keys(Key::Down).await?;
    page.focus_stays(&button(page, "Align center").await?)
        .await?;
    page.send_keys(Key::Right).await?;
    // Across the divider into the next group.
    page.send_keys(Key::Right).await?;
    page.wait_for_focus(&button(page, "Zoom in").await?).await?;
    page.send_keys(Key::Left).await?;
    page.wait_for_focus(&button(page, "Align right").await?)
        .await?;
    Ok(())
}

/// Tab leaves; Shift+Tab re-enters at the control focused last.
pub async fn tab_leaves_and_reenters(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    focus_button(page, "Align right").await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&button(page, "After").await?).await?;
    page.send_keys(Key::Shift + Key::Tab).await?;
    page.wait_for_focus(&button(page, "Align right").await?)
        .await?;
    page.send_keys(Key::Shift + Key::Tab).await?;
    page.wait_for_focus(&button(page, "Before").await?).await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&button(page, "Align right").await?)
        .await?;
    Ok(())
}

/// The arrow keys stop at either end.
pub async fn no_wrapping(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    focus_button(page, "Align right").await?;
    page.send_keys(Key::Left).await?;
    page.send_keys(Key::Left).await?;
    page.wait_for_focus(&button(page, "Align left").await?)
        .await?;
    page.send_keys(Key::Left).await?;
    page.focus_stays(&button(page, "Align left").await?).await?;
    focus_button(page, "Zoom out").await?;
    page.send_keys(Key::Right).await?;
    page.focus_stays(&button(page, "Zoom out").await?).await?;
    Ok(())
}

/// "supports keyboard navigation with orientation vertical".
pub async fn vertical(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let vertical = page.element("[aria-label=Vertical]").await?;
    assert_that!(vertical.attr("aria-orientation").await?)
        .get_some()
        .is_equal_to("vertical");
    button(page, "Up 1").await?.click().await?;
    page.send_keys(Key::Down).await?;
    page.wait_for_focus(&button(page, "Up 2").await?).await?;
    page.send_keys(Key::Right).await?;
    page.focus_stays(&button(page, "Up 2").await?).await?;
    page.send_keys(Key::Up).await?;
    page.wait_for_focus(&button(page, "Up 1").await?).await?;
    Ok(())
}

/// "supports RTL": the arrow keys follow the reading direction.
pub async fn right_to_left(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    button(page, "RTL 1").await?.click().await?;
    page.send_keys(Key::Left).await?;
    page.wait_for_focus(&button(page, "RTL 2").await?).await?;
    page.send_keys(Key::Right).await?;
    page.wait_for_focus(&button(page, "RTL 1").await?).await?;
    Ok(())
}

/// "supports RTL with orientation vertical": up and down move; left and right don't.
pub async fn right_to_left_vertical(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    button(page, "RV 1").await?.click().await?;
    page.send_keys(Key::Down).await?;
    page.wait_for_focus(&button(page, "RV 2").await?).await?;
    page.send_keys(Key::Up).await?;
    page.wait_for_focus(&button(page, "RV 1").await?).await?;
    page.send_keys(Key::Left).await?;
    page.send_keys(Key::Right).await?;
    page.focus_stays(&button(page, "RV 1").await?).await?;
    Ok(())
}

/// "supports all the aria example children": toggle buttons, a checkbox and a link, without
/// wrapping at the end.
pub async fn aria_example_children(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-toolbar-input-before")
        .await?
        .click()
        .await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&button(page, "B").await?).await?;
    page.send_keys(Key::Right).await?;
    page.wait_for_focus(&button(page, "U").await?).await?;
    page.send_keys(Key::Right).await?;
    page.wait_for_focus(&button(page, "I").await?).await?;
    page.send_keys(Key::Right).await?;
    let night_mode = page
        .element(xpath(
            "//label[contains(normalize-space(.), 'Night Mode')]//input[@type='checkbox']",
        ))
        .await?;
    page.wait_for_focus(&night_mode).await?;
    page.send_keys(Key::Right).await?;
    page.wait_for_focus(&page.element(role("link").text("Help")).await?)
        .await?;
    page.send_keys(Key::Right).await?;
    page.focus_stays(&page.element(role("link").text("Help")).await?)
        .await?;
    Ok(())
}
