// Upstream: react-aria/test/utils/getScrollParents.test.ts @ 99e6102368
// Upstream: react-aria/test/utils/scrollIntoView.test.ts @ 99e6102368
//! The scroll utilities: the root counts among the scroll parents ("includes root as a scroll
//! parent for a node in the document", "includes a scrollable intermediate parent", "excludes
//! non-scrollable ancestors"); scrolling the root into view ignores its border and scrollbar
//! ("excludes root border from scroll port when scrolling to start/end").
use browser_test::browser_test;
use rootcause::Report;

use crate::pages::{ElementActions, Page};

const PATH: &str = "/hooks/scroll";

/// A child of a scrolling box has the box and then the root as scroll parents, a child of a plain
/// box only the root ("includes a scrollable intermediate parent", "excludes non-scrollable
/// ancestors", "includes root as a scroll parent for a node in the document").
#[browser_test]
pub async fn scroll_parents(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-scroll-list-parents")
        .await?
        .click()
        .await?;
    page.element("#test-scroll-parents")
        .await?
        .wait_for_inner_text("test-scroll-box,HTML | HTML")
        .await?;
    Ok(())
}

/// Scrolling a target to the start or end of a root with a 100px border puts it exactly at the
/// viewport's top or bottom edge ("excludes root border from scroll port when scrolling to
/// start", "excludes root border from scroll port when scrolling to end").
#[browser_test]
pub async fn scroll_into_view(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let offset = page.element("#test-scroll-offset").await?;
    page.element("#test-scroll-to-start").await?.click().await?;
    offset.wait_for_inner_text("top: 0").await?;
    page.element("#test-scroll-to-end").await?.click().await?;
    offset.wait_for_inner_text("bottom: 0").await?;
    Ok(())
}
