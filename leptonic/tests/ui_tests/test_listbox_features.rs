// Upstream: react-aria-components/test/ListBox.test.js @ 99e6102368
// Upstream: react-aria/test/selection/useSelectableCollection.test.js @ 99e6102368
//! "should support sections" (one group element per section, named by its header or
//! `aria-label`), separators inside a listbox, and arrow keys across sections.
//!
//! `selectionBehavior="replace"`: replacing selection on press and focus, modifier keys toggle
//! or extend, the action on double click and Enter ("should trigger onAction on double click if
//! selectionBehavior="replace"", "should perform toggle selection in highlight mode when using
//! modifier keys").
//!
//! "should support onAction" (mouse and keyboard) without selection, and links: "should support
//! links with selectionMode="none"" and "single" (the link opens, nothing is selected); arrow
//! keys onto link items are handled (no native scrolling).
//!
//! "should support horizontal orientation" (also right-to-left), "should support grid layout",
//! PageDown/PageUp, `shouldFocusWrap`.
//!
//! `disabledBehavior="selection"` (on the listbox and on one item: focusable but not
//! selectable), "should support empty state", and focus moving on when the focused option is
//! removed, labels following the collection.
use assertr::{matchers::eq, prelude::*};
use browser_test::{browser_test, thirtyfour::prelude::*};
use leptonic::AriaRole;
use rootcause::{Report, prelude::ResultExt};

use crate::pages::{ElementActions, KeyKind, Page, SyntheticEvent, role};

const PATH: &str = "/atoms/listbox-features";

/// The option with the text `text` in the listbox inside `container`.
async fn option(page: &Page<'_>, container: &str, text: &str) -> Result<WebElement, Report> {
    let container = page.element(container).await?;
    container.element(role(AriaRole::Option).text(text)).await
}

/// The location's hash (`#fragment`), or `""`.
async fn hash(page: &Page<'_>) -> Result<String, Report> {
    Ok(page
        .low_level()
        .driver()
        .current_url()
        .await?
        .fragment()
        .map(|fragment| format!("#{fragment}"))
        .unwrap_or_default())
}

/// Each section renders as a `<section>` group named by its presentational header or its
/// `aria-label`, and a separator renders as a `<div>` ("should support sections").
#[browser_test]
pub async fn sections_and_separators(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.wait_for_count("#lbf-sections [role=group]", 2).await?;
    let groups = page.elements("#lbf-sections [role=group]").await?;
    assert_that!(&groups).has_length(2);
    for group in &groups {
        assert_that!(group)
            .has_attribute("class")
            .await
            .is_equal_to("leptonic-ListBoxSection");
        assert_that!(group.tag_name().await?).is_equal_to("section");
    }
    let heading_id = groups[0].attr("aria-labelledby").await?.unwrap_or_default();
    let heading = page.element(format!("#{heading_id}")).await?;
    assert_that!(heading)
        .inner_text()
        .await
        .is_equal_to("Veggies");
    assert_that!(heading)
        .has_attribute("role")
        .await
        .is_equal_to("presentation");
    assert_that!(groups[1])
        .has_attribute("aria-label")
        .await
        .is_equal_to("Protein");
    // A separator between the options is no `<hr>` (invalid inside a listbox).
    let separator = page
        .element("#lbf-sections [role=listbox] > [role=separator]")
        .await?;
    assert_that!(separator.tag_name().await?).is_equal_to("div");
    Ok(())
}

/// Arrow keys, Home and End move the focus across section boundaries.
#[browser_test]
pub async fn arrow_keys_cross_sections(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let tomato = option(page, "#lbf-sections", "Tomato").await?;
    tomato.click().await?;
    page.wait_for_focus(&tomato).await?;
    for (key, expected) in [
        (Key::Down, "Onion"),
        (Key::Down, "Ham"),
        (Key::Up, "Onion"),
        (Key::End, "Tofu"),
        (Key::Home, "Lettuce"),
    ] {
        page.send_keys(key.clone()).await?;
        page.wait_for_focus(&option(page, "#lbf-sections", expected).await?)
            .await
            .context_with(|| format!("after pressing {key:?}"))?;
    }
    Ok(())
}

/// Hovering a selectable option sets `data-hovered`, hovering a non-interactive one doesn't
/// ("should support hover", "should not show hover state when item is not interactive").
#[browser_test]
pub async fn hover(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let cat = option(page, "#lbf-replace", "Cat").await?;
    cat.hover().await?;
    cat.wait_for_attr("data-hovered", Some("true")).await?;
    let plain = option(page, "#lbf-horizontal", "Dog").await?;
    plain.hover().await?;
    cat.wait_for_attr("data-hovered", None).await?;
    plain
        .attr_stays("data-hovered", None, std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// With replace selection behavior, a click replaces the selection, Ctrl+click toggles, Shift+click
/// extends and a double click selects and performs the action ("should perform toggle selection in
/// highlight mode when using modifier keys", "should trigger onAction on double click if
/// selectionBehavior="replace"").
#[browser_test]
pub async fn replace_selection_by_press(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let selection = page.element("#lbf-replace-selection").await?;
    let cat = option(page, "#lbf-replace", "Cat").await?;
    let dog = option(page, "#lbf-replace", "Dog").await?;
    let kangaroo = option(page, "#lbf-replace", "Kangaroo").await?;
    cat.click().await?;
    selection.wait_for_inner_text("Cat").await?;
    dog.click().await?;
    selection.wait_for_inner_text("Dog").await?;
    page.low_level()
        .driver()
        .action_chain()
        .key_down(Key::Control)
        .click_element(&kangaroo)
        .key_up(Key::Control)
        .perform()
        .await?;
    selection.wait_for_inner_text("Dog,Kangaroo").await?;
    page.low_level()
        .driver()
        .action_chain()
        .key_down(Key::Shift)
        .click_element(&cat)
        .key_up(Key::Shift)
        .perform()
        .await?;
    selection.wait_for_inner_text("Cat,Dog,Kangaroo").await?;
    page.low_level()
        .driver()
        .action_chain()
        .double_click_element(&dog)
        .perform()
        .await?;
    page.element("#lbf-replace-actions")
        .await?
        .wait_for_inner_text("Dog")
        .await?;
    selection.wait_for_inner_text("Dog").await?;
    Ok(())
}

/// With replace selection behavior, arrow keys move the selection with the focus, Ctrl+arrow moves
/// only the focus, Ctrl+Space toggles and Enter performs the action ("replaces selection as focus
/// moves with arrow keys", "navigates focus when Control is held (non-contiguous)").
#[browser_test]
pub async fn replace_selection_by_keyboard(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let selection = page.element("#lbf-replace-selection").await?;
    let dog = option(page, "#lbf-replace", "Dog").await?;
    let kangaroo = option(page, "#lbf-replace", "Kangaroo").await?;
    dog.click().await?;
    selection.wait_for_inner_text("Dog").await?;
    page.wait_for_focus(&dog).await?;
    page.send_keys(Key::Down).await?;
    page.wait_for_focus(&kangaroo).await?;
    selection.wait_for_inner_text("Kangaroo").await?;
    page.send_keys(Key::Control + Key::Up).await?;
    page.wait_for_focus(&dog).await?;
    selection
        .inner_text_stays("Kangaroo", std::time::Duration::from_millis(100))
        .await?;
    page.send_keys(Key::Control + " ").await?;
    selection.wait_for_inner_text("Dog,Kangaroo").await?;
    page.send_keys(Key::Enter).await?;
    page.element("#lbf-replace-actions")
        .await?
        .wait_for_inner_text("Dog")
        .await?;
    Ok(())
}

/// Without selection, a press and Enter perform an option's action and select nothing ("should
/// support onAction, interactionType: $interactionType").
#[browser_test]
pub async fn actions_without_selection(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let actions = page.element("#lbf-action-actions").await?;
    let cat = option(page, "#lbf-action", "Cat").await?;
    let dog = option(page, "#lbf-action", "Dog").await?;
    cat.click().await?;
    actions.wait_for_inner_text("Cat").await?;
    page.wait_for_focus(&cat).await?;
    page.send_keys(Key::Down).await?;
    page.wait_for_focus(&dog).await?;
    page.send_keys(Key::Enter).await?;
    actions.wait_for_inner_text("Cat,Dog").await?;
    assert_that!(dog).attribute("aria-selected").await.is_none();
    Ok(())
}

/// Link options open on press and on Enter, and ArrowDown onto a link moves the focus with the
/// native scrolling prevented ("should support links with selectionMode="none"").
#[browser_test]
pub async fn links(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let one = option(page, "#lbf-links", "One").await?;
    one.click().await?;
    assert_that!(|| hash(page))
        .eventually_ok()
        .matches(eq("#lbf-one"))
        .await;
    page.wait_for_focus(&one).await?;
    let arrow_down = one
        .dispatch(SyntheticEvent::keyboard(KeyKind::Down, "ArrowDown"))
        .await?;
    assert_that!(arrow_down.default_prevented).is_true();
    page.wait_for_focus(&option(page, "#lbf-links", "Two").await?)
        .await?;
    page.send_keys(Key::Enter).await?;
    assert_that!(|| hash(page))
        .eventually_ok()
        .matches(eq("#lbf-two"))
        .await;
    Ok(())
}

/// With single selection, pressing a link option opens it without selecting it ("should support
/// links with selectionMode="%s"").
#[browser_test]
pub async fn links_with_single_selection(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let one = option(page, "#lbf-links-single", "One").await?;
    one.click().await?;
    assert_that!(|| hash(page))
        .eventually_ok()
        .matches(eq("#lbf-one"))
        .await;
    one.attr_stays(
        "aria-selected",
        Some("false"),
        std::time::Duration::from_millis(100),
    )
    .await?;
    Ok(())
}

/// Arrow keys move the focus by the layout in horizontal (also right-to-left), grid and wrapping
/// listboxes ("should support horizontal orientation", "should support grid layout", "wraps focus
/// with ArrowUp and ArrowDown when shouldFocusWrap is set").
#[browser_test]
pub async fn arrow_keys_per_layout(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    for (container, keys) in [
        (
            "#lbf-horizontal",
            [
                (Key::Right, "Dog"),
                (Key::Right, "Kangaroo"),
                (Key::Left, "Dog"),
            ],
        ),
        (
            "#lbf-rtl",
            [
                (Key::Left, "Dog"),
                (Key::Left, "Kangaroo"),
                (Key::Right, "Dog"),
            ],
        ),
        (
            "#lbf-grid",
            [
                (Key::Down, "Kangaroo"),
                (Key::Left, "Dog"),
                (Key::Left, "Cat"),
            ],
        ),
        (
            "#lbf-wrap",
            [
                (Key::Up, "Kangaroo"),
                (Key::Down, "Cat"),
                (Key::Down, "Dog"),
            ],
        ),
    ] {
        let cat = option(page, container, "Cat").await?;
        cat.click().await?;
        page.wait_for_focus(&cat).await?;
        for (key, expected) in keys {
            page.send_keys(key.clone()).await?;
            page.wait_for_focus(&option(page, container, expected).await?)
                .await
                .context_with(|| format!("{container}: after pressing {key:?}"))?;
        }
    }
    Ok(())
}

/// PageDown moves the focus a page of options down (at least three) and PageUp back ("navigates by
/// page with PageDown and PageUp").
#[browser_test]
pub async fn page_down_and_up(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let first = option(page, "#lbf-page", "Option 1").await?;
    first.click().await?;
    page.wait_for_focus(&first).await?;
    page.send_keys(Key::PageDown).await?;
    // Option 4 or later.
    assert_that!(|| async {
        let text = page.focused_element().await?.inner_text().await?;
        Ok::<_, Report>(
            text.strip_prefix("Option ")
                .and_then(|number| number.parse::<u32>().ok()),
        )
    })
    .eventually_ok()
    .satisfies(|number| {
        number.is_some_satisfying(|number| {
            number.is_greater_or_equal_to(4);
        });
    })
    .await;
    page.send_keys(Key::PageUp).await?;
    page.wait_for_focus(&first).await?;
    Ok(())
}

/// With `disabledBehavior="selection"` (on the listbox or the item), a disabled option is
/// focusable but neither Space nor a press selects it, while a fully disabled item is skipped.
#[browser_test]
pub async fn disabled_selection_behavior(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    for container in ["#lbf-disabled-selection", "#lbf-item-disabled-behavior"] {
        let cat = option(page, container, "Cat").await?;
        let dog = option(page, container, "Dog").await?;
        cat.click().await?;
        page.wait_for_focus(&cat).await?;
        page.send_keys(Key::Down).await?;
        page.wait_for_focus(&dog).await?;
        assert_that!(dog)
            .attribute("aria-disabled")
            .await
            .with_detail_message(container)
            .is_none();
        page.send_keys(" ").await?;
        dog.click().await?;
        dog.attr_stays(
            "aria-selected",
            Some("false"),
            std::time::Duration::from_millis(100),
        )
        .await
        .context_with(|| format!("{container}: Dog"))?;
    }
    let dog = option(page, "#lbf-item-disabled-behavior", "Dog").await?;
    page.send_keys(Key::Down).await?;
    page.focus_stays(&dog, std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// An empty listbox has `data-empty` and shows its empty-state content ("should support empty
/// state").
#[browser_test]
pub async fn empty_state(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let empty = page.element("#lbf-empty [role=listbox]").await?;
    assert_that!(empty)
        .has_attribute("data-empty")
        .await
        .is_equal_to("true");
    let empty_option = page.element("#lbf-empty [role=option]").await?;
    assert_that!(empty_option)
        .inner_text()
        .await
        .is_equal_to("No results");
    Ok(())
}

/// Removing the focused option moves focus to the next one.
#[browser_test]
pub async fn removing_the_focused_option(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let dog = option(page, "#lbf-removal", "Dog").await?;
    dog.click().await?;
    page.wait_for_focus(&dog).await?;
    page.element("#lbf-remove-dog")
        .await?
        .virtual_click()
        .await?;
    page.wait_for_count("#lbf-removal [role=option]", 2).await?;
    page.wait_for_focus(&option(page, "#lbf-removal", "Kangaroo").await?)
        .await?;
    Ok(())
}

/// Relabelling an item and a section in place updates their `aria-label`s ("should update
/// collection when descendants update").
#[browser_test]
pub async fn labels_follow_the_collection(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let cat = option(page, "#lbf-relabel", "Cat").await?;
    let group = page.element("#lbf-relabel [role=group]").await?;
    assert_that!(cat)
        .has_attribute("aria-label")
        .await
        .is_equal_to("Cat");
    assert_that!(group)
        .has_attribute("aria-label")
        .await
        .is_equal_to("Pets");
    page.element("#lbf-relabel-cat").await?.click().await?;
    cat.wait_for_attr("aria-label", Some("Kitten")).await?;
    group.wait_for_attr("aria-label", Some("Animals")).await?;
    Ok(())
}

/// Renaming an item and a section header in the collection under the same keys updates the
/// options rendered by `ListBoxItems` and the section's default heading.
#[browser_test]
pub async fn renamed_items_update(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let cat = option(page, "#lbf-rename", "Cat").await?;
    let heading = page.element("#lbf-rename-section header").await?;
    assert_that!(heading).inner_text().await.is_equal_to("Pets");
    page.element("#lbf-rename-cat").await?.click().await?;
    cat.wait_for_inner_text("Kitten").await?;
    heading.wait_for_inner_text("Animals").await?;
    Ok(())
}
