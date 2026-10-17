// No upstream: `id` is a plain DOM prop in react-aria-components.
//! Atoms whose element has an id of its own take the app's id as a prop (react-aria-components'
//! `id`): an `attr:id` can't replace it (the server renders both, the browser keeps the first).
//! Ids derived from it (rows, cells, tags) follow.
use assertr::prelude::*;
use browser_test::browser_test;
use leptonic::AriaRole;
use rootcause::Report;

use crate::pages::{ElementActions, Page, role};

const PATH: &str = "/atoms/own-ids";

/// Asserts that the element with the id `id` has the role `expected`.
async fn has_role(page: &Page<'_>, id: &str, expected: &str) -> Result<(), Report> {
    let element = page.element(format!("#{id}")).await?;
    assert_that!(element.accessible_role().await?).is_equal_to(expected.to_owned());
    Ok(())
}

/// The grid list is the element with the app's id, and its rows' ids derive from it.
#[browser_test]
pub async fn grid_list(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    has_role(page, "own-grid-list", "grid").await?;
    let row = page
        .element("#own-grid-list")
        .await?
        .first_element(role(AriaRole::Row))
        .await?;
    assert_that!(row.id().await?.unwrap_or_default()).starts_with("own-grid-list");
    Ok(())
}

/// The grid is the element with the app's id.
#[browser_test]
pub async fn grid(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    has_role(page, "own-grid", "grid").await
}

/// The table is the element with the app's id.
#[browser_test]
pub async fn table(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    has_role(page, "own-table", "grid").await
}

/// The tag list is the element with the app's id.
#[browser_test]
pub async fn tag_group(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    has_role(page, "own-tag-list", "grid").await
}

/// The combo box's input is the element with the app's id.
#[browser_test]
pub async fn combo_box(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    has_role(page, "own-combo-box", "combobox").await
}

/// The select's trigger is the element with the app's id.
#[browser_test]
pub async fn select(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let trigger = page.element("#own-select").await?;
    assert_that!(trigger)
        .has_attribute("aria-haspopup")
        .await
        .is_equal_to("listbox");
    Ok(())
}

/// The color swatch is the element with the app's id.
#[browser_test]
pub async fn color_swatch(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    has_role(page, "own-color-swatch", "image").await
}
