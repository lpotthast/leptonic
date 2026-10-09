// Upstream: @adobe/react-spectrum/test/color/ColorSlider.test.tsx @ 99e6102368
// Upstream: react-aria-components/test/ColorSlider.test.js @ 99e6102368
//! The `ColorSlider` atoms: input attributes, value text and labelling, keyboard steps, a press
//! on the track, disabled sliders, forms.
//!
//! Dragging thumbs and tracks (react-spectrum's `ColorSlider.test.tsx`, "dragging the thumb
//! works", "... when vertical", "clicking and dragging on the track works when vertical"), the
//! `Label`'s default text, the thumb's states, gradients, and parts that mount again.
use assertr::{matchers::eq, prelude::*};
use browser_test::{browser_test, thirtyfour::prelude::*};
use rootcause::{Report, report};

use crate::pages::{ElementActions, Page};

const PATH: &str = "/atoms/color-slider";

/// The range input of the slider `#id`.
async fn input(page: &Page<'_>, id: &str) -> Result<WebElement, Report> {
    page.element(format!("#{id} input[type=range]")).await
}

/// The change log of the RGB sliders: `change:<hex>` and `end:<hex>` entries, comma-separated.
async fn log(page: &Page<'_>) -> Result<WebElement, Report> {
    page.element("#test-cs-log").await
}

/// The change log of the hue sliders: `change:<hue>` and `end:<hue>` entries.
async fn hue_log(page: &Page<'_>) -> Result<WebElement, Report> {
    page.element("#test-cs-hue-log").await
}

/// Empties the change log (a script click, which leaves focus where it is).
async fn clear(page: &Page<'_>) -> Result<(), Report> {
    page.element("#test-cs-clear")
        .await?
        .virtual_click()
        .await?;
    log(page).await?.wait_for_inner_text("").await?;
    Ok(())
}

/// Empties the hue log (a script click, which leaves focus where it is).
async fn clear_hues(page: &Page<'_>) -> Result<(), Report> {
    page.element("#test-cs-hue-clear")
        .await?
        .virtual_click()
        .await?;
    hue_log(page).await?.wait_for_inner_text("").await?;
    Ok(())
}

/// The last entry of the hue log (dragging logs a change per move).
async fn last_hue(page: &Page<'_>) -> Result<String, Report> {
    let log = hue_log(page).await?.inner_text().await?;
    Ok(log.rsplit(',').next().unwrap_or_default().to_owned())
}

/// Waits until the last entry of the hue log is `expected`.
async fn wait_for_last_hue(page: &Page<'_>, expected: &str) -> Result<(), Report> {
    assert_that!(|| last_hue(page))
        .eventually_ok()
        .matches(eq(expected))
        .await;
    Ok(())
}

/// The hue of a hue log entry `<kind>:<hue>`.
fn hue_of(entry: &str, kind: &str) -> Option<f64> {
    entry.strip_prefix(kind)?.strip_prefix(':')?.parse().ok()
}

/// Waits until the last hue log entry is `kind:<hue>` with the hue within 2° of `expected`;
/// returns the hue.
async fn wait_for_last_hue_near(page: &Page<'_>, kind: &str, expected: f64) -> Result<f64, Report> {
    let entry = assert_that!(|| last_hue(page))
        .eventually_ok()
        .satisfies(|entry| {
            entry
                .derive_owned(|entry| hue_of(entry, kind))
                .is_some_satisfying(|hue| {
                    hue.is_close_to(expected, 2.0);
                });
        })
        .await
        .unwrap_inner();
    hue_of(&entry, kind).ok_or_else(|| report!("the last hue log entry {entry:?} has no hue"))
}

/// The input is a range from 0 to 255 with the value and the color's name as value text, and a
/// slider without a label is labelled by its track, named after the channel ("sets input props",
/// "sets a default aria-label when label={null}", "should render a slider with default class").
#[browser_test]
pub async fn input_props(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let red = input(page, "test-cs-red").await?;
    assert_that!(red)
        .has_attribute("type")
        .await
        .is_equal_to("range");
    assert_that!(red)
        .has_attribute("min")
        .await
        .is_equal_to("0");
    assert_that!(red)
        .has_attribute("max")
        .await
        .is_equal_to("255");
    assert_that!(red)
        .has_attribute("step")
        .await
        .is_equal_to("1");
    // "sets input props": the value and the color's name.
    assert_that!(red)
        .has_attribute("aria-valuetext")
        .await
        .is_equal_to("0, black");
    // "sets a default aria-label when label={null}": on the group, which labels the input.
    // "should render a slider with default class": the track is the group, inside the root.
    let group = page.element("#test-cs-red [role=group]").await?;
    assert_that!(group)
        .has_attribute("class")
        .await
        .contains("leptonic-ColorSliderTrack");
    let root = page.element("#test-cs-red .leptonic-ColorSlider").await?;
    assert_that!(root).attribute("role").await.is_none();
    assert_that!(group)
        .has_attribute("aria-label")
        .await
        .is_equal_to("Red");
    let group_id = group.id().await?;
    assert_that!(red)
        .attribute("aria-labelledby")
        .await
        .is_equal_to(group_id);
    assert_that!(red).attribute("aria-label").await.is_none();
    // The track is the root's child.
    page.element("#test-cs-red > .leptonic-ColorSlider > .leptonic-ColorSliderTrack")
        .await?;
    let output = page.element("#test-cs-red output").await?;
    assert_that!(output).inner_text().await.is_equal_to("0");
    Ok(())
}

/// The value text names the color the track shows: a hue slider names the hue at full saturation
/// (also on a gray), a channel slider of a transparent color the opaque color (react-aria's
/// `getDisplayColor`).
#[browser_test]
pub async fn value_text_names_the_display_color(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let gray_hue = input(page, "test-cs-gray-hue").await?;
    assert_that!(gray_hue)
        .has_attribute("aria-valuetext")
        .await
        .is_equal_to("200°, cyan blue");
    let alpha = input(page, "test-cs-alpha").await?;
    assert_that!(alpha)
        .has_attribute("aria-valuetext")
        .await
        .is_equal_to("255, vibrant red");
    Ok(())
}

/// The output shows the value for the input (`for`), unannounced while it changes
/// (`aria-live="off"`) and without a label of its own ("shows value label by default").
#[browser_test]
pub async fn output(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let red = input(page, "test-cs-red").await?;
    let output = page.element("#test-cs-red output").await?;
    let red_id = red.id().await?.unwrap_or_default();
    assert_that!(output)
        .has_attribute("for")
        .await
        .is_equal_to(red_id);
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

/// An `aria_label` names the track's group, which has an id and labels the input, also on a
/// vertical slider ("allows a custom aria-label", "supports custom aria-label with
/// orientation=vertical").
#[browser_test]
pub async fn aria_label(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    for id in ["test-cs-aria-label", "test-cs-aria-label-vertical"] {
        let group = page.element(format!("#{id} [role=group]")).await?;
        assert_that!(group)
            .has_attribute("aria-label")
            .await
            .is_equal_to("Test");
        assert_that!(group)
            .attribute("aria-labelledby")
            .await
            .is_none();
        let group_id = group.id().await?;
        assert_that!(input(page, id).await?)
            .attribute("aria-labelledby")
            .await
            .is_equal_to(group_id);
    }
    Ok(())
}

/// An `aria_labelledby` labels the track's group, without an `aria-label`, and the group labels the
/// input ("allows a custom aria-labelledby").
#[browser_test]
pub async fn aria_labelledby(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let group = page.element("#test-cs-labelledby [role=group]").await?;
    assert_that!(group)
        .has_attribute("aria-labelledby")
        .await
        .is_equal_to("test-cs-label-id");
    assert_that!(group).attribute("aria-label").await.is_none();
    let input = input(page, "test-cs-labelledby").await?;
    assert_that!(input)
        .attribute("aria-labelledby")
        .await
        .is_equal_to(Some("test-cs-label-id".to_owned()));
    assert_that!(input)
        .accessible_name()
        .await
        .is_equal_to("Shade");
    Ok(())
}

/// A hue slider's value text is the hue in degrees and its name, its `Label` names the group and
/// the input, and pressing the label focuses the input ("sets aria-valuetext to formatted value",
/// "allows a custom label", "clicking on label should focus input").
#[browser_test]
pub async fn hue_value_text_and_label(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let hue = input(page, "test-cs-hue").await?;
    assert_that!(hue)
        .has_attribute("max")
        .await
        .is_equal_to("360");
    assert_that!(hue)
        .has_attribute("aria-valuetext")
        .await
        .is_equal_to("10°, red orange");
    assert_that!(hue).attribute("aria-label").await.is_none();
    // "allows a custom label": the label names the group and the input.
    let label = page.element("#test-cs-hue .leptonic-Label").await?;
    let label_id = label.id().await?;
    let group = page.element("#test-cs-hue [role=group]").await?;
    assert_that!(group)
        .attribute("aria-labelledby")
        .await
        .is_equal_to(label_id.clone());
    assert_that!(group).attribute("aria-label").await.is_none();
    assert_that!(hue)
        .attribute("aria-labelledby")
        .await
        .is_equal_to(label_id);
    page.element("#test-cs-hue [id^=label]")
        .await?
        .click()
        .await?;
    page.wait_for_focus(&hue).await?;
    Ok(())
}

/// The arrow keys step the channel, Page Up by a page step, and Home/End go to its ends, each
/// reported as a change and a change end ("keyboard events" > "works").
#[browser_test]
pub async fn keyboard(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let red = input(page, "test-cs-red").await?;
    page.element("#test-cs-before").await?.focus().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&red).await?;
    let log = log(page).await?;
    for (key, expected) in [
        (Key::Right, "010000"),
        (Key::PageUp, "120000"),
        (Key::Home, "000000"),
        (Key::End, "FF0000"),
    ] {
        page.send_keys(key).await?;
        log.wait_for_inner_text(&format!("change:{expected},end:{expected}"))
            .await?;
        clear(page).await?;
    }
    Ok(())
}

/// Pressing the track a quarter along sets the channel to a quarter of its range ("clicking and
/// dragging on the track works").
#[browser_test]
pub async fn track_click(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let red = input(page, "test-cs-red").await?;
    let track = page.element("#test-cs-red [role=group]").await?;
    track.scroll_into_view().await?;
    page.low_level()
        .driver()
        .action_chain()
        .move_to_element_with_offset(&track, -50, 0)
        .click()
        .perform()
        .await?;
    assert_that!(|| red.number_value())
        .eventually_ok()
        .satisfies(|red| {
            red.is_close_to(64.0, 1.0);
        })
        .await;
    Ok(())
}

/// A disabled slider and its thumb are `data-disabled` and its input is disabled, so Tab skips it
/// ("disabled", "should support disabled state").
#[browser_test]
pub async fn disabled(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let disabled = input(page, "test-cs-disabled").await?;
    assert_that!(disabled).enabled().await.is_false();
    for part in ["ColorSlider", "ColorSliderTrack", "ColorThumb"] {
        let element = page
            .element(format!("#test-cs-disabled .leptonic-{part}"))
            .await?;
        assert_that!(element)
            .has_attribute("data-disabled")
            .await
            .is_equal_to("true");
    }
    page.element("#test-cs-a").await?.focus().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&page.element("#test-cs-b").await?)
        .await?;
    Ok(())
}

/// Dragging a disabled slider's thumb, or pressing and dragging on its track, changes nothing and
/// doesn't focus its input ("dragging the thumb doesn't works when disabled", "clicking and
/// dragging on the track doesn't work when disabled").
#[browser_test]
pub async fn disabled_drag(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let disabled = input(page, "test-cs-disabled").await?;
    let body = page.element("body").await?;
    let thumb = page
        .element("#test-cs-disabled .leptonic-ColorThumb")
        .await?;
    thumb.scroll_into_view().await?;
    let held = thumb.press_and_hold().await?;
    held.move_by(80, 0).await?;
    held.release().await?;
    let track = page.element("#test-cs-disabled [role=group]").await?;
    let held = track.press_and_hold().await?;
    held.move_by(40, 0).await?;
    held.release().await?;
    page.settle().await?;
    disabled
        .prop_stays("value", "0", std::time::Duration::from_millis(100))
        .await?;
    page.focus_stays(&body, std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// Tab moves focus to the input and on past it, Shift+Tab back to it ("the slider is focusable").
#[browser_test]
pub async fn focusable(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let red = input(page, "test-cs-red").await?;
    page.element("#test-cs-before").await?.focus().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&red).await?;
    page.send_keys(Key::Tab).await?;
    let hue = input(page, "test-cs-hue").await?;
    page.wait_for_focus(&hue).await?;
    page.send_keys(Key::Shift + Key::Tab).await?;
    page.wait_for_focus(&red).await?;
    Ok(())
}

/// Pressing the middle of a horizontal track sets 180° and focuses the input, dragging 40 pixels
/// right adds 72°, and releasing ends the change once ("clicking and dragging on the track
/// works").
#[browser_test]
pub async fn drag_track(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let drag = input(page, "test-cs-drag").await?;
    let track = page.element("#test-cs-drag [role=group]").await?;
    track.scroll_into_view().await?;
    let held = track.press_and_hold().await?;
    let pressed = wait_for_last_hue_near(page, "change", 180.0).await?;
    page.wait_for_focus(&drag).await?;
    held.move_by(40, 0).await?;
    let dragged = wait_for_last_hue_near(page, "change", pressed + 72.0).await?;
    held.release().await?;
    wait_for_last_hue(page, &format!("end:{dragged}")).await?;
    let log = hue_log(page).await?.inner_text().await?;
    assert_that!(log.matches("end:").count()).is_equal_to(1);
    Ok(())
}

/// Tabbing to the input shows a focus ring on the thumb (`data-focused`, `data-focus-visible`),
/// which goes when the focus leaves ("should support focus ring").
#[browser_test]
pub async fn thumb_focus_ring(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let thumb = page.element("#test-cs-red .leptonic-ColorThumb").await?;
    assert_that!(thumb)
        .attribute("data-focus-visible")
        .await
        .is_none();
    page.element("#test-cs-before").await?.focus().await?;
    page.send_keys(Key::Tab).await?;
    thumb
        .wait_for_attr("data-focus-visible", Some("true"))
        .await?;
    assert_that!(thumb)
        .has_attribute("data-focused")
        .await
        .is_equal_to("true");
    page.send_keys(Key::Tab).await?;
    thumb.wait_for_attr("data-focus-visible", None).await?;
    assert_that!(thumb)
        .attribute("data-focused")
        .await
        .is_none();
    Ok(())
}

/// The thumb and the track are `data-hovered` while the pointer is over the thumb ("should
/// support hover state").
#[browser_test]
pub async fn thumb_hover(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let thumb = page.element("#test-cs-red .leptonic-ColorThumb").await?;
    let track = page.element("#test-cs-red [role=group]").await?;
    thumb.scroll_into_view().await?;
    assert_that!(thumb)
        .attribute("data-hovered")
        .await
        .is_none();
    thumb.hover().await?;
    thumb.wait_for_attr("data-hovered", Some("true")).await?;
    track.wait_for_attr("data-hovered", Some("true")).await?;
    page.element("#test-cs-before").await?.hover().await?;
    thumb.wait_for_attr("data-hovered", None).await?;
    track.wait_for_attr("data-hovered", None).await?;
    Ok(())
}

/// The thumb is `data-dragging` while it is pressed ("should support dragging state").
#[browser_test]
pub async fn thumb_dragging(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let thumb = page.element("#test-cs-red .leptonic-ColorThumb").await?;
    thumb.scroll_into_view().await?;
    assert_that!(thumb)
        .attribute("data-dragging")
        .await
        .is_none();
    let held = thumb.press_and_hold().await?;
    thumb.wait_for_attr("data-dragging", Some("true")).await?;
    held.release().await?;
    thumb.wait_for_attr("data-dragging", None).await?;
    Ok(())
}

/// `classes` add to the slider's default class, other attributes reach every part's element, and
/// `form` associates the input with a form ("should render a slider with custom class", "should
/// support DOM props", "should support form prop").
#[browser_test]
pub async fn classes_attributes_and_form(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let root = page.element("#test-cs-props .leptonic-ColorSlider").await?;
    assert_that!(root)
        .has_attribute("class")
        .await
        .is_equal_to("leptonic-ColorSlider custom-slider");
    for (part, attribute_value) in [
        ("ColorSlider", "slider"),
        ("ColorSliderOutput", "output"),
        ("ColorSliderTrack", "track"),
        ("ColorThumb", "thumb"),
    ] {
        let element = page
            .element(format!("#test-cs-props .leptonic-{part}"))
            .await?;
        assert_that!(element)
            .has_attribute("data-foo")
            .await
            .is_equal_to(attribute_value);
    }
    assert_that!(input(page, "test-cs-props").await?)
        .has_attribute("form")
        .await
        .is_equal_to("test-cs-other-form");
    Ok(())
}

/// The track's gradient runs through the channel's range: a lightness through the vivid color in
/// the middle, and to the left in a right-to-left locale.
#[browser_test]
pub async fn track_gradients(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let lightness = page.element("#test-cs-lightness [role=group]").await?;
    assert_that!(lightness.css_value("background-image").await?).is_equal_to(
        "linear-gradient(to right, rgb(0, 0, 0), rgb(255, 0, 0), rgb(255, 255, 255))".to_owned(),
    );
    let rtl = page.element("#test-cs-rtl [role=group]").await?;
    assert_that!(rtl.css_value("background-image").await?)
        .is_equal_to("linear-gradient(to left, rgb(0, 0, 0), rgb(255, 0, 0))".to_owned());
    let vertical = page.element("#test-cs-vertical [role=group]").await?;
    assert_that!(vertical.css_value("background-image").await?)
        .starts_with("linear-gradient(to top, ");
    Ok(())
}

/// The input carries its form name, and resetting the form restores the default value ("supports
/// form name", "supports form reset").
#[browser_test]
pub async fn forms(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let form = input(page, "test-cs-form").await?;
    assert_that!(form)
        .has_attribute("name")
        .await
        .is_equal_to("red");
    assert_that!(form)
        .property("value")
        .await
        .get_some()
        .map_owned(|value| value.parse::<f64>())
        .get_ok()
        .is_equal_to(127.0);
    form.focus().await?;
    page.send_keys(Key::Right).await?;
    form.wait_for_attr("aria-valuetext", Some("128, dark vibrant red"))
        .await?;
    page.element("#test-cs-reset").await?.click().await?;
    form.wait_for_attr("aria-valuetext", Some("127, dark vibrant red"))
        .await?;
    Ok(())
}

/// A `Label` without text shows the channel's name and labels the track ("defaults to showing the
/// channel as a label").
#[browser_test]
pub async fn default_label(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let label = page.element("#test-cs-label .leptonic-Label").await?;
    assert_that!(label).inner_text().await.is_equal_to("Green");
    let group = page.element("#test-cs-label [role=group]").await?;
    let label_id = label.id().await?;
    assert_that!(group)
        .attribute("aria-labelledby")
        .await
        .is_equal_to(label_id.clone());
    assert_that!(group).attribute("aria-label").await.is_none();
    let green = input(page, "test-cs-label").await?;
    assert_that!(green)
        .attribute("aria-labelledby")
        .await
        .is_equal_to(label_id);
    assert_that!(green).attribute("aria-label").await.is_none();
    Ok(())
}

/// Pressing the thumb focuses the input without changing the hue, and dragging it 80 of 200 pixels
/// sets 144° ("dragging the thumb works").
#[browser_test]
pub async fn drag_thumb(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let drag = input(page, "test-cs-drag").await?;
    let thumb = page.element("#test-cs-drag .leptonic-ColorThumb").await?;
    thumb.scroll_into_view().await?;
    let held = thumb.press_and_hold().await?;
    page.wait_for_focus(&drag).await?;
    hue_log(page)
        .await?
        .inner_text_stays("", std::time::Duration::from_millis(100))
        .await?;
    held.move_by(80, 0).await?;
    wait_for_last_hue(page, "change:144").await?;
    held.release().await?;
    wait_for_last_hue(page, "end:144").await?;
    page.wait_for_focus(&drag).await?;
    Ok(())
}

/// A vertical slider is `aria-orientation="vertical"` and `data-orientation="vertical"`, and
/// dragging its thumb upwards increases the hue ("dragging the thumb works when vertical", "should
/// support orientation").
#[browser_test]
pub async fn drag_thumb_vertical(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let vertical = input(page, "test-cs-vertical").await?;
    assert_that!(vertical)
        .has_attribute("aria-orientation")
        .await
        .is_equal_to("vertical");
    let root = page
        .element("#test-cs-vertical .leptonic-ColorSlider")
        .await?;
    assert_that!(root)
        .has_attribute("data-orientation")
        .await
        .is_equal_to("vertical");
    let thumb = page
        .element("#test-cs-vertical .leptonic-ColorThumb")
        .await?;
    thumb.scroll_into_view().await?;
    let held = thumb.press_and_hold().await?;
    held.move_by(0, -80).await?;
    wait_for_last_hue(page, "change:144").await?;
    held.release().await?;
    wait_for_last_hue(page, "end:144").await?;
    Ok(())
}

/// Pressing the middle of a vertical track sets 180° and dragging 40 pixels up adds 72°; then Up
/// increases the hue ("clicking and dragging on the track works when vertical").
#[browser_test]
pub async fn drag_track_vertical(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let vertical = input(page, "test-cs-vertical").await?;
    let track = page.element("#test-cs-vertical [role=group]").await?;
    // All of it in view: WebDriver's center is that of the part in view.
    track.scroll_into_view().await?;
    let held = track.press_and_hold().await?;
    let pressed = wait_for_last_hue_near(page, "change", 180.0).await?;
    page.wait_for_focus(&vertical).await?;
    held.move_by(0, -40).await?;
    let dragged = wait_for_last_hue_near(page, "change", pressed + 72.0).await?;
    held.release().await?;
    wait_for_last_hue(page, &format!("end:{dragged}")).await?;
    clear_hues(page).await?;
    page.send_keys(Key::Up).await?;
    wait_for_last_hue(page, &format!("end:{}", dragged + 1.0)).await?;
    Ok(())
}

/// A slider mounted again (inside a `<Show>`) renders its value and responds to the keyboard.
#[browser_test]
pub async fn mounted_again(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let toggle = page.element("#test-cs-toggle").await?;
    toggle.click().await?;
    page.wait_for_count("#test-cs-show input[type=range]", 0)
        .await?;
    toggle.click().await?;
    page.wait_for_count("#test-cs-show input[type=range]", 1)
        .await?;
    let shown = input(page, "test-cs-show").await?;
    assert_that!(shown)
        .has_attribute("aria-valuetext")
        .await
        .starts_with("50, ");
    shown.focus().await?;
    page.send_keys(Key::Right).await?;
    page.element("#test-cs-show output")
        .await?
        .wait_for_inner_text("51")
        .await?;
    Ok(())
}
