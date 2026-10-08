// Upstream: react-aria/test/overlays/ariaHideOutside.test.js @ 99e6102368
//! `aria_hide_outside`: hides everything under the root except the targets (not traversing into
//! hidden containers), keeps author-set `aria-hidden`, hides the cells of a hidden row as well,
//! stacks hides restored in any order, hides a root that doesn't contain a target, shows
//! overlays registered from inside after its observer hid them, follows elements added while it
//! is active (outside, into hidden containers, inside a target, top-layer, reparented), and
//! restores rows that were reordered while hidden.
use assertr::prelude::*;
use rootcause::Report;

use crate::{
    pages::{ElementActions, Page, PageActions},
    polling::wait_for,
};

const PATH: &str = "/hooks/aria-hide-outside";

/// Waits until the elements `hidden` are hidden and the elements `visible` are not.
async fn expect_hidden(page: &Page<'_>, hidden: &[&str], visible: &[&str]) -> Result<(), Report> {
    for id in hidden {
        page.element(format!("#{id}"))
            .await?
            .wait_for_attr("aria-hidden", Some("true"))
            .await?;
    }
    for id in visible {
        page.element(format!("#{id}"))
            .await?
            .wait_for_attr("aria-hidden", None)
            .await?;
    }
    Ok(())
}

/// Negative check: the elements `visible` are not hidden and stay so (the hide's observer marks
/// elements added while it is active after the mutation).
async fn expect_stays_visible(page: &Page<'_>, visible: &[&str]) -> Result<(), Report> {
    for id in visible {
        page.element(format!("#{id}"))
            .await?
            .attr_stays("aria-hidden", None)
            .await?;
    }
    Ok(())
}

/// Click the fixture button `#id`.
async fn click(page: &Page<'_>, id: &str) -> Result<(), Report> {
    page.element(format!("#{id}")).await?.click().await?;
    Ok(())
}

/// "should hide everything except the provided element", "should not traverse into an already
/// hidden container", "should not overwrite an existing aria-hidden prop".
pub async fn hides_everything_but_the_target(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    click(page, "test-aho-hide-basic").await?;
    expect_hidden(
        page,
        &["test-aho-c1", "test-aho-wrap", "test-aho-author"],
        &["test-aho-c2", "test-aho-target", "test-aho-basic"],
    )
    .await?;
    click(page, "test-aho-revert-basic").await?;
    expect_hidden(
        page,
        &["test-aho-author"],
        &[
            "test-aho-c1",
            "test-aho-wrap",
            "test-aho-c2",
            "test-aho-target",
        ],
    )
    .await?;
    // Only the author's `aria-hidden` is left.
    assert_that!(page.count("#test-aho-basic [aria-hidden]").await?).is_equal_to(1);
    Ok(())
}

/// "should hide everything except the provided element [row]": the hidden row's cell is hidden as
/// well (VoiceOver on iOS), not the cell's content.
pub async fn hides_the_cells_of_a_hidden_row(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    click(page, "test-aho-hide-row").await?;
    expect_hidden(
        page,
        &["test-aho-row-1", "test-aho-cell-1"],
        &[
            "test-aho-span",
            "test-aho-row-2",
            "test-aho-cell-2",
            "test-aho-grid",
        ],
    )
    .await?;
    click(page, "test-aho-revert-row").await?;
    expect_hidden(page, &[], &["test-aho-row-1", "test-aho-cell-1"]).await?;
    Ok(())
}

const NESTED_CHECKBOXES: [&str; 2] = ["test-aho-n-c1", "test-aho-n-c2"];
const NESTED_RADIOS: [&str; 2] = ["test-aho-n-r1", "test-aho-n-r2"];

/// "work when called multiple times and restored out of order".
pub async fn nested_hides_restored_out_of_order(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let all = [NESTED_CHECKBOXES, NESTED_RADIOS].concat();
    click(page, "test-aho-hide-nested-1").await?;
    expect_hidden(page, &NESTED_CHECKBOXES, &NESTED_RADIOS).await?;
    click(page, "test-aho-hide-nested-2").await?;
    expect_hidden(page, &all, &["test-aho-n-button"]).await?;
    click(page, "test-aho-revert-nested-1").await?;
    expect_hidden(page, &all, &["test-aho-n-button"]).await?;
    click(page, "test-aho-revert-nested-2").await?;
    expect_hidden(page, &[], &all).await?;
    Ok(())
}

/// "work when called multiple times", restored in order.
pub async fn nested_hides_restored_in_order(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    click(page, "test-aho-hide-nested-1").await?;
    click(page, "test-aho-hide-nested-2").await?;
    click(page, "test-aho-revert-nested-2").await?;
    expect_hidden(page, &NESTED_CHECKBOXES, &NESTED_RADIOS).await?;
    click(page, "test-aho-revert-nested-1").await?;
    expect_hidden(page, &[], &[NESTED_CHECKBOXES, NESTED_RADIOS].concat()).await?;
    Ok(())
}

/// The root itself is hidden when the target is outside it.
pub async fn hides_a_root_without_the_target(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    click(page, "test-aho-hide-outer").await?;
    expect_hidden(page, &["test-aho-outer-root"], &[]).await?;
    click(page, "test-aho-revert-outer").await?;
    expect_hidden(page, &[], &["test-aho-outer-root"]).await?;
    Ok(())
}

/// Overlays opened from inside a hide, registered only after its observer hid them (Leptos
/// effects run after the observer's callback; agnite dev-ui's combo box in a modal): they become
/// visible again, the rest stays hidden.
pub async fn shows_overlays_registered_late(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    click(page, "test-aho-hide-late").await?;
    expect_hidden(page, &["test-aho-late-outside"], &["test-aho-late-dialog"]).await?;
    click(page, "test-aho-late-open-popover").await?;
    page.element("#test-aho-late-popover").await?;
    expect_hidden(
        page,
        &["test-aho-late-outside"],
        &["test-aho-late-popover-portal", "test-aho-late-popover"],
    )
    .await?;
    click(page, "test-aho-late-open-modal").await?;
    page.element("#test-aho-late-modal").await?;
    expect_hidden(
        page,
        &["test-aho-late-outside", "test-aho-late-dialog"],
        &["test-aho-late-modal-portal", "test-aho-late-modal"],
    )
    .await?;
    click(page, "test-aho-revert-late").await?;
    expect_hidden(
        page,
        &[],
        &[
            "test-aho-late-outside",
            "test-aho-late-dialog",
            "test-aho-late-popover-portal",
            "test-aho-late-modal-portal",
        ],
    )
    .await?;
    Ok(())
}

/// "should handle when a new element is added outside while active", "... added to an already
/// hidden container", "... added inside a target element", "... added along with a top layer
/// element", "... added and then reparented", "... reparented to a hidden container".
pub async fn mutations(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    click(page, "test-aho-hide-mo").await?;
    expect_hidden(page, &["test-aho-mo-container"], &["test-aho-mo-target"]).await?;

    click(page, "test-aho-mo-outside-button").await?;
    page.element("#test-aho-mo-outside").await?;
    expect_hidden(page, &["test-aho-mo-outside"], &["test-aho-mo-target"]).await?;

    // In a hidden container: the container stays hidden, the new element isn't marked itself.
    click(page, "test-aho-mo-in-hidden-button").await?;
    page.element("#test-aho-mo-in-hidden").await?;
    expect_hidden(page, &["test-aho-mo-container"], &[]).await?;
    expect_stays_visible(page, &["test-aho-mo-in-hidden"]).await?;

    click(page, "test-aho-mo-inside-button").await?;
    page.element("#test-aho-mo-inside").await?;
    expect_stays_visible(page, &["test-aho-mo-inside", "test-aho-mo-target"]).await?;

    click(page, "test-aho-mo-top-layer-button").await?;
    page.element("#test-aho-mo-top").await?;
    expect_hidden(
        page,
        &["test-aho-mo-top-checkbox"],
        &["test-aho-mo-top", "test-aho-mo-top-wrapper"],
    )
    .await?;

    // Reparented into the target: visible; into the hidden container: hidden with it.
    click(page, "test-aho-mo-reparent-target-button").await?;
    page.element("#test-aho-mo-li-target").await?;
    page.count_stays(
        "#test-aho-mo [aria-hidden] #test-aho-mo-li-target, \
             #test-aho-mo-li-target[aria-hidden], #test-aho-mo-li-target-list[aria-hidden]",
        0,
    )
    .await?;
    click(page, "test-aho-mo-reparent-hidden-button").await?;
    page.element("#test-aho-mo-li-hidden").await?;
    assert_that!(
        page.count("[aria-hidden=true] #test-aho-mo-li-hidden")
            .await?
    )
    .with_detail_message("the item reparented into the hidden container is hidden with it")
    .is_equal_to(1);

    // Reverted: everything added is visible.
    click(page, "test-aho-revert-mo").await?;
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
    assert_that!(page.count("#test-aho-mo [aria-hidden]").await?).is_equal_to(0);
    Ok(())
}

/// "should unhide after item reorder": rows moved while hidden are visible again after the
/// revert.
pub async fn unhide_after_reorder(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    click(page, "test-aho-hide-reorder").await?;
    page.element("#test-aho-reorder > [role=presentation][aria-hidden=true]")
        .await?;
    click(page, "test-aho-reorder-button").await?;
    wait_for("the row order")
        .observing(|| row_order(page))
        .to_be_equal_to(["b", "a", "c", "d"])
        .await?;
    click(page, "test-aho-reorder-button").await?;
    wait_for("the row order")
        .observing(|| row_order(page))
        .to_be_equal_to(["a", "b", "c", "d"])
        .await?;
    click(page, "test-aho-revert-reorder").await?;
    page.wait_for_count("#test-aho-reorder [aria-hidden]", 0)
        .await?;
    Ok(())
}

/// The keys of the reorder rows, in document order.
async fn row_order(page: &Page<'_>) -> Result<Vec<String>, Report> {
    let mut keys = Vec::new();
    for row in page.elements("#test-aho-reorder [role=row]").await? {
        let id = row.id().await?.unwrap_or_default();
        keys.push(id.trim_start_matches("test-aho-row-").to_owned());
    }
    Ok(keys)
}
