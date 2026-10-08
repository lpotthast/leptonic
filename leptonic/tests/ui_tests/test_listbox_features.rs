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
use assertr::prelude::*;
use browser_test::thirtyfour::prelude::*;
use rootcause::{Report, prelude::ResultExt};

use crate::{
    pages::{ElementActions, Page, PageActions, SyntheticEvent, xpath},
    polling::wait_for,
};

/// The option with the text `text` in the listbox inside `container`.
async fn option(page: &Page<'_>, container: &str, text: &str) -> Result<WebElement, Report> {
    let container = page.element(container).await?;
    container
        .element(xpath(format!(
            ".//*[@role='option'][normalize-space(.)='{text}']"
        )))
        .await
}

/// The location's hash (`#fragment`), or `""`.
async fn hash(page: &Page<'_>) -> Result<String, Report> {
    Ok(page
        .driver
        .current_url()
        .await?
        .fragment()
        .map(|fragment| format!("#{fragment}"))
        .unwrap_or_default())
}

/// Each section is a `<section>` group directly in the listbox, named by its header (a
/// presentation element) or its `aria-label`; a separator is a `<div>`.
pub async fn sections_and_separators(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/listbox-features").await?;
    page.element("#lbf-sections [role=group]").await?;
    let groups = page.elements("#lbf-sections [role=group]").await?;
    assert_that!(&groups).has_length(2);
    for group in &groups {
        assert_that!(group.class_name().await?)
            .get_some()
            .is_equal_to("leptonic-ListBoxSection");
        assert_that!(group.tag_name().await?).is_equal_to("section");
    }
    let heading_id = groups[0].attr("aria-labelledby").await?.unwrap_or_default();
    let heading = page.element(format!("#{heading_id}")).await?;
    assert_that!(heading.inner_text().await?).is_equal_to("Veggies");
    assert_that!(heading.attr("role").await?)
        .get_some()
        .is_equal_to("presentation");
    assert_that!(groups[1].attr("aria-label").await?)
        .get_some()
        .is_equal_to("Protein");
    // A separator between the options is no `<hr>` (invalid inside a listbox).
    let separator = page
        .element("#lbf-sections [role=listbox] > [role=separator]")
        .await?;
    assert_that!(separator.tag_name().await?).is_equal_to("div");
    Ok(())
}

/// Arrow keys, Home and End move across sections.
pub async fn arrow_keys_cross_sections(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/listbox-features").await?;
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

/// "should support hover": interactive options show it, others don't.
pub async fn hover(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/listbox-features").await?;
    let cat = option(page, "#lbf-replace", "Cat").await?;
    cat.hover().await?;
    cat.wait_for_attr("data-hovered", Some("true")).await?;
    let plain = option(page, "#lbf-horizontal", "Dog").await?;
    plain.hover().await?;
    cat.wait_for_attr("data-hovered", None).await?;
    plain.attr_stays("data-hovered", None).await?;
    Ok(())
}

/// A click replaces the selection, Ctrl+click toggles, Shift+click extends, a double click
/// performs the action (and selects).
pub async fn replace_selection_by_press(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/listbox-features").await?;
    let selection = page.element("#lbf-replace-selection").await?;
    let cat = option(page, "#lbf-replace", "Cat").await?;
    let dog = option(page, "#lbf-replace", "Dog").await?;
    let kangaroo = option(page, "#lbf-replace", "Kangaroo").await?;
    cat.click().await?;
    selection.wait_for_inner_text("Cat").await?;
    dog.click().await?;
    selection.wait_for_inner_text("Dog").await?;
    page.driver
        .action_chain()
        .key_down(Key::Control)
        .click_element(&kangaroo)
        .key_up(Key::Control)
        .perform()
        .await?;
    selection.wait_for_inner_text("Dog,Kangaroo").await?;
    page.driver
        .action_chain()
        .key_down(Key::Shift)
        .click_element(&cat)
        .key_up(Key::Shift)
        .perform()
        .await?;
    selection.wait_for_inner_text("Cat,Dog,Kangaroo").await?;
    page.driver
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

/// After a click, arrow keys move the selection with the focus; Ctrl moves focus only;
/// Ctrl+Space toggles the focused option; Enter performs its action.
pub async fn replace_selection_by_keyboard(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/listbox-features").await?;
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
    selection.inner_text_stays("Kangaroo").await?;
    page.send_keys(Key::Control + " ").await?;
    selection.wait_for_inner_text("Dog,Kangaroo").await?;
    page.send_keys(Key::Enter).await?;
    page.element("#lbf-replace-actions")
        .await?
        .wait_for_inner_text("Dog")
        .await?;
    Ok(())
}

/// "should support onAction": a press and Enter perform the action, nothing is selected.
pub async fn actions_without_selection(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/listbox-features").await?;
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
    assert_that!(dog.attr("aria-selected").await?).is_none();
    Ok(())
}

/// Links open on press and Enter; ArrowDown onto a link is handled (the default, scrolling, is
/// prevented).
pub async fn links(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/listbox-features").await?;
    let one = option(page, "#lbf-links", "One").await?;
    one.click().await?;
    wait_for("the location hash")
        .observing(|| hash(page))
        .to_be_equal_to("#lbf-one")
        .await?;
    page.wait_for_focus(&one).await?;
    let arrow_down = one
        .dispatch(SyntheticEvent::keyboard("keydown", "ArrowDown"))
        .await?;
    assert_that!(arrow_down.default_prevented).is_true();
    page.wait_for_focus(&option(page, "#lbf-links", "Two").await?)
        .await?;
    page.send_keys(Key::Enter).await?;
    wait_for("the location hash")
        .observing(|| hash(page))
        .to_be_equal_to("#lbf-two")
        .await?;
    Ok(())
}

/// A link item opens also with single selection, and isn't selected.
pub async fn links_with_single_selection(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/listbox-features").await?;
    let one = option(page, "#lbf-links-single", "One").await?;
    one.click().await?;
    wait_for("the location hash")
        .observing(|| hash(page))
        .to_be_equal_to("#lbf-one")
        .await?;
    one.attr_stays("aria-selected", Some("false")).await?;
    Ok(())
}

/// Horizontal (also right-to-left), grid and wrapping listboxes move focus by their layout.
pub async fn arrow_keys_per_layout(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/listbox-features").await?;
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

/// PageDown moves a page of options down (at least three), PageUp back.
pub async fn page_down_and_up(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/listbox-features").await?;
    let first = option(page, "#lbf-page", "Option 1").await?;
    first.click().await?;
    page.wait_for_focus(&first).await?;
    page.send_keys(Key::PageDown).await?;
    wait_for("the focused option")
        .observing(|| async { page.focused_element().await?.inner_text().await })
        .to_be("Option 4 or later", |text| {
            text.strip_prefix("Option ")
                .and_then(|number| number.parse::<u32>().ok())
                .is_some_and(|number| number >= 4)
        })
        .await?;
    page.send_keys(Key::PageUp).await?;
    page.wait_for_focus(&first).await?;
    Ok(())
}

/// `disabledBehavior="selection"` on the listbox and on one item: the disabled option is
/// focusable but neither Space nor a press selects it. An item disabled without its own behavior
/// is skipped.
pub async fn disabled_selection_behavior(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/listbox-features").await?;
    for container in ["#lbf-disabled-selection", "#lbf-item-disabled-behavior"] {
        let cat = option(page, container, "Cat").await?;
        let dog = option(page, container, "Dog").await?;
        cat.click().await?;
        page.wait_for_focus(&cat).await?;
        page.send_keys(Key::Down).await?;
        page.wait_for_focus(&dog).await?;
        assert_that!(dog.attr("aria-disabled").await?)
            .with_detail_message(container)
            .is_none();
        page.send_keys(" ").await?;
        dog.click().await?;
        dog.attr_stays("aria-selected", Some("false"))
            .await
            .context_with(|| format!("{container}: Dog"))?;
    }
    let dog = option(page, "#lbf-item-disabled-behavior", "Dog").await?;
    page.send_keys(Key::Down).await?;
    page.focus_stays(&dog).await?;
    Ok(())
}

/// "should support empty state": the listbox is marked empty and shows the empty content.
pub async fn empty_state(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/listbox-features").await?;
    let empty = page.element("#lbf-empty [role=listbox]").await?;
    assert_that!(empty.attr("data-empty").await?)
        .get_some()
        .is_equal_to("true");
    let empty_option = page.element("#lbf-empty [role=option]").await?;
    assert_that!(empty_option.inner_text().await?).is_equal_to("No results");
    Ok(())
}

/// Removing the focused option moves focus to the next one.
pub async fn removing_the_focused_option(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/listbox-features").await?;
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

/// Labels follow the collection: an item and a section relabelled in place.
pub async fn labels_follow_the_collection(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/listbox-features").await?;
    let cat = option(page, "#lbf-relabel", "Cat").await?;
    let group = page.element("#lbf-relabel [role=group]").await?;
    assert_that!(cat.attr("aria-label").await?)
        .get_some()
        .is_equal_to("Cat");
    assert_that!(group.attr("aria-label").await?)
        .get_some()
        .is_equal_to("Pets");
    page.element("#lbf-relabel-cat").await?.click().await?;
    cat.wait_for_attr("aria-label", Some("Kitten")).await?;
    group.wait_for_attr("aria-label", Some("Animals")).await?;
    Ok(())
}
