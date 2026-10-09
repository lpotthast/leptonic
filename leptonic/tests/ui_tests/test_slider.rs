// Upstream: react-aria-components/test/Slider.test.js @ 99e6102368
// Upstream: react-aria/test/slider/useSlider.test.js @ 99e6102368
// Upstream: react-aria/test/slider/useSliderThumb.test.js @ 99e6102368
// Upstream: @adobe/react-spectrum/test/slider/Slider.test.tsx @ 99e6102368
// Upstream: @adobe/react-spectrum/test/slider/RangeSlider.test.tsx @ 99e6102368
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
use assertr::{matchers::eq, prelude::*};
use browser_test::{browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::pages::{ElementActions, Page, PointerKind, PointerType, SyntheticEvent};

// -- Helpers --------------------------------------------------------------------------------------

const PATH: &str = "/atoms/slider";
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
    assert_that!(|| last_of(page, name, kind))
        .with_subject_name(format!("the last {kind} of {name}"))
        .eventually_ok()
        .matches(eq(Some(format!("{kind}:{expected}"))))
        .await;
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
    page.low_level()
        .driver()
        .action_chain()
        .move_to_element_with_offset(&track, x - 100, 0)
        .click()
        .perform()
        .await?;
    Ok(())
}

// -- SliderTests ----------------------------------------------------------------------------------

/// The slider is a group named by its label; its thumb's input is labelled by it too, carries min,
/// max, step, value and value text, and the output shows the value for the input, unannounced
/// ("should have the right labels with Slider-level label", "supports label").
#[browser_test]
pub async fn labelled_group(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let group = page.element("#test-slider-volume").await?;
    assert_that!(group)
        .has_attribute("role")
        .await
        .is_equal_to("group");
    let label_id = group.attr("aria-labelledby").await?.unwrap_or_default();
    assert_that!(group)
        .accessible_name()
        .await
        .is_equal_to("Volume");
    let volume = inputs_in(page, "#test-slider-volume").await?.remove(0);
    assert_that!(volume)
        .has_attribute("min")
        .await
        .is_equal_to("0");
    assert_that!(volume)
        .has_attribute("max")
        .await
        .is_equal_to("100");
    assert_that!(volume)
        .has_attribute("step")
        .await
        .is_equal_to("5");
    assert_that!(volume)
        .property("value")
        .await
        .get_some()
        .is_equal_to("30");
    assert_that!(volume)
        .has_attribute("aria-valuetext")
        .await
        .is_equal_to("30");
    assert_that!(volume)
        .has_attribute("aria-labelledby")
        .await
        .is_equal_to(label_id);
    let output = page.element("#test-slider-volume-output").await?;
    assert_that!(output).inner_text().await.is_equal_to("30");
    let volume_id = volume.id().await?;
    assert_that!(output)
        .attribute("for")
        .await
        .is_equal_to(volume_id);
    assert_that!(output)
        .has_attribute("aria-live")
        .await
        .is_equal_to("off");
    assert_that!(output)
        .attribute("aria-labelledby")
        .await
        .is_none();
    Ok(())
}

/// An `aria_label` names the group, which then has no `aria-labelledby`; a thumb without a name
/// of its own is labelled by the group, one with an `aria-label` by itself and the group; the
/// output of two thumbs is for both inputs ("should have the right labels when setting
/// aria-label", "supports aria-label", "should have the right labels with Slider thumb
/// aria-label").
#[browser_test]
pub async fn aria_label(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let price = page.element("[role=group][aria-label=Price]").await?;
    assert_that!(price)
        .attribute("aria-labelledby")
        .await
        .is_none();
    let price_id = price.id().await?.unwrap_or_default();
    let thumbs = inputs_in(page, "[aria-label=Price]").await?;
    let ids = [
        thumbs[0].id().await?.unwrap_or_default(),
        thumbs[1].id().await?.unwrap_or_default(),
    ];
    for (thumb, id) in thumbs.iter().zip(&ids) {
        assert_that!(thumb)
            .has_attribute("aria-labelledby")
            .await
            .is_equal_to(format!("{id} {price_id}"));
    }
    let output = page.element("#test-slider-price-output").await?;
    assert_that!(output)
        .has_attribute("for")
        .await
        .is_equal_to(ids.join(" "));
    let vertical = page.element("[role=group][aria-label=Vertical]").await?;
    let vertical_id = vertical.id().await?;
    let vertical_input = inputs_in(page, "[aria-label=Vertical]").await?.remove(0);
    assert_that!(vertical_input)
        .attribute("aria-labelledby")
        .await
        .is_equal_to(vertical_id);
    assert_that!(vertical_input)
        .has_attribute("aria-valuetext")
        .await;
    Ok(())
}

/// The `left`/`width` (or `bottom`/`height`) and orientation of a fill, from its inline style.
async fn fill_box(page: &Page<'_>, id: &str) -> Result<(String, String), Report> {
    let fill = page.element(format!("#{id}")).await?;
    let style = fill
        .attr("style")
        .await?
        .unwrap_or_default()
        .replace(' ', "");
    let orientation = fill.attr("data-orientation").await?.unwrap_or_default();
    Ok((style, orientation))
}

/// A fill with offset 50 spans from the thumb at 30 to the offset (40px of the 200px track), and
/// from the offset to the thumb once it is above it ("should support horizontal SliderFill with
/// offset").
#[browser_test]
pub async fn fill(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let fill = page.element("#test-slider-volume-fill").await?;
    assert_that!(fill.css_value("width").await?).is_equal_to("40px");
    let (style, orientation) = fill_box(page, "test-slider-volume-fill").await?;
    assert_that!(style.as_str())
        .contains("position:absolute")
        .contains("inset-inline-start:30%")
        .contains("width:20%");
    assert_that!(orientation).is_equal_to("horizontal".to_owned());
    let volume = inputs_in(page, "#test-slider-volume").await?.remove(0);
    volume.virtual_input("80").await?;
    assert_that!(|| fill_box(page, "test-slider-volume-fill"))
        .eventually_ok()
        .satisfies(|fill| {
            fill.derive(|(style, _)| style)
                .contains("inset-inline-start:50%")
                .contains("width:30%");
        })
        .await;
    Ok(())
}

/// A fill without offset spans from the track's start to the thumb ("should support horizontal
/// SliderFill").
#[browser_test]
pub async fn fill_from_the_start(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let (style, orientation) = fill_box(page, "test-slider-always-fill").await?;
    assert_that!(style.as_str())
        .contains("inset-inline-start:0%")
        .contains("width:30%");
    assert_that!(orientation).is_equal_to("horizontal".to_owned());
    Ok(())
}

/// A vertical fill grows from the bottom: to the thumb without offset, between the offset and the
/// thumb with one ("should support vertical SliderFill", "should support vertical SliderFill with
/// offset").
#[browser_test]
pub async fn vertical_fill(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let (style, orientation) = fill_box(page, "test-slider-vertical-fill").await?;
    assert_that!(style.as_str())
        .contains("bottom:0%")
        .contains("height:50%");
    assert_that!(orientation).is_equal_to("vertical".to_owned());
    let (style, _) = fill_box(page, "test-slider-vertical-offset-fill").await?;
    assert_that!(style.as_str())
        .contains("bottom:30%")
        .contains("height:20%");
    let input = inputs_in(page, "[aria-label='Vertical offset']")
        .await?
        .remove(0);
    input.virtual_input("80").await?;
    assert_that!(|| fill_box(page, "test-slider-vertical-offset-fill"))
        .eventually_ok()
        .satisfies(|fill| {
            fill.derive(|(style, _)| style)
                .contains("bottom:50%")
                .contains("height:30%");
        })
        .await;
    Ok(())
}

/// Values are formatted with the slider's format options, in the value text and the output, also
/// a range ("supports setting custom formatOptions").
#[browser_test]
pub async fn format_options(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let percent = inputs_in(page, "[aria-label=Percent]").await?.remove(0);
    let output = page.element("[aria-label=Percent] output").await?;
    assert_that!(percent)
        .has_attribute("aria-valuetext")
        .await
        .is_equal_to("20%");
    assert_that!(output).inner_text().await.is_equal_to("20%");
    percent.virtual_input("0.5").await?;
    output.wait_for_inner_text("50%").await?;
    percent.wait_for_attr("aria-valuetext", Some("50%")).await?;
    let range = page.element("[aria-label='Percent range'] output").await?;
    assert_that!(range)
        .inner_text()
        .await
        .is_equal_to("20% \u{2013} 60%");
    Ok(())
}

/// Resetting the form restores the values sliders started with: bound ones (one thumb, two) and
/// own ones ("supports form reset").
#[browser_test]
pub async fn form_reset(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let bound = inputs_in(page, "[aria-label='Reset bound']")
        .await?
        .remove(0);
    let range = inputs_in(page, "[aria-label='Reset range']").await?;
    let own = inputs_in(page, "[aria-label='Reset own']").await?.remove(0);
    bound.virtual_input("55").await?;
    range[0].virtual_input("30").await?;
    range[1].virtual_input("60").await?;
    own.virtual_input("70").await?;
    bound.wait_for_prop("value", "55").await?;
    range[1].wait_for_prop("value", "60").await?;
    own.wait_for_prop("value", "70").await?;
    page.element("#test-slider-reset").await?.click().await?;
    bound.wait_for_prop("value", "10").await?;
    range[0].wait_for_prop("value", "10").await?;
    range[1].wait_for_prop("value", "40").await?;
    own.wait_for_prop("value", "10").await?;
    Ok(())
}

/// Clicking the label focuses the thumb, whose value the arrows change by the step, Shift+arrows
/// and PageUp/PageDown by a page and Home/End to the ends; every key calls `on_change_end`.
#[browser_test]
pub async fn keyboard(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
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

/// Clicking the track at three quarters of its width moves the thumb to 75 ("should support
/// clicking on the track to move the thumb").
#[browser_test]
pub async fn track_click(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let track = page.element("#test-slider-volume-track").await?;
    page.low_level()
        .driver()
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

/// A pressed thumb is marked `data-dragging` until released and follows the pointer: 50px to the
/// left on the 200px track move it from 30 to 5, ending the change once ("should support dragging
/// state", "can be moved by dragging").
#[browser_test]
pub async fn dragging_state(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let thumb = page
        .element("#test-slider-volume .test-slider-thumb")
        .await?;
    let ends = page.element("#test-slider-volume-ends").await?;
    assert_that!(thumb)
        .attribute("data-dragging")
        .await
        .is_none();
    let held = thumb.press_and_hold().await?;
    thumb.wait_for_attr("data-dragging", Some("true")).await?;
    held.move_by(-50, 0).await?;
    page.element("#test-slider-volume-output")
        .await?
        .wait_for_inner_text("5")
        .await?;
    assert_that!(ends).inner_text().await.is_equal_to("0");
    held.release().await?;
    thumb.wait_for_attr("data-dragging", None).await?;
    ends.wait_for_inner_text("1").await?;
    Ok(())
}

/// Tabbing to a thumb's input shows a focus ring on the thumb (`data-focused`,
/// `data-focus-visible`), which moves on with the focus ("should support focus ring").
#[browser_test]
pub async fn thumb_focus_ring(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(INTERACTIONS, &["track"]).await?;
    let thumbs = page.elements("#track-track .thumb").await?;
    page.element("#before").await?.focus().await?;
    page.send_keys(Key::Tab).await?;
    thumbs[0]
        .wait_for_attr("data-focus-visible", Some("true"))
        .await?;
    assert_that!(thumbs[0])
        .has_attribute("data-focused")
        .await
        .is_equal_to("true");
    page.send_keys(Key::Tab).await?;
    thumbs[1]
        .wait_for_attr("data-focus-visible", Some("true"))
        .await?;
    thumbs[0].wait_for_attr("data-focus-visible", None).await?;
    assert_that!(thumbs[0])
        .attribute("data-focused")
        .await
        .is_none();
    Ok(())
}

/// A thumb and its track are `data-hovered` while the pointer is over the thumb ("should support
/// hover state").
#[browser_test]
pub async fn thumb_hover(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let thumb = page
        .element("#test-slider-volume .test-slider-thumb")
        .await?;
    let track = page.element("#test-slider-volume-track").await?;
    assert_that!(thumb)
        .attribute("data-hovered")
        .await
        .is_none();
    thumb.hover().await?;
    thumb.wait_for_attr("data-hovered", Some("true")).await?;
    track.wait_for_attr("data-hovered", Some("true")).await?;
    page.element("#test-slider-price").await?.hover().await?;
    thumb.wait_for_attr("data-hovered", None).await?;
    track.wait_for_attr("data-hovered", None).await?;
    Ok(())
}

/// Each of two thumbs is bounded by the other, so End moves the first only up to the second, and
/// the output shows both values ("should support two thumbs").
#[browser_test]
pub async fn two_thumbs(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let price = page.element("[role=group][aria-label=Price]").await?;
    let thumbs = inputs_in(page, "[aria-label=Price]").await?;
    let (minimum, maximum) = (&thumbs[0], &thumbs[1]);
    assert_that!(minimum)
        .has_attribute("max")
        .await
        .is_equal_to("80");
    assert_that!(maximum)
        .has_attribute("min")
        .await
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
    assert_that!(price)
        .attribute("data-disabled")
        .await
        .is_none();
    Ok(())
}

/// A vertical slider is `data-orientation="vertical"` and its input `aria-orientation="vertical"`;
/// Up and Right increase its value, Down and Left decrease it ("should support orientation").
#[browser_test]
pub async fn orientation(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let group = page.element("[role=group][aria-label=Vertical]").await?;
    assert_that!(group)
        .has_attribute("data-orientation")
        .await
        .is_equal_to("vertical");
    let vertical = inputs_in(page, "[aria-label=Vertical]").await?.remove(0);
    assert_that!(vertical)
        .has_attribute("aria-orientation")
        .await
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

/// A disabled slider and its thumb are marked `data-disabled` and the thumb's input is disabled
/// ("should support disabled state").
#[browser_test]
pub async fn disabled_state(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let disabled = page.element("[role=group][aria-label=Disabled]").await?;
    assert_that!(disabled)
        .has_attribute("data-disabled")
        .await
        .is_equal_to("true");
    let thumb = page
        .element("[aria-label=Disabled] .test-slider-thumb")
        .await?;
    assert_that!(thumb)
        .has_attribute("data-disabled")
        .await
        .is_equal_to("true");
    let disabled_input = inputs_in(page, "[aria-label=Disabled]").await?.remove(0);
    assert_that!(disabled_input).enabled().await.is_false();
    Ok(())
}

/// An "always" value tooltip shows the thumb's value from the start, and an "on hover" one appears
/// when its thumb is hovered.
#[browser_test]
pub async fn tooltips(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let always = page.element("[aria-label=Always] .tooltip").await?;
    assert_that!(always).has_attribute("data-visible").await;
    assert_that!(always).inner_text().await.is_equal_to("30");
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

/// Clicking the track moves the thumb closest to the click there and focuses it, the first thumb
/// for a click at 20 and the second for one at 90, each with a change and an end ("should allow
/// you to set value of closest thumb by clicking on track", "can click on track to move nearest
/// handle").
#[browser_test]
pub async fn closest_thumb_by_click(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(INTERACTIONS, &["track"]).await?;
    let log = page.element("#track-log").await?;
    let inputs = inputs_of(page, "track").await?;
    click_track(page, "track", 40).await?;
    log.wait_for_inner_text("change:[20, 80];end:[20, 80]")
        .await?;
    page.wait_for_focus(&inputs[0]).await?;
    click_track(page, "track", 180).await?;
    log.wait_for_inner_text("change:[20, 80];end:[20, 80];change:[20, 90];end:[20, 90]")
        .await?;
    page.wait_for_focus(&inputs[1]).await?;
    Ok(())
}

/// Pressing and dragging on the track moves the closest thumb with the pointer, a change per move
/// and a single end on release ("should allow you to set value of closest thumb by dragging on
/// track").
#[browser_test]
pub async fn closest_thumb_by_drag(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(INTERACTIONS, &["track"]).await?;
    let track = track_of(page, "track").await?;
    let held = track.press_and_hold_at(40 - 100, 0).await?;
    wait_for_last(page, "track", "change", "[20, 80]").await?;
    held.move_by(20, 0).await?;
    wait_for_last(page, "track", "change", "[30, 80]").await?;
    held.move_by(20, 0).await?;
    wait_for_last(page, "track", "change", "[40, 80]").await?;
    page.settle().await?;
    assert_that!(|| ends_of(page, "track"))
        .consistently_ok()
        .for_at_least(std::time::Duration::from_millis(100))
        .matches(eq(0))
        .await;
    held.release().await?;
    wait_for_last(page, "track", "end", "[40, 80]").await?;
    page.settle().await?;
    assert_that!(|| ends_of(page, "track"))
        .consistently_ok()
        .for_at_least(std::time::Duration::from_millis(100))
        .matches(eq(1))
        .await;
    Ok(())
}

/// Presses the track of `name` at `x` pixels from its start and waits for the change to
/// `values`, without an end before the release; after the release, waits for the end with the
/// same values.
async fn press_track(page: &Page<'_>, name: &str, x: i64, values: &str) -> Result<(), Report> {
    let ends = ends_of(page, name).await?;
    let track = track_of(page, name).await?;
    let held = track.press_and_hold_at(x - 100, 0).await?;
    wait_for_last(page, name, "change", values).await?;
    page.settle().await?;
    assert_that!(|| ends_of(page, name))
        .consistently_ok()
        .for_at_least(std::time::Duration::from_millis(100))
        .matches(eq(ends))
        .await;
    held.release().await?;
    wait_for_last(page, name, "end", values).await?;
    Ok(())
}

/// A track press before two stacked thumbs moves the first of them, ending on release ("should
/// allow you to set value of before thumbs when thumbs stacked").
#[browser_test]
pub async fn stacked_thumbs_before(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(INTERACTIONS, &["stacked"]).await?;
    press_track(page, "stacked", 40, "[20, 40]").await
}

/// A track press after two stacked thumbs moves the second of them, ending on release ("should
/// allow you to set value of after thumbs when thumbs stacked").
#[browser_test]
pub async fn stacked_thumbs_after(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(INTERACTIONS, &["stacked"]).await?;
    press_track(page, "stacked", 120, "[40, 60]").await
}

/// Among five thumbs with two stacked pairs, track presses before a pair move its first thumb
/// ("should allow you to set value of before thumbs when many thumbs and stacked").
#[browser_test]
pub async fn many_stacked_thumbs_before(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(INTERACTIONS, &["many"]).await?;
    press_track(page, "many", 140, "[25, 25, 50, 70, 75]").await?;
    press_track(page, "many", 40, "[20, 25, 50, 70, 75]").await
}

/// Among five thumbs with two stacked pairs, track presses after a pair move its second thumb
/// ("should allow you to set value of after thumbs when many thumbs and stacked").
#[browser_test]
pub async fn many_stacked_thumbs_after(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(INTERACTIONS, &["many"]).await?;
    press_track(page, "many", 160, "[25, 25, 50, 75, 80]").await?;
    press_track(page, "many", 60, "[25, 30, 50, 75, 80]").await
}

/// Pressing and dragging on a disabled slider's track changes nothing and focuses no thumb
/// ("should not allow you to set value if disabled", "cannot click on track to move nearest handle
/// when disabled").
#[browser_test]
pub async fn disabled_track(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(INTERACTIONS, &["disabled"]).await?;
    let track = track_of(page, "disabled").await?;
    page.low_level()
        .driver()
        .action_chain()
        .move_to_element_with_offset(&track, 40 - 100, 0)
        .click_and_hold()
        .move_by_offset(20, 0)
        .release()
        .perform()
        .await?;
    page.settle().await?;
    assert_that!(|| log_of(page, "disabled"))
        .consistently_ok()
        .for_at_least(std::time::Duration::from_millis(100))
        .matches(eq(Vec::<String>::new()))
        .await;
    let disabled = inputs_of(page, "disabled").await?;
    assert_that!(disabled[0])
        .property("value")
        .await
        .get_some()
        .is_equal_to("10");
    let body = page.element("body").await?;
    page.focus_stays(&body, std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// Pressing a thumb focuses its input without a change; dragging it moves it with the pointer,
/// ending once on release; a thumb dragged past its neighbor stops at it ("can be moved by
/// dragging", "can click and drag handle").
#[browser_test]
pub async fn drag_thumbs(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(INTERACTIONS, &["track"]).await?;
    let inputs = inputs_of(page, "track").await?;
    let thumbs = page.elements("#track-track .thumb").await?;
    thumbs[0].scroll_into_view().await?;
    // Off the thumb's center: a press that reached the track would move the thumb.
    let held = thumbs[0].press_and_hold_at(3, 0).await?;
    page.wait_for_focus(&inputs[0]).await?;
    page.settle().await?;
    assert_that!(|| log_of(page, "track"))
        .consistently_ok()
        .for_at_least(std::time::Duration::from_millis(100))
        .matches(eq(Vec::<String>::new()))
        .await;
    held.move_by(60, 0).await?;
    wait_for_last(page, "track", "change", "[40, 80]").await?;
    held.release().await?;
    wait_for_last(page, "track", "end", "[40, 80]").await?;
    let held = thumbs[1].press_and_hold().await?;
    page.wait_for_focus(&inputs[1]).await?;
    held.move_by(-40, 0).await?;
    wait_for_last(page, "track", "change", "[40, 60]").await?;
    held.move_by(-60, 0).await?;
    wait_for_last(page, "track", "change", "[40, 40]").await?;
    held.release().await?;
    wait_for_last(page, "track", "end", "[40, 40]").await?;
    assert_that!(ends_of(page, "track").await?).is_equal_to(2);
    Ok(())
}

/// Dragging a thumb beyond the track's ends stops at the range's ends ("can click and drag
/// handle": dragged to the minimum and the maximum).
#[browser_test]
pub async fn drag_beyond_the_ends(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(INTERACTIONS, &["keys"]).await?;
    let thumb = page.element("#keys-track .thumb").await?;
    thumb.scroll_into_view().await?;
    let held = thumb.press_and_hold().await?;
    held.move_by(-100, 0).await?;
    wait_for_last(page, "keys", "change", "[0]").await?;
    held.move_by(300, 0).await?;
    wait_for_last(page, "keys", "change", "[100]").await?;
    held.release().await?;
    wait_for_last(page, "keys", "end", "[100]").await?;
    Ok(())
}

/// Dragging a vertical slider's thumb upwards increases its value, ending once on release ("can be
/// moved by dragging (vertical)").
#[browser_test]
pub async fn drag_thumb_vertical(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(INTERACTIONS, &["vertical"]).await?;
    let thumb = page.first_element("#vertical-track .thumb").await?;
    thumb.scroll_into_view().await?;
    let held = thumb.press_and_hold().await?;
    held.move_by(0, -20).await?;
    wait_for_last(page, "vertical", "change", "[20, 80]").await?;
    held.move_by(0, -40).await?;
    wait_for_last(page, "vertical", "change", "[40, 80]").await?;
    held.release().await?;
    wait_for_last(page, "vertical", "end", "[40, 80]").await?;
    assert_that!(ends_of(page, "vertical").await?).is_equal_to(1);
    Ok(())
}

/// Dragging a disabled slider's thumb changes nothing and doesn't focus its input ("cannot click
/// and drag handle when disabled").
#[browser_test]
pub async fn disabled_thumb_drag(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(INTERACTIONS, &["disabled"]).await?;
    let thumb = page.first_element("#disabled-track .thumb").await?;
    thumb.scroll_into_view().await?;
    let held = thumb.press_and_hold().await?;
    held.move_by(20, 0).await?;
    held.release().await?;
    page.settle().await?;
    assert_that!(|| log_of(page, "disabled"))
        .consistently_ok()
        .for_at_least(std::time::Duration::from_millis(100))
        .matches(eq(Vec::<String>::new()))
        .await;
    let body = page.element("body").await?;
    page.focus_stays(&body, std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// While one touch drags a thumb, a second touch on the track moves nothing; the first touch's
/// moves keep dragging the thumb ("doesn't jump to second touch on track while already
/// dragging").
#[browser_test]
pub async fn second_touch_while_dragging(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(INTERACTIONS, &["keys"]).await?;
    let track = track_of(page, "keys").await?;
    let thumb = page.element("#keys-track .thumb").await?;
    let rect = track.client_rect().await?;
    let thumb_rect = thumb.client_rect().await?;
    let y = rect.top + rect.height / 2.0;
    let thumb_x = thumb_rect.left + thumb_rect.width / 2.0;
    let touch = |kind, id, x| {
        SyntheticEvent::pointer(kind)
            .pointer_type(PointerType::Touch)
            .pointer_id(id)
            .at(x, y)
    };
    thumb.dispatch(touch(PointerKind::Down, 1, thumb_x)).await?;
    let second_x = rect.left + rect.width * 0.6;
    track
        .dispatch(touch(PointerKind::Down, 2, second_x))
        .await?;
    track
        .dispatch(touch(PointerKind::Move, 2, second_x + 10.0))
        .await?;
    track
        .dispatch(touch(PointerKind::Up, 2, second_x + 10.0))
        .await?;
    page.settle().await?;
    assert_that!(|| log_of(page, "keys"))
        .consistently_ok()
        .for_at_least(std::time::Duration::from_millis(100))
        .matches(eq(Vec::<String>::new()))
        .await;
    thumb
        .dispatch(touch(PointerKind::Move, 1, thumb_x + 40.0))
        .await?;
    wait_for_last(page, "keys", "change", "[30]").await?;
    thumb
        .dispatch(touch(PointerKind::Up, 1, thumb_x + 40.0))
        .await?;
    wait_for_last(page, "keys", "end", "[30]").await?;
    assert_that!(log_of(page, "keys").await?).contains_exactly(["change:[30]", "end:[30]"]);
    Ok(())
}

/// Dragging upwards on a vertical track increases the closest thumb's value, and releasing ends
/// the change ("should allow you to set value of closest thumb by dragging on track (vertical)").
#[browser_test]
pub async fn vertical_drag(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(INTERACTIONS, &["vertical"]).await?;
    let track = track_of(page, "vertical").await?;
    let held = track.press_and_hold_at(0, 160 - 100).await?;
    wait_for_last(page, "vertical", "change", "[20, 80]").await?;
    held.move_by(0, -20).await?;
    wait_for_last(page, "vertical", "change", "[30, 80]").await?;
    held.move_by(0, -20).await?;
    wait_for_last(page, "vertical", "change", "[40, 80]").await?;
    held.release().await?;
    wait_for_last(page, "vertical", "end", "[40, 80]").await?;
    Ok(())
}

/// In a right-to-left locale the track starts on the right (a press 140px from its left edge sets
/// 30), and Left increases the value while Right decreases it.
#[browser_test]
pub async fn right_to_left(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(INTERACTIONS, &["rtl"]).await?;
    click_track(page, "rtl", 140).await?;
    wait_for_last(page, "rtl", "end", "[30, 80]").await?;
    let thumb = page.first_element("#rtl-track .thumb").await?;
    assert_that!(thumb)
        .has_attribute("style")
        .await
        .map_owned(|style| style.replace(' ', ""))
        .contains("left:70%");
    let rtl = inputs_of(page, "rtl").await?;
    rtl[0].focus().await?;
    page.send_keys(Key::Left).await?;
    rtl[0].wait_for_prop("value", "31").await?;
    page.send_keys(Key::Right).await?;
    rtl[0].wait_for_prop("value", "30").await?;
    Ok(())
}

/// In a right-to-left locale the keys other than Left/Right keep their direction: Up and PageUp
/// increase the value, Down and PageDown decrease it, Home and End go to the thumb's bounds, and a
/// key toward a bound at the bound changes nothing ("moves the slider in the correct direction"
/// rtl, "is clamped by min/max" rtl).
#[browser_test]
pub async fn right_to_left_keys(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(INTERACTIONS, &["rtl"]).await?;
    let rtl = inputs_of(page, "rtl").await?;
    rtl[0].focus().await?;
    for (key, expected) in [
        (Key::Up, "11"),
        (Key::Down, "10"),
        (Key::PageUp, "20"),
        (Key::PageDown, "10"),
        (Key::Home, "0"),
        (Key::End, "80"),
        (Key::Home, "0"),
    ] {
        page.send_keys(key).await?;
        rtl[0].wait_for_prop("value", expected).await?;
    }
    page.send_keys(Key::Right).await?;
    page.settle().await?;
    rtl[0]
        .prop_stays("value", "0", std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// On a vertical slider in a right-to-left locale, Right decreases the value and Left increases it
/// (react-aria mirrors the horizontal arrows whatever the orientation).
#[browser_test]
pub async fn right_to_left_vertical_keys(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(INTERACTIONS, &["rtl-vertical"]).await?;
    let input = inputs_of(page, "rtl-vertical").await?.remove(0);
    input.focus().await?;
    page.send_keys(Key::Right).await?;
    input.wait_for_prop("value", "9").await?;
    page.send_keys(Key::Left).await?;
    input.wait_for_prop("value", "10").await?;
    page.send_keys(Key::Up).await?;
    input.wait_for_prop("value", "11").await?;
    Ok(())
}

// -- SliderThumbTests -----------------------------------------------------------------------------

/// Each arrow key changes a horizontal slider's value by one step and ends the change: Right and
/// Up increase it, Left and Down decrease it ("can be moved with keys").
#[browser_test]
pub async fn keys(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(INTERACTIONS, &["keys"]).await?;
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

/// On a vertical slider, Right and Up increase the value, Down and Left decrease it ("can be moved
/// with keys (vertical)").
#[browser_test]
pub async fn keys_vertical(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(INTERACTIONS, &["keys-vertical"]).await?;
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
    for (key, expected) in [
        (Key::PageUp, "20"),
        (Key::PageDown, "10"),
        (Key::End, "100"),
        (Key::Home, "0"),
    ] {
        page.send_keys(key).await?;
        vertical[0].wait_for_prop("value", expected).await?;
    }
    Ok(())
}

/// A key toward the bound a thumb is at changes nothing but still ends the change; the key away
/// from it steps ("can be moved with keys at the beginning of the slider", "... at the end of the
/// slider", "is clamped by min/max").
#[browser_test]
pub async fn keys_at_the_bounds(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(INTERACTIONS, &["keys"]).await?;
    let keys = inputs_of(page, "keys").await?;
    let log = page.element("#keys-log").await?;
    keys[0].focus().await?;
    let mut entries: Vec<&str> = Vec::new();
    for (key, added) in [
        (Key::Home, &["change:[0]", "end:[0]"][..]),
        (Key::Left, &["end:[0]"][..]),
        (Key::Right, &["change:[1]", "end:[1]"][..]),
        (Key::End, &["change:[100]", "end:[100]"][..]),
        (Key::Right, &["end:[100]"][..]),
        (Key::Left, &["change:[99]", "end:[99]"][..]),
    ] {
        page.send_keys(key).await?;
        entries.extend(added);
        log.wait_for_inner_text(&entries.join(";")).await?;
    }
    Ok(())
}

/// On a vertical slider, Down at the bottom and Up at the top change nothing but end the change
/// ("can be moved with keys (vertical) at the bottom of the slider", "... at the top of the
/// slider").
#[browser_test]
pub async fn vertical_keys_at_the_bounds(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(INTERACTIONS, &["keys-vertical"]).await?;
    let vertical = inputs_of(page, "keys-vertical").await?;
    let log = page.element("#keys-vertical-log").await?;
    vertical[0].focus().await?;
    let mut entries: Vec<&str> = Vec::new();
    for (key, added) in [
        (Key::Home, &["change:[0]", "end:[0]"][..]),
        (Key::Down, &["end:[0]"][..]),
        (Key::Up, &["change:[1]", "end:[1]"][..]),
        (Key::End, &["change:[100]", "end:[100]"][..]),
        (Key::Up, &["end:[100]"][..]),
        (Key::Down, &["change:[99]", "end:[99]"][..]),
    ] {
        page.send_keys(key).await?;
        entries.extend(added);
        log.wait_for_inner_text(&entries.join(";")).await?;
    }
    Ok(())
}

/// Tab skips a disabled slider's thumbs ("supports disabled").
#[browser_test]
pub async fn tab_skips_disabled_thumbs(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(INTERACTIONS, &["many", "disabled", "vertical"])
        .await?;
    let many = inputs_of(page, "many").await?;
    let vertical = inputs_of(page, "vertical").await?;
    for input in inputs_of(page, "disabled").await? {
        assert_that!(input).enabled().await.is_false();
    }
    many[4].focus().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&vertical[0]).await?;
    Ok(())
}

/// Holding PageUp or PageDown (a keydown and two repeats) changes the value by three pages of 10
/// ("should support repeat keydown events when holding Page Up/Page Down").
#[browser_test]
pub async fn repeated_page_keys(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(INTERACTIONS, &["page"]).await?;
    let paged = inputs_of(page, "page").await?;
    paged[0].focus().await?;
    // A keydown and two repeats: three pages of 10.
    page.hold_key("PageUp", 2).await?;
    paged[0].wait_for_prop("value", "50").await?;
    page.hold_key("PageDown", 2).await?;
    paged[0].wait_for_prop("value", "20").await?;
    Ok(())
}

/// An `input` event setting the thumb's value (as assistive technology does) changes the slider's
/// value, and the arrow keys continue from it.
#[browser_test]
pub async fn input_event(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(INTERACTIONS, &["keys"]).await?;
    let keys = inputs_of(page, "keys").await?;
    keys[0].virtual_input("42").await?;
    wait_for_last(page, "keys", "change", "[42]").await?;
    keys[0].focus().await?;
    page.send_keys(Key::Right).await?;
    keys[0].wait_for_prop("value", "43").await?;
    Ok(())
}

/// A disabled thumb has a disabled input and stays put on a track press near it, while a press
/// near the enabled thumb moves that one.
#[browser_test]
pub async fn disabled_thumb(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(INTERACTIONS, &["thumb-disabled"])
        .await?;
    let thumb_disabled = inputs_of(page, "thumb-disabled").await?;
    assert_that!(thumb_disabled[0]).enabled().await.is_true();
    assert_that!(thumb_disabled[1]).enabled().await.is_false();
    click_track(page, "thumb-disabled", 180).await?;
    thumb_disabled[1]
        .prop_stays("value", "80", std::time::Duration::from_millis(100))
        .await?;
    click_track(page, "thumb-disabled", 40).await?;
    thumb_disabled[0].wait_for_prop("value", "20").await?;
    Ok(())
}

/// A thumb's `form` and `name` props are set on its input ("should support form prop").
#[browser_test]
pub async fn form_prop(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(INTERACTIONS, &["form"]).await?;
    let form = inputs_of(page, "form").await?;
    assert_that!(form[0])
        .has_attribute("form")
        .await
        .is_equal_to("test-form");
    assert_that!(form[0])
        .has_attribute("name")
        .await
        .is_equal_to("volume");
    Ok(())
}

/// A thumb's own `Label` or `aria-label` names its input, followed by the slider's label ("should
/// have the right labels with Slider thumb label", "should have the right labels with Slider
/// thumb aria-label").
#[browser_test]
pub async fn thumb_labels(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(INTERACTIONS, &["labels"]).await?;
    let group = page.element("#labels").await?;
    let slider_label = group.attr("aria-labelledby").await?.unwrap_or_default();
    let labels = inputs_in(page, "[role=group]#labels").await?;
    let min_id = labels[0].id().await?;
    let min_label = page.element("#labels .leptonic-SliderThumb label").await?;
    assert_that!(min_label)
        .inner_text()
        .await
        .is_equal_to("Min");
    assert_that!(min_label)
        .attribute("for")
        .await
        .is_equal_to(min_id);
    let min_label_id = min_label.id().await?.unwrap_or_default();
    labels[0]
        .wait_for_attr(
            "aria-labelledby",
            Some(&format!("{min_label_id} {slider_label}")),
        )
        .await?;
    let max_id = labels[1].id().await?.unwrap_or_default();
    assert_that!(labels[1])
        .has_attribute("aria-label")
        .await
        .is_equal_to("Max");
    labels[1]
        .wait_for_attr("aria-labelledby", Some(&format!("{max_id} {slider_label}")))
        .await?;
    // The slider's label is a different element with a different id.
    assert_that!(slider_label.as_str()).is_not_equal_to(min_label_id.as_str());
    assert_that!(group)
        .accessible_name()
        .await
        .is_equal_to("Price");
    Ok(())
}

/// A thumb's input has exact decimal `step` and `value` attributes (0.1, then 0.2, no `f64`
/// noise) and `aria-required`, `aria-invalid`, `aria-errormessage` and `aria-details`.
#[browser_test]
pub async fn attributes(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(INTERACTIONS, &["attributes"]).await?;
    let attributes = inputs_of(page, "attributes").await?;
    let input = &attributes[0];
    assert_that!(input)
        .has_attribute("step")
        .await
        .is_equal_to("0.1");
    assert_that!(input)
        .has_attribute("value")
        .await
        .is_equal_to("0.1");
    assert_that!(input)
        .has_attribute("aria-required")
        .await
        .is_equal_to("true");
    assert_that!(input)
        .has_attribute("aria-invalid")
        .await
        .is_equal_to("true");
    assert_that!(input)
        .has_attribute("aria-errormessage")
        .await
        .is_equal_to("attributes-error");
    assert_that!(input)
        .has_attribute("aria-details")
        .await
        .is_equal_to("attributes-details");
    input.focus().await?;
    page.send_keys(Key::Right).await?;
    input.wait_for_prop("value", "0.2").await?;
    assert_that!(input)
        .has_attribute("value")
        .await
        .is_equal_to("0.2");
    Ok(())
}

// -- SliderMultipleThumbsTests --------------------------------------------------------------------

/// A slider with three values has an input per thumb and an output listing all three values
/// ("should support three thumbs").
#[browser_test]
pub async fn three_thumbs(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(INTERACTIONS, &["three"]).await?;
    let three = inputs_of(page, "three").await?;
    assert_that!(values_of(&three).await?).contains_exactly(["30", "60", "80"]);
    page.element("#three-output")
        .await?
        .wait_for_inner_text("30, 60, 80")
        .await?;
    Ok(())
}

/// Thumbs bound to app state follow it when the app resets it, and arrow keys change only the
/// focused thumb's value ("should support multiple thumbs (controlled)").
#[browser_test]
pub async fn controlled_thumbs(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(INTERACTIONS, &["controlled"]).await?;
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
    assert_that!(controlled[1])
        .property("value")
        .await
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
    assert_that!(controlled[0])
        .property("value")
        .await
        .get_some()
        .is_equal_to("0");
    Ok(())
}

/// Tab moves through the thumbs and on past the slider; Shift+Tab back through them ("can be
/// focused").
#[browser_test]
pub async fn focus_order(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(INTERACTIONS, &["controlled"]).await?;
    let controlled = inputs_of(page, "controlled").await?;
    let before = page.element("#controlled-before").await?;
    before.focus().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&controlled[0]).await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&controlled[1]).await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&page.element("#controlled-reset").await?)
        .await?;
    page.send_keys(Key::Shift + Key::Tab).await?;
    page.wait_for_focus(&controlled[1]).await?;
    page.send_keys(Key::Shift + Key::Tab).await?;
    page.wait_for_focus(&controlled[0]).await?;
    page.send_keys(Key::Shift + Key::Tab).await?;
    page.wait_for_focus(&before).await?;
    Ok(())
}

/// Bound values out of the range (-20 and 150) render clamped to it, as 0 and 100 (react-stately's
/// `restrictValues`).
#[browser_test]
pub async fn restricted_values(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(INTERACTIONS, &["restricted"]).await?;
    let restricted = inputs_of(page, "restricted").await?;
    assert_that!(values_of(&restricted).await?).contains_exactly(["0", "100"]);
    page.element("#restricted-output")
        .await?
        .wait_for_inner_text("0 \u{2013} 100")
        .await?;
    Ok(())
}

/// A slider with one value and two thumbs renders both thumbs instead of panicking: the first at
/// its value, the second at the minimum, ignoring keys, with a development warning.
#[browser_test]
pub async fn missing_value(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(INTERACTIONS, &["missing"]).await?;
    let missing = inputs_of(page, "missing").await?;
    assert_that!(missing).has_length(2);
    assert_that!(missing[1])
        .property("value")
        .await
        .get_some()
        .is_equal_to("0");
    missing[1].focus().await?;
    page.send_keys(Key::Right).await?;
    page.settle().await?;
    missing[1]
        .prop_stays("value", "0", std::time::Duration::from_millis(100))
        .await?;
    assert_that!(missing[0])
        .property("value")
        .await
        .get_some()
        .is_equal_to("30");
    crate::fixtures::take_warnings(page, "Slider thumb 1 has no value", 1).await?;
    Ok(())
}
