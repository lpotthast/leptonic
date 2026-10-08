// Upstream: react-aria/test/overlays/ariaHideOutside.test.js @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{BrowserTest, async_trait, thirtyfour::WebDriver};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

/// `aria_hide_outside`: hides everything under the root except the targets (not traversing into
/// hidden containers), keeps author-set `aria-hidden`, hides the cells of a hidden row as well,
/// stacks hides restored in any order, hides a root that doesn't contain a target, shows
/// overlays registered from inside after its observer hid them, follows elements added while it
/// is active (outside, into hidden containers, inside a target, top-layer, reparented), and
/// restores rows that were reordered while hidden.
pub struct AriaHideOutsideTests {}

async fn expect_hidden(page: &Page<'_>, hidden: &[&str], visible: &[&str]) -> Result<(), Report> {
    for id in hidden {
        page.wait_for_attr(&page.element(id).await?, "aria-hidden", Some("true"))
            .await?;
    }
    for id in visible {
        page.wait_for_attr(&page.element(id).await?, "aria-hidden", None)
            .await?;
    }
    Ok(())
}

#[async_trait]
impl BrowserTest<str> for AriaHideOutsideTests {
    fn name(&self) -> Cow<'_, str> {
        "aria_hide_outside_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/hooks/aria-hide-outside").await?;

        // "should hide everything except the provided element", "should not traverse into an
        // already hidden container", "should not overwrite an existing aria-hidden prop".
        page.click_element_with_id("test-aho-hide-basic").await?;
        expect_hidden(
            &page,
            &["test-aho-c1", "test-aho-wrap", "test-aho-author"],
            &["test-aho-c2", "test-aho-target", "test-aho-basic"],
        )
        .await?;
        page.click_element_with_id("test-aho-revert-basic").await?;
        expect_hidden(
            &page,
            &["test-aho-author"],
            &[
                "test-aho-c1",
                "test-aho-wrap",
                "test-aho-c2",
                "test-aho-target",
            ],
        )
        .await?;

        // "should hide everything except the provided element [row]": the hidden row's cell is
        // hidden as well (VoiceOver on iOS), not the cell's content.
        page.click_element_with_id("test-aho-hide-row").await?;
        expect_hidden(
            &page,
            &["test-aho-row-1", "test-aho-cell-1"],
            &[
                "test-aho-span",
                "test-aho-row-2",
                "test-aho-cell-2",
                "test-aho-grid",
            ],
        )
        .await?;
        page.click_element_with_id("test-aho-revert-row").await?;
        expect_hidden(&page, &[], &["test-aho-row-1", "test-aho-cell-1"]).await?;

        // "work when called multiple times and restored out of order".
        let checkboxes = ["test-aho-n-c1", "test-aho-n-c2"];
        let radios = ["test-aho-n-r1", "test-aho-n-r2"];
        page.click_element_with_id("test-aho-hide-nested-1").await?;
        expect_hidden(&page, &checkboxes, &radios).await?;
        page.click_element_with_id("test-aho-hide-nested-2").await?;
        expect_hidden(
            &page,
            &[checkboxes, radios].concat(),
            &["test-aho-n-button"],
        )
        .await?;
        page.click_element_with_id("test-aho-revert-nested-1")
            .await?;
        expect_hidden(
            &page,
            &[checkboxes, radios].concat(),
            &["test-aho-n-button"],
        )
        .await?;
        page.click_element_with_id("test-aho-revert-nested-2")
            .await?;
        expect_hidden(&page, &[], &[checkboxes, radios].concat()).await?;

        // "work when called multiple times", restored in order.
        page.click_element_with_id("test-aho-hide-nested-1").await?;
        page.click_element_with_id("test-aho-hide-nested-2").await?;
        page.click_element_with_id("test-aho-revert-nested-2")
            .await?;
        expect_hidden(&page, &checkboxes, &radios).await?;
        page.click_element_with_id("test-aho-revert-nested-1")
            .await?;
        expect_hidden(&page, &[], &[checkboxes, radios].concat()).await?;

        // The root itself is hidden when the target is outside it.
        page.click_element_with_id("test-aho-hide-outer").await?;
        expect_hidden(&page, &["test-aho-outer-root"], &[]).await?;
        page.click_element_with_id("test-aho-revert-outer").await?;
        expect_hidden(&page, &[], &["test-aho-outer-root"]).await?;

        // Overlays opened from inside a hide, registered only after its observer hid them
        // (Leptos effects run after the observer's callback; agnite dev-ui's combo box in a
        // modal): they become visible again, the rest stays hidden.
        page.click_element_with_id("test-aho-hide-late").await?;
        expect_hidden(&page, &["test-aho-late-outside"], &["test-aho-late-dialog"]).await?;
        page.click_element_with_id("test-aho-late-open-popover")
            .await?;
        page.wait_for_selector("#test-aho-late-popover").await?;
        expect_hidden(
            &page,
            &["test-aho-late-outside"],
            &["test-aho-late-popover-portal", "test-aho-late-popover"],
        )
        .await?;
        page.click_element_with_id("test-aho-late-open-modal")
            .await?;
        page.wait_for_selector("#test-aho-late-modal").await?;
        expect_hidden(
            &page,
            &["test-aho-late-outside", "test-aho-late-dialog"],
            &["test-aho-late-modal-portal", "test-aho-late-modal"],
        )
        .await?;
        page.click_element_with_id("test-aho-revert-late").await?;
        expect_hidden(
            &page,
            &[],
            &[
                "test-aho-late-outside",
                "test-aho-late-dialog",
                "test-aho-late-popover-portal",
                "test-aho-late-modal-portal",
            ],
        )
        .await?;

        assert_that!(page.count_matching("#test-aho-basic [aria-hidden]").await?).is_equal_to(1);

        mutations(&page).await?;
        unhide_after_reorder(&page).await?;
        page.expect_no_page_errors().await
    }
}

/// "should handle when a new element is added outside while active", "... added to an already
/// hidden container", "... added inside a target element", "... added along with a top layer
/// element", "... added and then reparented", "... reparented to a hidden container".
async fn mutations(page: &Page<'_>) -> Result<(), Report> {
    page.click_element_with_id("test-aho-hide-mo").await?;
    expect_hidden(page, &["test-aho-mo-container"], &["test-aho-mo-target"]).await?;

    page.click_element_with_id("test-aho-mo-outside-button")
        .await?;
    page.wait_for_selector("#test-aho-mo-outside").await?;
    expect_hidden(page, &["test-aho-mo-outside"], &["test-aho-mo-target"]).await?;

    // In a hidden container: the container stays hidden, the new element isn't marked itself.
    page.click_element_with_id("test-aho-mo-in-hidden-button")
        .await?;
    page.wait_for_selector("#test-aho-mo-in-hidden").await?;
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    expect_hidden(page, &["test-aho-mo-container"], &["test-aho-mo-in-hidden"]).await?;

    page.click_element_with_id("test-aho-mo-inside-button")
        .await?;
    page.wait_for_selector("#test-aho-mo-inside").await?;
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    expect_hidden(page, &[], &["test-aho-mo-inside", "test-aho-mo-target"]).await?;

    page.click_element_with_id("test-aho-mo-top-layer-button")
        .await?;
    page.wait_for_selector("#test-aho-mo-top").await?;
    expect_hidden(
        page,
        &["test-aho-mo-top-checkbox"],
        &["test-aho-mo-top", "test-aho-mo-top-wrapper"],
    )
    .await?;

    // Reparented into the target: visible; into the hidden container: hidden with it.
    page.click_element_with_id("test-aho-mo-reparent-target-button")
        .await?;
    page.wait_for_selector("#test-aho-mo-li-target").await?;
    stays!(
        "the item reparented into the target is hidden",
        0,
        page.count_matching(
            "#test-aho-mo [aria-hidden] #test-aho-mo-li-target, #test-aho-mo-li-target[aria-hidden], #test-aho-mo-li-target-list[aria-hidden]"
        )
        .await?
    );
    page.click_element_with_id("test-aho-mo-reparent-hidden-button")
        .await?;
    page.wait_for_selector("#test-aho-mo-li-hidden").await?;
    assert_that!(
        page.count_matching("[aria-hidden=true] #test-aho-mo-li-hidden")
            .await?
    )
    .with_detail_message("the item reparented into the hidden container is hidden with it")
    .is_equal_to(1);

    // Reverted: everything added is visible.
    page.click_element_with_id("test-aho-revert-mo").await?;
    expect_hidden(
        page,
        &[],
        &[
            "test-aho-mo-container",
            "test-aho-mo-outside",
            "test-aho-mo-top-checkbox",
            "test-aho-mo-top-wrapper",
        ],
    )
    .await?;
    assert_that!(page.count_matching("#test-aho-mo [aria-hidden]").await?).is_equal_to(0);
    Ok(())
}

/// "should unhide after item reorder": rows moved while hidden are visible again after the
/// revert.
async fn unhide_after_reorder(page: &Page<'_>) -> Result<(), Report> {
    page.click_element_with_id("test-aho-hide-reorder").await?;
    page.wait_for_selector("#test-aho-reorder > [role=presentation][aria-hidden=true]")
        .await?;
    page.click_element_with_id("test-aho-reorder-button")
        .await?;
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    page.click_element_with_id("test-aho-reorder-button")
        .await?;
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    page.click_element_with_id("test-aho-revert-reorder")
        .await?;
    page.wait_for_no_selector("#test-aho-reorder [aria-hidden]")
        .await
}
