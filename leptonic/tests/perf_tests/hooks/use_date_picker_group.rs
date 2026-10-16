// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_date_picker_group` (`/perf/hooks/use_date_picker_group`, a
//! group of three plain focusable parts): what each thing it offers costs, measured
//! (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_date_picker_group.json`): creating and disposing of it, the arrow
//! keys moving between its parts, Alt+ArrowDown opening its overlay, and a press beside its parts
//! (focusing the last).
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_date_picker_group";

/// Registers the performance tests of `use_date_picker_group`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MoveBetweenParts))
        .with(fixture_test(OpenWithTheKeyboard))
        .with(fixture_test(PressBesideTheParts))
}
const GROUP: &str = "#perf-group";

/// Focuses the first part with the keyboard (Tab from the plain button before the group).
async fn focus_the_month(page: &Page<'_>) -> Result<(), Report> {
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&page.element("#perf-month").await?)
        .await
}

/// Creating the group and disposing of it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(GROUP, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(GROUP).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(GROUP, 0).await?;
    Ok(())
}

/// Arrow Right and Left move the focus between the parts.
#[browser_test]
pub async fn move_between_parts(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    focus_the_month(page).await?;
    page.measure("right", page.send_keys(Key::Right)).await?;
    page.wait_for_focus(&page.element("#perf-day").await?)
        .await?;
    page.measure("left", page.send_keys(Key::Left)).await?;
    page.wait_for_focus(&page.element("#perf-month").await?)
        .await?;
    Ok(())
}

/// Alt+ArrowDown opens the overlay.
#[browser_test]
pub async fn open_with_the_keyboard(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    focus_the_month(page).await?;
    page.measure("alt+down", page.send_keys(Key::Alt + Key::Down))
        .await?;
    page.element("#perf-open")
        .await?
        .wait_for_inner_text("true")
        .await?;
    Ok(())
}

/// A press on the group beside its parts focuses the last part.
#[browser_test]
pub async fn press_beside_the_parts(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let rect = page.element(GROUP).await?.client_rect().await?;
    #[allow(clippy::cast_possible_truncation)]
    let (x, y) = (
        (rect.right - 5.0) as i64,
        (rect.top + rect.height / 2.0) as i64,
    );
    page.measure("press", page.click_at(x, y)).await?;
    page.wait_for_focus(&page.element("#perf-year").await?)
        .await?;
    Ok(())
}
