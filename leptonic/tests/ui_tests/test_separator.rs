// Upstream: react-aria-components/test/Separator.test.js @ 99e6102368
//! The `Separator` atom: an `<hr>` with the default class, ARIA props, and a `<div
//! role="separator">` while vertical (switching with the orientation).
use assertr::prelude::*;
use browser_test::browser_test;
use rootcause::Report;

use crate::pages::Page;

const PATH: &str = "/atoms/separator";

/// A horizontal separator is an `<hr>` without a role, with the default class before its own
/// ("should render a separator with default class").
#[browser_test]
pub async fn default_class(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let plain = page.element(".test-sep-plain").await?;
    assert_that!(plain.tag_name().await?).is_equal_to("hr");
    assert_that!(plain)
        .has_attribute("class")
        .await
        .is_equal_to("leptonic-Separator test-sep-plain");
    assert_that!(plain).attribute("role").await.is_none();
    Ok(())
}

/// A separator renders its `aria-label` and `aria-labelledby` props ("should support
/// accessibility props").
#[browser_test]
pub async fn accessibility_props(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let labelled = page.element(".test-sep-labelled").await?;
    assert_that!(labelled)
        .has_attribute("aria-label")
        .await
        .is_equal_to("label");
    let by = page.element("#test-sep-by").await?;
    assert_that!(by)
        .has_attribute("aria-labelledby")
        .await
        .is_equal_to("test-sep-heading");
    Ok(())
}

/// A vertical separator is a `<div role="separator" aria-orientation="vertical">`, and switching
/// its orientation turns it into an `<hr>` and back.
#[browser_test]
pub async fn orientation(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let switching = page.element(".test-sep-switching").await?;
    assert_that!(switching.tag_name().await?).is_equal_to("div");
    assert_that!(switching)
        .has_attribute("role")
        .await
        .is_equal_to("separator");
    assert_that!(switching)
        .has_attribute("aria-orientation")
        .await
        .is_equal_to("vertical");
    page.element("#test-sep-toggle").await?.click().await?;
    page.element("hr.test-sep-switching").await?;
    let switched = page.element(".test-sep-switching").await?;
    assert_that!(switched)
        .attribute("aria-orientation")
        .await
        .is_none();
    page.element("#test-sep-toggle").await?.click().await?;
    page.element("div.test-sep-switching[role=separator][aria-orientation=vertical]")
        .await?;
    Ok(())
}
