// Upstream: react-aria-components/test/Select.test.js @ 99e6102368
// Upstream: react-aria/test/select/HiddenSelect.test.tsx @ 99e6102368
// Upstream: @adobe/react-spectrum/test/picker/Picker.test.js @ 99e6102368
//! Select behavior: the popover's own content, labelling and descriptions, opening and closing
//! by pointer and keys, selecting in the popover (keys, hover, type-ahead), the closed
//! trigger's arrow keys, `should_close_on_select`, fixed (controlled) values and open states,
//! focus changes, the hidden form element (`form`, `autocomplete`, markup), validation, sections
//! and scrolling to the selected option.
use std::time::Duration;

use assertr::prelude::*;
use browser_test::{browser_test, thirtyfour::prelude::*};
use leptonic::AriaRole;
use rootcause::Report;

use crate::pages::{ElementActions, Page, css, role};

const PATH: &str = "/atoms/select-behavior";

/// How long "nothing changes" is observed for.
const STAYS: Duration = Duration::from_millis(100);

/// The trigger of the select in `#<section>`.
async fn trigger(page: &Page<'_>, section: &str) -> Result<WebElement, Report> {
    page.element(format!("#{section} [aria-haspopup=listbox]"))
        .await
}

/// The option `text` of the open select's listbox.
async fn option(page: &Page<'_>, text: &str) -> Result<WebElement, Report> {
    page.element(role(AriaRole::Listbox))
        .await?
        .element(role(AriaRole::Option).text(text))
        .await
}

/// The log of the select in `#<section>`.
async fn log(page: &Page<'_>, section: &str) -> Result<WebElement, Report> {
    page.element(format!("#{section}-log")).await
}

/// Focuses `#<section>`'s trigger from the keyboard: a click on the button before it, then Tab.
async fn tab_to_trigger(page: &Page<'_>, section: &str) -> Result<WebElement, Report> {
    let trigger = trigger(page, section).await?;
    page.element(format!("#{section}-before"))
        .await?
        .click()
        .await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&trigger).await?;
    Ok(trigger)
}

/// A label, button and description inside the popover don't belong to the select: the label has
/// no id, the select isn't described by the text ("should clear contexts inside popover").
#[browser_test]
pub async fn popover_content_isnt_part_of_the_select(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["contexts"]).await?;
    let trigger = trigger(page, "contexts").await?;
    assert_that!(trigger)
        .accessible_name()
        .await
        .is_equal_to("Cat Favorite animal");
    trigger.click().await?;
    let popover = page.element(".test-sb-contexts-popover").await?;
    popover.element(role(AriaRole::Listbox)).await?;
    let label = popover.element(css("label").text("Hello")).await?;
    assert_that!(label).attribute("id").await.is_none();
    assert_that!(label).attribute("for").await.is_none();
    let button = popover.element(css("button").text("Yo")).await?;
    assert_that!(button)
        .attribute("aria-expanded")
        .await
        .is_none();
    assert_that!(trigger)
        .attribute("aria-describedby")
        .await
        .is_none();
    page.send_keys(Key::Escape).await?;
    page.wait_for_count(role(AriaRole::Listbox), 0).await?;
    assert_that!(trigger)
        .accessible_name()
        .await
        .is_equal_to("Cat Favorite animal");
    crate::fixtures::take_warnings(page, "A <Description> describes nothing", 1).await?;
    Ok(())
}

/// Without a visible label, `aria-label` names the trigger (after its value) and the listbox
/// ("supports labeling via aria-label").
#[browser_test]
pub async fn labelled_by_aria_label(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["aria-label"]).await?;
    let trigger = trigger(page, "aria-label").await?;
    assert_that!(trigger)
        .accessible_name()
        .await
        .is_equal_to("Select an item Pick an animal");
    trigger.click().await?;
    assert_that!(page.element(role(AriaRole::Listbox)).await?)
        .accessible_name()
        .await
        .is_equal_to("Pick an animal");
    Ok(())
}

/// `aria-labelledby` names the trigger (after its value) and the listbox ("supports labeling
/// via aria-labelledby").
#[browser_test]
pub async fn labelled_by_aria_labelledby(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["aria-labelledby"]).await?;
    let trigger = trigger(page, "aria-labelledby").await?;
    assert_that!(trigger)
        .accessible_name()
        .await
        .is_equal_to("Select an item External label");
    trigger.click().await?;
    assert_that!(page.element(role(AriaRole::Listbox)).await?)
        .accessible_name()
        .await
        .is_equal_to("External label");
    Ok(())
}

/// With both, the trigger and the listbox are named by the referenced label and the
/// `aria-label` ("supports labeling via aria-label and aria-labelledby").
#[browser_test]
pub async fn labelled_by_both(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["both"]).await?;
    let trigger = trigger(page, "both").await?;
    assert_that!(trigger)
        .accessible_name()
        .await
        .is_equal_to("Select an item Pick an animal Both label");
    trigger.click().await?;
    assert_that!(page.element(role(AriaRole::Listbox)).await?)
        .accessible_name()
        .await
        .is_equal_to("Pick an animal Both label");
    Ok(())
}

/// The description and the error message describe the trigger, which is pressed while the
/// popover (marked as the select's) is open ("supports description", "supports error message",
/// "provides slots").
#[browser_test]
pub async fn described_and_pressed_while_open(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["described"]).await?;
    let trigger = trigger(page, "described").await?;
    assert_that!(trigger)
        .accessible_description()
        .await
        .is_equal_to("Pick wisely Invalid animal");
    trigger.click().await?;
    let popover = page.element(".leptonic-SelectPopover").await?;
    assert_that!(popover)
        .has_attribute("data-trigger")
        .await
        .is_equal_to("Select");
    trigger.wait_for_attr("data-pressed", Some("true")).await?;
    page.send_keys(Key::Escape).await?;
    trigger.wait_for_attr("data-pressed", None).await?;
    Ok(())
}

/// The popover opens as the pointer goes down (before it goes up), with focus on the listbox
/// while nothing is selected ("can be opened on mouse down").
#[browser_test]
pub async fn opens_on_pointer_down(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["empty"]).await?;
    let trigger = trigger(page, "empty").await?;
    let held = trigger.press_and_hold().await?;
    let listbox = page.element(role(AriaRole::Listbox)).await?;
    trigger.wait_for_attr("aria-expanded", Some("true")).await?;
    page.wait_for_focus(&listbox).await?;
    held.release().await?;
    Ok(())
}

/// Space and Enter open the popover onto the first option, ArrowUp onto the last ("can be
/// opened on Space key down", "can be opened on Enter key down", "can be opened on ArrowUp key
/// down and auto focuses the last item").
#[browser_test]
pub async fn opens_by_keys(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["empty"]).await?;
    let trigger = tab_to_trigger(page, "empty").await?;
    page.send_keys(" ").await?;
    page.wait_for_focus(&option(page, "Cat").await?).await?;
    page.send_keys(Key::Escape).await?;
    page.wait_for_focus(&trigger).await?;
    page.send_keys(Key::Enter).await?;
    page.wait_for_focus(&option(page, "Cat").await?).await?;
    page.send_keys(Key::Escape).await?;
    page.wait_for_focus(&trigger).await?;
    page.send_keys(Key::Up).await?;
    page.wait_for_focus(&option(page, "Kangaroo").await?)
        .await?;
    Ok(())
}

/// The popover closes by clicking the trigger again and by the hidden dismiss button, and focus
/// returns to the trigger, which no longer references the listbox ("can be closed by clicking on
/// the button", "should have a hidden dismiss button for screen readers").
#[browser_test]
pub async fn closes_by_trigger_and_dismiss_button(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["empty"]).await?;
    let trigger = trigger(page, "empty").await?;
    trigger.click().await?;
    page.element(role(AriaRole::Listbox)).await?;
    // The modal makes the trigger inert. A physical click at its position reaches the
    // backdrop, which dismisses the popover.
    trigger.press_and_hold().await?.release().await?;
    page.wait_for_count(role(AriaRole::Listbox), 0).await?;
    page.wait_for_focus(&trigger).await?;
    assert_that!(trigger)
        .attribute("aria-controls")
        .await
        .is_none();
    trigger.click().await?;
    page.element(role(AriaRole::Listbox)).await?;
    page.first_element("button[aria-label=Dismiss]")
        .await?
        .virtual_click()
        .await?;
    page.wait_for_count(role(AriaRole::Listbox), 0).await?;
    page.wait_for_focus(&trigger).await?;
    Ok(())
}

/// Tab inside the open popover keeps it open and the focus inside ("does not shift focus when
/// tabbing").
#[browser_test]
pub async fn tab_keeps_the_popover_open(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["empty"]).await?;
    let trigger = tab_to_trigger(page, "empty").await?;
    page.send_keys(Key::Down).await?;
    page.wait_for_focus(&option(page, "Cat").await?).await?;
    page.send_keys(Key::Tab).await?;
    page.count_stays(role(AriaRole::Listbox), 1, STAYS).await?;
    let focused = page.focused_element().await?;
    assert_that!(focused.is_within(".leptonic-SelectPopover").await?).is_true();
    assert_that!(trigger)
        .has_attribute("aria-expanded")
        .await
        .is_equal_to("true");
    Ok(())
}

/// A single select set to stay open keeps its popover after a selection, and a multiple select
/// set to close closes it ("should stay open on selecting an option if shouldCloseOnSelect is
/// false and single selection mode", "should close on selecting an option if shouldCloseOnSelect
/// is true and multiple selection mode").
#[browser_test]
pub async fn should_close_on_select_overrides_the_mode(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["stay-open", "close-multiple"])
        .await?;
    trigger(page, "stay-open").await?.click().await?;
    option(page, "Dog").await?.click().await?;
    log(page, "stay-open")
        .await?
        .wait_for_inner_text("dog")
        .await?;
    page.count_stays(role(AriaRole::Listbox), 1, STAYS).await?;
    page.send_keys(Key::Escape).await?;
    page.wait_for_count(role(AriaRole::Listbox), 0).await?;
    trigger(page, "close-multiple").await?.click().await?;
    option(page, "Dog").await?.click().await?;
    log(page, "close-multiple")
        .await?
        .wait_for_inner_text("dog")
        .await?;
    page.wait_for_count(role(AriaRole::Listbox), 0).await?;
    Ok(())
}

/// In the open popover, Space selects the focused option, closes and returns focus to the
/// trigger ("can select items with the Space key").
#[browser_test]
pub async fn space_selects_in_the_popover(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["empty"]).await?;
    let trigger = tab_to_trigger(page, "empty").await?;
    page.send_keys(Key::Down).await?;
    page.wait_for_focus(&option(page, "Cat").await?).await?;
    page.send_keys(Key::Down).await?;
    page.wait_for_focus(&option(page, "Dog").await?).await?;
    page.send_keys(" ").await?;
    log(page, "empty").await?.wait_for_inner_text("dog").await?;
    page.wait_for_count(role(AriaRole::Listbox), 0).await?;
    page.wait_for_focus(&trigger).await?;
    trigger.wait_for_inner_text("Dog").await?;
    Ok(())
}

/// Hovering an option focuses it; arrow keys continue from there and Enter selects ("focuses
/// items on hover").
#[browser_test]
pub async fn hover_focuses_options(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["empty"]).await?;
    trigger(page, "empty").await?.click().await?;
    let kangaroo = option(page, "Kangaroo").await?;
    kangaroo.hover().await?;
    page.wait_for_focus(&kangaroo).await?;
    page.send_keys(Key::Up).await?;
    page.wait_for_focus(&option(page, "Dog").await?).await?;
    page.send_keys(Key::Enter).await?;
    log(page, "empty").await?.wait_for_inner_text("dog").await?;
    Ok(())
}

/// Typing in the open popover focuses the matching option, and Enter selects it ("supports type
/// to select").
#[browser_test]
pub async fn type_ahead_in_the_popover(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["typeahead"]).await?;
    let trigger = trigger(page, "typeahead").await?;
    trigger.click().await?;
    page.wait_for_focus(&page.element(role(AriaRole::Listbox)).await?)
        .await?;
    page.type_text("foo baz").await?;
    page.wait_for_focus(&option(page, "Foo Baz").await?).await?;
    page.send_keys(Key::Enter).await?;
    log(page, "typeahead")
        .await?
        .wait_for_inner_text("Foo Baz")
        .await?;
    trigger.wait_for_inner_text("Foo Baz").await?;
    Ok(())
}

/// Pressing the selected option closes the popover and keeps the value without reporting a
/// change ("does not deselect when pressing an already selected item").
#[browser_test]
pub async fn pressing_the_selected_option_keeps_it(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["empty"]).await?;
    let trigger = trigger(page, "empty").await?;
    let log = log(page, "empty").await?;
    trigger.click().await?;
    option(page, "Cat").await?.click().await?;
    log.wait_for_inner_text("cat").await?;
    page.wait_for_count(role(AriaRole::Listbox), 0).await?;
    trigger.click().await?;
    option(page, "Cat").await?.click().await?;
    page.wait_for_count(role(AriaRole::Listbox), 0).await?;
    log.inner_text_stays("cat", STAYS).await?;
    trigger.wait_for_inner_text("Cat").await?;
    Ok(())
}

/// On the closed trigger, ArrowRight without a value selects the first option, the arrow keys
/// stop at the ends without reporting a change, and a held arrow key keeps moving ("move
/// selection on Arrow-Left/Right", "should support arrow key navigation to a falsy key", "should
/// support repeat keydown events when holding an arrow key").
#[browser_test]
pub async fn trigger_arrow_keys(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["empty"]).await?;
    let trigger = tab_to_trigger(page, "empty").await?;
    let log = log(page, "empty").await?;
    page.send_keys(Key::Right).await?;
    log.wait_for_inner_text("cat").await?;
    page.send_keys(Key::Left).await?;
    log.inner_text_stays("cat", STAYS).await?;
    page.hold_key("ArrowRight", 1).await?;
    log.wait_for_inner_text("cat|dog|kangaroo").await?;
    page.send_keys(Key::Right).await?;
    log.inner_text_stays("cat|dog|kangaroo", STAYS).await?;
    trigger.wait_for_inner_text("Kangaroo").await?;
    assert_that!(trigger)
        .has_attribute("aria-expanded")
        .await
        .is_equal_to("false");
    Ok(())
}

/// A fixed value reports the option picked but keeps showing its own; a fixed multiple value is
/// listed and its options are selected ("supports controlled selection", "should support
/// controlled multi-selection").
#[browser_test]
pub async fn fixed_values(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["controlled", "controlled-multiple"])
        .await?;
    let single = trigger(page, "controlled").await?;
    assert_that!(single).inner_text().await.is_equal_to("Dog");
    single.click().await?;
    option(page, "Cat").await?.click().await?;
    log(page, "controlled")
        .await?
        .wait_for_inner_text("cat")
        .await?;
    single.inner_text_stays("Dog", STAYS).await?;

    let multiple = trigger(page, "controlled-multiple").await?;
    assert_that!(multiple)
        .inner_text()
        .await
        .is_equal_to("Dog and Kangaroo");
    multiple.click().await?;
    assert_that!(option(page, "Dog").await?)
        .has_attribute("aria-selected")
        .await
        .is_equal_to("true");
    assert_that!(option(page, "Kangaroo").await?)
        .has_attribute("aria-selected")
        .await
        .is_equal_to("true");
    assert_that!(option(page, "Cat").await?)
        .has_attribute("aria-selected")
        .await
        .is_equal_to("false");
    Ok(())
}

/// Focus entering and leaving the closed select is reported, but opening the popover,
/// selecting and returning to the trigger is not ("calls onBlur and onFocus for the closed
/// Picker", "does not call blur when an item is selected").
#[browser_test]
pub async fn focus_changes(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["focus"]).await?;
    let trigger = tab_to_trigger(page, "focus").await?;
    let log = log(page, "focus").await?;
    log.wait_for_inner_text("true").await?;
    page.send_keys(Key::Enter).await?;
    page.wait_for_focus(&option(page, "Cat").await?).await?;
    page.send_keys(Key::Enter).await?;
    page.wait_for_focus(&trigger).await?;
    log.inner_text_stays("true", STAYS).await?;
    page.send_keys(Key::Tab).await?;
    log.wait_for_inner_text("true|false").await?;
    Ok(())
}

/// The hidden native select belongs to the form named by `form`, carries the autocomplete hint,
/// is out of the tab order, starts with an empty option with a non-empty label, and its container
/// tells accessibility linters that it is hidden on purpose ("should support form prop", "should
/// have a hidden select element for form autocomplete", "should include a non-empty placeholder
/// option for native select markup", "should always add a data attribute
/// data-a11y-ignore="aria-hidden-focus"").
#[browser_test]
pub async fn hidden_select_markup(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["form"]).await?;
    let select = page.element("#form select").await?;
    assert_that!(select)
        .has_attribute("form")
        .await
        .is_equal_to("sb-outer-form");
    assert_that!(select)
        .has_attribute("autocomplete")
        .await
        .is_equal_to("off");
    assert_that!(select)
        .has_attribute("tabindex")
        .await
        .is_equal_to("-1");
    let empty = select.first_element("option").await?;
    assert_that!(empty)
        .has_attribute("value")
        .await
        .is_equal_to("");
    assert_that!(empty)
        .has_attribute("label")
        .await
        .is_equal_to("\u{A0}");
    assert_that!(page.element("#form [data-a11y-ignore]").await?)
        .has_attribute("data-a11y-ignore")
        .await
        .is_equal_to("aria-hidden-focus");
    let form = page.element("#sb-outer-form").await?;
    assert_that!(form.form_values("animal").await?).contains_exactly(["cat"]);
    Ok(())
}

/// With native validation, a value the `validate` function rejects shows its error once the
/// form is submitted, a valid value clears it, and resetting the form clears the error ("supports
/// validate function", "clears validation on reset").
#[browser_test]
pub async fn native_validate_function(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["validate"]).await?;
    let trigger = trigger(page, "validate-form").await?;
    let error = ".leptonic-FieldError";
    assert_that!(page.count(error).await?).is_equal_to(0);
    page.element("#validate-submit").await?.click().await?;
    page.element(error)
        .await?
        .wait_for_inner_text("Dogs are not allowed")
        .await?;
    page.wait_for_focus(&trigger).await?;
    page.send_keys(Key::Left).await?;
    page.wait_for_count(error, 0).await?;
    page.send_keys(Key::Right).await?;
    page.element("#validate-submit").await?.click().await?;
    page.element(error).await?;
    page.element("#validate-reset").await?.click().await?;
    page.wait_for_count(error, 0).await?;
    Ok(())
}

/// With ARIA validation, a rejected value shows its error right away, and selecting a valid
/// value clears it ("supports validate function" with validationBehavior="aria").
#[browser_test]
pub async fn aria_validate_function(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["validate-aria"]).await?;
    let trigger = trigger(page, "validate-aria-form").await?;
    page.element(".leptonic-FieldError")
        .await?
        .wait_for_inner_text("Dogs are not allowed")
        .await?;
    assert_that!(trigger)
        .accessible_description()
        .await
        .is_equal_to("Dogs are not allowed");
    trigger.click().await?;
    option(page, "Cat").await?.click().await?;
    page.wait_for_count(".leptonic-FieldError", 0).await?;
    Ok(())
}

/// A server error for the select's name is shown until the value changes ("supports server
/// validation").
#[browser_test]
pub async fn server_validation(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["server"]).await?;
    let trigger = trigger(page, "server-form").await?;
    page.element(".leptonic-FieldError")
        .await?
        .wait_for_inner_text("Server says no")
        .await?;
    assert_that!(trigger)
        .accessible_description()
        .await
        .is_equal_to("Server says no");
    trigger.click().await?;
    option(page, "Cat").await?.click().await?;
    page.wait_for_count(".leptonic-FieldError", 0).await?;
    Ok(())
}

/// A select opened by default shows its popover with focus on the listbox and closes on
/// Escape ("supports default open state", "closes in default open state").
#[browser_test]
pub async fn default_open(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["default-open"]).await?;
    let listbox = page.element(role(AriaRole::Listbox)).await?;
    page.wait_for_focus(&listbox).await?;
    page.send_keys(Key::Escape).await?;
    page.wait_for_count(role(AriaRole::Listbox), 0).await?;
    Ok(())
}

/// A select whose open state is fixed reports Escape's close request but stays open ("does not
/// close in controlled open state").
#[browser_test]
pub async fn fixed_open_state(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["fixed-open"]).await?;
    let listbox = page.element(role(AriaRole::Listbox)).await?;
    page.wait_for_focus(&listbox).await?;
    page.send_keys(Key::Escape).await?;
    log(page, "fixed-open")
        .await?
        .wait_for_inner_text("false")
        .await?;
    page.count_stays(role(AriaRole::Listbox), 1, STAYS).await?;
    Ok(())
}

/// Sections are groups named by their headings, options are named and described by their label
/// and description, and arrow keys cross into the next section ("supports sections and complex
/// items").
#[browser_test]
pub async fn sections_and_complex_options(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["sections"]).await?;
    let trigger = tab_to_trigger_without_button(page, "sections").await?;
    page.send_keys(Key::Down).await?;
    page.wait_for_focus(&option(page, "Cat").await?).await?;
    let mut group_names = Vec::new();
    for group in page.elements(role(AriaRole::Group)).await? {
        group_names.push(group.accessible_name().await?);
    }
    assert_that!(group_names).contains_exactly(["Mammals", "Marsupials"]);
    let kangaroo = option(page, "KangarooJumps").await?;
    assert_that!(kangaroo)
        .accessible_name()
        .await
        .is_equal_to("Kangaroo");
    assert_that!(kangaroo)
        .accessible_description()
        .await
        .is_equal_to("Jumps");
    page.send_keys(Key::Down).await?;
    page.send_keys(Key::Down).await?;
    page.wait_for_focus(&kangaroo).await?;
    page.send_keys(Key::Enter).await?;
    log(page, "sections")
        .await?
        .wait_for_inner_text("kangaroo")
        .await?;
    trigger.wait_for_inner_text("Kangaroo").await?;
    Ok(())
}

/// Focuses `#<section>`'s trigger directly (sections without buttons around them).
async fn tab_to_trigger_without_button(
    page: &Page<'_>,
    section: &str,
) -> Result<WebElement, Report> {
    let trigger = trigger(page, section).await?;
    trigger.focus().await?;
    page.wait_for_focus(&trigger).await?;
    Ok(trigger)
}

/// Opening a popover whose selected option is far down scrolls it into view ("scrolls the
/// selected item into view on menu open").
#[browser_test]
pub async fn opening_scrolls_to_the_selected_option(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["scroll"]).await?;
    trigger(page, "scroll").await?.click().await?;
    let selected = option(page, "Option 40").await?;
    page.wait_for_focus(&selected).await?;
    let listbox = page.element(role(AriaRole::Listbox)).await?;
    // The option's distances from the listbox's visible top and bottom edges.
    assert_that!(|| async {
        let (option, listbox) = (selected.client_rect().await?, listbox.client_rect().await?);
        Ok::<_, Report>((option.top - listbox.top, listbox.bottom - option.bottom))
    })
    .eventually_ok()
    .satisfies(|distances| {
        distances
            .derive(|(above, _)| above)
            .is_greater_or_equal_to(-1.0);
        distances
            .derive(|(_, below)| below)
            .is_greater_or_equal_to(-1.0);
    })
    .await;
    Ok(())
}
