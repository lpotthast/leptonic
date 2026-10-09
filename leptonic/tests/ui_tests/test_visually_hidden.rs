// Upstream: react-aria/test/visually-hidden/VisuallyHidden.test.tsx @ 99e6102368
//! `VisuallyHidden`: hidden by inline styles; with `is_focusable`, shown while focus is within it
//! (also when `is_focusable` changes at runtime).
use assertr::prelude::*;
use browser_test::{browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::pages::{ElementActions, Page};

const PATH: &str = "/atoms/visually-hidden";

/// Visually hidden content is hidden by its inline styles, also while focus is within it ("hides
/// element").
#[browser_test]
pub async fn hides_element(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let plain = page.element(".test-vh-plain").await?;
    page.element("#test-vh-plain-a").await?.click().await?;
    let hidden_style = assert_that!(plain)
        .has_attribute("style")
        .await
        .contains("clip-path")
        .actual()
        .clone();
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&page.element("#test-vh-plain-b").await?)
        .await?;
    plain
        .attr_stays(
            "style",
            Some(&hidden_style),
            std::time::Duration::from_millis(100),
        )
        .await?;
    Ok(())
}

/// Focusable visually hidden content is shown while focus is within it and hidden again once focus
/// leaves ("unhides element if focused and isFocusable").
#[browser_test]
pub async fn unhides_focused_focusable(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    // The style that hides the plain element hides the focusable one too.
    let hidden_style = page
        .element(".test-vh-plain")
        .await?
        .attr("style")
        .await?
        .unwrap_or_default();
    let focusable = page.element(".test-vh-focusable").await?;
    page.element("#test-vh-focusable-a").await?.click().await?;
    focusable
        .attr_stays(
            "style",
            Some(hidden_style.as_str()),
            std::time::Duration::from_millis(100),
        )
        .await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&page.element("#test-vh-focusable-b").await?)
        .await?;
    focusable.wait_for_attr("style", None).await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&page.element("#test-vh-focusable-c").await?)
        .await?;
    focusable
        .wait_for_attr("style", Some(&hidden_style))
        .await?;
    Ok(())
}

/// Content made focusable at runtime (`is_focusable` turned on) is shown once focus moves into it.
#[browser_test]
pub async fn reactive_is_focusable(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let plain = page.element(".test-vh-plain").await?;
    page.element("#test-vh-toggle").await?.click().await?;
    page.element("#test-vh-plain-a").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&page.element("#test-vh-plain-b").await?)
        .await?;
    plain.wait_for_attr("style", None).await?;
    Ok(())
}
