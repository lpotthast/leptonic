// Upstream: react-aria-components/test/TagGroup.test.js @ 99e6102368
// Upstream: react-aria/test/tag/useTagGroup.test.js @ 99e6102368
//! Behavior of the tag group hooks: grid structure, arrow navigation (wrapping, horizontal),
//! removing tags with Delete/Backspace and remove buttons, and focus after removal.
use assertr::prelude::*;
use browser_test::{browser_test, thirtyfour::prelude::*};
use leptonic::AriaRole;
use rootcause::{Report, prelude::ResultExt};

use crate::pages::{ElementActions, KeyKind, Page, SyntheticEvent, role};

const PATH: &str = "/hooks/tag-group";

async fn tag(page: &Page<'_>, text: &str) -> Result<WebElement, Report> {
    page.element(role(AriaRole::Row).text(text)).await
}

/// The tag group is a grid labelled by its visible label, with one row per tag and remove buttons
/// labelled "Remove".
#[browser_test]
pub async fn aria_structure(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let grid = page.element("[role=grid]").await?;
    assert_that!(grid)
        .accessible_name()
        .await
        .is_equal_to("Categories");
    assert_that!(grid.elements("[role=row]").await?).has_length(4);
    for remove in grid.elements("[role=row] button").await? {
        assert_that!(remove)
            .has_attribute("aria-label")
            .await
            .is_equal_to("Remove");
    }
    Ok(())
}

/// Tab focuses the first tag, and Right and Left move focus between the tags, wrapping around at
/// the ends.
#[browser_test]
pub async fn keyboard_navigation(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-tg-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&tag(page, "News×").await?).await?;
    for (key, expected) in [
        (Key::Right, "Travel×"),
        (Key::Right, "Gaming×"),
        (Key::Right, "Shopping×"),
        // Tag groups wrap around.
        (Key::Right, "News×"),
        (Key::Left, "Shopping×"),
    ] {
        page.send_keys(key.clone()).await?;
        page.wait_for_focus(&tag(page, expected).await?)
            .await
            .context_with(|| format!("after pressing {key:?}"))?;
    }
    Ok(())
}

/// Delete or Backspace removes the focused tag and moves focus to the next one, or to the previous
/// one at the end.
#[browser_test]
pub async fn removing_with_the_keyboard_moves_focus_on(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    tag(page, "Gaming×").await?.click().await?;
    page.wait_for_focus(&tag(page, "Gaming×").await?).await?;
    let tags = page.element("#test-tg-tags").await?;
    page.send_keys(Key::Delete).await?;
    tags.wait_for_inner_text("News,Travel,Shopping").await?;
    page.wait_for_focus(&tag(page, "Shopping×").await?).await?;
    page.send_keys(Key::Backspace).await?;
    tags.wait_for_inner_text("News,Travel").await?;
    page.wait_for_focus(&tag(page, "Travel×").await?).await?;
    Ok(())
}

/// Pressing a tag's remove button removes the tag ("should support removing items").
#[browser_test]
pub async fn remove_button(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let button = tag(page, "News×").await?.element("button").await?;
    button.click().await?;
    page.element("#test-tg-tags")
        .await?
        .wait_for_inner_text("Travel,Gaming,Shopping")
        .await?;
    Ok(())
}

/// When the last tag is removed, the (now empty) group keeps focus and becomes a plain group.
#[browser_test]
pub async fn removing_every_tag_focuses_the_group(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let tags = page.element("#test-tg-tags").await?;
    // Every tag but Travel goes, with its remove button.
    for other in ["News×", "Gaming×", "Shopping×"] {
        tag(page, other)
            .await?
            .element("button")
            .await?
            .click()
            .await?;
    }
    tags.wait_for_inner_text("Travel").await?;
    tag(page, "Travel×").await?.click().await?;
    page.wait_for_focus(&tag(page, "Travel×").await?).await?;
    page.send_keys(Key::Delete).await?;
    tags.wait_for_inner_text("").await?;
    let group = page.element("[role=group][aria-labelledby]").await?;
    page.wait_for_focus(&group).await?;
    Ok(())
}

/// Holding Backspace keeps removing: every auto-repeated key press removes the focused tag, and
/// the focus moves on to the next one ("should support repeat keydown events when holding the
/// remove key").
#[browser_test]
pub async fn holding_the_remove_key(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let tags = page.element("#test-tg-tags").await?;
    let news = tag(page, "News×").await?;
    news.click().await?;
    page.wait_for_focus(&news).await?;
    page.focused_element()
        .await?
        .dispatch(SyntheticEvent::keyboard(KeyKind::Down, "Backspace"))
        .await?;
    tags.wait_for_inner_text("Travel,Gaming,Shopping").await?;
    for (focused, remaining) in [
        ("Travel×", "Gaming,Shopping"),
        ("Gaming×", "Shopping"),
        ("Shopping×", ""),
    ] {
        page.wait_for_focus(&tag(page, focused).await?).await?;
        page.focused_element()
            .await?
            .dispatch(SyntheticEvent::keyboard(KeyKind::Down, "Backspace").repeat(true))
            .await?;
        tags.wait_for_inner_text(remaining).await?;
    }
    Ok(())
}
