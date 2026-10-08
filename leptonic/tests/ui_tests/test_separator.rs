// Upstream: react-aria-components/test/Separator.test.js @ 99e6102368
//! The `Separator` atom: an `<hr>` with the default class, ARIA props, and a `<div
//! role="separator">` while vertical (switching with the orientation).
use assertr::prelude::*;
use rootcause::Report;

use crate::pages::{Page, PageActions};

const PATH: &str = "/atoms/separator";

/// "should render a separator with default class".
pub async fn default_class(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let plain = page.element(".test-sep-plain").await?;
    assert_that!(plain.tag_name().await?).is_equal_to("hr");
    assert_that!(plain.attr("class").await?)
        .get_some()
        .is_equal_to("leptonic-Separator test-sep-plain");
    assert_that!(plain.attr("role").await?).is_none();
    Ok(())
}

/// "should support accessibility props".
pub async fn accessibility_props(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let labelled = page.element(".test-sep-labelled").await?;
    assert_that!(labelled.attr("aria-label").await?)
        .get_some()
        .is_equal_to("label");
    let by = page.element("#test-sep-by").await?;
    assert_that!(by.attr("aria-labelledby").await?)
        .get_some()
        .is_equal_to("test-sep-heading");
    Ok(())
}

/// Vertical: a `<div role="separator" aria-orientation="vertical">`; horizontal again: an `<hr>`.
pub async fn orientation(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let switching = page.element(".test-sep-switching").await?;
    assert_that!(switching.tag_name().await?).is_equal_to("div");
    assert_that!(switching.attr("role").await?)
        .get_some()
        .is_equal_to("separator");
    assert_that!(switching.attr("aria-orientation").await?)
        .get_some()
        .is_equal_to("vertical");
    page.element("#test-sep-toggle").await?.click().await?;
    page.element("hr.test-sep-switching").await?;
    let switched = page.element(".test-sep-switching").await?;
    assert_that!(switched.attr("aria-orientation").await?).is_none();
    page.element("#test-sep-toggle").await?.click().await?;
    page.element("div.test-sep-switching[role=separator][aria-orientation=vertical]")
        .await?;
    Ok(())
}
