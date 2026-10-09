// Upstream: react-aria-components/test/Toolbar.test.tsx @ 99e6102368
//! The toolbar atom ("supports keyboard navigation"): one tab stop, arrow keys along its
//! orientation across nested toolbars and dividers without wrapping, Tab leaving and re-entering
//! at the control focused last; nested toolbars are groups; vertical and right-to-left toolbars;
//! toolbars of toggle buttons, checkboxes and links.
use assertr::prelude::*;
use browser_test::{browser_test, thirtyfour::prelude::*};
use leptonic::AriaRole;
use rootcause::Report;

use crate::pages::{ElementActions, Page, css, role};

const PATH: &str = "/atoms/toolbar";

/// The button with the text `text`.
async fn button(page: &Page<'_>, text: &str) -> Result<WebElement, Report> {
    page.element(role(AriaRole::Button).text(text)).await
}

/// Clicks the button `text` in the Tools toolbar and waits until it has the focus.
async fn focus_tools_button(page: &Page<'_>, text: &str) -> Result<WebElement, Report> {
    let button = page
        .element("[aria-label=Tools]")
        .await?
        .element(role(AriaRole::Button).text(text))
        .await?;
    button.click().await?;
    page.wait_for_focus(&button).await?;
    Ok(button)
}

/// The toolbar is horizontal by default, and toolbars nested in it are groups ("renders").
#[browser_test]
pub async fn structure(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let tools = page.element("[aria-label=Tools]").await?;
    assert_that!(tools)
        .has_attribute("role")
        .await
        .is_equal_to("toolbar");
    assert_that!(tools)
        .has_attribute("aria-orientation")
        .await
        .is_equal_to("horizontal");
    let align = page.element("[aria-label='Align text']").await?;
    assert_that!(align)
        .has_attribute("role")
        .await
        .is_equal_to("group");
    Ok(())
}

/// Tab enters the toolbar at its first control, and ArrowRight/Left move across nested toolbars and
/// dividers while ArrowDown does nothing ("supports keyboard navigation").
#[browser_test]
pub async fn keyboard_navigation(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let align_right = page
        .element("[aria-label=Tools]")
        .await?
        .element(role(AriaRole::Button).text("Align right"))
        .await?;
    button(page, "Before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&button(page, "Align left").await?)
        .await?;
    page.send_keys(Key::Right).await?;
    page.wait_for_focus(&button(page, "Align center").await?)
        .await?;
    // Down does nothing in a horizontal toolbar.
    page.send_keys(Key::Down).await?;
    page.focus_stays(
        &button(page, "Align center").await?,
        std::time::Duration::from_millis(100),
    )
    .await?;
    page.send_keys(Key::Right).await?;
    // Across the divider into the next group.
    page.send_keys(Key::Right).await?;
    page.wait_for_focus(&button(page, "Zoom in").await?).await?;
    page.send_keys(Key::Left).await?;
    page.wait_for_focus(&align_right).await?;
    Ok(())
}

/// Tab leaves the toolbar, and Tab or Shift+Tab back into it lands on the control focused last
/// ("supports keyboard navigation").
#[browser_test]
pub async fn tab_leaves_and_reenters(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let align_right = focus_tools_button(page, "Align right").await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&button(page, "After").await?).await?;
    page.send_keys(Key::Shift + Key::Tab).await?;
    page.wait_for_focus(&align_right).await?;
    page.send_keys(Key::Shift + Key::Tab).await?;
    page.wait_for_focus(&button(page, "Before").await?).await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&align_right).await?;
    Ok(())
}

/// The arrow keys stop at either end of the toolbar instead of wrapping ("supports keyboard
/// navigation").
#[browser_test]
pub async fn no_wrapping(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    focus_tools_button(page, "Align right").await?;
    page.send_keys(Key::Left).await?;
    page.send_keys(Key::Left).await?;
    page.wait_for_focus(&button(page, "Align left").await?)
        .await?;
    page.send_keys(Key::Left).await?;
    page.focus_stays(
        &button(page, "Align left").await?,
        std::time::Duration::from_millis(100),
    )
    .await?;
    focus_tools_button(page, "Zoom out").await?;
    page.send_keys(Key::Right).await?;
    page.focus_stays(
        &button(page, "Zoom out").await?,
        std::time::Duration::from_millis(100),
    )
    .await?;
    Ok(())
}

/// In a vertical toolbar, ArrowDown and ArrowUp move focus and ArrowRight does nothing ("supports
/// keyboard navigation with orientation vertical").
#[browser_test]
pub async fn vertical(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let vertical = page.element("[aria-label=Vertical]").await?;
    assert_that!(vertical)
        .has_attribute("aria-orientation")
        .await
        .is_equal_to("vertical");
    button(page, "Up 1").await?.click().await?;
    page.send_keys(Key::Down).await?;
    page.wait_for_focus(&button(page, "Up 2").await?).await?;
    page.send_keys(Key::Right).await?;
    page.focus_stays(
        &button(page, "Up 2").await?,
        std::time::Duration::from_millis(100),
    )
    .await?;
    page.send_keys(Key::Up).await?;
    page.wait_for_focus(&button(page, "Up 1").await?).await?;
    Ok(())
}

/// In a right-to-left toolbar, ArrowLeft moves focus to the next control and ArrowRight to the
/// previous ("supports RTL").
#[browser_test]
pub async fn right_to_left(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    button(page, "RTL 1").await?.click().await?;
    page.send_keys(Key::Left).await?;
    page.wait_for_focus(&button(page, "RTL 2").await?).await?;
    page.send_keys(Key::Right).await?;
    page.wait_for_focus(&button(page, "RTL 1").await?).await?;
    Ok(())
}

/// In a right-to-left vertical toolbar, ArrowDown and ArrowUp move focus while ArrowLeft and
/// ArrowRight do nothing ("supports RTL with orientation vertical").
#[browser_test]
pub async fn right_to_left_vertical(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    button(page, "RV 1").await?.click().await?;
    page.send_keys(Key::Down).await?;
    page.wait_for_focus(&button(page, "RV 2").await?).await?;
    page.send_keys(Key::Up).await?;
    page.wait_for_focus(&button(page, "RV 1").await?).await?;
    page.send_keys(Key::Left).await?;
    page.send_keys(Key::Right).await?;
    page.focus_stays(
        &button(page, "RV 1").await?,
        std::time::Duration::from_millis(100),
    )
    .await?;
    Ok(())
}

/// ArrowRight moves focus through a toolbar of toggle buttons, a checkbox and a link, and stops at
/// the link ("supports all the aria example children").
#[browser_test]
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
        .element(css("label").text("Night Mode"))
        .await?
        .element("input[type=checkbox]")
        .await?;
    page.wait_for_focus(&night_mode).await?;
    page.send_keys(Key::Right).await?;
    page.wait_for_focus(&page.element(role(AriaRole::Link).text("Help")).await?)
        .await?;
    page.send_keys(Key::Right).await?;
    page.focus_stays(
        &page.element(role(AriaRole::Link).text("Help")).await?,
        std::time::Duration::from_millis(100),
    )
    .await?;
    Ok(())
}

/// A toolbar carries its default class and its orientation as `data-orientation` ("renders",
/// "support render props").
#[browser_test]
pub async fn default_class_and_orientation(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let tools = page.element("[aria-label=Tools]").await?;
    assert_that!(tools)
        .has_attribute("class")
        .await
        .is_equal_to("leptonic-Toolbar");
    assert_that!(tools)
        .has_attribute("data-orientation")
        .await
        .is_equal_to("horizontal");
    let vertical = page.element("[aria-label=Vertical]").await?;
    assert_that!(vertical)
        .has_attribute("data-orientation")
        .await
        .is_equal_to("vertical");
    assert_that!(vertical)
        .has_attribute("aria-orientation")
        .await
        .is_equal_to("vertical");
    Ok(())
}

/// Dividers between a toolbar's controls stay separators ("renders dividers").
#[browser_test]
pub async fn dividers(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let tools = page.element("[aria-label=Tools]").await?;
    assert_that!(tools.count(role(AriaRole::Separator)).await?).is_equal_to(1);
    Ok(())
}

/// A toolbar with an `aria_label` is named by it alone: it has no `aria-labelledby`, also when one
/// is given ("sets aria-label").
#[browser_test]
pub async fn aria_label_wins_over_labelledby(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let toolbar = page.element("[aria-label='Labelled twice']").await?;
    assert_that!(toolbar)
        .attribute("aria-labelledby")
        .await
        .is_none();
    assert_that!(toolbar)
        .accessible_name()
        .await
        .is_equal_to("Labelled twice");
    let tools = page.element("[aria-label=Tools]").await?;
    assert_that!(tools)
        .attribute("aria-labelledby")
        .await
        .is_none();
    Ok(())
}
