// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `ColorSliderOutput` (`/perf/atoms/color_slider_output`, in a
//! `ColorSlider` of the red channel): what each thing it offers costs, measured
//! (`Page::measure`) and checked against its budget
//! (`tests/budgets/atoms/color_slider_output.json`): mounting and unmounting it, the slider's
//! value changing (its text), and the slider disabling it.
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/color_slider_output";

/// Registers the performance tests of `ColorSliderOutput`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(ChangeTheValue))
        .with(fixture_test(DisabledSlider))
}
const OUTPUT: &str = ".leptonic-ColorSliderOutput";

/// Mounting the output and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(OUTPUT, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(OUTPUT)
        .await?
        .wait_for_inner_text("128")
        .await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(OUTPUT, 0).await?;
    Ok(())
}

/// The slider's value changing (the output's text) and back.
#[browser_test]
pub async fn change_the_value(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let output = page.element(OUTPUT).await?;
    page.measure("set blue", page.control("set-blue")).await?;
    output.wait_for_inner_text("0").await?;
    page.measure("set purple", page.control("set-purple"))
        .await?;
    output.wait_for_inner_text("128").await?;
    Ok(())
}

/// The slider disabling the output and enabling it again.
#[browser_test]
pub async fn disabled_slider(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let output = page.element(OUTPUT).await?;
    page.measure("disable", page.control("disable")).await?;
    output.wait_for_attr("data-disabled", Some("true")).await?;
    page.measure("enable", page.control("enable")).await?;
    output.wait_for_attr("data-disabled", None).await?;
    Ok(())
}
