// Upstream: react-aria-components/test/GridList.test.js @ 99e6102368
//! Cases of react-aria-components' `GridList.test.js` on `/atoms/grid-list-cases`, one section
//! each: auto focus (with `selectionBehavior="replace"`), focus ring and press state, Escape with
//! `escapeKeyBehavior="none"`, the empty state, a grid layout, a horizontal grid layout, sections labelled by a header
//! and/or an `aria-label`, `shouldSelectOnPressUp`, and clicking a text input in a row.
use std::time::Duration;

use assertr::prelude::*;
use browser_test::{browser_test, thirtyfour::prelude::*};
use leptonic::AriaRole;
use rootcause::{Report, prelude::ResultExt};

use crate::pages::{ElementActions, Page, css, role};

const PATH: &str = "/atoms/grid-list-cases";

/// The grid list labelled `label`.
async fn list(page: &Page<'_>, label: &str) -> Result<WebElement, Report> {
    page.element(format!("[role=grid][aria-label='{label}']"))
        .await
}

/// The row `text` of the grid list labelled `label`.
async fn row(page: &Page<'_>, label: &str, text: &str) -> Result<WebElement, Report> {
    list(page, label)
        .await?
        .element(role(AriaRole::Row).text(text))
        .await
}

/// The `aria-selected` values of the rows of the grid list labelled `label`.
async fn selected_states(page: &Page<'_>, label: &str) -> Result<Vec<Option<String>>, Report> {
    let mut states = Vec::new();
    for row in list(page, label).await?.elements("[role=row]").await? {
        states.push(row.attr("aria-selected").await?);
    }
    Ok(states)
}

/// With `auto_focus`, the grid list itself takes focus when it mounts and nothing is selected
/// ("should support autoFocus").
#[browser_test]
pub async fn auto_focus(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["autofocus"]).await?;
    page.wait_for_focus(&list(page, "Autofocus").await?).await
}

/// In single replace selection, auto focusing the first row selects it ("selects the autofocused
/// row when selectOnFocus with autoFocus=first").
#[browser_test]
pub async fn auto_focus_first_selects_it(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["autofocus-first"]).await?;
    let cat = row(page, "Autofocus first", "Cat").await?;
    page.wait_for_focus(&cat).await?;
    cat.wait_for_attr("aria-selected", Some("true")).await?;
    Ok(())
}

/// In single replace selection, auto focusing the last row selects it ("selects the autofocused
/// row when selectOnFocus with autoFocus=last").
#[browser_test]
pub async fn auto_focus_last_selects_it(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["autofocus-last"]).await?;
    let kangaroo = row(page, "Autofocus last", "Kangaroo").await?;
    page.wait_for_focus(&kangaroo).await?;
    kangaroo
        .wait_for_attr("aria-selected", Some("true"))
        .await?;
    Ok(())
}

/// Without selection, the auto focused row is focused but not selectable ("does not select the
/// autofocused row when selectionMode="none"").
#[browser_test]
pub async fn auto_focus_without_selection(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["autofocus-none"]).await?;
    let cat = row(page, "Autofocus without selection", "Cat").await?;
    page.wait_for_focus(&cat).await?;
    cat.attr_stays("aria-selected", None, Duration::from_millis(100))
        .await?;
    Ok(())
}

/// Auto focusing a row keeps an "all" selection ("does not change an existing "all" selection when
/// autofocusing").
#[browser_test]
pub async fn auto_focus_keeps_an_all_selection(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["autofocus-all"]).await?;
    let label = "Autofocus all selected";
    page.wait_for_focus(&row(page, label, "Cat").await?).await?;
    page.settle().await?;
    let all = Some("true".to_owned());
    assert_that!(selected_states(page, label).await?).is_equal_to(vec![
        all.clone(),
        all.clone(),
        all,
    ]);
    page.element("#test-glc-autofocus-all-selection")
        .await?
        .inner_text_stays("all", Duration::from_millis(100))
        .await?;
    Ok(())
}

/// Tabbing to a row shows its focus ring (`data-focus-visible`, `data-focus-visible-within`),
/// tabbing away removes it ("should support focus ring").
#[browser_test]
pub async fn focus_ring(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["interactive"]).await?;
    let cat = row(page, "Interactive", "Cat").await?;
    assert_that!(cat)
        .attribute("data-focus-visible")
        .await
        .is_none();
    page.element("#test-glc-interactive-before")
        .await?
        .click()
        .await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&cat).await?;
    cat.wait_for_attr("data-focus-visible", Some("true"))
        .await?;
    cat.wait_for_attr("data-focus-visible-within", Some("true"))
        .await?;
    assert_that!(cat)
        .has_attribute("data-selection-mode")
        .await
        .is_equal_to("multiple");
    page.send_keys(Key::Tab).await?;
    cat.wait_for_attr("data-focus-visible", None).await?;
    cat.wait_for_attr("data-focus-visible-within", None).await?;
    Ok(())
}

/// A selectable row is `data-pressed` while the pointer holds it ("should support press state").
#[browser_test]
pub async fn press_state(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["interactive"]).await?;
    let cat = row(page, "Interactive", "Cat").await?;
    assert_that!(cat).attribute("data-pressed").await.is_none();
    let held = cat.press_and_hold().await?;
    cat.wait_for_attr("data-pressed", Some("true")).await?;
    held.release().await?;
    cat.wait_for_attr("data-pressed", None).await?;
    Ok(())
}

/// A row without selection or action never shows the pressed state ("should not show press state
/// when not interactive").
#[browser_test]
pub async fn no_press_state_when_not_interactive(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["static"]).await?;
    let cat = row(page, "Static", "Cat").await?;
    let held = cat.press_and_hold().await?;
    cat.attr_stays("data-pressed", None, Duration::from_millis(100))
        .await?;
    held.release().await?;
    cat.attr_stays("data-pressed", None, Duration::from_millis(100))
        .await?;
    Ok(())
}

/// With `EscapeKeyBehavior::None`, Escape keeps the selection ("should prevent Esc from clearing
/// selection if escapeKeyBehavior is "none"").
#[browser_test]
pub async fn escape_keeps_the_selection(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["escape"]).await?;
    let selection = page.element("#test-glc-escape-selection").await?;
    row(page, "Escape", "Cat").await?.click().await?;
    selection.wait_for_inner_text("Cat").await?;
    row(page, "Escape", "Dog").await?.click().await?;
    selection.wait_for_inner_text("Cat,Dog").await?;
    page.send_keys(Key::Escape).await?;
    selection
        .inner_text_stays("Cat,Dog", Duration::from_millis(100))
        .await?;
    Ok(())
}

/// An empty grid list is marked `data-empty` and stays reachable by Tab ("should support empty
/// state").
#[browser_test]
pub async fn empty_state(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["empty"]).await?;
    let empty = list(page, "Empty").await?;
    assert_that!(empty)
        .has_attribute("data-empty")
        .await
        .is_equal_to("true");
    page.element("#test-glc-empty-before")
        .await?
        .click()
        .await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&empty).await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&page.element("#test-glc-empty-after").await?)
        .await
}

/// In a grid layout, ArrowRight and ArrowLeft move between the rows laid out side by side, and Tab
/// moves into a row's button and then out of the grid list ("should support grid layout").
#[browser_test]
pub async fn grid_layout(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["grid-layout"]).await?;
    let cat = row(page, "Grid layout", "Cat").await?;
    let dog = row(page, "Grid layout", "Dog ⓘ").await?;
    let kangaroo = row(page, "Grid layout", "Kangaroo").await?;
    page.element("#test-glc-grid-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&cat).await?;
    page.send_keys(Key::Right).await?;
    page.wait_for_focus(&dog).await?;
    page.send_keys(Key::Right).await?;
    page.wait_for_focus(&kangaroo).await?;
    page.send_keys(Key::Left).await?;
    page.wait_for_focus(&dog).await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&dog.element(css("button[aria-label='Info']")).await?)
        .await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&page.element("#test-glc-grid-after").await?)
        .await
}

/// Press the keys of `steps` one after another, each expected to focus its row of the grid list
/// labelled `label`.
async fn arrows_focus(page: &Page<'_>, label: &str, steps: &[(Key, &str)]) -> Result<(), Report> {
    for (key, text) in steps {
        page.send_keys(key.clone()).await?;
        page.wait_for_focus(&row(page, label, text).await?)
            .await
            .context_with(|| format!("after pressing {key:?}, expected {text}"))?;
    }
    Ok(())
}

/// In a horizontal grid layout (two rows flowing into columns), ArrowUp/ArrowDown move to the
/// previous/next row in collection order, also across columns, and ArrowLeft/ArrowRight to the row
/// in the same line of the neighboring column ("should support horizontal orientation with grid
/// layout", ListBox.test.js "should support horizontal grid layout").
#[browser_test]
pub async fn horizontal_grid_layout(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["horizontal-grid-layout"])
        .await?;
    let label = "Horizontal grid";
    list(page, label)
        .await?
        .wait_for_attr("data-orientation", Some("horizontal"))
        .await?;
    page.element("#test-glc-hgrid-before")
        .await?
        .click()
        .await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&row(page, label, "Cat").await?).await?;
    arrows_focus(
        page,
        label,
        &[
            (Key::Down, "Dog"),
            (Key::Right, "Koala"),
            (Key::Up, "Kangaroo"),
            (Key::Right, "Panda"),
            (Key::Down, "Snake"),
            (Key::Left, "Koala"),
            (Key::Down, "Panda"),
            (Key::Left, "Kangaroo"),
            (Key::Left, "Cat"),
        ],
    )
    .await?;
    // The first column has nothing left of it.
    page.send_keys(Key::Left).await?;
    page.focus_stays(&row(page, label, "Cat").await?, Duration::from_millis(100))
        .await
}

/// Right to left, the columns run from right to left: ArrowLeft moves to the next column and
/// ArrowRight to the previous one.
#[browser_test]
pub async fn horizontal_grid_layout_right_to_left(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["horizontal-grid-layout-rtl"])
        .await?;
    let label = "Horizontal grid RTL";
    page.element("#test-glc-hgrid-rtl-before")
        .await?
        .click()
        .await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&row(page, label, "Cat").await?).await?;
    arrows_focus(
        page,
        label,
        &[
            (Key::Left, "Kangaroo"),
            (Key::Down, "Koala"),
            (Key::Right, "Dog"),
            (Key::Up, "Cat"),
        ],
    )
    .await?;
    // The first column has nothing right of it.
    page.send_keys(Key::Right).await?;
    page.focus_stays(&row(page, label, "Cat").await?, Duration::from_millis(100))
        .await
}

/// A section is a row group labelled by its header, by its `aria-label`, or by both ("should
/// support sections").
#[browser_test]
pub async fn sections(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["sections"]).await?;
    let groups = list(page, "Sections")
        .await?
        .elements("[role=rowgroup]")
        .await?;
    assert_that!(groups.len()).is_equal_to(3);
    assert_that!(groups[0])
        .accessible_name()
        .await
        .is_equal_to("Favorite Animal");
    assert_that!(groups[0])
        .attribute("aria-label")
        .await
        .is_none();
    // Labelled by the header once it rendered.
    let header = header_id(&groups[0]).await?;
    groups[0]
        .wait_for_attr("aria-labelledby", Some(header.as_str()))
        .await?;
    assert_that!(groups[1])
        .accessible_name()
        .await
        .is_equal_to("Favorite Ice Cream");
    assert_that!(groups[1])
        .attribute("aria-labelledby")
        .await
        .is_none();
    // Both: the label first, then the header (react-aria's `useLabels`).
    assert_that!(|| groups[2].accessible_name())
        .eventually_ok()
        .matches(assertr::matchers::eq("Favorite Fruit".to_owned()))
        .await;
    Ok(())
}

/// The id of the header cell of the section `group`.
async fn header_id(group: &WebElement) -> Result<String, Report> {
    let header = group.element("[role=rowheader]").await?;
    Ok(header.attr("id").await?.unwrap_or_default())
}

/// Without `should_select_on_press_up`, a row is selected when the press starts ("should select an
/// item on pressing down when shouldSelectOnPressUp is not provided").
#[browser_test]
pub async fn selects_on_press_down_by_default(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["press-up-default"]).await?;
    let selection = page.element("#test-glc-press-up-default-selection").await?;
    let held = row(page, "Press default", "Cat")
        .await?
        .press_and_hold()
        .await?;
    selection.wait_for_inner_text("Cat").await?;
    held.release().await?;
    Ok(())
}

/// With `should_select_on_press_up`, a row is selected only when the press ends ("should select an
/// item on pressing up when shouldSelectOnPressUp is true").
#[browser_test]
pub async fn selects_on_press_up(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["press-up-true"]).await?;
    let selection = page.element("#test-glc-press-up-true-selection").await?;
    let held = row(page, "Press up", "Cat").await?.press_and_hold().await?;
    selection
        .inner_text_stays("", Duration::from_millis(100))
        .await?;
    held.release().await?;
    selection.wait_for_inner_text("Cat").await?;
    Ok(())
}

/// Clicking a text input in a row of an arrow-navigated grid list focuses the input and selects
/// nothing ("should not trigger selection when clicking on a tabbable child in arrow navigation
/// mode").
#[browser_test]
pub async fn clicking_a_tabbable_child_in_arrow_navigation(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["input-arrow"]).await?;
    let input = page.element(css("input[aria-label='input 1']")).await?;
    input.click().await?;
    page.wait_for_focus(&input).await?;
    page.focus_stays(&input, Duration::from_millis(100)).await?;
    page.element("#test-glc-input-arrow-selection")
        .await?
        .inner_text_stays("", Duration::from_millis(100))
        .await?;
    Ok(())
}
