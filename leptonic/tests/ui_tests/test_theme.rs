// No upstream: react-aria-components has no theme provider; leptonic's `ThemeProvider` applies
//! The theme provider: its context reaches its children only (not its later siblings), its element
//! carries `data-theme` and its default class, and `use_theme` switches it.
// `data-theme` and offers the theme to `use_theme`.
use assertr::prelude::*;
use browser_test::browser_test;
use leptonic::AriaRole;
use rootcause::Report;

use crate::pages::{ElementActions, Page, role};

const PATH: &str = "/atoms/theme";

/// The provider's children see its theme; the sibling after the dark provider sees the app's
/// provider (light). The provider's element carries `data-theme` and its default class.
#[browser_test]
pub async fn context(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    assert_that!(page.element("#test-theme-inside").await?)
        .inner_text()
        .await
        .is_equal_to("Dark");
    assert_that!(page.element("#test-theme-after").await?)
        .inner_text()
        .await
        .is_equal_to("Light");

    let provider = page.element(".test-theme-dark").await?;
    assert_that!(provider)
        .has_attribute("data-theme")
        .await
        .is_equal_to("dark");
    assert_that!(provider)
        .has_attribute("class")
        .await
        .is_equal_to("leptonic-ThemeProvider test-theme-dark");
    Ok(())
}

/// `use_theme` switches the nested provider only; it leaves `<html data-theme>` to the app's
/// (outermost) provider.
#[browser_test]
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
        .inner_text_stays("Light", std::time::Duration::from_millis(100))
        .await?;
    page.element("#test-theme-toggle").await?.click().await?;
    page.element("#test-theme-inside")
        .await?
        .wait_for_inner_text("Dark")
        .await?;
    assert_that!(page.element("html").await?)
        .has_attribute("data-theme")
        .await
        .is_equal_to("light");
    Ok(())
}

/// A provider with `set_theme` but without `theme` keeps its own theme: switching shows the new
/// theme and hands it to `set_theme`; `on_theme_change` is called only when the theme changes.
#[browser_test]
pub async fn setter_without_theme(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let inside = page.element("#test-theme-setter-inside").await?;
    let changes = page.element("#test-theme-setter-changes").await?;
    assert_that!(inside).inner_text().await.is_equal_to("Light");

    // Already light: no change.
    page.element("#test-theme-setter-light")
        .await?
        .click()
        .await?;
    page.settle().await?;
    changes
        .inner_text_stays("0", std::time::Duration::from_millis(100))
        .await?;

    page.element("#test-theme-setter-dark")
        .await?
        .click()
        .await?;
    inside.wait_for_inner_text("Dark").await?;
    page.element(".test-theme-setter")
        .await?
        .wait_for_attr("data-theme", Some("dark"))
        .await?;
    page.element("#test-theme-setter-written")
        .await?
        .wait_for_inner_text("Dark")
        .await?;
    changes.wait_for_inner_text("1").await?;
    Ok(())
}

/// A provider with a fixed `theme` and no setter keeps its theme: switching only reports the
/// requested theme to `on_theme_change`.
#[browser_test]
pub async fn controlled_without_setter(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-theme-controlled-light")
        .await?
        .click()
        .await?;
    page.element("#test-theme-controlled-requests")
        .await?
        .wait_for_inner_text("Light;")
        .await?;
    page.element("#test-theme-controlled-inside")
        .await?
        .inner_text_stays("Dark", std::time::Duration::from_millis(100))
        .await?;
    assert_that!(page.element(".test-theme-controlled").await?)
        .has_attribute("data-theme")
        .await
        .is_equal_to("dark");
    Ok(())
}

/// A root provider removes its document attribute on unmount, including after remounting.
#[browser_test]
pub async fn root_removes_document_theme_on_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let html = page.element("html").await?;
    html.wait_for_attr("data-theme", Some("light")).await?;
    set_document_theme(page, None).await?;

    for _ in 0..2 {
        page.element(role(AriaRole::Button).text("Mount root provider"))
            .await?
            .click()
            .await?;
        html.wait_for_attr("data-theme", Some("dark")).await?;
        page.element(role(AriaRole::Button).text("Unmount root provider"))
            .await?
            .click()
            .await?;
        page.wait_for_count(".test-theme-root", 0).await?;
        html.wait_for_attr("data-theme", None).await?;
    }
    Ok(())
}

/// Theme updates must retain the original document value for cleanup.
#[browser_test]
pub async fn root_restores_previous_document_theme(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let html = page.element("html").await?;
    html.wait_for_attr("data-theme", Some("light")).await?;
    set_document_theme(page, Some("author-theme")).await?;
    page.element(role(AriaRole::Button).text("Mount root provider"))
        .await?
        .click()
        .await?;
    html.wait_for_attr("data-theme", Some("dark")).await?;
    page.element(".test-theme-root")
        .await?
        .element(role(AriaRole::Button).text("Light"))
        .await?
        .click()
        .await?;
    html.wait_for_attr("data-theme", Some("light")).await?;
    page.element(role(AriaRole::Button).text("Unmount root provider"))
        .await?
        .click()
        .await?;
    page.wait_for_count(".test-theme-root", 0).await?;
    html.wait_for_attr("data-theme", Some("author-theme"))
        .await?;
    Ok(())
}

/// A nested provider never owns the document attribute, including during cleanup.
#[browser_test]
pub async fn nested_unmount_preserves_document_theme(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element(role(AriaRole::Button).text("Mount root provider"))
        .await?
        .click()
        .await?;
    let html = page.element("html").await?;
    html.wait_for_attr("data-theme", Some("dark")).await?;
    page.element(role(AriaRole::Button).text("Unmount nested provider"))
        .await?
        .click()
        .await?;
    page.wait_for_count(".test-theme-nested", 0).await?;
    html.attr_stays(
        "data-theme",
        Some("dark"),
        std::time::Duration::from_millis(100),
    )
    .await?;
    Ok(())
}

/// Cleanup leaves an attribute written by another owner after the provider's last update.
#[browser_test]
pub async fn root_unmount_preserves_external_document_theme(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element(role(AriaRole::Button).text("Mount root provider"))
        .await?
        .click()
        .await?;
    let html = page.element("html").await?;
    html.wait_for_attr("data-theme", Some("dark")).await?;
    set_document_theme(page, Some("external-theme")).await?;
    page.element(role(AriaRole::Button).text("Unmount root provider"))
        .await?
        .click()
        .await?;
    page.wait_for_count(".test-theme-root", 0).await?;
    html.attr_stays(
        "data-theme",
        Some("external-theme"),
        std::time::Duration::from_millis(100),
    )
    .await?;
    Ok(())
}

/// Simulates an application document value set outside ThemeProvider.
async fn set_document_theme(page: &Page<'_>, theme: Option<&str>) -> Result<(), Report> {
    page.low_level()
        .eval(
            "if (arguments[0] === null) {
                document.documentElement.removeAttribute('data-theme');
            } else {
                document.documentElement.setAttribute('data-theme', arguments[0]);
            }",
            vec![serde_json::to_value(theme)?],
        )
        .await
}
