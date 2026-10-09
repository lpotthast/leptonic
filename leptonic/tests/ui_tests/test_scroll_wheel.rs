// No upstream: react-aria has no tests of `useScrollWheel`.
//! `use_scroll_wheel`: a wheel over the element reports its deltas in pixels, whatever the event's
//! delta mode, and neither scrolls nor reaches ancestors; Control+wheel (zoom) is left alone.
use std::time::Duration;

use assertr::prelude::*;
use browser_test::{browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::pages::{DeltaMode, ElementActions, Modifier, Page, SyntheticEvent};

const PATH: &str = "/hooks/scroll-wheel";

/// The log of scrolls (and wheel events reaching the wrapper).
async fn log(page: &Page<'_>) -> Result<WebElement, Report> {
    page.element("#test-scroll-wheel-log").await
}

/// A pixel wheel reports its deltas as they are, prevents the page's scroll and doesn't reach the
/// wrapper.
#[browser_test]
pub async fn pixel_deltas(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let target = page.element("#test-scroll-wheel-target").await?;
    let dispatched = target
        .dispatch(SyntheticEvent::wheel().delta_y(30.0))
        .await?;
    assert_that!(dispatched.default_prevented).is_true();
    let log = log(page).await?;
    log.wait_for_inner_text("0,30").await?;
    log.inner_text_stays("0,30", Duration::from_millis(100))
        .await?;
    Ok(())
}

/// A wheel counting lines (Firefox with a mouse wheel) reports 16 pixels per line.
#[browser_test]
pub async fn line_deltas_in_pixels(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-scroll-wheel-target")
        .await?
        .dispatch(
            SyntheticEvent::wheel()
                .delta_y(3.0)
                .delta_mode(DeltaMode::Line),
        )
        .await?;
    log(page).await?.wait_for_inner_text("0,48").await?;
    Ok(())
}

/// Control+wheel zooms: the hook leaves it alone (no scroll reported, not prevented, it bubbles).
#[browser_test]
pub async fn control_wheel_zooms(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let dispatched = page
        .element("#test-scroll-wheel-target")
        .await?
        .dispatch(
            SyntheticEvent::wheel()
                .delta_y(30.0)
                .modifiers(&[Modifier::Control]),
        )
        .await?;
    assert_that!(dispatched.default_prevented).is_false();
    let log = log(page).await?;
    log.wait_for_inner_text("wrapper").await?;
    log.inner_text_stays("wrapper", Duration::from_millis(100))
        .await?;
    Ok(())
}
