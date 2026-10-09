//! Checks the documentation sidebar: collapsible groups, layer markers of concepts, badges of building blocks, and the
//! tabs of concept pages.

use assertr::prelude::*;
use browser_test::thirtyfour::By;
use leptos_browser_test::{Report, ResultExt};

use crate::pages::BookPage;

/// `aria-expanded` of the sidebar's toggle of the group named `group`.
fn expanded(group: &str) -> String {
    format!(
        "return document.querySelector('#book-doc-sidebar .book-nav-group-toggle[aria-label=\"{group} pages\"]')\
         ?.getAttribute('aria-expanded') === 'true';"
    )
}

fn collapsed(group: &str) -> String {
    format!(
        "return document.querySelector('#book-doc-sidebar .book-nav-group-toggle[aria-label=\"{group} pages\"]')\
         ?.getAttribute('aria-expanded') === 'false';"
    )
}

/// Loads the hook page of Button at desktop width, where the sidebar shows.
async fn open_button_hook(page: &BookPage<'_>) -> Result<(), Report> {
    page.set_viewport(1600, 1000).await?;
    page.goto("/doc/button/hook").await
}

/// The sidebar expands the group of the current page and keeps others collapsed.
pub async fn current_group_is_expanded(page: &BookPage<'_>) -> Result<(), Report> {
    open_button_hook(page).await?;
    page.wait_until(
        "the group of the current page is expanded",
        &expanded("Buttons"),
    )
    .await?;
    page.wait_until("other concept groups are collapsed", &collapsed("Fields"))
        .await?;
    Ok(())
}

/// Concepts show the layers they exist in, building blocks the badge of their kind.
pub async fn concepts_show_layers_and_building_blocks_badges(
    page: &BookPage<'_>,
) -> Result<(), Report> {
    open_button_hook(page).await?;
    // Button exists as hooks and atoms.
    let present = page
        .strings(
            "return [...document.querySelectorAll('#book-doc-sidebar a.book-nav-item[href=\"/doc/button\"] \
             .book-layer-marks [data-present=true]')].map(mark => mark.textContent);",
        )
        .await?;
    assert_that!(present).is_equal_to(vec!["H".to_owned(), "A".to_owned()]);

    let badges = page
        .strings(
            "return [...document.querySelectorAll('#book-doc-sidebar \
             a.book-nav-item[href=\"/doc/interactions/use-press\"] .book-badge')].map(badge => badge.textContent);",
        )
        .await?;
    assert_that!(badges).is_equal_to(vec!["hook".to_owned()]);
    Ok(())
}

/// The tabs of a concept are its overview and the tabs of its layers, as the navigation defines them.
pub async fn concept_tabs_are_named_after_the_layers(page: &BookPage<'_>) -> Result<(), Report> {
    open_button_hook(page).await?;
    let tabs = page
        .strings("return [...document.querySelectorAll('.doc-concept-tabs a')].map(tab => tab.textContent);")
        .await?;
    let button = book_ssr::nav::nav()
        .concept_at("/doc/button/hook")
        .expect("Button is a concept in the navigation");
    let expected: Vec<String> = std::iter::once("Overview")
        .chain(button.tabs.iter().map(book_ssr::nav::NavTab::label))
        .map(str::to_owned)
        .collect();
    assert_that!(tabs).is_equal_to(expected);
    Ok(())
}

/// A group's toggle expands it; navigating to the overview of a collapsed area expands that area, and the group the
/// user expanded stays expanded.
pub async fn toggles_and_navigation_expand_groups(page: &BookPage<'_>) -> Result<(), Report> {
    open_button_hook(page).await?;
    let toggle = page
        .driver
        .find(By::Css(
            "#book-doc-sidebar .book-nav-group-toggle[aria-label=\"Fields pages\"]",
        ))
        .await
        .context("the Fields group has a toggle")?;
    toggle.scroll_into_view().await?;
    toggle.click().await?;
    page.wait_until("the toggle expands the group", &expanded("Fields"))
        .await?;
    page.wait_until(
        "the expanded group shows its entries",
        "return document.querySelector('#book-doc-sidebar a.book-nav-item[href=\"/doc/checkbox\"]')\
         .getBoundingClientRect().height > 0;",
    )
    .await?;

    let interactions = page
        .driver
        .find(By::Css(
            "#book-doc-sidebar a.book-nav-group-title[href=\"/doc/interactions\"]",
        ))
        .await
        .context("the Interactions area links its overview")?;
    interactions.scroll_into_view().await?;
    interactions.click().await?;
    page.wait_until(
        "the overview opens",
        "return location.pathname === '/doc/interactions';",
    )
    .await?;
    page.wait_until(
        "the area of the new page is expanded",
        &expanded("Interactions"),
    )
    .await?;
    page.wait_until(
        "a group the user expanded stays expanded",
        &expanded("Fields"),
    )
    .await?;
    Ok(())
}
