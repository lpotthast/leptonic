// Upstream: react-aria-components/test/ListBox.test.js @ 99e6102368
// Upstream: react-aria-components/test/ListBox.browser.test.tsx @ 99e6102368
// Upstream: react-aria/test/selection/useSelectableCollection.test.js @ 99e6102368
// Upstream: @adobe/react-spectrum/test/listbox/ListBox.test.js @ 99e6102368
//! ListBox selection: single and multiple selection with default and fixed (controlled)
//! selections, replace selection behavior (focus selects, Home/End, entering the listbox, touch
//! and screen reader presses toggle), long press selection next to an action, Escape without
//! clearing, links with selection, label/description slots, collections that change, grid
//! layouts, type-ahead, classes and attributes, and auto focus.
use std::time::Duration;

use assertr::{matchers::eq, prelude::*};
use browser_test::{browser_test, thirtyfour::prelude::*};
use leptonic::AriaRole;
use rootcause::Report;

use crate::pages::{ElementActions, Page, PointerKind, PointerType, SyntheticEvent, css, role};

const PATH: &str = "/atoms/listbox-selection";

/// How long "nothing changes" is observed for.
const STAYS: Duration = Duration::from_millis(100);

/// The option with the text `text` in the listbox `#<section>`.
async fn option(page: &Page<'_>, section: &str, text: &str) -> Result<WebElement, Report> {
    page.element(format!("#{section}"))
        .await?
        .element(role(AriaRole::Option).text(text))
        .await
}

/// The log of the selection changes of the listbox `#<section>`.
async fn changes(page: &Page<'_>, section: &str) -> Result<WebElement, Report> {
    page.element(format!("#{section}-changes")).await
}

/// A touch press (pointer down, then up) on `element`.
async fn tap(element: &WebElement) -> Result<(), Report> {
    element
        .dispatch(SyntheticEvent::pointer(PointerKind::Down).pointer_type(PointerType::Touch))
        .await?;
    element
        .dispatch(SyntheticEvent::pointer(PointerKind::Up).pointer_type(PointerType::Touch))
        .await?;
    Ok(())
}

/// With single selection, the default option is selected, and a click, Space or Enter selects
/// another one instead ("supports defaultSelectedKeys (uncontrolled)", "supports using click to
/// change item selection", "supports using space key to change item selection").
#[browser_test]
pub async fn single_selection(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["single"]).await?;
    let changes = changes(page, "single").await?;
    let cat = option(page, "single", "Cat").await?;
    let dog = option(page, "single", "Dog").await?;
    let kangaroo = option(page, "single", "Kangaroo").await?;
    assert_that!(dog)
        .has_attribute("aria-selected")
        .await
        .is_equal_to("true");
    cat.click().await?;
    changes.wait_for_inner_text("Cat").await?;
    dog.wait_for_attr("aria-selected", Some("false")).await?;
    page.wait_for_focus(&cat).await?;
    page.send_keys(Key::Down).await?;
    page.wait_for_focus(&dog).await?;
    page.send_keys(" ").await?;
    changes.wait_for_inner_text("Cat|Dog").await?;
    page.send_keys(Key::Down).await?;
    page.wait_for_focus(&kangaroo).await?;
    page.send_keys(Key::Enter).await?;
    changes.wait_for_inner_text("Cat|Dog|Kangaroo").await?;
    assert_that!(cat)
        .has_attribute("aria-selected")
        .await
        .is_equal_to("false");
    assert_that!(kangaroo)
        .has_attribute("data-selected")
        .await
        .is_equal_to("true");
    Ok(())
}

/// A fixed single selection reports the option a click selects, but keeps showing its own
/// ("supports selectedKeys (controlled)").
#[browser_test]
pub async fn fixed_single_selection(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["single-controlled"]).await?;
    let changes = changes(page, "single-controlled").await?;
    let cat = option(page, "single-controlled", "Cat").await?;
    let dog = option(page, "single-controlled", "Dog").await?;
    cat.click().await?;
    changes.wait_for_inner_text("Cat").await?;
    dog.attr_stays("aria-selected", Some("true"), STAYS).await?;
    assert_that!(cat)
        .has_attribute("aria-selected")
        .await
        .is_equal_to("false");
    Ok(())
}

/// Ctrl+A selects nothing in a single selection listbox ("does not select all with Mod+A when
/// selection mode is single").
#[browser_test]
pub async fn select_all_does_nothing_with_single_selection(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["single"]).await?;
    let changes = changes(page, "single").await?;
    let cat = option(page, "single", "Cat").await?;
    cat.click().await?;
    changes.wait_for_inner_text("Cat").await?;
    page.send_keys(page.primary_modifier().await? + "a").await?;
    changes.inner_text_stays("Cat", STAYS).await?;
    Ok(())
}

/// With multiple selection, the default options are selected and clicks toggle options
/// ("supports multiple defaultSelectedKeys (uncontrolled)").
#[browser_test]
pub async fn multiple_default_selection(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["multi-default"]).await?;
    let changes = changes(page, "multi-default").await?;
    let cat = option(page, "multi-default", "Cat").await?;
    let dog = option(page, "multi-default", "Dog").await?;
    let kangaroo = option(page, "multi-default", "Kangaroo").await?;
    assert_that!(cat)
        .has_attribute("aria-selected")
        .await
        .is_equal_to("true");
    assert_that!(kangaroo)
        .has_attribute("aria-selected")
        .await
        .is_equal_to("true");
    dog.click().await?;
    changes.wait_for_inner_text("Cat,Dog,Kangaroo").await?;
    cat.click().await?;
    changes
        .wait_for_inner_text("Cat,Dog,Kangaroo|Dog,Kangaroo")
        .await?;
    cat.wait_for_attr("aria-selected", Some("false")).await?;
    Ok(())
}

/// A fixed multiple selection reports the selection a click would make, but keeps showing its
/// own ("supports multiple selectedKeys (controlled)").
#[browser_test]
pub async fn fixed_multiple_selection(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["multi-controlled"]).await?;
    let changes = changes(page, "multi-controlled").await?;
    let dog = option(page, "multi-controlled", "Dog").await?;
    dog.click().await?;
    changes.wait_for_inner_text("Cat,Dog,Kangaroo").await?;
    dog.attr_stays("aria-selected", Some("false"), STAYS)
        .await?;
    assert_that!(option(page, "multi-controlled", "Cat").await?)
        .has_attribute("aria-selected")
        .await
        .is_equal_to("true");
    Ok(())
}

/// With `EscapeKeyBehavior::None`, Escape leaves the selection as it is ("should prevent Esc
/// from clearing selection if escapeKeyBehavior is "none"").
#[browser_test]
pub async fn escape_key_behavior_none(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["escape-none"]).await?;
    let changes = changes(page, "escape-none").await?;
    option(page, "escape-none", "Dog").await?.click().await?;
    changes.wait_for_inner_text("Cat,Dog").await?;
    page.send_keys(Key::Escape).await?;
    changes.inner_text_stays("Cat,Dog", STAYS).await?;
    Ok(())
}

/// With replace selection behavior, tabbing into the listbox focuses and selects the first
/// option, and Home and End move the selection with the focus ("selects the first item it
/// focuses if selectOnFocus", "replaces selection with Home and End").
#[browser_test]
pub async fn replace_selection_on_entry_and_home_end(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["replace"]).await?;
    let changes = changes(page, "replace").await?;
    page.element("#replace-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&option(page, "replace", "Cat").await?)
        .await?;
    changes.wait_for_inner_text("Cat").await?;
    page.send_keys(Key::End).await?;
    page.wait_for_focus(&option(page, "replace", "Kangaroo").await?)
        .await?;
    changes.wait_for_inner_text("Cat|Kangaroo").await?;
    page.send_keys(Key::Home).await?;
    changes.wait_for_inner_text("Cat|Kangaroo|Cat").await?;
    Ok(())
}

/// Tabbing into a listbox with a selection focuses the first selected option, Shift+Tab the last
/// one, and neither changes the selection ("focuses first/last selected item on focus enter and
/// does not change the selection").
#[browser_test]
pub async fn entering_focuses_the_selection(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["replace-default"]).await?;
    let changes = changes(page, "replace-default").await?;
    page.element("#replace-default-before")
        .await?
        .click()
        .await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&option(page, "replace-default", "Dog").await?)
        .await?;
    changes.inner_text_stays("", STAYS).await?;
    // Afresh (the listbox remembers its focused option), from below.
    page.goto_sections(PATH, &["replace-default"]).await?;
    let changes = self::changes(page, "replace-default").await?;
    page.element("#replace-default-after")
        .await?
        .click()
        .await?;
    page.send_keys(Key::Shift + Key::Tab).await?;
    page.wait_for_focus(&option(page, "replace-default", "Kangaroo").await?)
        .await?;
    changes.inner_text_stays("", STAYS).await?;
    Ok(())
}

/// With replace selection behavior, Ctrl+arrow keys (Option+arrow on Apple devices) move the
/// focus without selecting, and Space then selects only the focused option ("can navigate without replacing the selection in
/// multiple selection selectOnFocus").
#[browser_test]
pub async fn space_replaces_after_moving_focus_only(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["replace"]).await?;
    let changes = changes(page, "replace").await?;
    let cat = option(page, "replace", "Cat").await?;
    cat.click().await?;
    changes.wait_for_inner_text("Cat").await?;
    page.send_keys(page.non_contiguous_selection_modifier().await? + Key::Down)
        .await?;
    page.wait_for_focus(&option(page, "replace", "Dog").await?)
        .await?;
    changes.inner_text_stays("Cat", STAYS).await?;
    page.send_keys(" ").await?;
    changes.wait_for_inner_text("Cat|Dog").await?;
    Ok(())
}

/// With replace selection behavior, clicking the selected option again keeps it selected
/// ("should perform replace selection in highlight mode when not using modifier keys").
#[browser_test]
pub async fn pressing_the_selected_option_keeps_it(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["replace"]).await?;
    let changes = changes(page, "replace").await?;
    let cat = option(page, "replace", "Cat").await?;
    cat.click().await?;
    changes.wait_for_inner_text("Cat").await?;
    cat.click().await?;
    changes.inner_text_stays("Cat", STAYS).await?;
    assert_that!(cat)
        .has_attribute("aria-selected")
        .await
        .is_equal_to("true");
    Ok(())
}

/// With single selection and replace behavior, a click selects an option instead of the selected
/// one, and Ctrl+click on the selected option deselects it ("should perform selection with
/// single selection").
#[browser_test]
pub async fn replace_behavior_with_single_selection(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["replace-single"]).await?;
    let changes = changes(page, "replace-single").await?;
    let dog = option(page, "replace-single", "Dog").await?;
    let kangaroo = option(page, "replace-single", "Kangaroo").await?;
    kangaroo.click().await?;
    changes.wait_for_inner_text("Kangaroo").await?;
    dog.click().await?;
    changes.wait_for_inner_text("Kangaroo|Dog").await?;
    kangaroo
        .wait_for_attr("aria-selected", Some("false"))
        .await?;
    page.click_with_primary_modifier(&dog).await?;
    changes.wait_for_inner_text("Kangaroo|Dog|").await?;
    dog.wait_for_attr("aria-selected", Some("false")).await?;
    Ok(())
}

/// With replace selection behavior, presses of screen readers (virtual clicks) and touch toggle
/// options instead of replacing the selection ("uses toggle mode when the interaction is touch").
#[browser_test]
pub async fn touch_and_screen_reader_presses_toggle(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["replace"]).await?;
    let changes = changes(page, "replace").await?;
    option(page, "replace", "Cat").await?.click().await?;
    changes.wait_for_inner_text("Cat").await?;
    option(page, "replace", "Dog")
        .await?
        .virtual_click()
        .await?;
    changes.wait_for_inner_text("Cat|Cat,Dog").await?;
    tap(&option(page, "replace", "Kangaroo").await?).await?;
    changes
        .wait_for_inner_text("Cat|Cat,Dog|Cat,Dog,Kangaroo")
        .await?;
    Ok(())
}

/// With toggle selection and an action, a tap performs the action, a long press selects, and
/// once something is selected a tap toggles ("should trigger selection on long press if both
/// onAction and selection exist (touch only)", "should support onAction, interactionType:
/// touch").
#[browser_test]
pub async fn long_press_selects_next_to_an_action(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["toggle-action"]).await?;
    let changes = changes(page, "toggle-action").await?;
    let actions = page.element("#toggle-action-actions").await?;
    let cat = option(page, "toggle-action", "Cat").await?;
    tap(&cat).await?;
    actions.wait_for_inner_text("Cat").await?;
    changes.inner_text_stays("", STAYS).await?;
    cat.dispatch(SyntheticEvent::pointer(PointerKind::Down).pointer_type(PointerType::Touch))
        .await?;
    // Past the 500 ms long press threshold.
    changes.wait_for_inner_text("Cat").await?;
    cat.dispatch(SyntheticEvent::pointer(PointerKind::Up).pointer_type(PointerType::Touch))
        .await?;
    tap(&option(page, "toggle-action", "Dog").await?).await?;
    changes.wait_for_inner_text("Cat|Cat,Dog").await?;
    actions.inner_text_stays("Cat", STAYS).await?;
    Ok(())
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

/// With multiple (toggle) selection, pressing a link option opens it without selecting it
/// ("should support links with selectionMode="multiple"").
#[browser_test]
pub async fn links_with_multiple_selection(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["links-multiple"]).await?;
    let one = option(page, "links-multiple", "One").await?;
    one.click().await?;
    assert_that!(|| hash(page))
        .eventually_ok()
        .matches(eq("#lbs-one"))
        .await;
    changes(page, "links-multiple")
        .await?
        .inner_text_stays("", STAYS)
        .await?;
    Ok(())
}

/// With replace selection behavior, a click or Space selects a link option without opening it,
/// and a double click or Enter opens it ("should support links with selectionMode="%s"
/// selectionBehavior="replace"").
#[browser_test]
pub async fn links_with_replace_selection(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["links-replace"]).await?;
    let changes = changes(page, "links-replace").await?;
    let one = option(page, "links-replace", "One").await?;
    let two = option(page, "links-replace", "Two").await?;
    one.click().await?;
    changes.wait_for_inner_text("one").await?;
    assert_that!(hash(page).await?).is_equal_to(String::new());
    page.low_level()
        .driver()
        .action_chain()
        .double_click_element(&one)
        .perform()
        .await?;
    assert_that!(|| hash(page))
        .eventually_ok()
        .matches(eq("#lbs-one"))
        .await;
    page.wait_for_focus(&one).await?;
    page.send_keys(page.non_contiguous_selection_modifier().await? + Key::Down)
        .await?;
    page.wait_for_focus(&two).await?;
    page.send_keys(" ").await?;
    changes.wait_for_inner_text("one|two").await?;
    page.send_keys(Key::Enter).await?;
    assert_that!(|| hash(page))
        .eventually_ok()
        .matches(eq("#lbs-two"))
        .await;
    Ok(())
}

/// An option's label and description name and describe it ("should support slots", "supports
/// complex options with aria-labelledby and aria-describedby").
#[browser_test]
pub async fn label_and_description_slots(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["slots"]).await?;
    let cat = option(page, "slots", "CatMeows").await?;
    assert_that!(cat).accessible_name().await.is_equal_to("Cat");
    assert_that!(cat)
        .accessible_description()
        .await
        .is_equal_to("Meows");
    Ok(())
}

/// The options follow the collection: an inserted option appears in its place, reordered options
/// are reordered, and an option moved to another section belongs to it, for arrow keys too
/// ("should support dynamic collections", "should update collection when moving item to a
/// different section", "should handle when an item changes sections").
#[browser_test]
pub async fn options_follow_the_collection(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["dynamic"]).await?;
    let options = "#dynamic [role=option]";
    page.element("#dynamic-insert").await?.click().await?;
    page.wait_for_count(options, 5).await?;
    assert_that!(page.inner_texts(options).await?)
        .contains_exactly(["Lettuce", "Onion", "Tomato", "Ham", "Tuna"]);
    page.element("#dynamic-reverse").await?.click().await?;
    assert_that!(|| page.inner_texts(options))
        .eventually_ok()
        .matches(eq(vec![
            "Tomato".to_owned(),
            "Onion".to_owned(),
            "Lettuce".to_owned(),
            "Ham".to_owned(),
            "Tuna".to_owned(),
        ]))
        .await;
    page.element("#dynamic-move").await?.click().await?;
    let veggies = page
        .element(role(AriaRole::Group).has(css("header").text("Veggies")))
        .await?;
    assert_that!(|| veggies.inner_texts(role(AriaRole::Option)))
        .eventually_ok()
        .matches(eq(vec![
            "Tomato".to_owned(),
            "Onion".to_owned(),
            "Lettuce".to_owned(),
            "Ham".to_owned(),
        ]))
        .await;
    let protein = page
        .element(role(AriaRole::Group).has(css("header").text("Protein")))
        .await?;
    assert_that!(protein.inner_texts(role(AriaRole::Option)).await?).contains_exactly(["Tuna"]);
    // Arrow keys follow the new order.
    let lettuce = option(page, "dynamic", "Lettuce").await?;
    lettuce.click().await?;
    page.wait_for_focus(&lettuce).await?;
    page.send_keys(Key::Down).await?;
    page.wait_for_focus(&option(page, "dynamic", "Ham").await?)
        .await?;
    page.send_keys(Key::Down).await?;
    page.wait_for_focus(&option(page, "dynamic", "Tuna").await?)
        .await?;
    Ok(())
}

/// In a grid layout, options are selected by click and by keyboard, arrow keys move by row and
/// column, and stop at the grid's edges ("should support keyboard navigation across grid layout
/// via the test util", "selects an option via $interactionType in real browser grid layout",
/// "should not throw TypeError at boundaries of vertical grid layout when keyboard navigating
/// (up/down)").
#[browser_test]
pub async fn grid_selection_and_edges(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["grid"]).await?;
    let changes = changes(page, "grid").await?;
    let item = |i: u32| {
        let text = format!("Item {i}");
        async move { option(page, "grid", &text).await }
    };
    let five = item(5).await?;
    five.click().await?;
    changes.wait_for_inner_text("5").await?;
    // 0 1 2
    // 3 4 5
    // 6 7 8
    page.send_keys(Key::Up).await?;
    page.wait_for_focus(&item(2).await?).await?;
    page.send_keys(Key::Up).await?;
    page.focus_stays(&item(2).await?, STAYS).await?;
    page.send_keys(Key::Left).await?;
    page.send_keys(Key::Left).await?;
    let zero = item(0).await?;
    page.wait_for_focus(&zero).await?;
    page.send_keys(" ").await?;
    changes.wait_for_inner_text("5|0").await?;
    page.send_keys(Key::Down).await?;
    page.send_keys(Key::Down).await?;
    page.wait_for_focus(&item(6).await?).await?;
    page.send_keys(Key::Down).await?;
    page.focus_stays(&item(6).await?, STAYS).await?;
    page.send_keys(Key::Right).await?;
    page.send_keys(Key::Right).await?;
    page.wait_for_focus(&item(8).await?).await?;
    page.send_keys(Key::Enter).await?;
    changes.wait_for_inner_text("5|0|8").await?;
    assert_that!(five)
        .has_attribute("aria-selected")
        .await
        .is_equal_to("false");
    Ok(())
}

/// Type-ahead continues across a space instead of selecting, Space selects once the search timed
/// out (after a second), and a search wraps around past the last option ("supports the space
/// character in a search", "supports item selection using the Spacebar after search times out",
/// "resets the search text after a timeout", "wraps around when no items past the current one
/// match").
#[browser_test]
pub async fn type_ahead_spaces_timeout_and_wrap(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["typeahead"]).await?;
    let changes = changes(page, "typeahead").await?;
    let bar = option(page, "typeahead", "Bar").await?;
    bar.click().await?;
    changes.wait_for_inner_text("Bar").await?;
    page.type_text("foo baz").await?;
    let foo_baz = option(page, "typeahead", "Foo Baz").await?;
    page.wait_for_focus(&foo_baz).await?;
    changes.inner_text_stays("Bar", STAYS).await?;
    // Past the type-ahead's 1 s timeout.
    tokio::time::sleep(Duration::from_millis(1100)).await;
    page.send_keys(" ").await?;
    changes.wait_for_inner_text("Bar|Foo Baz").await?;
    page.type_text("z").await?;
    page.wait_for_focus(&option(page, "typeahead", "Zoo").await?)
        .await?;
    // Past the timeout: a new search, from Zoo on, wraps around to the first match.
    tokio::time::sleep(Duration::from_millis(1100)).await;
    page.type_text("f").await?;
    page.wait_for_focus(&option(page, "typeahead", "Foo Bar").await?)
        .await?;
    Ok(())
}

/// The listbox and its options have their default classes, the classes given and attributes
/// set on them ("should render with default classes", "should render with custom classes",
/// "should support DOM props", "supports custom data attributes", "items support custom data
/// attributes").
#[browser_test]
pub async fn classes_and_attributes(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["attrs"]).await?;
    let listbox = page.element("#attrs [role=listbox]").await?;
    assert_that!(listbox)
        .has_attribute("class")
        .await
        .is_equal_to("leptonic-ListBox test-custom-listbox");
    assert_that!(listbox)
        .has_attribute("data-test")
        .await
        .is_equal_to("listbox");
    let cat = option(page, "attrs", "Cat").await?;
    assert_that!(cat)
        .has_attribute("class")
        .await
        .is_equal_to("leptonic-ListBoxItem test-custom-option");
    assert_that!(cat)
        .has_attribute("data-test")
        .await
        .is_equal_to("option");
    assert_that!(option(page, "attrs", "Dog").await?)
        .has_attribute("class")
        .await
        .is_equal_to("leptonic-ListBoxItem");
    Ok(())
}

/// Auto focus without a selection focuses the listbox itself ("should support autoFocus").
#[browser_test]
pub async fn auto_focus_without_selection(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["autofocus-selected"]).await?;
    page.wait_for_focus(&page.element("#autofocus-selected [role=listbox]").await?)
        .await?;
    Ok(())
}

/// Auto focus on the first or last option focuses it, and with replace selection behavior
/// selects it ("selects the autofocused item if selectOnFocus with autoFocus=$autoFocus").
#[browser_test]
pub async fn auto_focus_first_and_last(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["autofocus-first"]).await?;
    page.wait_for_focus(&option(page, "autofocus-first", "Cat").await?)
        .await?;
    changes(page, "autofocus-first")
        .await?
        .wait_for_inner_text("Cat")
        .await?;
    page.goto_sections(PATH, &["autofocus-last"]).await?;
    page.wait_for_focus(&option(page, "autofocus-last", "Kangaroo").await?)
        .await?;
    changes(page, "autofocus-last")
        .await?
        .wait_for_inner_text("Kangaroo")
        .await?;
    Ok(())
}

/// Auto focus goes to the selected option and leaves the selection unchanged ("does not change
/// the selection when autofocusing an already selected collection").
#[browser_test]
pub async fn auto_focus_keeps_the_selection(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["autofocus-selected-default"])
        .await?;
    page.wait_for_focus(&option(page, "autofocus-selected-default", "Dog").await?)
        .await?;
    changes(page, "autofocus-selected-default")
        .await?
        .inner_text_stays("", STAYS)
        .await?;
    Ok(())
}
