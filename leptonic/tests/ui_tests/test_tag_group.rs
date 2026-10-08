// Upstream: react-aria-components/test/TagGroup.test.js @ 99e6102368
//! Behavior of the tag group hooks: grid structure, arrow navigation (wrapping, horizontal),
//! removing tags with Delete/Backspace and remove buttons, and focus after removal.
use assertr::prelude::*;
use browser_test::thirtyfour::prelude::*;
use rootcause::{Report, prelude::ResultExt};

use crate::pages::{ElementActions, Page, PageActions, role};

const PATH: &str = "/hooks/tag-group";

async fn tag(page: &Page<'_>, text: &str) -> Result<WebElement, Report> {
    page.element(role("row").text(text)).await
}

async fn expect_focus(page: &Page<'_>, text: &str) -> Result<(), Report> {
    page.wait_for_focus(&tag(page, text).await?).await?;
    Ok(())
}

pub async fn aria_structure(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let grid = page.element("[role=grid]").await?;
    assert_that!(grid.referenced_text("aria-labelledby").await?).is_equal_to("Categories");
    assert_that!(grid.elements("[role=row]").await?).has_length(4);
    let remove = page.element("[role=row] button").await?;
    assert_that!(remove.attr("aria-label").await?)
        .get_some()
        .is_equal_to("Remove");
    Ok(())
}

pub async fn keyboard_navigation(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-tg-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    expect_focus(page, "News×").await?;
    for (key, expected) in [
        (Key::Right, "Travel×"),
        (Key::Right, "Gaming×"),
        (Key::Right, "Shopping×"),
        // Tag groups wrap around.
        (Key::Right, "News×"),
        (Key::Left, "Shopping×"),
    ] {
        page.send_keys(key.clone()).await?;
        expect_focus(page, expected)
            .await
            .context_with(|| format!("after pressing {key:?}"))?;
    }
    Ok(())
}

/// Removing the focused tag moves focus to the next one (or the previous one at the end).
pub async fn removing_with_the_keyboard_moves_focus_on(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    tag(page, "Gaming×").await?.click().await?;
    expect_focus(page, "Gaming×").await?;
    let tags = page.element("#test-tg-tags").await?;
    page.send_keys(Key::Delete).await?;
    tags.wait_for_inner_text("News,Travel,Shopping").await?;
    expect_focus(page, "Shopping×").await?;
    page.send_keys(Key::Backspace).await?;
    tags.wait_for_inner_text("News,Travel").await?;
    expect_focus(page, "Travel×").await?;
    Ok(())
}

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
    expect_focus(page, "Travel×").await?;
    page.send_keys(Key::Delete).await?;
    tags.wait_for_inner_text("").await?;
    let group = page.element("[role=group][aria-labelledby]").await?;
    page.wait_for_focus(&group).await?;
    Ok(())
}
