// Upstream: @adobe/react-spectrum/test/color/ColorArea.test.tsx @ 99e6102368
// Upstream: react-aria-components/test/ColorArea.test.js @ 99e6102368
//! The `ColorArea`/`ColorThumb` atoms: the hidden inputs' attributes and labelling, keyboard
//! steps (arrows, Shift, PageUp/PageDown, Home/End), pressing and dragging, disabled areas,
//! forms.
//!
//! HSV and HSL areas (their gradients' layer order with swapped axes, percentages), right to
//! left, the inputs' `value` property and `input` event (assistive technology), the thumb's
//! color without alpha and states, and a thumb that mounts again.
use assertr::{matchers::eq, prelude::*};
use browser_test::{browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::pages::{ElementActions, Page, Platform};

const PATH: &str = "/atoms/color-area";

/// The x and y inputs of the area `#id`.
async fn inputs(page: &Page<'_>, id: &str) -> Result<(WebElement, WebElement), Report> {
    let found = page.elements(format!("#{id} input[type=range]")).await?;
    assert_that!(found).has_length(2);
    let mut found = found.into_iter();
    Ok((
        found.next().expect("x input"),
        found.next().expect("y input"),
    ))
}

/// The change log of the areas: `change:<hex>` and `end:<hex>` entries, comma-separated.
async fn log(page: &Page<'_>) -> Result<WebElement, Report> {
    page.element("#test-ca-log").await
}

/// Empties the log (a script click, which leaves focus where it is).
async fn clear(page: &Page<'_>) -> Result<(), Report> {
    page.element("#test-ca-clear")
        .await?
        .virtual_click()
        .await?;
    log(page).await?.wait_for_inner_text("").await?;
    Ok(())
}

/// Waits until the log's last entry starts with `expected` (dragging logs many changes).
async fn wait_for_last_log(page: &Page<'_>, expected: &str) -> Result<(), Report> {
    assert_that!(|| last_log(page))
        .eventually_ok()
        .satisfies(|entry| {
            entry.starts_with(expected);
        })
        .await;
    Ok(())
}

async fn last_log(page: &Page<'_>) -> Result<String, Report> {
    let log = log(page).await?.inner_text().await?;
    Ok(log.rsplit(',').next().unwrap_or_default().to_owned())
}

/// The inputs' values are the expected channel values, ±1.
async fn expect_channels(
    x: &WebElement,
    y: &WebElement,
    (expected_x, expected_y): (f64, f64),
) -> Result<(), Report> {
    assert_that!(x)
        .property("value")
        .await
        .some()
        .map_owned(|value| value.parse::<f64>())
        .ok()
        .is_close_to(expected_x, 1.0);
    assert_that!(y)
        .property("value")
        .await
        .some()
        .map_owned(|value| value.parse::<f64>())
        .ok()
        .is_close_to(expected_y, 1.0);
    Ok(())
}

/// The layers of the computed background of the area in `#id`.
async fn gradient_layers(page: &Page<'_>, id: &str) -> Result<Vec<String>, Report> {
    let area = page.element(format!("#{id} [role=group]")).await?;
    let background = area.css_value("background-image").await?;
    Ok(background
        .split(", linear-gradient(")
        .map(str::to_owned)
        .collect())
}

/// On a phone both inputs are tabbable and the y input isn't hidden (touch screen readers reach
/// each input on its own), also when the server rendered them for a desktop (useColorArea's
/// `isMobile`).
#[browser_test]
pub async fn both_inputs_reachable_on_phones(page: &Page<'_>) -> Result<(), Report> {
    page.emulate_platform(Platform::Android).await?;
    page.goto_path(PATH).await?;
    let (x, y) = inputs(page, "test-ca-default").await?;
    y.wait_for_attr("aria-hidden", None).await?;
    assert_that!(y).attribute("tabindex").await.is_none();
    assert_that!(x).attribute("tabindex").await.is_none();
    assert_that!(x).attribute("aria-hidden").await.is_none();
    Ok(())
}

/// The x and y inputs are labelled ranges from 0 to 255 with the color described as value text;
/// only the x input is tabbable, the y input is hidden; both are in the thumb inside the area
/// ("sets input props", "should render a color area with default class").
#[browser_test]
pub async fn input_props(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let (x, y) = inputs(page, "test-ca-default").await?;
    for input in [&x, &y] {
        assert_that!(input)
            .has_attribute("type")
            .await
            .is_equal_to("range");
        assert_that!(input)
            .has_attribute("aria-label")
            .await
            .is_equal_to("Color picker");
        assert_that!(input)
            .has_attribute("min")
            .await
            .is_equal_to("0");
        assert_that!(input)
            .has_attribute("max")
            .await
            .is_equal_to("255");
        assert_that!(input)
            .has_attribute("step")
            .await
            .is_equal_to("1");
    }
    assert_that!(x)
        .has_attribute("aria-valuetext")
        .await
        .is_equal_to("Red: 255, Green: 0, Blue: 255, light vibrant magenta");
    assert_that!(y)
        .has_attribute("aria-valuetext")
        .await
        .is_equal_to("Green: 0, Red: 255, Blue: 255, light vibrant magenta");
    assert_that!(x).attribute("tabindex").await.is_none();
    assert_that!(y)
        .has_attribute("tabindex")
        .await
        .is_equal_to("-1");
    assert_that!(y)
        .has_attribute("aria-hidden")
        .await
        .is_equal_to("true");
    assert_that!(
        page.count("#test-ca-default > .leptonic-ColorArea > .leptonic-ColorThumb > input")
            .await?
    )
    .is_equal_to(2);
    Ok(())
}

/// An area without a default value is white ("defaults uncontrolled" > "sets input props").
#[browser_test]
pub async fn white_by_default(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let (x, y) = inputs(page, "test-ca-white").await?;
    assert_that!(x)
        .has_attribute("aria-valuetext")
        .await
        .is_equal_to("Red: 255, Green: 255, Blue: 255, white");
    assert_that!(y)
        .has_attribute("aria-valuetext")
        .await
        .is_equal_to("Green: 255, Red: 255, Blue: 255, white");
    Ok(())
}

/// The inputs describe their own channel first, then the other axis, then the third channel:
/// blue × green, and an HSL area of lightness × saturation ("sets input props rgb", "sets input
/// props hsl").
#[browser_test]
pub async fn channel_order_in_value_texts(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let (x, y) = inputs(page, "test-ca-blue-green").await?;
    assert_that!(x)
        .has_attribute("aria-valuetext")
        .await
        .is_equal_to("Blue: 255, Green: 255, Red: 0, very light vibrant cyan");
    assert_that!(y)
        .has_attribute("aria-valuetext")
        .await
        .is_equal_to("Green: 255, Blue: 255, Red: 0, very light vibrant cyan");
    let (x, y) = inputs(page, "test-ca-hsl-swapped").await?;
    assert_that!(x)
        .has_attribute("aria-valuetext")
        .await
        .is_equal_to("Lightness: 50%, Saturation: 100%, Hue: 0°, vibrant red");
    assert_that!(y)
        .has_attribute("aria-valuetext")
        .await
        .is_equal_to("Saturation: 100%, Lightness: 50%, Hue: 0°, vibrant red");
    Ok(())
}

/// Tab focuses the x input and then leaves the area (the y input isn't tabbable); Shift+Tab
/// returns to the x input, whose value text is the full description ("defaults uncontrolled" >
/// "the slider is focusable").
#[browser_test]
pub async fn focusable(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let (x, _) = inputs(page, "test-ca-default").await?;
    let (shift_x, _) = inputs(page, "test-ca-shift").await?;
    page.element("#test-ca-before").await?.focus().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&x).await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&shift_x).await?;
    page.send_keys(Key::Shift + Key::Tab).await?;
    page.wait_for_focus(&x).await?;
    assert_that!(x)
        .has_attribute("aria-valuetext")
        .await
        .is_equal_to("Red: 255, Green: 0, Blue: 255, light vibrant magenta");
    Ok(())
}

/// Left/Right step the x channel and Up/Down the y channel, focusing and revealing the input of
/// the axis that moved ("left/right", "up/down").
#[browser_test]
pub async fn keyboard(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let (x, y) = inputs(page, "test-ca-default").await?;
    let log = log(page).await?;
    page.element("#test-ca-before").await?.focus().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&x).await?;
    page.send_keys(Key::Left).await?;
    log.wait_for_inner_text("change:FE00FF,end:FE00FF").await?;
    // After a keyboard change, both inputs name only their channel; the focused one is tabbable,
    // the other not, and neither is hidden.
    assert_that!(x)
        .has_attribute("aria-valuetext")
        .await
        .is_equal_to("Red: 254, light vibrant magenta");
    assert_that!(y)
        .has_attribute("aria-valuetext")
        .await
        .is_equal_to("Green: 0, light vibrant magenta");
    assert_that!(x).attribute("tabindex").await.is_none();
    assert_that!(x).attribute("aria-hidden").await.is_none();
    assert_that!(y)
        .has_attribute("tabindex")
        .await
        .is_equal_to("-1");
    assert_that!(y).attribute("aria-hidden").await.is_none();
    page.send_keys(Key::Right).await?;
    log.wait_for_inner_text("change:FE00FF,end:FE00FF,change:FF00FF,end:FF00FF")
        .await?;
    clear(page).await?;
    page.send_keys(Key::Up).await?;
    log.wait_for_inner_text("change:FF01FF,end:FF01FF").await?;
    // The input of the axis that moved has the focus, both are revealed.
    page.wait_for_focus(&y).await?;
    assert_that!(x).attribute("aria-hidden").await.is_none();
    clear(page).await?;
    Ok(())
}

/// Shift with an arrow key, Page Up/Down and Home/End change the channels by a page step
/// ("shiftleft/shiftright", "shiftup/shiftdown", "pageup/pagedown", "home/end").
#[browser_test]
pub async fn keyboard_steps(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let (shift_x, _) = inputs(page, "test-ca-shift").await?;
    let log = log(page).await?;
    shift_x.focus().await?;
    for (keys, expected) in [
        (Key::Shift + Key::Left, "DF00F0"),
        (Key::Shift + Key::Right, "F000F0"),
        (Key::Shift + Key::Up, "F011F0"),
        (Key::Shift + Key::Down, "F000F0"),
        (Key::PageUp.into(), "F011F0"),
        (Key::PageDown.into(), "F000F0"),
        (Key::Home.into(), "DF00F0"),
        (Key::End.into(), "F000F0"),
    ] {
        page.send_keys(keys).await?;
        log.wait_for_inner_text(&format!("change:{expected},end:{expected}"))
            .await?;
        clear(page).await?;
    }
    Ok(())
}

/// Pressing the area chooses the color at that point and focuses the x input, and dragging the
/// thumb moves the color along ("clicking on the area chooses the color at that point", "dragging
/// the thumb works").
#[browser_test]
pub async fn press_and_drag_thumb(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let (x, y) = inputs(page, "test-ca-default").await?;
    let area = page.element("#test-ca-default [role=group]").await?;
    // Pointer actions don't scroll: keep the area in view.
    area.scroll_into_view().await?;
    page.low_level()
        .driver()
        .action_chain()
        .move_to_element_with_offset(&area, -50, 50)
        .click()
        .perform()
        .await?;
    // The point a quarter in from the left and bottom (±1: sub-pixel positions).
    wait_for_last_log(page, "end:").await?;
    expect_channels(&x, &y, (64.0, 64.0)).await?;
    page.wait_for_focus(&x).await?;
    clear(page).await?;
    let thumb = page.element("#test-ca-default [role=presentation]").await?;
    page.low_level()
        .driver()
        .action_chain()
        .click_and_hold_element(&thumb)
        .move_by_offset(100, -100)
        .release()
        .perform()
        .await?;
    wait_for_last_log(page, "end:").await?;
    expect_channels(&x, &y, (192.0, 192.0)).await?;
    clear(page).await?;
    Ok(())
}

/// A press held on the area changes the color at once and ends the change only when released;
/// dragging on moves the color with the pointer, ending once with the dragged color, and focuses
/// the x input ("clicking and dragging on the track works").
#[browser_test]
pub async fn press_and_drag_area(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let (x, y) = inputs(page, "test-ca-default").await?;
    let area = page.element("#test-ca-default [role=group]").await?;
    area.scroll_into_view().await?;
    let held = area.press_and_hold_at(-50, 50).await?;
    wait_for_last_log(page, "change:").await?;
    page.wait_for_focus(&x).await?;
    page.settle().await?;
    // No end while the press is held.
    assert_that!(|| async {
        Ok::<_, Report>(log(page).await?.inner_text().await?.matches("end:").count())
    })
    .consistently_ok()
    .for_at_least(std::time::Duration::from_millis(100))
    .matches(eq(0))
    .await;
    expect_channels(&x, &y, (64.0, 64.0)).await?;
    held.move_by(100, -100).await?;
    assert_that!(|| async { Ok::<_, Report>((x.number_value().await?, y.number_value().await?)) })
        .eventually_ok()
        .satisfies(|channels| {
            channels.derive(|(x, _)| x).is_close_to(192.0, 1.0);
            channels.derive(|(_, y)| y).is_close_to(192.0, 1.0);
        })
        .await;
    held.release().await?;
    wait_for_last_log(page, "end:").await?;
    let entries = log(page).await?.inner_text().await?;
    assert_that!(entries.matches("end:").count()).is_equal_to(1);
    page.wait_for_focus(&x).await?;
    Ok(())
}

/// Pressing the thumb focuses the x input without changing the color ("dragging the thumb
/// works").
#[browser_test]
pub async fn thumb_press_focuses(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let (x, _) = inputs(page, "test-ca-default").await?;
    let thumb = page
        .element("#test-ca-default .leptonic-ColorThumb")
        .await?;
    thumb.scroll_into_view().await?;
    let held = thumb.press_and_hold().await?;
    page.wait_for_focus(&x).await?;
    page.settle().await?;
    log(page)
        .await?
        .inner_text_stays("", std::time::Duration::from_millis(100))
        .await?;
    held.release().await?;
    Ok(())
}

/// A disabled area and its thumb are `data-disabled` and its inputs are disabled, so Tab and
/// Shift+Tab skip it ("disabled", "should support disabled state").
#[browser_test]
pub async fn disabled(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let (disabled_x, disabled_y) = inputs(page, "test-ca-disabled").await?;
    assert_that!(disabled_x).enabled().await.is_false();
    assert_that!(disabled_y).enabled().await.is_false();
    for part in ["ColorArea", "ColorThumb"] {
        let element = page
            .element(format!("#test-ca-disabled .leptonic-{part}"))
            .await?;
        assert_that!(element)
            .has_attribute("data-disabled")
            .await
            .is_equal_to("true");
    }
    let a = page.element("#test-ca-a").await?;
    a.focus().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&page.element("#test-ca-b").await?)
        .await?;
    page.send_keys(Key::Shift + Key::Tab).await?;
    page.wait_for_focus(&a).await?;
    Ok(())
}

/// Dragging a disabled area's thumb, or pressing and dragging on the area, changes nothing and
/// doesn't focus its inputs ("dragging the thumb doesn't works when disabled", "clicking and
/// dragging on the track doesn't work when disabled").
#[browser_test]
pub async fn disabled_drag(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let (x, y) = inputs(page, "test-ca-disabled").await?;
    let body = page.element("body").await?;
    let thumb = page
        .element("#test-ca-disabled .leptonic-ColorThumb")
        .await?;
    thumb.scroll_into_view().await?;
    let held = thumb.press_and_hold().await?;
    held.move_by(-20, 20).await?;
    held.release().await?;
    let area = page.element("#test-ca-disabled [role=group]").await?;
    let held = area.press_and_hold().await?;
    held.move_by(10, 10).await?;
    held.release().await?;
    page.settle().await?;
    x.prop_stays("value", "255", std::time::Duration::from_millis(100))
        .await?;
    y.prop_stays("value", "0", std::time::Duration::from_millis(100))
        .await?;
    page.focus_stays(&body, std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// Holding Page Up steps the y channel a page per repeated keydown, from 0 to 51, and holding Page
/// Down back to 0 ("should support repeat keydown events when holding Page Up/Page Down").
#[browser_test]
pub async fn repeated_page_keys(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let (x, y) = inputs(page, "test-ca-red").await?;
    x.focus().await?;
    page.hold_key("PageUp", 2).await?;
    y.wait_for_prop("value", "51").await?;
    page.hold_key("PageDown", 2).await?;
    y.wait_for_prop("value", "0").await?;
    Ok(())
}

/// Tabbing to the x input shows a focus ring on the thumb (`data-focused`, `data-focus-visible`),
/// which goes when the focus leaves ("should support focus ring").
#[browser_test]
pub async fn thumb_focus_ring(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let thumb = page
        .element("#test-ca-default .leptonic-ColorThumb")
        .await?;
    assert_that!(thumb)
        .attribute("data-focus-visible")
        .await
        .is_none();
    page.element("#test-ca-before").await?.focus().await?;
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
        .element("#test-ca-default .leptonic-ColorThumb")
        .await?;
    thumb.scroll_into_view().await?;
    assert_that!(thumb)
        .attribute("data-hovered")
        .await
        .is_none();
    thumb.hover().await?;
    thumb.wait_for_attr("data-hovered", Some("true")).await?;
    page.element("#test-ca-before").await?.hover().await?;
    thumb.wait_for_attr("data-hovered", None).await?;
    Ok(())
}

/// The thumb is `data-dragging` while it is pressed ("should support dragging state").
#[browser_test]
pub async fn thumb_dragging(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let thumb = page
        .element("#test-ca-default .leptonic-ColorThumb")
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

/// `classes` add to the area's default class, other attributes reach the area and the thumb,
/// `form` and `aria_details` reach the inputs, and `x_channel_step` sets the x step ("should
/// render a slider with custom class", "should support DOM props", "should support form prop").
#[browser_test]
pub async fn classes_attributes_and_form(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let area = page.element("#test-ca-props .leptonic-ColorArea").await?;
    assert_that!(area)
        .has_attribute("class")
        .await
        .is_equal_to("leptonic-ColorArea custom-area");
    assert_that!(area)
        .has_attribute("data-foo")
        .await
        .is_equal_to("area");
    let thumb = page.element("#test-ca-props .leptonic-ColorThumb").await?;
    assert_that!(thumb)
        .has_attribute("data-bar")
        .await
        .is_equal_to("thumb");
    let (x, y) = inputs(page, "test-ca-props").await?;
    for input in [&x, &y] {
        assert_that!(input)
            .has_attribute("form")
            .await
            .is_equal_to("test-ca-other-form");
        assert_that!(input)
            .has_attribute("aria-details")
            .await
            .is_equal_to("test-ca-details");
    }
    assert_that!(x).has_attribute("step").await.is_equal_to("5");
    x.focus().await?;
    page.send_keys(Key::Right).await?;
    x.wait_for_prop("value", "5").await?;
    Ok(())
}

/// An `aria-label` labels the group and its inputs, joined with the default label; with
/// `aria-labelledby`, the group is labelled by that element and each input by itself and it
/// ("should support a custom aria-label", "should support a custom aria-labelledby").
#[browser_test]
pub async fn labelling(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let (label_x, label_y) = inputs(page, "test-ca-label").await?;
    for input in [&label_x, &label_y] {
        assert_that!(input)
            .has_attribute("aria-label")
            .await
            .is_equal_to("Color hue, Color picker");
        assert_that!(input)
            .attribute("aria-labelledby")
            .await
            .is_none();
    }
    let group = page.element("#test-ca-label [role=group]").await?;
    assert_that!(group)
        .has_attribute("aria-label")
        .await
        .is_equal_to("Color hue, Color picker");
    let (lb_x, lb_y) = inputs(page, "test-ca-labelledby").await?;
    for input in [&lb_x, &lb_y] {
        let id = input.id().await?.unwrap_or_default();
        assert_that!(input)
            .has_attribute("aria-labelledby")
            .await
            .is_equal_to(format!("{id} test-ca-label-id"));
    }
    let group = page.element("#test-ca-labelledby [role=group]").await?;
    assert_that!(group)
        .has_attribute("aria-labelledby")
        .await
        .is_equal_to("test-ca-label-id");
    assert_that!(group).attribute("aria-label").await.is_none();
    Ok(())
}

/// The inputs carry their form names, and resetting the form restores the default color ("supports
/// form name", "supports form reset").
#[browser_test]
pub async fn forms(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let (form_x, form_y) = inputs(page, "test-ca-form").await?;
    assert_that!(form_x)
        .has_attribute("name")
        .await
        .is_equal_to("red");
    assert_that!(form_y)
        .has_attribute("name")
        .await
        .is_equal_to("green");
    form_x.focus().await?;
    page.send_keys(Key::Right).await?;
    form_x
        .wait_for_attr(
            "aria-valuetext",
            Some("Red: 11, very dark grayish cyan blue"),
        )
        .await?;
    page.element("#test-ca-reset").await?.click().await?;
    // Focus left the area: the full text again.
    form_x
        .wait_for_attr(
            "aria-valuetext",
            Some("Red: 10, Green: 20, Blue: 30, very dark grayish cyan blue"),
        )
        .await?;
    Ok(())
}

/// An HSV area's inputs range from 0 to 1 in steps of 0.01 and describe saturation and brightness
/// as percentages.
#[browser_test]
pub async fn hsv(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let (x, y) = inputs(page, "test-ca-hsv").await?;
    for input in [&x, &y] {
        assert_that!(input)
            .has_attribute("min")
            .await
            .is_equal_to("0");
        assert_that!(input)
            .has_attribute("max")
            .await
            .is_equal_to("1");
        assert_that!(input)
            .has_attribute("step")
            .await
            .is_equal_to("0.01");
    }
    assert_that!(x)
        .has_attribute("aria-valuetext")
        .await
        .starts_with("Saturation: 50%, Brightness: 50%, Hue: 0°, ");
    x.focus().await?;
    page.send_keys(Key::Right).await?;
    x.wait_for_prop("value", "0.51").await?;
    assert_that!(x)
        .has_attribute("aria-valuetext")
        .await
        .starts_with("Saturation: 51%, ");
    Ok(())
}

/// The gradient of the color space's later channel is the top background layer, whichever axis it
/// is on (as in react-aria's `useColorAreaGradient`).
#[browser_test]
pub async fn gradients(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let layers = gradient_layers(page, "test-ca-hsv").await?;
    assert_that!(layers[0].as_str()).contains("rgb(0, 0, 0), rgba(0, 0, 0, 0)");
    let layers = gradient_layers(page, "test-ca-hsv-swapped").await?;
    assert_that!(layers[0].as_str())
        .contains("rgb(0, 0, 0), rgba(0, 0, 0, 0)")
        .contains("to right");
    assert_that!(layers[1].as_str()).contains("to top");
    let layers = gradient_layers(page, "test-ca-hsl-swapped").await?;
    assert_that!(layers[0].as_str()).contains("rgb(0, 0, 0), rgba(0, 0, 0, 0), rgb(255, 255, 255)");
    Ok(())
}

/// In a right-to-left locale, x grows to the left: a press a quarter in from the left chooses 75%,
/// and ArrowLeft increases x ("left/right RTL").
#[browser_test]
pub async fn right_to_left(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let (rtl_x, rtl_y) = inputs(page, "test-ca-rtl").await?;
    let area = page.element("#test-ca-rtl [role=group]").await?;
    area.scroll_into_view().await?;
    page.low_level()
        .driver()
        .action_chain()
        .move_to_element_with_offset(&area, -50, 50)
        .click()
        .perform()
        .await?;
    assert_that!(|| async {
        Ok::<_, Report>((rtl_x.number_value().await?, rtl_y.number_value().await?))
    })
    .eventually_ok()
    .satisfies(|channels| {
        channels.derive(|(x, _)| x).is_close_to(191.0, 1.0);
        channels.derive(|(_, y)| y).is_close_to(64.0, 1.0);
    })
    .await;
    let before = rtl_x.number_value().await?;
    rtl_x.focus().await?;
    page.send_keys(Key::Left).await?;
    rtl_x
        .wait_for_prop("value", &(before + 1.0).to_string())
        .await?;
    Ok(())
}

/// In a right-to-left locale, the horizontal keys are mirrored: Right decreases x, Shift+Right and
/// End decrease it by a page, Home increases it by a page; the vertical keys are unchanged
/// (`it.each` RTL: "left/right", "shiftleft/shiftright", "home/end", "up/down", "pageup/pagedown").
#[browser_test]
pub async fn right_to_left_keys(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let (x, _) = inputs(page, "test-ca-rtl-log").await?;
    let log = page.element("#test-ca-rtl-log-entries").await?;
    x.focus().await?;
    let mut entries = Vec::new();
    for (keys, expected) in [
        (Key::Right.into(), "EF00F0"),
        (Key::Left.into(), "F000F0"),
        (Key::Shift + Key::Right, "DF00F0"),
        (Key::Shift + Key::Left, "F000F0"),
        (Key::End.into(), "DF00F0"),
        (Key::Home.into(), "F000F0"),
        (Key::Up.into(), "F001F0"),
        (Key::Down.into(), "F000F0"),
        (Key::PageUp.into(), "F011F0"),
        (Key::PageDown.into(), "F000F0"),
    ] {
        page.send_keys(keys).await?;
        entries.push(format!("rtl:{expected}"));
        log.wait_for_inner_text(&entries.join(",")).await?;
    }
    Ok(())
}

/// Setting an input's value with an `input` event, as assistive technology does, changes the
/// color, and the keyboard then steps on from the new value.
#[browser_test]
pub async fn input_event(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let (input_x, _) = inputs(page, "test-ca-input").await?;
    input_x.virtual_input("100").await?;
    page.element("#test-ca-input-log")
        .await?
        .wait_for_inner_text("input:640000")
        .await?;
    input_x.focus().await?;
    page.send_keys(Key::Right).await?;
    input_x.wait_for_prop("value", "101").await?;
    Ok(())
}

/// The thumb shows the color without its alpha (react-aria's `getDisplayColor`).
#[browser_test]
pub async fn thumb_without_alpha(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let thumb = page.element("#test-ca-alpha .leptonic-ColorThumb").await?;
    assert_that!(thumb.css_value("background-color").await?).is_equal_to("rgba(255, 0, 255, 1)");
    Ok(())
}

/// A thumb mounted again (inside a `<Show>`) renders its inputs and responds to the keyboard.
#[browser_test]
pub async fn mounted_again(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let toggle = page.element("#test-ca-toggle").await?;
    toggle.click().await?;
    page.wait_for_count("#test-ca-show input[type=range]", 0)
        .await?;
    toggle.click().await?;
    page.wait_for_count("#test-ca-show input[type=range]", 2)
        .await?;
    let (shown_x, _) = inputs(page, "test-ca-show").await?;
    shown_x.focus().await?;
    page.send_keys(Key::Right).await?;
    shown_x.wait_for_prop("value", "11").await?;
    Ok(())
}
