// Upstream: react-aria/test/overlays/ariaHideOutside.test.js @ 99e6102368
//! `aria_hide_outside`: hides everything under the root except the targets (not traversing into
//! hidden containers), keeps author-set `aria-hidden`, hides the cells of a hidden row as well,
//! stacks hides restored in any order, hides a root that doesn't contain a target, shows
//! overlays registered from inside after its observer hid them, follows elements added while it
//! is active (outside, into hidden containers, inside a target, top-layer, reparented), and
//! restores rows that were reordered while hidden; in inert mode, SVG elements get `aria-hidden`.
use assertr::{matchers::eq, prelude::*};
use browser_test::browser_test;
use rootcause::Report;

use crate::pages::{ElementActions, Page};

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
            .attr_stays("aria-hidden", None, std::time::Duration::from_millis(100))
            .await?;
    }
    Ok(())
}

/// Click the fixture button `#id`.
async fn click(page: &Page<'_>, id: &str) -> Result<(), Report> {
    page.element(format!("#{id}")).await?.click().await?;
    Ok(())
}

/// Everything under the root but the target is hidden and shown again on revert, without
/// traversing into a hidden container or removing an author-set `aria-hidden` ("should hide
/// everything except the provided element [button]", "should not traverse into an already hidden
/// container", "should not overwrite an existing aria-hidden prop").
#[browser_test]
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

/// Hiding outside a row hides the other rows and their cells (for VoiceOver on iOS) but not the
/// cells' content ("should hide everything except the provided element [row]").
#[browser_test]
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

/// Of two stacked hides, reverting the first keeps everything hidden and reverting the second shows
/// it all again ("work when called multiple times and restored out of order").
#[browser_test]
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

/// Of two stacked hides, reverting the second restores what the first hid and reverting the first
/// shows it all again ("work when called multiple times").
#[browser_test]
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

/// A root that doesn't contain the target is hidden itself, and shown again on revert.
#[browser_test]
pub async fn hides_a_root_without_the_target(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    click(page, "test-aho-hide-outer").await?;
    expect_hidden(page, &["test-aho-outer-root"], &[]).await?;
    click(page, "test-aho-revert-outer").await?;
    expect_hidden(page, &[], &["test-aho-outer-root"]).await?;
    Ok(())
}

/// A popover and a modal opened from inside the hidden-outside dialog become visible though they
/// register only after the hide's observer hid them (Leptos effects run later); the rest stays
/// hidden.
#[browser_test]
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

/// Starts the hide of the mutation cases: `#test-aho-mo-target` visible, its sibling container
/// hidden.
async fn hide_mutations(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    click(page, "test-aho-hide-mo").await?;
    expect_hidden(page, &["test-aho-mo-container"], &["test-aho-mo-target"]).await
}

/// An element added outside the target while the hide is active is hidden, and shown again on
/// revert ("should handle when a new element is added outside while active").
#[browser_test]
pub async fn added_outside(page: &Page<'_>) -> Result<(), Report> {
    hide_mutations(page).await?;
    click(page, "test-aho-mo-outside-button").await?;
    page.element("#test-aho-mo-outside").await?;
    expect_hidden(page, &["test-aho-mo-outside"], &["test-aho-mo-target"]).await?;
    click(page, "test-aho-revert-mo").await?;
    expect_hidden(page, &[], &["test-aho-mo-outside", "test-aho-mo-container"]).await?;
    Ok(())
}

/// An element added to a hidden container isn't marked itself (the container hides it), and the
/// container is shown on revert ("should handle when a new element is added to an already hidden
/// container").
#[browser_test]
pub async fn added_to_a_hidden_container(page: &Page<'_>) -> Result<(), Report> {
    hide_mutations(page).await?;
    click(page, "test-aho-mo-in-hidden-button").await?;
    page.element("#test-aho-mo-in-hidden").await?;
    expect_hidden(page, &["test-aho-mo-container"], &[]).await?;
    expect_stays_visible(page, &["test-aho-mo-in-hidden"]).await?;
    click(page, "test-aho-revert-mo").await?;
    expect_hidden(
        page,
        &[],
        &["test-aho-mo-container", "test-aho-mo-in-hidden"],
    )
    .await?;
    Ok(())
}

/// An element added inside the target stays visible ("should handle when a new element is added
/// inside a target element").
#[browser_test]
pub async fn added_inside_the_target(page: &Page<'_>) -> Result<(), Report> {
    hide_mutations(page).await?;
    click(page, "test-aho-mo-inside-button").await?;
    page.element("#test-aho-mo-inside").await?;
    expect_stays_visible(page, &["test-aho-mo-inside", "test-aho-mo-target"]).await?;
    Ok(())
}

/// A top-layer element (`data-leptonic-top-layer`, by presence) added with a checkbox stays
/// visible, the checkbox is hidden and shown on revert ("should handle when a new element is added
/// along with a top layer element", "should recognize dynamically added top layer element with
/// data-react-aria-top-layer="" (attribute presence)").
#[browser_test]
pub async fn added_with_a_top_layer_element(page: &Page<'_>) -> Result<(), Report> {
    hide_mutations(page).await?;
    click(page, "test-aho-mo-top-layer-button").await?;
    page.element("#test-aho-mo-top").await?;
    expect_hidden(
        page,
        &["test-aho-mo-top-checkbox"],
        &["test-aho-mo-top", "test-aho-mo-top-wrapper"],
    )
    .await?;
    click(page, "test-aho-revert-mo").await?;
    expect_hidden(
        page,
        &[],
        &["test-aho-mo-top-checkbox", "test-aho-mo-top-wrapper"],
    )
    .await?;
    Ok(())
}

/// An element added, then reparented into the target stays visible ("should handle when a new
/// element is added and then reparented").
#[browser_test]
pub async fn reparented_into_the_target(page: &Page<'_>) -> Result<(), Report> {
    hide_mutations(page).await?;
    click(page, "test-aho-mo-reparent-target-button").await?;
    page.element("#test-aho-mo-li-target").await?;
    page.count_stays(
        "#test-aho-mo [aria-hidden] #test-aho-mo-li-target, \
             #test-aho-mo-li-target[aria-hidden], #test-aho-mo-li-target-list[aria-hidden]",
        0,
        std::time::Duration::from_millis(100),
    )
    .await?;
    Ok(())
}

/// An element added, then reparented into a hidden container is hidden with it ("should handle
/// when a new element is added and then reparented to a hidden container").
#[browser_test]
pub async fn reparented_into_a_hidden_container(page: &Page<'_>) -> Result<(), Report> {
    hide_mutations(page).await?;
    click(page, "test-aho-mo-reparent-hidden-button").await?;
    page.element("#test-aho-mo-li-hidden").await?;
    assert_that!(
        page.count("[aria-hidden=true] #test-aho-mo-li-hidden")
            .await?
    )
    .with_detail_message("the item reparented into the hidden container is hidden with it")
    .is_equal_to(1);
    click(page, "test-aho-revert-mo").await?;
    page.wait_for_count("#test-aho-mo [aria-hidden]", 0).await?;
    Ok(())
}

/// Rows reordered while hidden are visible again after the revert ("should unhide after item
/// reorder").
#[browser_test]
pub async fn unhide_after_reorder(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    click(page, "test-aho-hide-reorder").await?;
    page.first_element("#test-aho-reorder > [role=presentation][aria-hidden=true]")
        .await?;
    click(page, "test-aho-reorder-button").await?;
    assert_that!(|| row_order(page))
        .eventually_ok()
        .matches(eq(["b", "a", "c", "d"]))
        .await;
    click(page, "test-aho-reorder-button").await?;
    assert_that!(|| row_order(page))
        .eventually_ok()
        .matches(eq(["a", "b", "c", "d"]))
        .await;
    click(page, "test-aho-revert-reorder").await?;
    page.wait_for_count("#test-aho-reorder [aria-hidden]", 0)
        .await?;
    Ok(())
}

/// In inert mode, HTML elements outside the target become `inert` and an SVG element, which
/// has no `inert`, `aria-hidden`. HTML also gets `aria-hidden` so that a referenced label
/// keeps naming the visible target in Chromium. Reverting restores the author's attributes.
#[browser_test]
pub async fn inert_mode_hides_svg_with_aria_hidden(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let span = page.element("#test-aho-inert-span").await?;
    let svg = page.element("#test-aho-inert-svg").await?;
    click(page, "test-aho-hide-inert").await?;
    span.wait_for_attr("inert", Some("")).await?;
    svg.wait_for_attr("aria-hidden", Some("true")).await?;
    assert_that!(svg).attribute("inert").await.is_none();
    span.wait_for_attr("aria-hidden", Some("true")).await?;
    assert_that!(page.element("#test-aho-inert-target").await?)
        .accessible_name()
        .await
        .is_equal_to("Outside");

    click(page, "test-aho-revert-inert").await?;
    span.wait_for_attr("inert", None).await?;
    span.wait_for_attr("aria-hidden", Some("false")).await?;
    svg.wait_for_attr("aria-hidden", None).await?;
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
