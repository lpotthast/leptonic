// Upstream: react-aria/test/focus/FocusScope.test.js @ 99e6102368
//! The focus manager of `FocusScope`: next, previous, first and last, with wrapping, tabbable
//! filtering, an accept filter, radio groups, hidden and inert elements, and from outside the
//! scope.
use browser_test::{browser_test, thirtyfour::Key};
use rootcause::Report;

use crate::{
    fixtures::focus_manager::FocusManagerActions,
    pages::{ElementActions, Page, PointerKind, SyntheticEvent},
};

const PATH: &str = "/hooks/focus-manager";

/// `focus_first`, `focus_next`, `focus_previous` and `focus_last` move focus to the matching item
/// of the scope ("should move focus forward", "should move focus backward").
#[browser_test]
pub async fn basic_navigation(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    for (control, expected) in [
        ("focus-first", "item-1"),
        ("focus-next", "item-2"),
        ("focus-next", "item-3"),
        ("focus-prev", "item-2"),
        ("focus-last", "item-3"),
    ] {
        FocusManagerActions::new(page)
            .item(control)
            .await?
            .click()
            .await?;
        page.wait_for_focus(&FocusManagerActions::new(page).item(expected).await?)
            .await?;
    }
    Ok(())
}

/// With `wrap`, `focus_next` on the last item moves focus to the first ("should move focus forward
/// and wrap around").
#[browser_test]
pub async fn wrap_next(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    FocusManagerActions::new(page)
        .item("item-3")
        .await?
        .click()
        .await?;
    page.wait_for_focus(&FocusManagerActions::new(page).item("item-3").await?)
        .await?;
    FocusManagerActions::new(page)
        .item("wrap-focus-next")
        .await?
        .click()
        .await?;
    page.wait_for_focus(&FocusManagerActions::new(page).item("item-1").await?)
        .await?;
    Ok(())
}

/// With `wrap`, `focus_previous` on the first item moves focus to the last ("should move focus
/// backward and wrap around").
#[browser_test]
pub async fn wrap_prev(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    FocusManagerActions::new(page)
        .item("item-1")
        .await?
        .click()
        .await?;
    page.wait_for_focus(&FocusManagerActions::new(page).item("item-1").await?)
        .await?;
    FocusManagerActions::new(page)
        .item("wrap-focus-prev")
        .await?
        .click()
        .await?;
    page.wait_for_focus(&FocusManagerActions::new(page).item("item-3").await?)
        .await?;
    Ok(())
}

/// Without `wrap`, `focus_next` on the last item keeps focus there ("should move focus forward").
#[browser_test]
pub async fn nowrap_boundary_next(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    FocusManagerActions::new(page)
        .item("item-3")
        .await?
        .click()
        .await?;
    page.wait_for_focus(&FocusManagerActions::new(page).item("item-3").await?)
        .await?;
    FocusManagerActions::new(page)
        .item("focus-next")
        .await?
        .click()
        .await?;
    page.focus_stays(
        &FocusManagerActions::new(page).item("item-3").await?,
        std::time::Duration::from_millis(100),
    )
    .await?;
    Ok(())
}

/// Without `wrap`, `focus_previous` on the first item keeps focus there ("should move focus
/// backward").
#[browser_test]
pub async fn nowrap_boundary_prev(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    FocusManagerActions::new(page)
        .item("item-1")
        .await?
        .click()
        .await?;
    page.wait_for_focus(&FocusManagerActions::new(page).item("item-1").await?)
        .await?;
    FocusManagerActions::new(page)
        .item("focus-prev")
        .await?
        .click()
        .await?;
    page.focus_stays(
        &FocusManagerActions::new(page).item("item-1").await?,
        std::time::Duration::from_millis(100),
    )
    .await?;
    Ok(())
}

/// With `tabbable`, `focus_next` and `focus_previous` skip the item with tab index -1 ("should move
/// focus forward but only to tabbable elements", "should move focus backward but only to tabbable
/// elements").
#[browser_test]
pub async fn tabbable_skip(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    FocusManagerActions::new(page)
        .item("tabbable-item-1")
        .await?
        .click()
        .await?;
    page.wait_for_focus(
        &FocusManagerActions::new(page)
            .item("tabbable-item-1")
            .await?,
    )
    .await?;
    FocusManagerActions::new(page)
        .item("tabbable-focus-next")
        .await?
        .click()
        .await?;
    page.wait_for_focus(
        &FocusManagerActions::new(page)
            .item("tabbable-item-3")
            .await?,
    )
    .await?;
    FocusManagerActions::new(page)
        .item("tabbable-focus-prev")
        .await?
        .click()
        .await?;
    page.wait_for_focus(
        &FocusManagerActions::new(page)
            .item("tabbable-item-1")
            .await?,
    )
    .await?;
    Ok(())
}

/// Without `tabbable`, `focus_next` moves focus to the item with tab index -1.
#[browser_test]
pub async fn nontabbable_include(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    FocusManagerActions::new(page)
        .item("tabbable-item-1")
        .await?
        .click()
        .await?;
    page.wait_for_focus(
        &FocusManagerActions::new(page)
            .item("tabbable-item-1")
            .await?,
    )
    .await?;
    FocusManagerActions::new(page)
        .item("nontabbable-focus-next")
        .await?
        .click()
        .await?;
    page.wait_for_focus(
        &FocusManagerActions::new(page)
            .item("tabbable-item-2")
            .await?,
    )
    .await?;
    Ok(())
}

/// `focus_next` and `focus_previous` skip the item an `accept` filter rejects ("should move focus
/// forward and allow users to skip certain elements", "should move focus backward and allow users
/// to skip certain elements").
#[browser_test]
pub async fn accept_filter(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    FocusManagerActions::new(page)
        .item("item-1")
        .await?
        .click()
        .await?;
    page.wait_for_focus(&FocusManagerActions::new(page).item("item-1").await?)
        .await?;
    FocusManagerActions::new(page)
        .item("accept-focus-next")
        .await?
        .click()
        .await?;
    page.wait_for_focus(&FocusManagerActions::new(page).item("item-3").await?)
        .await?;
    FocusManagerActions::new(page)
        .item("accept-focus-prev")
        .await?
        .click()
        .await?;
    page.wait_for_focus(&FocusManagerActions::new(page).item("item-1").await?)
        .await?;
    Ok(())
}

/// In a radio group with a checked radio, tabbable navigation in both directions stops only at the
/// checked radio ("skips radio buttons that are in the same group and are not the selectable one
/// forwards", "... backwards").
#[browser_test]
pub async fn radio_group_checked(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    FocusManagerActions::new(page)
        .item("radio-btn-before")
        .await?
        .click()
        .await?;
    page.wait_for_focus(
        &FocusManagerActions::new(page)
            .item("radio-btn-before")
            .await?,
    )
    .await?;
    // Skips the unchecked radios a and c.
    FocusManagerActions::new(page)
        .item("radio-focus-next")
        .await?
        .click()
        .await?;
    page.wait_for_focus(&FocusManagerActions::new(page).item("radio-b").await?)
        .await?;
    FocusManagerActions::new(page)
        .item("radio-focus-next")
        .await?
        .click()
        .await?;
    page.wait_for_focus(
        &FocusManagerActions::new(page)
            .item("radio-btn-after")
            .await?,
    )
    .await?;
    FocusManagerActions::new(page)
        .item("radio-focus-prev")
        .await?
        .click()
        .await?;
    page.wait_for_focus(&FocusManagerActions::new(page).item("radio-b").await?)
        .await?;
    Ok(())
}

/// In a radio group without a checked radio, tabbable navigation stops only at the first radio.
#[browser_test]
pub async fn radio_group_none_checked(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    FocusManagerActions::new(page)
        .item("radio-none-btn-before")
        .await?
        .click()
        .await?;
    page.wait_for_focus(
        &FocusManagerActions::new(page)
            .item("radio-none-btn-before")
            .await?,
    )
    .await?;
    FocusManagerActions::new(page)
        .item("radio-none-focus-next")
        .await?
        .click()
        .await?;
    page.wait_for_focus(&FocusManagerActions::new(page).item("radio-none-a").await?)
        .await?;
    // Skips radios b and c (same group).
    FocusManagerActions::new(page)
        .item("radio-none-focus-next")
        .await?
        .click()
        .await?;
    page.wait_for_focus(
        &FocusManagerActions::new(page)
            .item("radio-none-btn-after")
            .await?,
    )
    .await?;
    Ok(())
}

/// With `wrap` and `tabbable`, `focus_next` on the checked radio of a scope of one radio group
/// skips the group's other radios, wraps around and keeps focus on the checked radio.
#[browser_test]
pub async fn radio_group_wrap_next(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    FocusManagerActions::new(page)
        .item("radio-wrap-b")
        .await?
        .click()
        .await?;
    page.wait_for_focus(&FocusManagerActions::new(page).item("radio-wrap-b").await?)
        .await?;
    FocusManagerActions::new(page)
        .item("radio-wrap-focus-next")
        .await?
        .click()
        .await?;
    page.focus_stays(
        &FocusManagerActions::new(page).item("radio-wrap-b").await?,
        std::time::Duration::from_millis(100),
    )
    .await?;
    Ok(())
}

/// With `wrap` and `tabbable`, `focus_previous` on the checked radio of a scope of one radio group
/// skips the group's other radios, wraps around and keeps focus on the checked radio.
#[browser_test]
pub async fn radio_group_wrap_prev(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    FocusManagerActions::new(page)
        .item("radio-wrap-b")
        .await?
        .click()
        .await?;
    page.wait_for_focus(&FocusManagerActions::new(page).item("radio-wrap-b").await?)
        .await?;
    FocusManagerActions::new(page)
        .item("radio-wrap-focus-prev")
        .await?
        .click()
        .await?;
    page.focus_stays(
        &FocusManagerActions::new(page).item("radio-wrap-b").await?,
        std::time::Duration::from_millis(100),
    )
    .await?;
    Ok(())
}

/// `focus_next` and `focus_previous` skip hidden items (`display: none`, `hidden`, `visibility:
/// hidden`).
#[browser_test]
pub async fn hidden_elements_skipped(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    FocusManagerActions::new(page)
        .item("vis-item-1")
        .await?
        .click()
        .await?;
    page.wait_for_focus(&FocusManagerActions::new(page).item("vis-item-1").await?)
        .await?;
    FocusManagerActions::new(page)
        .item("vis-focus-next")
        .await?
        .click()
        .await?;
    page.wait_for_focus(&FocusManagerActions::new(page).item("vis-item-5").await?)
        .await?;
    FocusManagerActions::new(page)
        .item("vis-focus-prev")
        .await?
        .click()
        .await?;
    page.wait_for_focus(&FocusManagerActions::new(page).item("vis-item-1").await?)
        .await?;
    Ok(())
}

/// `focus_next` and `focus_previous` skip items in an inert subtree.
#[browser_test]
pub async fn inert_elements_skipped(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    FocusManagerActions::new(page)
        .item("inert-item-1")
        .await?
        .click()
        .await?;
    page.wait_for_focus(&FocusManagerActions::new(page).item("inert-item-1").await?)
        .await?;
    FocusManagerActions::new(page)
        .item("inert-focus-next")
        .await?
        .click()
        .await?;
    page.wait_for_focus(&FocusManagerActions::new(page).item("inert-item-3").await?)
        .await?;
    FocusManagerActions::new(page)
        .item("inert-focus-prev")
        .await?
        .click()
        .await?;
    page.wait_for_focus(&FocusManagerActions::new(page).item("inert-item-1").await?)
        .await?;
    Ok(())
}

/// `focus_next` while focus is outside the scope focuses the scope's first item.
#[browser_test]
pub async fn focus_next_from_outside_scope(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    FocusManagerActions::new(page)
        .item("outside-external")
        .await?
        .click()
        .await?;
    page.wait_for_focus(
        &FocusManagerActions::new(page)
            .item("outside-external")
            .await?,
    )
    .await?;
    FocusManagerActions::new(page)
        .item("focus-next")
        .await?
        .click()
        .await?;
    page.wait_for_focus(&FocusManagerActions::new(page).item("item-1").await?)
        .await?;
    Ok(())
}

/// `focus_previous` while focus is outside the scope focuses the scope's last item.
#[browser_test]
pub async fn focus_previous_from_outside_scope(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    FocusManagerActions::new(page)
        .item("outside-external")
        .await?
        .click()
        .await?;
    page.wait_for_focus(
        &FocusManagerActions::new(page)
            .item("outside-external")
            .await?,
    )
    .await?;
    FocusManagerActions::new(page)
        .item("focus-prev")
        .await?
        .click()
        .await?;
    page.wait_for_focus(&FocusManagerActions::new(page).item("item-3").await?)
        .await?;
    Ok(())
}

/// The focus manager a `FocusScope` provides (`use_focus_manager_context`) moves focus forward
/// and backward through the scope, wrapping around ("should move focus forward", "should move
/// focus forward and wrap around", "should move focus backward and wrap around").
#[browser_test]
pub async fn scope_manager(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let fm = FocusManagerActions::new(page);
    let first = fm.item("own-1").await?;
    first.focus().await?;
    page.wait_for_focus(&first).await?;
    for (key, expected) in [
        (Key::Right, "own-2"),
        (Key::Right, "own-3"),
        (Key::Right, "own-1"),
        (Key::Left, "own-3"),
        (Key::Left, "own-2"),
    ] {
        page.send_keys(key).await?;
        page.wait_for_focus(&fm.item(expected).await?).await?;
    }
    Ok(())
}

/// `focus_next` from a group element moves into the group: to its first tabbable item ("should
/// move focus forward but only to tabbable elements while accounting for container elements
/// within the scope").
#[browser_test]
pub async fn from_a_container(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let fm = FocusManagerActions::new(page);
    fm.item("group-2")
        .await?
        .dispatch(SyntheticEvent::pointer(PointerKind::Down))
        .await?;
    page.wait_for_focus(&fm.item("group-item-3").await?).await?;
    fm.item("group-1")
        .await?
        .dispatch(SyntheticEvent::pointer(PointerKind::Down))
        .await?;
    page.wait_for_focus(&fm.item("group-item-2").await?).await?;
    Ok(())
}
