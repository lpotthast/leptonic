// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `Slider` (`/perf/atoms/slider`): what each thing it offers costs,
//! measured (`Page::measure`) and checked against its budget (`tests/budgets/atoms/slider.json`):
//! mounting and unmounting it with its parts (one thumb, and two), its values controlled by app
//! state, a click on its label (focusing the thumb), and changes of its reactive props (disabled,
//! orientation, range, step, format, required, invalid). Its parts are measured in
//! `slider_track.rs`, `slider_thumb.rs`, `slider_fill.rs`, `slider_output.rs`, ...
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/slider";

/// Registers the performance tests of `Slider`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmountRange))
        .with(fixture_test(ControlledValues))
        .with(fixture_test(LabelClick))
        .with(fixture_test(DisableAndEnable))
        .with(fixture_test(ChangeOrientation))
        .with(fixture_test(RangeChanges))
        .with(fixture_test(StepChanges))
        .with(fixture_test(FormatChanges))
        .with(fixture_test(RequiredAndOptional))
        .with(fixture_test(InvalidAndValid))
}
const SLIDER: &str = "#perf-slider";
const INPUT: &str = "#perf-slider input";
const OUTPUT: &str = "#perf-slider output";
const THUMB: &str = "#perf-slider .leptonic-SliderThumb";

/// Mounting and unmounting the slider with the control `mount` (`mount-range`) and `thumbs`
/// thumbs.
async fn mount_and_unmount_with(page: &Page<'_>, mount: &str, thumbs: usize) -> Result<(), Report> {
    page.measure("mount", page.control(mount)).await?;
    page.wait_for_count(INPUT, thumbs).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(SLIDER, 0).await?;
    Ok(())
}

/// Measures the control `on` (named after it), waits until the slider's input has the attribute
/// `attr` set to `value`, then measures the control `off` and waits until it is gone.
async fn input_attr_round_trip(
    page: &Page<'_>,
    attr: &str,
    value: &str,
    on: &str,
    off: &str,
) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure(on, page.control(on)).await?;
    page.wait_for_attr(INPUT, attr, Some(value)).await?;
    page.measure(off, page.control(off)).await?;
    page.wait_for_attr(INPUT, attr, None).await?;
    Ok(())
}

/// Mounting the slider with one thumb and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(SLIDER, 0).await?;
    mount_and_unmount_with(page, "mount", 1).await
}

/// Mounting the slider with two thumbs and unmounting it.
#[browser_test]
pub async fn mount_and_unmount_range(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(SLIDER, 0).await?;
    page.control("set-range-values").await?;
    mount_and_unmount_with(page, "mount-range", 2).await
}

/// The app setting the value to 60 and back to 30.
#[browser_test]
pub async fn controlled_values(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let output = page.element(OUTPUT).await?;
    page.measure("set", page.control("set-value")).await?;
    output.wait_for_inner_text("60").await?;
    page.wait_for_attr(INPUT, "value", Some("60")).await?;
    page.measure("reset", page.control("reset-value")).await?;
    output.wait_for_inner_text("30").await?;
    Ok(())
}

/// A click on the label, focusing the thumb.
#[browser_test]
pub async fn label_click(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let label = page.element("#perf-slider .leptonic-Label").await?;
    page.measure("click", label.click()).await?;
    let input = page.element(INPUT).await?;
    page.wait_for_focus(&input).await?;
    Ok(())
}

/// Disabling the slider and enabling it again.
#[browser_test]
pub async fn disable_and_enable(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("disable", page.control("disable")).await?;
    page.wait_for_attr(SLIDER, "data-disabled", Some("true"))
        .await?;
    page.wait_for_attr(THUMB, "data-disabled", Some("true"))
        .await?;
    page.measure("enable", page.control("enable")).await?;
    page.wait_for_attr(SLIDER, "data-disabled", None).await?;
    page.wait_for_attr(THUMB, "data-disabled", None).await?;
    Ok(())
}

/// Laying the slider out vertically and horizontally again.
#[browser_test]
pub async fn change_orientation(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("make vertical", page.control("make-vertical"))
        .await?;
    page.wait_for_attr(SLIDER, "data-orientation", Some("vertical"))
        .await?;
    page.measure("make horizontal", page.control("make-horizontal"))
        .await?;
    page.wait_for_attr(SLIDER, "data-orientation", Some("horizontal"))
        .await?;
    Ok(())
}

/// The maximum growing from 100 to 200 and shrinking back.
#[browser_test]
pub async fn range_changes(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("widen", page.control("widen")).await?;
    page.wait_for_attr(INPUT, "max", Some("200")).await?;
    page.measure("narrow", page.control("narrow")).await?;
    page.wait_for_attr(INPUT, "max", Some("100")).await?;
    Ok(())
}

/// The step growing from 1 to 5 and shrinking back.
#[browser_test]
pub async fn step_changes(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("coarsen", page.control("coarsen")).await?;
    page.wait_for_attr(INPUT, "step", Some("5")).await?;
    page.measure("refine", page.control("refine")).await?;
    page.wait_for_attr(INPUT, "step", Some("1")).await?;
    Ok(())
}

/// The format changing to one fraction digit and back.
#[browser_test]
pub async fn format_changes(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let output = page.element(OUTPUT).await?;
    page.measure("format fraction", page.control("format-fraction"))
        .await?;
    output.wait_for_inner_text("30.0").await?;
    page.wait_for_attr(INPUT, "aria-valuetext", Some("30.0"))
        .await?;
    page.measure("format integer", page.control("format-integer"))
        .await?;
    output.wait_for_inner_text("30").await?;
    Ok(())
}

/// Requiring a value and making it optional again.
#[browser_test]
pub async fn required_and_optional(page: &Page<'_>) -> Result<(), Report> {
    input_attr_round_trip(page, "aria-required", "true", "require", "make-optional").await
}

/// Marking the slider invalid and valid again.
#[browser_test]
pub async fn invalid_and_valid(page: &Page<'_>) -> Result<(), Report> {
    input_attr_round_trip(page, "aria-invalid", "true", "invalidate", "validate").await
}
