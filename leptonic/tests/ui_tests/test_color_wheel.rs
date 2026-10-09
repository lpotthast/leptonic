// Upstream: @adobe/react-spectrum/test/color/ColorWheel.test.tsx @ 99e6102368
// Upstream: react-aria/test/color/useColorWheel.test.tsx @ 99e6102368
// Upstream: react-aria-components/test/ColorWheel.test.js @ 99e6102368
//! The `ColorWheel` atoms: input attributes and labelling, keyboard steps wrapping around 0°,
//! a press on the ring (0° at 3 o'clock, clockwise), disabled wheels, forms.
//!
//! Dragging the thumb and the ring, the input's `value` property and `input` event (assistive
//! technology), RGB colors (the hue of their HSL form), the thumb's states, radii that change,
//! and parts that mount again.
use assertr::{matchers::eq, prelude::*};
use browser_test::{browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::pages::{ElementActions, Page};

const PATH: &str = "/atoms/color-wheel";

/// The range input of the wheel `#id`.
async fn input(page: &Page<'_>, id: &str) -> Result<WebElement, Report> {
    page.element(format!("#{id} input[type=range]")).await
}

/// The change log of the wheels: `change:<hue>` and `end:<hue>` entries, comma-separated.
async fn log(page: &Page<'_>) -> Result<WebElement, Report> {
    page.element("#test-cw-log").await
}

/// Empties the log (a script click, which leaves focus where it is).
async fn clear(page: &Page<'_>) -> Result<(), Report> {
    page.element("#test-cw-clear")
        .await?
        .virtual_click()
        .await?;
    log(page).await?.wait_for_inner_text("").await?;
    Ok(())
}

/// Waits until the log's last entry is `expected` (dragging logs many changes).
async fn wait_for_last_log(page: &Page<'_>, expected: &str) -> Result<(), Report> {
    let log = log(page).await?;
    assert_that!(|| async {
        Ok::<_, Report>(
            log.inner_text()
                .await?
                .rsplit(',')
                .next()
                .unwrap_or_default()
                .to_owned(),
        )
    })
    .eventually_ok()
    .matches(eq(expected))
    .await;
    Ok(())
}

/// The input is a range from 0 to 360 in steps of 1 at the hue, named "Hue" without a label, with
/// the hue and its name as value text, inside the thumb inside the wheel ("sets input props",
/// "should render a color wheel with default class").
#[browser_test]
pub async fn input_props(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let wheel = input(page, "test-cw-default").await?;
    page.element(
        "#test-cw-default > .leptonic-ColorWheel > .leptonic-ColorThumb input[type=range]",
    )
    .await?;
    assert_that!(wheel)
        .property("value")
        .await
        .get_some()
        .is_equal_to("0");
    assert_that!(wheel)
        .has_attribute("aria-valuetext")
        .await
        .is_equal_to("0°, red");
    assert_that!(wheel)
        .has_attribute("type")
        .await
        .is_equal_to("range");
    assert_that!(wheel)
        .has_attribute("min")
        .await
        .is_equal_to("0");
    assert_that!(wheel)
        .has_attribute("max")
        .await
        .is_equal_to("360");
    assert_that!(wheel)
        .has_attribute("step")
        .await
        .is_equal_to("1");
    assert_that!(wheel)
        .has_attribute("aria-label")
        .await
        .is_equal_to("Hue");
    Ok(())
}

/// The arrow keys step the hue by 1°, Shift+arrow and Page Up/Down by 15°, wrapping around 0°
/// ("works", "left/right works", "up/down works", "wraps around", "steps with page up/down",
/// "respects page steps", "respects page steps from shift arrow").
#[browser_test]
pub async fn keyboard(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let wheel = input(page, "test-cw-default").await?;
    page.element("#test-cw-before").await?.focus().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&wheel).await?;
    let log = log(page).await?;
    for (keys, expected) in [
        (Key::Right.into(), "1"),
        (Key::Left.into(), "0"),
        (Key::Left.into(), "359"),
        (Key::Up.into(), "0"),
        (Key::Down.into(), "359"),
        (Key::Up.into(), "0"),
        (Key::Shift + Key::Right, "15"),
        (Key::Shift + Key::Left, "0"),
        (Key::PageUp.into(), "15"),
        (Key::PageDown.into(), "0"),
        (Key::PageDown.into(), "345"),
    ] {
        page.send_keys(keys).await?;
        log.wait_for_inner_text(&format!("change:{expected},end:{expected}"))
            .await?;
        wheel.wait_for_prop("value", expected).await?;
        clear(page).await?;
    }
    Ok(())
}

/// Tab moves focus to the wheel's input and on past it, Shift+Tab back to it ("the slider is
/// focusable").
#[browser_test]
pub async fn focusable(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let wheel = input(page, "test-cw-default").await?;
    page.element("#test-cw-before").await?.focus().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&wheel).await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&page.element("#test-cw-a").await?)
        .await?;
    page.send_keys(Key::Shift + Key::Tab).await?;
    page.wait_for_focus(&wheel).await?;
    Ok(())
}

/// Holding Page Up steps the hue a page per repeated keydown, from 0° to 45°, and holding Page Down
/// back to 0° ("should support repeat keydown events when holding Page Up/Page Down").
#[browser_test]
pub async fn repeated_page_keys(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let wheel = input(page, "test-cw-default").await?;
    wheel.focus().await?;
    page.hold_key("PageUp", 2).await?;
    wheel.wait_for_prop("value", "45").await?;
    page.hold_key("PageDown", 2).await?;
    wheel.wait_for_prop("value", "0").await?;
    Ok(())
}

/// Pressing the ring below the center sets 90° and focuses the input, as 0° is at 3 o'clock and
/// hues go clockwise ("clicking on the track works").
#[browser_test]
pub async fn ring_press(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let wheel = input(page, "test-cw-default").await?;
    let track = page
        .element("#test-cw-default .leptonic-ColorWheelTrack")
        .await?;
    track.scroll_into_view().await?;
    page.low_level()
        .driver()
        .action_chain()
        .move_to_element_with_offset(&track, 0, 87)
        .click()
        .perform()
        .await?;
    log(page)
        .await?
        .wait_for_inner_text("change:90,end:90")
        .await?;
    page.wait_for_focus(&wheel).await?;
    Ok(())
}

/// Pressing the ring sets the hue there and focuses the input; dragging on moves the hue with the
/// pointer, and releasing ends the change once ("clicking and dragging on the track works").
#[browser_test]
pub async fn ring_drag(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let wheel = input(page, "test-cw-default").await?;
    let track = page
        .element("#test-cw-default .leptonic-ColorWheelTrack")
        .await?;
    track.scroll_into_view().await?;
    let held = track.press_and_hold_at(0, 87).await?;
    log(page).await?.wait_for_inner_text("change:90").await?;
    page.wait_for_focus(&wheel).await?;
    held.move_by(-87, -87).await?;
    wait_for_last_log(page, "change:180").await?;
    held.release().await?;
    wait_for_last_log(page, "end:180").await?;
    let log = log(page).await?.inner_text().await?;
    assert_that!(log.matches("end:").count()).is_equal_to(1);
    page.wait_for_focus(&wheel).await?;
    Ok(())
}

/// A press inside the inner radius (off the ring) changes nothing.
#[browser_test]
pub async fn press_inside_the_ring(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let track = page
        .element("#test-cw-default .leptonic-ColorWheelTrack")
        .await?;
    track.scroll_into_view().await?;
    page.low_level()
        .driver()
        .action_chain()
        .move_to_element_with_offset(&track, 0, 40)
        .click()
        .perform()
        .await?;
    page.settle().await?;
    log(page)
        .await?
        .inner_text_stays("", std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// A disabled wheel, its track and thumb are `data-disabled` and its input is disabled, so Tab
/// and Shift+Tab skip it ("disabled", "should support disabled state").
#[browser_test]
pub async fn disabled(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let disabled = input(page, "test-cw-disabled").await?;
    assert_that!(disabled).enabled().await.is_false();
    for part in ["ColorWheel", "ColorWheelTrack", "ColorThumb"] {
        let element = page
            .element(format!("#test-cw-disabled .leptonic-{part}"))
            .await?;
        assert_that!(element)
            .has_attribute("data-disabled")
            .await
            .is_equal_to("true");
    }
    let a = page.element("#test-cw-a").await?;
    a.focus().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&page.element("#test-cw-b").await?)
        .await?;
    page.send_keys(Key::Shift + Key::Tab).await?;
    page.wait_for_focus(&a).await?;
    Ok(())
}

/// Dragging a disabled wheel's thumb, or pressing and dragging on its ring, changes nothing and
/// doesn't focus its input ("dragging the thumb doesn't work when disabled", "clicking and
/// dragging on the track doesn't work when disabled").
#[browser_test]
pub async fn disabled_drag(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let disabled = input(page, "test-cw-disabled").await?;
    let body = page.element("body").await?;
    let thumb = page
        .element("#test-cw-disabled .leptonic-ColorThumb")
        .await?;
    thumb.scroll_into_view().await?;
    let held = thumb.press_and_hold().await?;
    held.move_by(-40, 40).await?;
    held.release().await?;
    let track = page
        .element("#test-cw-disabled .leptonic-ColorWheelTrack")
        .await?;
    let held = track.press_and_hold_at(0, 40).await?;
    held.move_by(-40, -40).await?;
    held.release().await?;
    page.settle().await?;
    disabled
        .prop_stays("value", "0", std::time::Duration::from_millis(100))
        .await?;
    page.focus_stays(&body, std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// An `aria_label` names the input, which carries its form name, and resetting the form restores
/// the default hue ("should support a custom aria-label", "supports form name", "supports form
/// reset").
#[browser_test]
pub async fn forms(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let form = input(page, "test-cw-form").await?;
    assert_that!(form)
        .has_attribute("aria-label")
        .await
        .is_equal_to("Tint");
    assert_that!(form)
        .has_attribute("name")
        .await
        .is_equal_to("hue");
    form.focus().await?;
    page.send_keys(Key::Right).await?;
    form.wait_for_prop("value", "11").await?;
    page.element("#test-cw-reset").await?.click().await?;
    form.wait_for_prop("value", "10").await?;
    Ok(())
}

/// Pressing the thumb focuses the input without a change; dragging it from 0° (3 o'clock) to below
/// the center sets 90°, and releasing ends the change once ("dragging the thumb works").
#[browser_test]
pub async fn drag_thumb(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let wheel = input(page, "test-cw-default").await?;
    let thumb = page
        .element("#test-cw-default .leptonic-ColorThumb")
        .await?;
    thumb.scroll_into_view().await?;
    let held = thumb.press_and_hold().await?;
    page.wait_for_focus(&wheel).await?;
    page.settle().await?;
    log(page)
        .await?
        .inner_text_stays("", std::time::Duration::from_millis(100))
        .await?;
    held.move_by(-87, 87).await?;
    wait_for_last_log(page, "change:90").await?;
    held.release().await?;
    wait_for_last_log(page, "end:90").await?;
    let log = log(page).await?.inner_text().await?;
    assert_that!(log.matches("end:").count()).is_equal_to(1);
    page.wait_for_focus(&wheel).await?;
    Ok(())
}

/// Tabbing to the input shows a focus ring on the thumb (`data-focused`, `data-focus-visible`),
/// which goes when the focus leaves ("should support focus ring").
#[browser_test]
pub async fn thumb_focus_ring(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let thumb = page
        .element("#test-cw-default .leptonic-ColorThumb")
        .await?;
    assert_that!(thumb)
        .attribute("data-focus-visible")
        .await
        .is_none();
    page.element("#test-cw-before").await?.focus().await?;
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

/// The thumb is `data-hovered` while the pointer is over it ("should support hover state").
#[browser_test]
pub async fn thumb_hover(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let thumb = page
        .element("#test-cw-default .leptonic-ColorThumb")
        .await?;
    thumb.scroll_into_view().await?;
    assert_that!(thumb)
        .attribute("data-hovered")
        .await
        .is_none();
    thumb.hover().await?;
    thumb.wait_for_attr("data-hovered", Some("true")).await?;
    page.element("#test-cw-before").await?.hover().await?;
    thumb.wait_for_attr("data-hovered", None).await?;
    Ok(())
}

/// The thumb is `data-dragging` while it is pressed ("should support dragging state").
#[browser_test]
pub async fn thumb_dragging(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let thumb = page
        .element("#test-cw-default .leptonic-ColorThumb")
        .await?;
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

/// `aria_labelledby` labels the input, without an `aria-label`; a wheel without a default value is
/// red ("should support a custom aria-labelledby").
#[browser_test]
pub async fn labelledby(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let wheel = input(page, "test-cw-labelledby").await?;
    assert_that!(wheel).attribute("aria-label").await.is_none();
    assert_that!(wheel)
        .has_attribute("aria-labelledby")
        .await
        .is_equal_to("test-cw-label");
    assert_that!(wheel)
        .accessible_name()
        .await
        .is_equal_to("Tint");
    assert_that!(wheel)
        .has_attribute("aria-valuetext")
        .await
        .is_equal_to("0°, red");
    Ok(())
}

/// The thumb of a half transparent color shows the hue opaque ("thumb background color should not
/// include alpha channel").
#[browser_test]
pub async fn thumb_without_alpha(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let thumb = page.element("#test-cw-alpha .leptonic-ColorThumb").await?;
    assert_that!(thumb.css_value("background-color").await?)
        .is_equal_to("rgba(255, 0, 0, 1)".to_owned());
    Ok(())
}

/// `classes` add to the wheel's default class, other attributes reach its element, and `form`
/// associates the input with a form ("should render a color wheel with custom class", "should
/// support DOM props", "should support form prop").
#[browser_test]
pub async fn classes_attributes_and_form(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let wheel = page.element("#test-cw-props > div").await?;
    assert_that!(wheel)
        .has_attribute("class")
        .await
        .is_equal_to("leptonic-ColorWheel custom-wheel");
    assert_that!(wheel)
        .has_attribute("data-foo")
        .await
        .is_equal_to("bar");
    assert_that!(input(page, "test-cw-props").await?)
        .has_attribute("form")
        .await
        .is_equal_to("test-cw-other-form");
    Ok(())
}

/// Radii given as signals resize the wheel: its track grows from 100 to 160 pixels and the thumb
/// moves out to the new ring.
#[browser_test]
pub async fn radii_change(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let track = page
        .element("#test-cw-radius .leptonic-ColorWheelTrack")
        .await?;
    let thumb = page.element("#test-cw-radius .leptonic-ColorThumb").await?;
    // The thumb's center below the track's top: at 90° (below the center), on the ring's middle.
    let thumb_offset = || async {
        let (track, thumb) = (track.client_rect().await?, thumb.client_rect().await?);
        Ok::<_, Report>(thumb.top + thumb.height / 2.0 - track.top)
    };
    let track_width = || async { Ok::<_, Report>(track.client_rect().await?.width) };
    assert_that!(track_width().await?).is_close_to(100.0, 0.5);
    assert_that!(thumb_offset().await?).is_close_to(50.0 + 40.0, 0.5);
    page.element("#test-cw-grow").await?.click().await?;
    assert_that!(track_width)
        .eventually_ok()
        .satisfies(|width| {
            width.is_close_to(160.0, 0.5);
        })
        .await;
    assert_that!(thumb_offset)
        .eventually_ok()
        .satisfies(|offset| {
            offset.is_close_to(80.0 + 70.0, 0.5);
        })
        .await;
    Ok(())
}

/// Setting the input's value with an `input` event, as assistive technology does, changes the hue,
/// and the keyboard then steps on from the new value.
#[browser_test]
pub async fn input_event(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let wheel = input(page, "test-cw-default").await?;
    wheel.virtual_input("200").await?;
    wait_for_last_log(page, "change:200").await?;
    wheel.wait_for_prop("value", "200").await?;
    assert_that!(wheel)
        .has_attribute("aria-valuetext")
        .await
        .starts_with("200°, ");
    wheel.focus().await?;
    page.send_keys(Key::Right).await?;
    wheel.wait_for_prop("value", "201").await?;
    Ok(())
}

/// The wheel changes an RGB color's hue through its HSL form and keeps it RGB; on a gray, which has
/// no hue, it keeps the hue it set while the color stays gray.
#[browser_test]
pub async fn rgb_colors(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let rgb = input(page, "test-cw-rgb").await?;
    rgb.focus().await?;
    page.send_keys(Key::Right).await?;
    page.element("#test-cw-rgb-log")
        .await?
        .wait_for_inner_text("rgb:FF0400")
        .await?;
    rgb.wait_for_prop("value", "1").await?;
    let gray = input(page, "test-cw-gray").await?;
    gray.focus().await?;
    page.send_keys(Key::Right).await?;
    gray.wait_for_prop("value", "1").await?;
    page.send_keys(Key::Right).await?;
    gray.wait_for_prop("value", "2").await?;
    page.element("#test-cw-rgb-log")
        .await?
        .inner_text_stays("rgb:FF0400", std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// A wheel's track and thumb mounted again (inside a `<Show>`) render the value and respond to the
/// keyboard.
#[browser_test]
pub async fn mounted_again(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let toggle = page.element("#test-cw-toggle").await?;
    toggle.click().await?;
    page.wait_for_count("#test-cw-show input[type=range]", 0)
        .await?;
    toggle.click().await?;
    page.wait_for_count("#test-cw-show input[type=range]", 1)
        .await?;
    let shown = input(page, "test-cw-show").await?;
    shown.wait_for_prop("value", "20").await?;
    shown.focus().await?;
    page.send_keys(Key::Right).await?;
    shown.wait_for_prop("value", "21").await?;
    Ok(())
}
