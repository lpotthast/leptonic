// No upstream: `VirtualList` is a leptonic addition (react-aria has no virtualized plain list).
//! A virtualized log (`VirtualList`, no react-aria equivalent): it renders a slice of its 2,000
//! lines and follows its end while lines are appended; scrolling away stops following, toggling
//! it on scrolls back to the end; rows holding the text selection stay rendered; the rows' text
//! is selectable; rows of plain text are measured again when they resize; lines wider than the
//! list scroll horizontally.
//!
//! A view rebuilt in place (the same type, a new owner: tachys reuses the DOM) whose elements carry
//! a hook's props: the old owner's handlers must be gone (agnite dev-ui, 2026-10-07: a click
//! panicked on disposed signals).
//!
//! Attributes spread onto a component (`<Comp {..props.into_attrs()} />`, with
//! `--cfg=erase_components`: a `Vec<AnyAttribute>`) whose view is rebuilt in place: Leptos'
//! `Vec<AnyAttribute>::rebuild` removes the old attributes by key but not the old event
//! listeners, so the disposed owner's handlers still run (and panic). A bug in tachys (0.2.19 and
//! its main branch, 2026-03-14), not in leptonic.
//!
//! Following turns off when the user scrolls away from the end: measured row sizes must survive
//! that (the layout options change, but only `anchor_to_end`), else the content jumps and every row is
//! measured again. A behavior guard: the original bug didn't reproduce here (its timing); the
//! regression test is the unit test `anchoring_changes_keep_measured_sizes`.
use std::time::Duration;

use assertr::{
    matchers::{all_of, eq, gt, lt},
    prelude::*,
};
use browser_test::{browser_test, thirtyfour::prelude::*};
use rootcause::{Report, prelude::ResultExt};

use crate::{
    fixtures::virtual_list::{LogView, VirtualListActions, WideView},
    pages::{ElementActions, Page},
};

const PATH: &str = "/atoms/virtual-list";

/// Wait until the log is scrolled to its end (within 2px).
async fn wait_for_the_end(page: &Page<'_>) -> Result<(), Report> {
    let log = VirtualListActions::new(page).log().await?;
    assert_that!(|| async { Ok::<_, Report>(log.scroll_extent().await?.distance_to_end()) })
        .eventually_ok()
        .matches(all_of(matchers![gt(-2.0), lt(2.0)]))
        .await;
    Ok(())
}

/// Wait until the line with the text `line` is rendered.
async fn wait_for_rendered(page: &Page<'_>, line: &str) -> Result<(), Report> {
    let list = VirtualListActions::new(page);
    assert_that!(|| async { Ok::<_, Report>(list.view().await?.texts()) })
        .eventually_ok()
        .satisfies(|lines| {
            lines.contains(line.to_owned());
        })
        .await;
    Ok(())
}

/// Scrolls the log to its top, which stops following.
async fn scroll_to_the_top(page: &Page<'_>) -> Result<(), Report> {
    VirtualListActions::new(page)
        .log()
        .await?
        .scroll_to_top(0.0)
        .await?;
    page.element("#test-vl-follow")
        .await?
        .wait_for_inner_text("not following")
        .await
}

/// The rendered rows' tops are in visual order in the DOM.
async fn assert_rows_in_visual_order(page: &Page<'_>) -> Result<(), Report> {
    let tops = VirtualListActions::new(page).view().await?.tops();
    let mut sorted = tops.clone();
    sorted.sort_by(f64::total_cmp);
    assert_that!(tops).is_equal_to(sorted);
    Ok(())
}

/// The log starts at its end with the last line rendered and only a slice of its lines in the
/// DOM, and the lines' text is selectable.
#[browser_test]
pub async fn follows_its_end(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    wait_for_the_end(page).await?;
    wait_for_rendered(page, "Line 1999").await?;
    assert_that!(page.count("#test-vl-log .line").await?).is_less_than(100);
    let line = page.first_element("#test-vl-log .line").await?;
    assert_that!(line.css_value("user-select").await?).is_not_equal_to("none");
    Ok(())
}

/// Appended lines come into view, in visual order in the DOM.
#[browser_test]
pub async fn appended_lines_come_into_view(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-vl-append").await?.click().await?;
    wait_for_rendered(page, "Line 2049").await?;
    wait_for_the_end(page).await?;
    assert_rows_in_visual_order(page).await?;
    Ok(())
}

/// Lines appended while the page (not the list) scrolls come into view, and the list keeps
/// following once the page's scrolling ended.
#[browser_test]
pub async fn page_scroll_keeps_following(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    // The append button out of view: the click scrolls the page to it first, so the lines are
    // appended while the page scrolls.
    VirtualListActions::new(page).scroll_the_page_down().await?;
    page.element("#test-vl-append").await?.click().await?;
    wait_for_rendered(page, "Line 2049").await?;
    wait_for_the_end(page).await?;
    let follow = page.element("#test-vl-follow").await?;
    // Past the end of the page's scrolling (300ms after its last scroll event).
    page.settle().await?;
    assert_that!(|| follow.inner_text())
        .consistently_ok()
        .for_at_least(Duration::from_millis(500))
        .matches(eq("following"))
        .await;
    Ok(())
}

/// After scroll jumps, the rendered rows cover the view in visual order in the DOM, so a selection
/// from one row to another holds exactly the rows between.
#[browser_test]
pub async fn scroll_jumps_render_rows_in_order(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let list = VirtualListActions::new(page);
    let log = list.log().await?;
    for top in [10_000.0, 5_000.0, 20_000.0] {
        log.scroll_to_top(top).await?;
        assert_that!(|| list.view())
            .with_subject_name(format!("the rendered rows, scrolled to {top}"))
            .eventually_ok()
            .satisfies(|view| {
                view.derive_owned(LogView::covers_the_view).is_true();
            })
            .await;
        assert_rows_in_visual_order(page)
            .await
            .context_with(|| format!("scrolled to {top}"))?;
    }
    let (text, [first, last]) = list.select_three_short_lines().await?;
    assert_that!(text.as_str())
        .starts_with(&first)
        .ends_with(&last);
    // Only the rows between (no long line: the rows were picked from short ones in a run).
    assert_that!(text.matches("Line ").count()).is_less_than(5);
    Ok(())
}

/// Scrolling away stops following; appended lines don't move the view.
#[browser_test]
pub async fn scrolling_away_stops_following(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    scroll_to_the_top(page).await?;
    let log = VirtualListActions::new(page).log().await?;
    let height = log.scroll_extent().await?.scroll_height;
    page.element("#test-vl-append").await?.click().await?;
    // The appended lines are laid out: the content grew.
    assert_that!(|| async { Ok::<_, Report>(log.scroll_extent().await?.scroll_height) })
        .eventually_ok()
        .satisfies(|grown| {
            grown.is_greater_than(height);
        })
        .await;
    page.settle().await?;
    assert_that!(|| async { Ok::<_, Report>(log.scroll_extent().await?.top) })
        .consistently_ok()
        .for_at_least(Duration::from_millis(100))
        .matches(eq(0.0))
        .await;
    page.element("#test-vl-follow")
        .await?
        .inner_text_stays("not following", Duration::from_millis(100))
        .await?;
    Ok(())
}

/// A row holding the text selection stays rendered while scrolled away; without the selection,
/// it goes.
#[browser_test]
pub async fn selected_row_stays_rendered(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let list = VirtualListActions::new(page);
    scroll_to_the_top(page).await?;
    wait_for_rendered(page, "Line 1").await?;
    // Returns once the list's `selectionchange` listener ran: a user doesn't select and scroll
    // within one task.
    list.select_line("Line 1").await?;
    // Scrolling back to the end follows again; the selected row stays rendered.
    let log = list.log().await?;
    let follow = page.element("#test-vl-follow").await?;
    assert_that!(|| async {
        let end = log.scroll_extent().await?.scroll_height;
        log.scroll_to_top(end).await?;
        follow.inner_text().await
    })
    .eventually_ok()
    .matches(eq("following"))
    .await;
    wait_for_rendered(page, "Line 1999").await?;
    assert_that!(list.view().await?.texts()).contains("Line 1".to_owned());
    assert_that!(list.selection_text().await?).is_equal_to("Line 1");

    list.clear_selection().await?;
    assert_that!(|| async { Ok::<_, Report>(list.view().await?.texts()) })
        .eventually_ok()
        .satisfies(|lines| {
            lines.does_not_contain("Line 1".to_owned());
        })
        .await;
    Ok(())
}

/// Turning following on scrolls to the end.
#[browser_test]
pub async fn turning_following_on_scrolls_to_the_end(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    scroll_to_the_top(page).await?;
    let follow = page.element("#test-vl-follow").await?;
    follow.click().await?;
    follow.wait_for_inner_text("following").await?;
    wait_for_rendered(page, "Line 1999").await?;
    wait_for_the_end(page).await?;
    Ok(())
}

/// After a view with a hook's props is rebuilt in place, clicks and Tab reach only the new owner's
/// handlers, never the disposed owner's (which would panic and fail the test).
#[browser_test]
pub async fn rebuilt_views_drop_the_old_handlers(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.first_element("#test-vl-rebuilt-list .line").await?;
    for round in 1..=3 {
        page.element("#test-vl-source").await?.click().await?;
        page.element("#test-vl-rebuilt-plain")
            .await?
            .wait_for_inner_text(&format!("Plain {round}"))
            .await?;
        page.element("#test-vl-rebuilt-list").await?.click().await?;
        page.element("#test-vl-rebuilt-plain")
            .await?
            .click()
            .await?;
        page.send_keys(Key::Tab).await?;
    }
    Ok(())
}

/// After a component with spread attributes is rebuilt in place, a click on it reaches only live
/// handlers. Known issue: tachys keeps the old event listeners.
#[browser_test]
pub async fn rebuilt_component_spread_drops_the_old_handlers(
    page: &Page<'_>,
) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-vl-source").await?.click().await?;
    page.element("#test-vl-rebuilt-plain")
        .await?
        .wait_for_inner_text("Plain 1")
        .await?;
    page.element("#test-vl-rebuilt-wrapper")
        .await?
        .click()
        .await?;
    Ok(())
}

/// When a user scroll away from the end turns following off, the content doesn't move and no
/// measured row height goes back to the estimate.
#[browser_test]
pub async fn follow_toggle_keeps_measured_sizes(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let list = VirtualListActions::new(page);
    wait_for_the_end(page).await?;
    wait_for_rendered(page, "Line 1999").await?;
    list.record_reestimated_rows().await?;

    // The user scroll: one step every 50ms (paced in the page, so the scroll doesn't end in
    // between), 30 up, 12 down. Then the first row in view and its offset, before the scroll ends
    // (300ms later) and turns following off.
    let steps: Vec<f64> = std::iter::repeat_n(-150.0, 30)
        .chain(std::iter::repeat_n(150.0, 12))
        .collect();
    list.log()
        .await?
        .scroll_by_steps(&steps, Duration::from_millis(50))
        .await?;
    // The page handled the last step (well before the scroll ends).
    page.settle().await?;
    let before = list.view().await?.anchor();
    page.element("#test-vl-follow")
        .await?
        .wait_for_inner_text("not following")
        .await?;
    // Settle (a re-layout runs in effects and frames), then check: the content didn't move.
    page.settle().await?;
    assert_that!(|| async { Ok::<_, Report>(list.view().await?.anchor()) })
        .consistently_ok()
        .for_at_least(Duration::from_millis(500))
        .matches(eq(before))
        .await;
    assert_that!(list.reestimated_rows().await?).is_empty();
    Ok(())
}

/// Rows of plain text (no element of their own) are measured again when their content resizes:
/// a bigger font makes them taller (react-aria observes only an item's element children).
#[browser_test]
pub async fn text_rows_are_measured_again_when_they_resize(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    // The first row's wrapper (the list > its content box > the rows).
    let row = || page.first_element("#test-vl-text > [role=presentation] > [role=presentation]");
    assert_that!(|| async { row().await?.client_rect().await })
        .eventually_ok()
        .satisfies(|rect| {
            rect.derive(|rect| &rect.height).is_less_than(30.0);
        })
        .await;
    page.element("#test-vl-text-bigger").await?.click().await?;
    assert_that!(|| async { row().await?.client_rect().await })
        .eventually_ok()
        .satisfies(|rect| {
            rect.derive(|rect| &rect.height).is_greater_than(40.0);
        })
        .await;
    Ok(())
}

/// Lines wider than the list (that don't wrap) widen its content: it scrolls horizontally to the
/// end of the widest one, and the rows stay rendered there (react-aria's `ListLayout` keeps the
/// content as wide as the view and hides the overflow; a plain scrolling element shows it).
#[browser_test]
pub async fn wide_lines_scroll_horizontally(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let list = VirtualListActions::new(page);
    assert_that!(|| list.wide_view())
        .eventually_ok()
        .satisfies(|view| {
            view.derive(|view| &view.overflow_x).is_equal_to("auto");
            view.derive_owned(|view| view.scroll_width - view.widest.ceil())
                .matches(all_of(matchers![gt(-1.0), lt(1.0)]));
            view.derive_owned(|view| view.scroll_width - view.client_width)
                .is_greater_than(200.0);
        })
        .await;

    list.scroll_wide_to_the_right().await?;
    assert_that!(|| list.wide_view())
        .eventually_ok()
        .satisfies(|view| {
            view.derive_owned(|view| view.scroll_width - view.client_width - view.left)
                .matches(all_of(matchers![gt(-1.0), lt(1.0)]));
        })
        .await;
    page.settle().await?;
    assert_that!(|| async { Ok::<_, Report>(list.wide_view().await?.rendered) })
        .consistently_ok()
        .for_at_least(Duration::from_millis(500))
        .matches(gt(5))
        .await;
    Ok(())
}

/// Wrapping the lines (a change of their style, not of the items) makes them taller and no wider
/// than the list: they are measured again and the list no longer scrolls horizontally; unwrapping
/// them makes it scroll again.
#[browser_test]
pub async fn wrapping_lines_ends_horizontal_scrolling(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let list = VirtualListActions::new(page);
    let is_wide = |view: &WideView| view.scroll_width - view.client_width > 200.0;
    assert_that!(|| list.wide_view())
        .eventually_ok()
        .satisfies(|view| {
            view.derive_owned(is_wide).is_true();
        })
        .await;
    let toggle = page.element("#test-vl-wide-wrap").await?;

    toggle.click().await?;
    toggle.wait_for_inner_text("wrapping").await?;
    assert_that!(|| list.wide_view())
        .eventually_ok()
        .satisfies(|view| {
            view.derive_owned(|view| view.scroll_width - view.client_width)
                .matches(all_of(matchers![gt(-1.0), lt(1.0)]));
            view.derive(|view| &view.overflow_x).is_equal_to("hidden");
            view.derive(|view| &view.tallest).is_greater_than(60.0);
        })
        .await;

    toggle.click().await?;
    toggle.wait_for_inner_text("not wrapping").await?;
    assert_that!(|| list.wide_view())
        .eventually_ok()
        .satisfies(|view| {
            view.derive_owned(is_wide).is_true();
            view.derive(|view| &view.tallest).is_less_than(30.0);
        })
        .await;
    Ok(())
}
