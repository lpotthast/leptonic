// Upstream: @adobe/react-spectrum/test/color/ColorWheel.test.tsx @ 99e6102368
// Upstream: react-aria/test/color/useColorWheel.test.tsx @ 99e6102368
//! The `ColorWheel` atoms: input attributes and labelling, keyboard steps wrapping around 0°,
//! a press on the ring (0° at 3 o'clock, clockwise), disabled wheels, forms.
//!
//! Dragging the thumb, the input's `value` property and `input` event (assistive technology),
//! RGB colors (the hue of their HSL form), and parts that mount again.
use assertr::prelude::*;
use browser_test::thirtyfour::prelude::*;
use rootcause::Report;

use crate::{
    pages::{ElementActions, Page, PageActions},
    polling::wait_for,
};

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
    wait_for("the last entry of the change log")
        .observing(|| async {
            Ok(log
                .inner_text()
                .await?
                .rsplit(',')
                .next()
                .unwrap_or_default()
                .to_owned())
        })
        .to_be_equal_to(expected)
        .await?;
    Ok(())
}

/// "sets input props"; the hue channel names a wheel without labels.
pub async fn input_props(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/color-wheel").await?;
    let wheel = input(page, "test-cw-default").await?;
    assert_that!(wheel.attr("type").await?)
        .get_some()
        .is_equal_to("range");
    assert_that!(wheel.attr("min").await?)
        .get_some()
        .is_equal_to("0");
    assert_that!(wheel.attr("max").await?)
        .get_some()
        .is_equal_to("360");
    assert_that!(wheel.attr("step").await?)
        .get_some()
        .is_equal_to("1");
    assert_that!(wheel.attr("aria-label").await?)
        .get_some()
        .is_equal_to("Hue");
    Ok(())
}

/// Keyboard: arrows step, Shift and PageUp/PageDown by 15°, around 0°.
pub async fn keyboard(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/color-wheel").await?;
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
        (Key::Shift + Key::Right, "15"),
        (Key::PageDown.into(), "0"),
        (Key::PageDown.into(), "345"),
    ] {
        page.send_keys(keys).await?;
        log.wait_for_inner_text(&format!("change:{expected},end:{expected}"))
            .await?;
        clear(page).await?;
    }
    Ok(())
}

/// A press on the ring below the center: 90°.
pub async fn ring_press(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/color-wheel").await?;
    let track = page.element("#test-cw-default > div > div").await?;
    track.scroll_into_view().await?;
    page.driver
        .action_chain()
        .move_to_element_with_offset(&track, 0, 87)
        .click()
        .perform()
        .await?;
    log(page)
        .await?
        .wait_for_inner_text("change:90,end:90")
        .await?;
    Ok(())
}

/// "disabled".
pub async fn disabled(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/color-wheel").await?;
    let disabled = input(page, "test-cw-disabled").await?;
    assert_that!(disabled.is_enabled().await?).is_false();
    page.element("#test-cw-a").await?.focus().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&page.element("#test-cw-b").await?)
        .await?;
    Ok(())
}

/// An `aria_label`; "supports form name", "supports form reset".
pub async fn forms(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/color-wheel").await?;
    let form = input(page, "test-cw-form").await?;
    assert_that!(form.attr("aria-label").await?)
        .get_some()
        .is_equal_to("Tint");
    assert_that!(form.attr("name").await?)
        .get_some()
        .is_equal_to("hue");
    form.focus().await?;
    page.send_keys(Key::Right).await?;
    form.wait_for_prop("value", "11").await?;
    page.element("#test-cw-reset").await?.click().await?;
    form.wait_for_prop("value", "10").await?;
    Ok(())
}

/// Dragging the thumb (at 0°, 3 o'clock, 87 pixels from the center) to below the center: 90°.
pub async fn drag_thumb(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/color-wheel").await?;
    let wheel = input(page, "test-cw-default").await?;
    let thumb = page
        .element("#test-cw-default .leptonic-ColorThumb")
        .await?;
    thumb.scroll_into_view().await?;
    page.driver
        .action_chain()
        .click_and_hold_element(&thumb)
        .move_by_offset(-87, 87)
        .release()
        .perform()
        .await?;
    wait_for_last_log(page, "end:90").await?;
    page.wait_for_focus(&wheel).await?;
    clear(page).await?;
    Ok(())
}

/// The `input` event (assistive technology sets the value), then the keyboard: the value property
/// follows the state.
pub async fn input_event(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/color-wheel").await?;
    let wheel = input(page, "test-cw-default").await?;
    wheel.virtual_input("200").await?;
    wait_for_last_log(page, "change:200").await?;
    wheel.wait_for_prop("value", "200").await?;
    assert_that!(wheel.attr("aria-valuetext").await?)
        .get_some()
        .starts_with("200°, ");
    wheel.focus().await?;
    page.send_keys(Key::Right).await?;
    wheel.wait_for_prop("value", "201").await?;
    Ok(())
}

/// An RGB color: the wheel changes the hue of its HSL form and keeps the RGB type. A gray has no
/// hue in RGB: the wheel keeps the hue it set (the color stays gray).
pub async fn rgb_colors(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/color-wheel").await?;
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
        .inner_text_stays("rgb:FF0400")
        .await?;
    Ok(())
}

/// Track and thumb mounted again (inside a `<Show>`) render and work.
pub async fn mounted_again(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/color-wheel").await?;
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
