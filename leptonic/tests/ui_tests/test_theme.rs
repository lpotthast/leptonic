// No upstream: react-aria-components has no theme provider; leptonic's `ThemeProvider` applies
//! The theme provider: its context reaches its children only (not its later siblings), its element
//! carries `data-theme` and its default class, and `use_theme` switches it.
// `data-theme` and offers the theme to `use_theme`.
use assertr::prelude::*;
use rootcause::Report;

use crate::pages::{ElementActions, Page, PageActions};

const PATH: &str = "/atoms/theme";

/// The provider's children see its theme; the sibling after the dark provider sees the app's
/// provider (light). The provider's element carries `data-theme` and its default class.
pub async fn context(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    assert_that!(
        page.element("#test-theme-inside")
            .await?
            .inner_text()
            .await?
    )
    .is_equal_to("Dark");
    assert_that!(
        page.element("#test-theme-after")
            .await?
            .inner_text()
            .await?
    )
    .is_equal_to("Light");

    let provider = page.element(".test-theme-dark").await?;
    assert_that!(provider.attr("data-theme").await?)
        .get_some()
        .is_equal_to("dark");
    assert_that!(provider.attr("class").await?)
        .get_some()
        .is_equal_to("leptonic-ThemeProvider test-theme-dark");
    Ok(())
}

/// `use_theme` switches the nested provider only; it leaves `<html data-theme>` to the app's
/// (outermost) provider.
pub async fn switching(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let provider = page.element(".test-theme-dark").await?;
    page.element("#test-theme-toggle").await?.click().await?;
    page.element("#test-theme-inside")
        .await?
        .wait_for_inner_text("Light")
        .await?;
    provider.wait_for_attr("data-theme", Some("light")).await?;
    page.element("#test-theme-after")
        .await?
        .inner_text_stays("Light")
        .await?;
    page.element("#test-theme-toggle").await?.click().await?;
    page.element("#test-theme-inside")
        .await?
        .wait_for_inner_text("Dark")
        .await?;
    assert_that!(page.element("html").await?.attr("data-theme").await?)
        .get_some()
        .is_equal_to("light");
    Ok(())
}
