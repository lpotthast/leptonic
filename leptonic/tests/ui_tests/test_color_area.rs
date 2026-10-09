// Upstream: @adobe/react-spectrum/test/color/ColorArea.test.tsx @ 99e6102368
// Upstream: react-aria-components/test/ColorArea.test.js @ 99e6102368
//! The `ColorArea`/`ColorThumb` atoms: the hidden inputs' attributes and labelling, keyboard
//! steps (arrows, Shift, PageUp/PageDown, Home/End), pressing and dragging, disabled areas,
//! forms.
//!
//! HSV and HSL areas (their gradients' layer order with swapped axes, percentages), right to
//! left, the inputs' `value` property and `input` event (assistive technology), the thumb's
//! color without alpha, and a thumb that mounts again.
use assertr::prelude::*;
use browser_test::thirtyfour::prelude::*;
use rootcause::Report;

use crate::pages::{ElementActions, Page, PageActions};

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

/// The value of a range input as a number.
async fn number(input: &WebElement) -> Result<f64, Report> {
    Ok(input.value().await?.unwrap_or_default().parse()?)
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
    assert_that!(number(x).await?).is_close_to(expected_x, 1.0);
    assert_that!(number(y).await?).is_close_to(expected_y, 1.0);
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

/// "sets input props".
pub async fn input_props(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/color-area").await?;
    let (x, y) = inputs(page, "test-ca-default").await?;
    for input in [&x, &y] {
        assert_that!(input.attr("type").await?)
            .get_some()
            .is_equal_to("range");
        assert_that!(input.attr("aria-label").await?)
            .get_some()
            .is_equal_to("Color picker");
        assert_that!(input.attr("min").await?)
            .get_some()
            .is_equal_to("0");
        assert_that!(input.attr("max").await?)
            .get_some()
            .is_equal_to("255");
        assert_that!(input.attr("step").await?)
            .get_some()
            .is_equal_to("1");
    }
    assert_that!(x.attr("aria-valuetext").await?)
        .get_some()
        .is_equal_to("Red: 255, Green: 0, Blue: 255, light vibrant magenta");
    assert_that!(y.attr("aria-valuetext").await?)
        .get_some()
        .is_equal_to("Green: 0, Red: 255, Blue: 255, light vibrant magenta");
    assert_that!(x.attr("tabindex").await?).is_none();
    assert_that!(y.attr("tabindex").await?)
        .get_some()
        .is_equal_to("-1");
    assert_that!(y.attr("aria-hidden").await?)
        .get_some()
        .is_equal_to("true");
    Ok(())
}

/// Keyboard: "left/right", "up/down".
pub async fn keyboard(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/color-area").await?;
    let (x, y) = inputs(page, "test-ca-default").await?;
    let log = log(page).await?;
    page.element("#test-ca-before").await?.focus().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&x).await?;
    page.send_keys(Key::Left).await?;
    log.wait_for_inner_text("change:FE00FF,end:FE00FF").await?;
    assert_that!(x.attr("aria-valuetext").await?)
        .get_some()
        .is_equal_to("Red: 254, light vibrant magenta");
    page.send_keys(Key::Right).await?;
    log.wait_for_inner_text("change:FE00FF,end:FE00FF,change:FF00FF,end:FF00FF")
        .await?;
    clear(page).await?;
    page.send_keys(Key::Up).await?;
    log.wait_for_inner_text("change:FF01FF,end:FF01FF").await?;
    // The input of the axis that moved has the focus, both are revealed.
    page.wait_for_focus(&y).await?;
    assert_that!(x.attr("aria-hidden").await?).is_none();
    clear(page).await?;
    Ok(())
}

/// "shiftleft/shiftright", "shiftup/shiftdown", "pageup/pagedown", "home/end".
pub async fn keyboard_steps(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/color-area").await?;
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

/// "clicking on the area chooses the color at that point", then dragging the thumb.
pub async fn press_and_drag(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/color-area").await?;
    let (x, y) = inputs(page, "test-ca-default").await?;
    let area = page.element("#test-ca-default [role=group]").await?;
    // Pointer actions don't scroll: keep the area in view.
    area.scroll_into_view().await?;
    page.driver
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
    page.driver
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

/// "disabled": not focusable, no events.
pub async fn disabled(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/color-area").await?;
    let (disabled_x, disabled_y) = inputs(page, "test-ca-disabled").await?;
    assert_that!(disabled_x.is_enabled().await?).is_false();
    assert_that!(disabled_y.is_enabled().await?).is_false();
    page.element("#test-ca-a").await?.focus().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&page.element("#test-ca-b").await?)
        .await?;
    Ok(())
}

/// Labelling: "should support a custom aria-label", "... aria-labelledby".
pub async fn labelling(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/color-area").await?;
    let (label_x, label_y) = inputs(page, "test-ca-label").await?;
    for input in [&label_x, &label_y] {
        assert_that!(input.attr("aria-label").await?)
            .get_some()
            .is_equal_to("Color hue, Color picker");
        assert_that!(input.attr("aria-labelledby").await?).is_none();
    }
    let group = page.element("#test-ca-label [role=group]").await?;
    assert_that!(group.attr("aria-label").await?)
        .get_some()
        .is_equal_to("Color hue, Color picker");
    let (lb_x, lb_y) = inputs(page, "test-ca-labelledby").await?;
    for input in [&lb_x, &lb_y] {
        let id = input.id().await?.unwrap_or_default();
        assert_that!(input.attr("aria-labelledby").await?)
            .get_some()
            .is_equal_to(format!("{id} test-ca-label-id"));
    }
    let group = page.element("#test-ca-labelledby [role=group]").await?;
    assert_that!(group.attr("aria-labelledby").await?)
        .get_some()
        .is_equal_to("test-ca-label-id");
    assert_that!(group.attr("aria-label").await?).is_none();
    Ok(())
}

/// "supports form name", "supports form reset".
pub async fn forms(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/color-area").await?;
    let (form_x, form_y) = inputs(page, "test-ca-form").await?;
    assert_that!(form_x.attr("name").await?)
        .get_some()
        .is_equal_to("red");
    assert_that!(form_y.attr("name").await?)
        .get_some()
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

/// An HSV area: saturation and brightness from 0 to 1, formatted as percentages.
pub async fn hsv(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/color-area").await?;
    let (x, y) = inputs(page, "test-ca-hsv").await?;
    for input in [&x, &y] {
        assert_that!(input.attr("min").await?)
            .get_some()
            .is_equal_to("0");
        assert_that!(input.attr("max").await?)
            .get_some()
            .is_equal_to("1");
        assert_that!(input.attr("step").await?)
            .get_some()
            .is_equal_to("0.01");
    }
    assert_that!(x.attr("aria-valuetext").await?)
        .get_some()
        .starts_with("Saturation: 50%, Brightness: 50%, Hue: 0°, ");
    x.focus().await?;
    page.send_keys(Key::Right).await?;
    x.wait_for_prop("value", "0.51").await?;
    assert_that!(x.attr("aria-valuetext").await?)
        .get_some()
        .starts_with("Saturation: 51%, ");
    Ok(())
}

/// Gradients: the space's later channel on top (react-aria's `useColorAreaGradient`), whichever
/// axis it is on.
pub async fn gradients(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/color-area").await?;
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

/// Right to left: x grows to the left (a press a quarter in from the left is 75%), and
/// ArrowLeft increases it.
pub async fn right_to_left(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/color-area").await?;
    let (rtl_x, rtl_y) = inputs(page, "test-ca-rtl").await?;
    let area = page.element("#test-ca-rtl [role=group]").await?;
    area.scroll_into_view().await?;
    page.driver
        .action_chain()
        .move_to_element_with_offset(&area, -50, 50)
        .click()
        .perform()
        .await?;
    assert_that!(|| async { Ok::<_, Report>((number(&rtl_x).await?, number(&rtl_y).await?)) })
        .eventually_ok()
        .satisfies(|channels| {
            channels.derive(|(x, _)| x).is_close_to(191.0, 1.0);
            channels.derive(|(_, y)| y).is_close_to(64.0, 1.0);
        })
        .await;
    let before = number(&rtl_x).await?;
    rtl_x.focus().await?;
    page.send_keys(Key::Left).await?;
    rtl_x
        .wait_for_prop("value", &(before + 1.0).to_string())
        .await?;
    Ok(())
}

/// The `input` event (assistive technology sets the value), then the keyboard: the value property
/// follows the state.
pub async fn input_event(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/color-area").await?;
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
pub async fn thumb_without_alpha(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/color-area").await?;
    let thumb = page.element("#test-ca-alpha .leptonic-ColorThumb").await?;
    assert_that!(thumb.css_value("background-color").await?).is_equal_to("rgba(255, 0, 255, 1)");
    Ok(())
}

/// A thumb mounted again (inside a `<Show>`) renders and works.
pub async fn mounted_again(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/color-area").await?;
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
