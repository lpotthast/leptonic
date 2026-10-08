// Upstream: react-aria-components/test/Slider.test.js @ 99e6102368
// Upstream: react-aria/test/slider/useSliderThumb.test.js @ 99e6102368
//! The slider atoms: a labelled group of range inputs (min, max, step, value, value text), the
//! output, keyboard changes (arrows, Shift+arrows, PageUp/PageDown, Home/End) ending in
//! `on_change_end`, thumbs bounded by their neighbors, track presses, dragging, vertical and
//! disabled sliders, the fill, and value tooltips.
//!
//! Presses and drags on the track (react-aria's `useSlider.test.js`, "interactions on track using
//! pointerEvents"): the closest thumb moves to the press and follows the pointer, stacked thumbs,
//! disabled sliders, vertical and right-to-left tracks.
//!
//! The thumbs' keyboard (react-aria's `useSliderThumb.test.js`, "using KeyEvents"; react-aria-
//! components' `Slider.test.js`), labels and attributes.
//!
//! Several thumbs (react-aria-components' `Slider.test.js`): three thumbs with an output, thumbs
//! bound to app state, bound values out of the range, a thumb without a value.
use assertr::prelude::*;
use browser_test::thirtyfour::prelude::*;
use rootcause::Report;

use crate::{
    pages::{ElementActions, Page, PageActions},
    polling::{expect, wait_for},
};

// -- Helpers --------------------------------------------------------------------------------------

const INTERACTIONS: &str = "/atoms/slider-interactions";

/// The thumbs' range inputs of the slider matching the CSS `slider`.
async fn inputs_in(page: &Page<'_>, slider: &str) -> Result<Vec<WebElement>, Report> {
    page.elements(format!("{slider} input[type=range]")).await
}

/// The thumbs' range inputs of the slider named `name` (its `aria-label`).
async fn inputs_of(page: &Page<'_>, name: &str) -> Result<Vec<WebElement>, Report> {
    inputs_in(page, &format!("[role=group][aria-label='{name}']")).await
}

/// The `value` properties of `inputs`.
async fn values_of(inputs: &[WebElement]) -> Result<Vec<String>, Report> {
    let mut values = Vec::new();
    for input in inputs {
        values.push(input.value().await?.unwrap_or_default());
    }
    Ok(values)
}

/// The log of the slider named `name`.
async fn log_of(page: &Page<'_>, name: &str) -> Result<Vec<String>, Report> {
    let text = page
        .element(format!("#{name}-log"))
        .await?
        .inner_text()
        .await?;
    Ok(text
        .split(';')
        .filter(|entry| !entry.is_empty())
        .map(str::to_owned)
        .collect())
}

/// The last log entry of the slider named `name` starting with `kind` (`change` or `end`).
async fn last_of(page: &Page<'_>, name: &str, kind: &str) -> Result<Option<String>, Report> {
    Ok(log_of(page, name)
        .await?
        .into_iter()
        .rev()
        .find(|entry| entry.starts_with(&format!("{kind}:"))))
}

/// Waits until the last log entry of the slider named `name` starting with `kind` (`change` or
/// `end`) is `kind:expected` (dragging logs a change per pointer move).
async fn wait_for_last(
    page: &Page<'_>,
    name: &str,
    kind: &str,
    expected: &str,
) -> Result<(), Report> {
    wait_for(format!("the last {kind} of {name}"))
        .observing(|| last_of(page, name, kind))
        .to_be_equal_to(Some(format!("{kind}:{expected}")))
        .await?;
    Ok(())
}

/// The number of `end` entries in the log of the slider named `name`.
async fn ends_of(page: &Page<'_>, name: &str) -> Result<usize, Report> {
    Ok(log_of(page, name)
        .await?
        .iter()
        .filter(|entry| entry.starts_with("end:"))
        .count())
}

/// Scrolls the track of the slider named `name` into view (pointer actions don't scroll) and
/// returns it.
async fn track_of(page: &Page<'_>, name: &str) -> Result<WebElement, Report> {
    let track = page.element(format!("#{name}-track")).await?;
    track.scroll_into_view().await?;
    Ok(track)
}

/// Presses (down and up) the 200 × 20 pixel track of `name` at `x` pixels from its start.
async fn click_track(page: &Page<'_>, name: &str, x: i64) -> Result<(), Report> {
    let track = track_of(page, name).await?;
    page.driver
        .action_chain()
        .move_to_element_with_offset(&track, x - 100, 0)
        .click()
        .perform()
        .await?;
    Ok(())
}

// -- SliderTests ----------------------------------------------------------------------------------

/// A labelled group; the thumb's input is named by the label and has the range's attributes.
pub async fn labelled_group(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/slider").await?;
    let group = page.element("#test-slider-volume").await?;
    assert_that!(group.attr("role").await?)
        .get_some()
        .is_equal_to("group");
    let label_id = group.attr("aria-labelledby").await?.unwrap_or_default();
    assert_that!(group.referenced_text("aria-labelledby").await?).is_equal_to("Volume");
    let volume = inputs_in(page, "#test-slider-volume").await?.remove(0);
    assert_that!(volume.attr("min").await?)
        .get_some()
        .is_equal_to("0");
    assert_that!(volume.attr("max").await?)
        .get_some()
        .is_equal_to("100");
    assert_that!(volume.attr("step").await?)
        .get_some()
        .is_equal_to("5");
    assert_that!(volume.value().await?)
        .get_some()
        .is_equal_to("30");
    assert_that!(volume.attr("aria-valuetext").await?)
        .get_some()
        .is_equal_to("30");
    assert_that!(volume.attr("aria-labelledby").await?)
        .get_some()
        .contains(&label_id);
    let output = page.element("#test-slider-volume-output").await?;
    assert_that!(output.inner_text().await?).is_equal_to("30");
    let volume_id = volume.id().await?;
    assert_that!(output.attr("for").await?).is_equal_to(volume_id);
    Ok(())
}

/// The fill runs from the offset (50) to the thumb (30).
pub async fn fill(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/slider").await?;
    let fill = page.element("#test-slider-volume-fill").await?;
    assert_that!(fill.css_value("width").await?).is_equal_to("40px");
    Ok(())
}

/// Clicking the label focuses the thumb; the keyboard changes the value by the step, Shift by a
/// page, PageUp/PageDown by a page, Home/End to the ends. Each change ends.
pub async fn keyboard(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/slider").await?;
    let group = page.element("#test-slider-volume").await?;
    let label_id = group.attr("aria-labelledby").await?.unwrap_or_default();
    let volume = inputs_in(page, "#test-slider-volume").await?.remove(0);
    let output = page.element("#test-slider-volume-output").await?;
    let ends = page.element("#test-slider-volume-ends").await?;
    page.element(format!("#{label_id}")).await?.click().await?;
    page.wait_for_focus(&volume).await?;
    page.send_keys(Key::Right).await?;
    output.wait_for_inner_text("35").await?;
    ends.wait_for_inner_text("1").await?;
    // The arrows of the other axis step too (useSliderThumb's `onMove`): up increments, down
    // decrements.
    page.send_keys(Key::Up).await?;
    output.wait_for_inner_text("40").await?;
    page.send_keys(Key::Down).await?;
    output.wait_for_inner_text("35").await?;
    page.send_keys(Key::Shift + Key::Right).await?;
    output.wait_for_inner_text("45").await?;
    page.send_keys(Key::PageDown).await?;
    output.wait_for_inner_text("35").await?;
    page.send_keys(Key::End).await?;
    output.wait_for_inner_text("100").await?;
    page.send_keys(Key::Home).await?;
    output.wait_for_inner_text("0").await?;
    ends.wait_for_inner_text("7").await?;
    Ok(())
}

/// "should support clicking on the track to move the thumb".
pub async fn track_click(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/slider").await?;
    let track = page.element("#test-slider-volume-track").await?;
    page.driver
        .action_chain()
        .move_to_element_with_offset(&track, 50, 0)
        .click()
        .perform()
        .await?;
    page.element("#test-slider-volume-output")
        .await?
        .wait_for_inner_text("75")
        .await?;
    Ok(())
}

/// "should support dragging state": the thumb follows the pointer (from 30, 50px on the 200px
/// track are 25).
pub async fn dragging_state(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/slider").await?;
    let thumb = page
        .element("#test-slider-volume .test-slider-thumb")
        .await?;
    page.driver
        .action_chain()
        .click_and_hold_element(&thumb)
        .move_by_offset(-50, 0)
        .perform()
        .await?;
    page.element("#test-slider-volume .test-slider-thumb[data-dragging]")
        .await?;
    page.driver.action_chain().release().perform().await?;
    page.element("#test-slider-volume-output")
        .await?
        .wait_for_inner_text("5")
        .await?;
    page.wait_for_count("#test-slider-volume .test-slider-thumb[data-dragging]", 0)
        .await?;
    Ok(())
}

/// "should support two thumbs": each is bounded by the other; the output shows both.
pub async fn two_thumbs(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/slider").await?;
    let price = page.element("[role=group][aria-label=Price]").await?;
    let thumbs = inputs_in(page, "[aria-label=Price]").await?;
    let (minimum, maximum) = (&thumbs[0], &thumbs[1]);
    assert_that!(minimum.attr("max").await?)
        .get_some()
        .is_equal_to("80");
    assert_that!(maximum.attr("min").await?)
        .get_some()
        .is_equal_to("20");
    page.element("#test-slider-price-output")
        .await?
        .wait_for_inner_text("20 \u{2013} 80")
        .await?;
    minimum.focus().await?;
    page.send_keys(Key::End).await?;
    page.element("#test-slider-price")
        .await?
        .wait_for_inner_text("[80, 80]")
        .await?;
    assert_that!(price.attr("data-disabled").await?).is_none();
    Ok(())
}

/// "should support orientation": vertical arrows; "can be moved with keys (vertical)", and the
/// arrows of the other axis.
pub async fn orientation(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/slider").await?;
    let vertical = inputs_in(page, "[aria-label=Vertical]").await?.remove(0);
    assert_that!(vertical.attr("aria-orientation").await?)
        .get_some()
        .is_equal_to("vertical");
    vertical.focus().await?;
    page.send_keys(Key::Up).await?;
    vertical.wait_for_prop("value", "51").await?;
    page.send_keys(Key::Down).await?;
    vertical.wait_for_prop("value", "50").await?;
    page.send_keys(Key::Right).await?;
    vertical.wait_for_prop("value", "51").await?;
    page.send_keys(Key::Left).await?;
    vertical.wait_for_prop("value", "50").await?;
    Ok(())
}

/// "should support disabled state".
pub async fn disabled_state(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/slider").await?;
    let disabled = page.element("[role=group][aria-label=Disabled]").await?;
    assert_that!(disabled.attr("data-disabled").await?).is_some();
    let disabled_input = inputs_in(page, "[aria-label=Disabled]").await?.remove(0);
    assert_that!(disabled_input.is_enabled().await?).is_false();
    Ok(())
}

/// Tooltips: always visible, or while hovered.
pub async fn tooltips(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/slider").await?;
    let always = page.element("[aria-label=Always] .tooltip").await?;
    assert_that!(always.attr("data-visible").await?).is_some();
    assert_that!(always.inner_text().await?).is_equal_to("30");
    let hover_thumb = page
        .element("[aria-label='On hover'] .test-slider-thumb")
        .await?;
    hover_thumb.scroll_into_view().await?;
    hover_thumb.hover().await?;
    page.element("[aria-label='On hover'] .tooltip[data-visible]")
        .await?;
    Ok(())
}

// -- SliderTrackTests -----------------------------------------------------------------------------

/// "should allow you to set value of closest thumb by clicking on track".
pub async fn closest_thumb_by_click(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(INTERACTIONS).await?;
    let log = page.element("#track-log").await?;
    click_track(page, "track", 40).await?;
    log.wait_for_inner_text("change:[20, 80];end:[20, 80]")
        .await?;
    click_track(page, "track", 180).await?;
    log.wait_for_inner_text("change:[20, 80];end:[20, 80];change:[20, 90];end:[20, 90]")
        .await?;
    Ok(())
}

/// "... by dragging on track": changes while dragging, the end on release.
pub async fn closest_thumb_by_drag(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(INTERACTIONS).await?;
    let track = track_of(page, "drag").await?;
    page.driver
        .action_chain()
        .move_to_element_with_offset(&track, 40 - 100, 0)
        .click_and_hold()
        .perform()
        .await?;
    wait_for_last(page, "drag", "change", "[20, 80]").await?;
    page.driver
        .action_chain()
        .move_by_offset(20, 0)
        .perform()
        .await?;
    wait_for_last(page, "drag", "change", "[30, 80]").await?;
    page.driver
        .action_chain()
        .move_by_offset(20, 0)
        .perform()
        .await?;
    wait_for_last(page, "drag", "change", "[40, 80]").await?;
    expect("the ends of drag")
        .observing(|| ends_of(page, "drag"))
        .to_stay_equal_to(0)
        .await?;
    page.driver.action_chain().release().perform().await?;
    wait_for_last(page, "drag", "end", "[40, 80]").await?;
    expect("the ends of drag")
        .observing(|| ends_of(page, "drag"))
        .to_stay_equal_to(1)
        .await?;
    Ok(())
}

/// "... before thumbs when thumbs stacked", "... after thumbs when thumbs stacked".
pub async fn stacked_thumbs(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(INTERACTIONS).await?;
    click_track(page, "stacked-before", 40).await?;
    wait_for_last(page, "stacked-before", "end", "[20, 40]").await?;
    click_track(page, "stacked-after", 120).await?;
    wait_for_last(page, "stacked-after", "end", "[40, 60]").await?;
    Ok(())
}

/// "... before thumbs when many thumbs and stacked", "... after thumbs when many thumbs and
/// stacked".
pub async fn many_stacked_thumbs(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(INTERACTIONS).await?;
    click_track(page, "many-before", 140).await?;
    wait_for_last(page, "many-before", "end", "[25, 25, 50, 70, 75]").await?;
    click_track(page, "many-before", 40).await?;
    wait_for_last(page, "many-before", "end", "[20, 25, 50, 70, 75]").await?;
    click_track(page, "many-after", 160).await?;
    wait_for_last(page, "many-after", "end", "[25, 25, 50, 75, 80]").await?;
    click_track(page, "many-after", 60).await?;
    wait_for_last(page, "many-after", "end", "[25, 30, 50, 75, 80]").await?;
    Ok(())
}

/// "should not allow you to set value if disabled".
pub async fn disabled_track(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(INTERACTIONS).await?;
    let track = track_of(page, "disabled").await?;
    page.driver
        .action_chain()
        .move_to_element_with_offset(&track, 40 - 100, 0)
        .click_and_hold()
        .move_by_offset(20, 0)
        .release()
        .perform()
        .await?;
    expect("the log of disabled")
        .observing(|| log_of(page, "disabled"))
        .to_stay_equal_to(Vec::<String>::new())
        .await?;
    let disabled = inputs_of(page, "disabled").await?;
    assert_that!(disabled[0].value().await?)
        .get_some()
        .is_equal_to("10");
    Ok(())
}

/// "... by dragging on track (vertical)": the value grows upwards.
pub async fn vertical_drag(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(INTERACTIONS).await?;
    let track = track_of(page, "vertical").await?;
    page.driver
        .action_chain()
        .move_to_element_with_offset(&track, 0, 160 - 100)
        .click_and_hold()
        .perform()
        .await?;
    wait_for_last(page, "vertical", "change", "[20, 80]").await?;
    page.driver
        .action_chain()
        .move_by_offset(0, -20)
        .perform()
        .await?;
    wait_for_last(page, "vertical", "change", "[30, 80]").await?;
    page.driver
        .action_chain()
        .move_by_offset(0, -20)
        .perform()
        .await?;
    wait_for_last(page, "vertical", "change", "[40, 80]").await?;
    page.driver.action_chain().release().perform().await?;
    wait_for_last(page, "vertical", "end", "[40, 80]").await?;
    Ok(())
}

/// Right to left: the track starts on the right; the arrow keys follow the reading direction
/// (Left increases).
pub async fn right_to_left(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(INTERACTIONS).await?;
    click_track(page, "rtl", 140).await?;
    wait_for_last(page, "rtl", "end", "[30, 80]").await?;
    let thumb = page.element("#rtl-track .thumb").await?;
    let style = thumb
        .attr("style")
        .await?
        .unwrap_or_default()
        .replace(' ', "");
    assert_that!(style).contains("left:70%");
    let rtl = inputs_of(page, "rtl").await?;
    rtl[0].focus().await?;
    page.send_keys(Key::Left).await?;
    rtl[0].wait_for_prop("value", "31").await?;
    page.send_keys(Key::Right).await?;
    rtl[0].wait_for_prop("value", "30").await?;
    Ok(())
}

// -- SliderThumbTests -----------------------------------------------------------------------------

/// "can be moved with keys": each key changes and ends. All four arrows step a horizontal
/// slider: Up increases, Down decreases.
pub async fn keys(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(INTERACTIONS).await?;
    let keys = inputs_of(page, "keys").await?;
    let log = page.element("#keys-log").await?;
    keys[0].focus().await?;
    page.send_keys(Key::Right).await?;
    log.wait_for_inner_text("change:[11];end:[11]").await?;
    page.send_keys(Key::Left).await?;
    log.wait_for_inner_text("change:[11];end:[11];change:[10];end:[10]")
        .await?;
    page.send_keys(Key::Up).await?;
    keys[0].wait_for_prop("value", "11").await?;
    page.send_keys(Key::Down).await?;
    keys[0].wait_for_prop("value", "10").await?;
    Ok(())
}

/// "can be moved with keys (vertical)": Right and Up increase, Down and Left decrease.
pub async fn keys_vertical(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(INTERACTIONS).await?;
    let vertical = inputs_of(page, "keys-vertical").await?;
    vertical[0].focus().await?;
    page.send_keys(Key::Right).await?;
    wait_for_last(page, "keys-vertical", "change", "[11]").await?;
    page.send_keys(Key::Up).await?;
    wait_for_last(page, "keys-vertical", "change", "[12]").await?;
    page.send_keys(Key::Down).await?;
    wait_for_last(page, "keys-vertical", "change", "[11]").await?;
    page.send_keys(Key::Left).await?;
    wait_for_last(page, "keys-vertical", "change", "[10]").await?;
    vertical[0].wait_for_prop("value", "10").await?;
    Ok(())
}

/// "should support repeat keydown events when holding Page Up/Page Down".
pub async fn repeated_page_keys(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(INTERACTIONS).await?;
    let paged = inputs_of(page, "page").await?;
    paged[0].focus().await?;
    // A keydown and two repeats: three pages of 10.
    page.hold_key("PageUp", 2).await?;
    paged[0].wait_for_prop("value", "50").await?;
    page.hold_key("PageDown", 2).await?;
    paged[0].wait_for_prop("value", "20").await?;
    Ok(())
}

/// The `input` event (assistive technology sets the value): the state follows, and the property
/// keeps following the state afterwards.
pub async fn input_event(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(INTERACTIONS).await?;
    let keys = inputs_of(page, "keys").await?;
    keys[0].virtual_input("42").await?;
    wait_for_last(page, "keys", "change", "[42]").await?;
    keys[0].focus().await?;
    page.send_keys(Key::Right).await?;
    keys[0].wait_for_prop("value", "43").await?;
    Ok(())
}

/// A disabled thumb: not changed by track presses near it, its input disabled.
pub async fn disabled_thumb(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(INTERACTIONS).await?;
    let thumb_disabled = inputs_of(page, "thumb-disabled").await?;
    assert_that!(thumb_disabled[0].is_enabled().await?).is_true();
    assert_that!(thumb_disabled[1].is_enabled().await?).is_false();
    click_track(page, "thumb-disabled", 180).await?;
    thumb_disabled[1].prop_stays("value", "80").await?;
    click_track(page, "thumb-disabled", 40).await?;
    thumb_disabled[0].wait_for_prop("value", "20").await?;
    Ok(())
}

/// "should support form prop".
pub async fn form_prop(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(INTERACTIONS).await?;
    let form = inputs_of(page, "form").await?;
    assert_that!(form[0].attr("form").await?)
        .get_some()
        .is_equal_to("test-form");
    assert_that!(form[0].attr("name").await?)
        .get_some()
        .is_equal_to("volume");
    Ok(())
}

/// Labels ("should have the right labels with Slider thumb label", "... thumb aria-label"): a
/// thumb's own `Label` labels its input (`for`), before the slider's.
pub async fn thumb_labels(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(INTERACTIONS).await?;
    let group = page.element("#labels").await?;
    let slider_label = group.attr("aria-labelledby").await?.unwrap_or_default();
    let labels = inputs_in(page, "[role=group]#labels").await?;
    let min_id = labels[0].id().await?;
    let min_label = page.element("#labels .leptonic-SliderThumb label").await?;
    assert_that!(min_label.inner_text().await?).is_equal_to("Min");
    assert_that!(min_label.attr("for").await?).is_equal_to(min_id);
    let min_label_id = min_label.id().await?.unwrap_or_default();
    labels[0]
        .wait_for_attr(
            "aria-labelledby",
            Some(&format!("{min_label_id} {slider_label}")),
        )
        .await?;
    let max_id = labels[1].id().await?.unwrap_or_default();
    assert_that!(labels[1].attr("aria-label").await?)
        .get_some()
        .is_equal_to("Max");
    labels[1]
        .wait_for_attr("aria-labelledby", Some(&format!("{max_id} {slider_label}")))
        .await?;
    // The slider's label is a different element with a different id.
    assert_that!(slider_label.as_str()).is_not_equal_to(min_label_id.as_str());
    assert_that!(group.referenced_text("aria-labelledby").await?).is_equal_to("Price");
    Ok(())
}

/// Attributes: exact decimals (no `f64` noise), required, invalid, error message and details.
pub async fn attributes(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(INTERACTIONS).await?;
    let attributes = inputs_of(page, "attributes").await?;
    let input = &attributes[0];
    assert_that!(input.attr("step").await?)
        .get_some()
        .is_equal_to("0.1");
    assert_that!(input.attr("value").await?)
        .get_some()
        .is_equal_to("0.1");
    assert_that!(input.attr("aria-required").await?)
        .get_some()
        .is_equal_to("true");
    assert_that!(input.attr("aria-invalid").await?)
        .get_some()
        .is_equal_to("true");
    assert_that!(input.attr("aria-errormessage").await?)
        .get_some()
        .is_equal_to("attributes-error");
    assert_that!(input.attr("aria-details").await?)
        .get_some()
        .is_equal_to("attributes-details");
    input.focus().await?;
    page.send_keys(Key::Right).await?;
    input.wait_for_prop("value", "0.2").await?;
    assert_that!(input.attr("value").await?)
        .get_some()
        .is_equal_to("0.2");
    Ok(())
}

// -- SliderMultipleThumbsTests --------------------------------------------------------------------

/// "should support three thumbs".
pub async fn three_thumbs(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(INTERACTIONS).await?;
    let three = inputs_of(page, "three").await?;
    assert_that!(values_of(&three).await?).contains_exactly(["30", "60", "80"]);
    page.element("#three-output")
        .await?
        .wait_for_inner_text("30, 60, 80")
        .await?;
    Ok(())
}

/// "should support multiple thumbs (controlled)".
pub async fn controlled_thumbs(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(INTERACTIONS).await?;
    let controlled = inputs_of(page, "controlled").await?;
    assert_that!(values_of(&controlled).await?).contains_exactly(["30", "60"]);
    page.element("#controlled-reset").await?.click().await?;
    controlled[0].wait_for_prop("value", "0").await?;
    controlled[1].wait_for_prop("value", "100").await?;
    // From the element before the slider (react-aria-components' test: from the body).
    page.element("#controlled-before").await?.focus().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&controlled[0]).await?;
    for _ in 0..3 {
        page.send_keys(Key::Right).await?;
    }
    controlled[0].wait_for_prop("value", "3").await?;
    assert_that!(controlled[1].value().await?)
        .get_some()
        .is_equal_to("100");
    page.element("#controlled-reset").await?.click().await?;
    controlled[0].wait_for_prop("value", "0").await?;
    page.element("#controlled-before").await?.focus().await?;
    page.send_keys(Key::Tab).await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&controlled[1]).await?;
    for _ in 0..3 {
        page.send_keys(Key::Left).await?;
    }
    controlled[1].wait_for_prop("value", "97").await?;
    assert_that!(controlled[0].value().await?)
        .get_some()
        .is_equal_to("0");
    Ok(())
}

/// Bound values out of the range render within it (react-stately's `restrictValues`).
pub async fn restricted_values(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(INTERACTIONS).await?;
    let restricted = inputs_of(page, "restricted").await?;
    assert_that!(values_of(&restricted).await?).contains_exactly(["0", "100"]);
    page.element("#restricted-output")
        .await?
        .wait_for_inner_text("0 \u{2013} 100")
        .await?;
    Ok(())
}

/// A thumb without a value renders (at the minimum) instead of panicking.
pub async fn missing_value(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(INTERACTIONS).await?;
    let missing = inputs_of(page, "missing").await?;
    assert_that!(missing).has_length(2);
    assert_that!(missing[0].value().await?)
        .get_some()
        .is_equal_to("30");
    Ok(())
}
