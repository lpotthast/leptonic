// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `ColorSwatch` (`/perf/atoms/color_swatch`, of an RGB color): what
//! each thing it offers costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/atoms/color_swatch.json`): mounting and unmounting it, and changes of its
//! reactive props (the color, its name, the label).
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/color_swatch";

/// Registers the performance tests of `ColorSwatch`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(ChangeTheColor))
        .with(fixture_test(ChangeTheName))
        .with(fixture_test(ChangeTheLabel))
}
const SWATCH: &str = ".leptonic-ColorSwatch";

/// Mounting the swatch and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(SWATCH, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_attr(SWATCH, "role", Some("img")).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(SWATCH, 0).await?;
    Ok(())
}

/// Changing the color (its background and name) and back.
#[browser_test]
pub async fn change_the_color(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let swatch = page.element(SWATCH).await?;
    let red = swatch.attr("aria-label").await?;
    let red_background = swatch.style_property("background-color").await?;
    page.measure("set blue", page.control("set-blue")).await?;
    assert_that!(swatch.attr("aria-label").await?).is_not_equal_to(red.clone());
    assert_that!(swatch.style_property("background-color").await?)
        .is_not_equal_to(red_background.clone());
    page.measure("set red", page.control("set-red")).await?;
    assert_that!(swatch.attr("aria-label").await?).is_equal_to(red);
    assert_that!(swatch.style_property("background-color").await?).is_equal_to(red_background);
    Ok(())
}

/// Naming the color and removing the name again.
#[browser_test]
pub async fn change_the_name(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let swatch = page.element(SWATCH).await?;
    let red = swatch.attr("aria-label").await?;
    page.measure("set", page.control("set-name")).await?;
    swatch
        .wait_for_attr("aria-label", Some("Fire truck"))
        .await?;
    page.measure("unset", page.control("unset-name")).await?;
    assert_that!(swatch.attr("aria-label").await?).is_equal_to(red);
    Ok(())
}

/// Labeling the swatch (after its color's name) and removing the label again.
#[browser_test]
pub async fn change_the_label(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let swatch = page.element(SWATCH).await?;
    let red = swatch.attr("aria-label").await?.unwrap_or_default();
    page.measure("set", page.control("set-label")).await?;
    swatch
        .wait_for_attr("aria-label", Some(&format!("{red}, Background")))
        .await?;
    page.measure("unset", page.control("unset-label")).await?;
    swatch.wait_for_attr("aria-label", Some(&red)).await?;
    Ok(())
}
