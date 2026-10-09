// Upstream: react-aria/test/table/tableResizingTests.tsx @ 99e6102368
// Upstream: @adobe/react-spectrum/test/table/TableSizing.test.tsx @ 99e6102368
//! Table column resizing (`ResizableTableContainer` and resizable columns of the `Table`
//! atoms): the initial column widths, resizing with the mouse (react-aria's
//! `tableResizingTests`, in 900 pixel wide tables) and with the keyboard (react-spectrum's
//! `TableSizing` keyboard tests, reaching the resizer the react-aria-components way: arrowing onto
//! the column header focuses its resizer).

use assertr::{matchers::eq, prelude::*};
use browser_test::{browser_test, thirtyfour::prelude::*};
use leptonic::AriaRole;
use rootcause::Report;

use crate::pages::{ElementActions, Page, css, role};

const PATH: &str = "/atoms/table-resizing";

/// Opens the fixture in a window with room for the 900 pixel wide tables and the drags across
/// them.
async fn open(page: &Page<'_>) -> Result<(), Report> {
    page.low_level()
        .driver()
        .set_window_rect(0, 0, 1400, 1000)
        .await?;
    page.goto_path(PATH).await?;
    Ok(())
}

/// The `style.width` of the column headers of the table `label`.
async fn widths(page: &Page<'_>, label: &str) -> Result<Vec<f64>, Report> {
    let header_row = page
        .element(format!("[role=grid][aria-label='{label}'] thead tr"))
        .await?;
    page.low_level()
        .eval(
            "return Array.from(arguments[0].children).map(cell => parseFloat(cell.style.width));",
            vec![header_row.to_json()?],
        )
        .await
}

/// Wait until the column headers of the table `label` have the widths `expected`.
async fn expect_widths(page: &Page<'_>, label: &str, expected: &[f64]) -> Result<(), Report> {
    assert_that!(|| widths(page, label))
        .with_subject_name(format!("the column widths of {label}"))
        .eventually_ok()
        .matches(eq(expected.to_vec()))
        .await;
    Ok(())
}

async fn column_header(page: &Page<'_>, label: &str, column: &str) -> Result<WebElement, Report> {
    page.element(format!("[role=grid][aria-label='{label}']"))
        .await?
        .element(role(AriaRole::Columnheader).text(column))
        .await
}

async fn resizer(page: &Page<'_>, label: &str, column: &str) -> Result<WebElement, Report> {
    column_header(page, label, column)
        .await?
        .element("[data-column-resizer]")
        .await
}

/// Drag the resizer of `column` by `delta` pixels with the mouse.
async fn resize_col(page: &Page<'_>, label: &str, column: &str, delta: i64) -> Result<(), Report> {
    let resizer = resizer(page, label, column).await?;
    resizer.scroll_into_view().await?;
    let chain = page
        .low_level()
        .driver()
        .action_chain()
        .move_to_element_center(&resizer)
        .click_and_hold();
    let chain = if delta == 0 {
        chain
    } else {
        chain.move_by_offset(delta, 0)
    };
    chain.release().perform().await?;
    Ok(())
}

/// Columns initially share the 900px table width by their fractions, also with minimum widths;
/// each resizer is a range input labelled "Resizer" and its column's name, with the width as value
/// text ("100 pixels"), the column's minimum and maximum and `data-resizable-direction="both"`.
#[browser_test]
pub async fn initial_widths(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    expect_widths(page, "Pokemon", &[100.0, 100.0, 100.0, 100.0, 500.0]).await?;
    expect_widths(page, "Ratios", &[113.0, 112.0, 113.0, 112.0, 450.0]).await?;
    expect_widths(page, "Minimums", &[113.0, 112.0, 113.0, 112.0, 450.0]).await?;

    // The resizers are range inputs labelled by themselves and their column's name (not the whole
    // header, which contains the resizer: browsers would name it "Resizer Name Resizer").
    let header = column_header(page, "Pokemon", "Name").await?;
    let input = header.element("input[type=range]").await?;
    let input_id = input.attr("id").await?.unwrap_or_default();
    let name_id = header
        .element(css("span[id]").text("Name"))
        .await?
        .attr("id")
        .await?
        .unwrap_or_default();
    assert_that!(input)
        .has_attribute("aria-labelledby")
        .await
        .is_equal_to(format!("{input_id} {name_id}"));
    assert_that!(input)
        .has_attribute("aria-label")
        .await
        .is_equal_to("Resizer");
    assert_that!(input)
        .accessible_name()
        .await
        .is_equal_to("Resizer Name");
    assert_that!(input)
        .has_attribute("aria-valuetext")
        .await
        .is_equal_to("100 pixels");
    assert_that!(input)
        .has_attribute("min")
        .await
        .is_equal_to("75");
    assert_that!(input)
        .has_attribute("max")
        .await
        .is_equal_to("9007199254740991");
    assert_that!(resizer(page, "Pokemon", "Name").await?)
        .has_attribute("data-resizable-direction")
        .await
        .is_equal_to("both");
    Ok(())
}

/// Dragging any column's resizer 50px left or right resizes that column (not below its 75px
/// minimum), redistributes the fr columns after it and reports the new sizes once to `on_resize`
/// and `on_resize_end` ("can resize $col to be $delta px different").
#[browser_test]
pub async fn resizing_each_column(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let cases: [(&str, i64, [f64; 5], &str); 10] = [
        (
            "Name",
            -50,
            [75.0, 103.0, 103.0, 103.0, 516.0],
            "75 1fr 1fr 1fr 5fr",
        ),
        (
            "Name",
            50,
            [150.0, 94.0, 94.0, 93.0, 469.0],
            "150 1fr 1fr 1fr 5fr",
        ),
        (
            "Type",
            -50,
            [100.0, 75.0, 104.0, 103.0, 518.0],
            "100 75 1fr 1fr 5fr",
        ),
        (
            "Type",
            50,
            [100.0, 150.0, 93.0, 93.0, 464.0],
            "100 150 1fr 1fr 5fr",
        ),
        (
            "Height",
            -50,
            [100.0, 100.0, 75.0, 104.0, 521.0],
            "100 100 75 1fr 5fr",
        ),
        (
            "Height",
            50,
            [100.0, 100.0, 150.0, 92.0, 458.0],
            "100 100 150 1fr 5fr",
        ),
        (
            "Weight",
            -50,
            [100.0, 100.0, 100.0, 75.0, 525.0],
            "100 100 100 75 5fr",
        ),
        (
            "Weight",
            50,
            [100.0, 100.0, 100.0, 150.0, 450.0],
            "100 100 100 150 5fr",
        ),
        (
            "Level",
            -50,
            [100.0, 100.0, 100.0, 100.0, 450.0],
            "100 100 100 100 450",
        ),
        (
            "Level",
            50,
            [100.0, 100.0, 100.0, 100.0, 550.0],
            "100 100 100 100 550",
        ),
    ];
    for (column, delta, expected, sizes) in cases {
        page.goto_path(PATH).await?;
        resize_col(page, "Pokemon", column, delta).await?;
        expect_widths(page, "Pokemon", &expected).await?;
        page.element("#test-pokemon-resize-count")
            .await?
            .wait_for_inner_text("1")
            .await?;
        page.element("#test-pokemon-resize")
            .await?
            .wait_for_inner_text(sizes)
            .await?;
        page.element("#test-pokemon-resize-end-count")
            .await?
            .wait_for_inner_text("1")
            .await?;
        page.element("#test-pokemon-resize-end")
            .await?
            .wait_for_inner_text(sizes)
            .await?;
        let grid = page.element("[role=grid][aria-label='Pokemon']").await?;
        let inputs = grid.elements("input[type=range]").await?;
        assert_that!(inputs.as_slice()).has_length(expected.len());
        for (input, width) in inputs.iter().zip(expected) {
            assert_that!(input)
                .has_attribute("value")
                .await
                .is_equal_to(width.to_string());
            assert_that!(input)
                .has_attribute("min")
                .await
                .is_equal_to("75");
        }
    }
    Ok(())
}

/// Dragging the columns, first to last, below their 100px minimum width stops each at 100px, and a
/// column at its minimum can only grow ("cannot resize to be less than a minWidth, from start to
/// end").
#[browser_test]
pub async fn cannot_resize_below_the_min_width(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let steps: [(&str, i64, [f64; 5], &str); 5] = [
        (
            "Name",
            -50,
            [100.0, 114.0, 115.0, 114.0, 457.0],
            "100 1fr 1fr 1fr 4fr",
        ),
        (
            "Type",
            -50,
            [100.0, 100.0, 117.0, 116.0, 467.0],
            "100 100 1fr 1fr 4fr",
        ),
        (
            "Height",
            -100,
            [100.0, 100.0, 100.0, 120.0, 480.0],
            "100 100 100 1fr 4fr",
        ),
        (
            "Weight",
            -100,
            [100.0, 100.0, 100.0, 100.0, 500.0],
            "100 100 100 100 4fr",
        ),
        (
            "Level",
            -500,
            [100.0, 100.0, 100.0, 100.0, 100.0],
            "100 100 100 100 100",
        ),
    ];
    for (count, (column, delta, expected, sizes)) in steps.into_iter().enumerate() {
        resize_col(page, "Minimums", column, delta).await?;
        expect_widths(page, "Minimums", &expected).await?;
        page.element("#test-minimums-resize-count")
            .await?
            .wait_for_inner_text(&(count + 1).to_string())
            .await?;
        page.element("#test-minimums-resize")
            .await?
            .wait_for_inner_text(sizes)
            .await?;
    }
    let grid = page.element("[role=grid][aria-label='Minimums']").await?;
    for input in grid.elements("input[type=range]").await? {
        assert_that!(input)
            .has_attribute("min")
            .await
            .is_equal_to("100");
    }
    // At its minimum width, a column can only grow.
    assert_that!(resizer(page, "Minimums", "Name").await?)
        .has_attribute("data-resizable-direction")
        .await
        .is_equal_to("left");
    Ok(())
}

/// Resizing the first column keeps the ratios of the fr columns after it, and resizing it back
/// restores the initial widths ("resizing the starter column will preserve fr column ratios to the
/// right").
#[browser_test]
pub async fn resizing_the_first_column_preserves_fr_ratios(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    resize_col(page, "Ratios", "Name", -50).await?;
    expect_widths(page, "Ratios", &[75.0, 118.0, 118.0, 118.0, 471.0]).await?;
    resize_col(page, "Ratios", "Name", 38).await?;
    expect_widths(page, "Ratios", &[113.0, 112.0, 113.0, 112.0, 450.0]).await?;
    Ok(())
}

/// Narrowing and widening the last column keeps the columns before it at their pixel widths
/// ("resizing the last column will lock columns to pixels to the left").
#[browser_test]
pub async fn resizing_the_last_column_locks_the_columns_before_it(
    page: &Page<'_>,
) -> Result<(), Report> {
    open(page).await?;
    resize_col(page, "Ratios", "Level", -50).await?;
    expect_widths(page, "Ratios", &[113.0, 112.0, 113.0, 112.0, 400.0]).await?;
    resize_col(page, "Ratios", "Level", 50).await?;
    expect_widths(page, "Ratios", &[113.0, 112.0, 113.0, 112.0, 450.0]).await?;
    Ok(())
}

/// Starting a drag reports the sizes with the columns up to the resized one in pixels and the
/// others as they were ("onResizeStart called with expected values").
#[browser_test]
pub async fn on_resize_start_reports_the_sizes(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    resize_col(page, "Ratios", "Height", -50).await?;
    page.element("#test-ratios-resize-start")
        .await?
        .wait_for_inner_text("113 112 113 1fr 4fr")
        .await?;
    Ok(())
}

/// A press on a resizer without moving calls `on_resize_end` but not `on_resize`, and keeps the
/// widths ("onResize end called with values even if no resizing took place").
#[browser_test]
pub async fn on_resize_end_without_moving(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    resize_col(page, "Minimums", "Type", 0).await?;
    page.element("#test-minimums-resize-end")
        .await?
        .wait_for_inner_text("113 112 1fr 1fr 4fr")
        .await?;
    page.element("#test-minimums-resize-count")
        .await?
        .wait_for_inner_text("0")
        .await?;
    expect_widths(page, "Minimums", &[113.0, 112.0, 113.0, 112.0, 450.0]).await?;
    Ok(())
}

/// Focus the resizer of the first column of the "Pokemon" table: Tab into the table (its first
/// row), then ArrowUp onto the column header, whose resizer takes the focus.
async fn focus_first_resizer(page: &Page<'_>) -> Result<WebElement, Report> {
    page.goto_path(PATH).await?;
    page.element("#test-resizing-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.send_keys(Key::Up).await?;
    let input = column_header(page, "Pokemon", "Name")
        .await?
        .element("input[type=range]")
        .await?;
    page.wait_for_focus(&input).await?;
    Ok(input)
}

async fn press(page: &Page<'_>, key: Key, times: usize) -> Result<(), Report> {
    for _ in 0..times {
        page.send_keys(key.clone()).await?;
    }
    Ok(())
}

/// Enter on a focused resizer starts resizing, in which Right and Up widen the column by 10px and
/// Left and Down narrow it; Escape ends it with `on_resize_end`, keeping focus on the resizer
/// ("arrow keys the resizer works - desktop").
#[browser_test]
pub async fn keyboard_resizing(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let input = focus_first_resizer(page).await?;
    // Arrow keys navigate until resizing starts.
    page.send_keys(Key::Enter).await?;
    page.element("[aria-label='Pokemon'] th[data-resizing]")
        .await?;

    press(page, Key::Right, 2).await?;
    input.wait_for_attr("value", Some("120")).await?;
    input
        .wait_for_attr("aria-valuetext", Some("120 pixels"))
        .await?;
    expect_widths(page, "Pokemon", &[120.0, 98.0, 97.0, 98.0, 487.0]).await?;
    press(page, Key::Left, 2).await?;
    expect_widths(page, "Pokemon", &[100.0, 100.0, 100.0, 100.0, 500.0]).await?;
    press(page, Key::Up, 2).await?;
    expect_widths(page, "Pokemon", &[120.0, 98.0, 97.0, 98.0, 487.0]).await?;
    press(page, Key::Down, 2).await?;
    expect_widths(page, "Pokemon", &[100.0, 100.0, 100.0, 100.0, 500.0]).await?;
    page.wait_for_focus(&input).await?;

    page.send_keys(Key::Escape).await?;
    page.element("#test-pokemon-resize-end-count")
        .await?
        .wait_for_inner_text("1")
        .await?;
    page.element("#test-pokemon-resize-end")
        .await?
        .wait_for_inner_text("100 1fr 1fr 1fr 5fr")
        .await?;
    page.wait_for_count("[aria-label='Pokemon'] th[data-resizing]", 0)
        .await?;
    // The resizer had the focus when resizing started: it keeps it.
    page.wait_for_focus(&input).await?;
    Ok(())
}

/// Enter or Tab ends keyboard resizing with `on_resize_end` and keeps focus on the resizer, and
/// moving focus away ends it too ("can exit resize via Enter", "can exit resize via Tab").
#[browser_test]
pub async fn exiting_keyboard_resizing(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    for exit in [Key::Enter, Key::Tab] {
        let input = focus_first_resizer(page).await?;
        page.send_keys(Key::Enter).await?;
        page.element("[aria-label='Pokemon'] th[data-resizing]")
            .await?;
        press(page, Key::Right, 1).await?;
        page.send_keys(exit).await?;
        page.element("#test-pokemon-resize-end-count")
            .await?
            .wait_for_inner_text("1")
            .await?;
        page.element("#test-pokemon-resize-end")
            .await?
            .wait_for_inner_text("110 1fr 1fr 1fr 5fr")
            .await?;
        page.wait_for_count("[aria-label='Pokemon'] th[data-resizing]", 0)
            .await?;
        page.wait_for_focus(&input).await?;
    }

    // Leaving the resizer ends resizing too.
    focus_first_resizer(page).await?;
    page.send_keys(Key::Enter).await?;
    page.element("[aria-label='Pokemon'] th[data-resizing]")
        .await?;
    page.element("#test-resizing-before").await?.click().await?;
    page.element("#test-pokemon-resize-end-count")
        .await?
        .wait_for_inner_text("1")
        .await?;
    page.wait_for_count("[aria-label='Pokemon'] th[data-resizing]", 0)
        .await?;
    Ok(())
}
