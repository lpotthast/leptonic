// Upstream: react-aria/test/utils/getScrollParents.test.ts @ 99e6102368
// Upstream: react-aria/test/utils/scrollIntoView.test.ts @ 99e6102368
//! The scroll utilities: the root counts among the scroll parents ("includes root as a scroll
//! parent for a node in the document", "includes a scrollable intermediate parent", "excludes
//! non-scrollable ancestors"); scrolling the root into view ignores its border and scrollbar
//! ("excludes root border from scroll port when scrolling to start/end").
use rootcause::Report;

use crate::pages::{ElementActions, Page, PageActions};

const PATH: &str = "/hooks/scroll";

/// The child of the scrolling box: the box, then the root; the child of the plain box: the root
/// only (neither the plain box nor the body).
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

/// The root's border (100px) isn't part of its scroll port: the target ends up at the viewport's
/// top edge, and at its bottom edge.
pub async fn scroll_into_view(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let offset = page.element("#test-scroll-offset").await?;
    page.element("#test-scroll-to-start").await?.click().await?;
    offset.wait_for_inner_text("top: 0").await?;
    page.element("#test-scroll-to-end").await?.click().await?;
    offset.wait_for_inner_text("bottom: 0").await?;
    Ok(())
}
