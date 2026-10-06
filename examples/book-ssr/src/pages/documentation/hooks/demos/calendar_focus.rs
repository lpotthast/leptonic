//! Moves the browser focus along with a calendar's focused date.
//!
//! A stopgap shared by the calendar and date picker demos: `use_calendar_cell` only moves its `tabindex` to the
//! focused date, it doesn't move DOM focus yet (tracked in leptonic's roadmap). Remove it once the cells focus
//! themselves.
use leptonic::utils::time::{Day, Week};
use leptos::{html, prelude::*};
use leptos_use::use_document;
use time::OffsetDateTime;
use wasm_bindgen::JsCast;

/// Focuses the tabbable day of `grid` whenever `focused_date` changes, unless focus is elsewhere on the page (e.g.
/// on the month buttons). With `auto_focus`, it also focuses the day once the grid is rendered.
pub fn follow_focused_date(
    focused_date: Signal<OffsetDateTime>,
    grid: NodeRef<html::Table>,
    auto_focus: bool,
) {
    Effect::watch(
        move || focused_date.get(),
        move |_, previous, _| {
            let is_first_run = previous.is_none();
            // Wait for the new month to render.
            request_animation_frame(move || {
                let Some(grid) = grid.get_untracked() else {
                    return;
                };
                // Focus is on the body when the previously focused day was removed by a month change.
                let focus_in_grid = use_document().active_element().is_none_or(|active| {
                    active.tag_name().eq_ignore_ascii_case("body") || grid.contains(Some(&active))
                });
                if (is_first_run || focus_in_grid)
                    && let Ok(Some(day)) = grid.query_selector("[tabindex='0']")
                    && let Ok(day) = day.dyn_into::<web_sys::HtmlElement>()
                {
                    let _ = day.focus();
                }
            });
        },
        auto_focus,
    );
}

/// Key for the weeks of a calendar grid. The weeks are recomputed whenever the focused date changes; keying them by
/// their first day keeps the DOM, and the focused button, while you move within a month. A date appears in three
/// months (e.g. as a day of the next month), hence `in_month`.
pub fn week_key(week: &Week) -> Option<(time::Date, u8)> {
    week.days.first().map(|day: &Day| (day.date_time.date(), day.in_month as u8))
}
