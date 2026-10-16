// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `ResizableTableContainer` (`/perf/atoms/resizable_table_container`,
//! a table with resizable columns in it): what each thing it offers costs, measured
//! (`Page::measure`) and checked against its budget
//! (`tests/budgets/atoms/resizable_table_container.json`): mounting and unmounting it, its width
//! changing (the columns follow), and resizing a column by dragging its resizer and with the
//! keyboard, in tables of 10 rows and of 1,000.
use assertr::{matchers::eq, prelude::*};
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/resizable_table_container";

/// Registers the performance tests of `ResizableTableContainer`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmountLarge))
        .with(fixture_test(WidthChangeSmall))
        .with(fixture_test(WidthChangeLarge))
        .with(fixture_test(DragSmall))
        .with(fixture_test(DragLarge))
        .with(fixture_test(KeyboardResizeSmall))
        .with(fixture_test(KeyboardResizeLarge))
}

const CONTAINER: &str = ".leptonic-ResizableTableContainer";
const ROWS: &str = "[role=grid] > tbody > tr";
/// The column header "Name".
const NAME_HEADER: &str = "[role=grid] > thead th:nth-child(1)";
/// The resizer of the column "Name".
const NAME_RESIZER: &str = "[role=grid] > thead th:nth-child(1) [data-column-resizer]";

/// Opens the fixture with its 10 rows.
async fn open(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.wait_for_count(ROWS, 10).await?;
    Ok(())
}

/// Opens the fixture and mounts the table with 1,000 rows (unmeasured).
async fn open_large(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.control("unmount").await?;
    page.wait_for_count(CONTAINER, 0).await?;
    page.control("large").await?;
    page.control("mount").await?;
    page.wait_for_count(ROWS, 1_000).await?;
    Ok(())
}

/// The width of the column "Name" (its header's `style.width`, in pixels).
async fn name_width(page: &Page<'_>) -> Result<f64, Report> {
    let header = page.element(NAME_HEADER).await?;
    page.low_level()
        .eval(
            "return parseFloat(arguments[0].style.width);",
            vec![header.to_json()?],
        )
        .await
}

/// Waits until the column "Name" is `expected` pixels wide.
async fn expect_name_width(page: &Page<'_>, expected: f64) -> Result<(), Report> {
    assert_that!(|| name_width(page))
        .with_subject_name("the width of the column Name")
        .eventually_ok()
        .matches(eq(expected))
        .await;
    Ok(())
}

/// Mounting the container with its table of `rows` rows and unmounting it.
async fn mount_and_unmount_container(page: &Page<'_>, rows: usize) -> Result<(), Report> {
    page.control("unmount").await?;
    page.wait_for_count(CONTAINER, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_count(ROWS, rows).await?;
    expect_name_width(page, 300.0).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(CONTAINER, 0).await?;
    Ok(())
}

/// Mounting the container with 10 rows and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    mount_and_unmount_container(page, 10).await
}

/// Mounting the container with 1,000 rows and unmounting it.
#[browser_test]
pub async fn mount_and_unmount_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page).await?;
    mount_and_unmount_container(page, 1_000).await
}

/// The container narrowing from 900 to 600 pixels (each column from 300 to 200) and widening
/// again.
async fn width_change(page: &Page<'_>) -> Result<(), Report> {
    expect_name_width(page, 300.0).await?;
    page.measure("narrow", page.control("narrow")).await?;
    expect_name_width(page, 200.0).await?;
    page.measure("widen", page.control("wide")).await?;
    expect_name_width(page, 300.0).await?;
    Ok(())
}

/// The width of a container of 10 rows changing.
#[browser_test]
pub async fn width_change_small(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    width_change(page).await
}

/// The width of a container of 1,000 rows changing.
#[browser_test]
pub async fn width_change_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page).await?;
    width_change(page).await
}

/// Dragging the resizer of the column "Name" 50 pixels to the right: pressing it, moving, and
/// releasing it (the resize ends).
async fn drag(page: &Page<'_>) -> Result<(), Report> {
    expect_name_width(page, 300.0).await?;
    let resizer = page.element(NAME_RESIZER).await?;
    let held = page.measure("start", resizer.press_and_hold()).await?;
    page.measure("move", held.move_by(50, 0)).await?;
    expect_name_width(page, 350.0).await?;
    page.measure("end", held.release()).await?;
    assert_that!(page.control("resize-ends").await?).is_equal_to(1.0);
    Ok(())
}

/// Dragging a resizer in a table of 10 rows.
#[browser_test]
pub async fn drag_small(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    drag(page).await
}

/// Dragging a resizer in a table of 1,000 rows.
#[browser_test]
pub async fn drag_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page).await?;
    drag(page).await
}

/// Resizing the column "Name" with the keyboard: its focused resizer (Tab into the table,
/// ArrowUp onto the header), Enter starting the resize, ArrowRight widening the column by 10
/// pixels, Escape ending the resize.
async fn keyboard_resize(page: &Page<'_>) -> Result<(), Report> {
    expect_name_width(page, 300.0).await?;
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.send_keys(Key::Up).await?;
    let input = page
        .element(format!("{NAME_HEADER} input[type=range]"))
        .await?;
    page.wait_for_focus(&input).await?;
    let header = page.element(NAME_HEADER).await?;
    page.measure("start", page.send_keys(Key::Enter)).await?;
    header.wait_for_attr("data-resizing", Some("true")).await?;
    page.measure("widen", page.send_keys(Key::Right)).await?;
    expect_name_width(page, 310.0).await?;
    page.measure("end", page.send_keys(Key::Escape)).await?;
    header.wait_for_attr("data-resizing", None).await?;
    assert_that!(page.control("resize-ends").await?).is_equal_to(1.0);
    Ok(())
}

/// Resizing with the keyboard in a table of 10 rows.
#[browser_test]
pub async fn keyboard_resize_small(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    keyboard_resize(page).await
}

/// Resizing with the keyboard in a table of 1,000 rows.
#[browser_test]
pub async fn keyboard_resize_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page).await?;
    keyboard_resize(page).await
}
