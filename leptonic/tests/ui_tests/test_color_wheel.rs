// Upstream: @adobe/react-spectrum/test/color/ColorWheel.test.tsx @ 99e6102368
// Upstream: react-aria/test/color/useColorWheel.test.tsx @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{By, Key, WebDriver, WebElement},
};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

/// The `ColorWheel` atoms: input attributes and labelling, keyboard steps wrapping around 0°,
/// a press on the ring (0° at 3 o'clock, clockwise), disabled wheels, forms.
pub struct ColorWheelTests {}

#[async_trait]
impl BrowserTest<str> for ColorWheelTests {
    fn name(&self) -> Cow<'_, str> {
        "color_wheel_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/color-wheel").await?;

        // "sets input props"; the hue channel names a wheel without labels.
        let wheel = input(&page, "test-cw-default").await?;
        assert_that!(attr(&wheel, "type").await?).is_equal_to(Some("range".to_owned()));
        assert_that!(attr(&wheel, "min").await?).is_equal_to(Some("0".to_owned()));
        assert_that!(attr(&wheel, "max").await?).is_equal_to(Some("360".to_owned()));
        assert_that!(attr(&wheel, "step").await?).is_equal_to(Some("1".to_owned()));
        assert_that!(attr(&wheel, "aria-label").await?).is_equal_to(Some("Hue".to_owned()));

        // Keyboard: arrows step, Shift and PageUp/PageDown by 15°, around 0°.
        page.element("test-cw-before").await?.focus().await?;
        page.press_tab().await?;
        page.wait_for_active_id(&attr(&wheel, "id").await?.unwrap_or_default())
            .await?;
        for (keys, expected) in [
            (Key::Right.into(), "1"),
            (Key::Left.into(), "0"),
            (Key::Left.into(), "359"),
            (Key::Up.into(), "0"),
            (Key::Shift + Key::Right, "15"),
            (Key::PageDown.into(), "0"),
            (Key::PageDown.into(), "345"),
        ] {
            page.send_keys_to_active(keys).await?;
            page.wait_for_text("test-cw-log", &format!("change:{expected},end:{expected}"))
                .await?;
            clear(&page).await?;
        }

        // A press on the ring below the center: 90°.
        let track = page.css("#test-cw-default > div > div").await?;
        driver
            .execute(
                "arguments[0].scrollIntoView({block: 'center'});",
                vec![track.to_json()?],
            )
            .await?;
        driver
            .action_chain()
            .move_to_element_with_offset(&track, 0, 87)
            .click()
            .perform()
            .await?;
        page.wait_for_text("test-cw-log", "change:90,end:90")
            .await?;

        // "disabled".
        let disabled = input(&page, "test-cw-disabled").await?;
        assert_that!(attr(&disabled, "disabled").await?.is_some()).is_true();
        page.element("test-cw-a").await?.focus().await?;
        page.press_tab().await?;
        page.wait_for_active_id("test-cw-b").await?;

        // An `aria_label`; "supports form name", "supports form reset".
        let form = input(&page, "test-cw-form").await?;
        assert_that!(attr(&form, "aria-label").await?).is_equal_to(Some("Tint".to_owned()));
        assert_that!(attr(&form, "name").await?).is_equal_to(Some("hue".to_owned()));
        form.focus().await?;
        page.send_keys_to_active(Key::Right).await?;
        wait_for_value(&form, "11").await?;
        page.element("test-cw-reset").await?.click().await?;
        wait_for_value(&form, "10").await?;

        page.expect_no_page_errors().await
    }
}

/// Dragging the thumb, the input's `value` property and `input` event (assistive technology),
/// RGB colors (the hue of their HSL form), and parts that mount again.
pub struct ColorWheelInteractionTests {}

#[async_trait]
impl BrowserTest<str> for ColorWheelInteractionTests {
    fn name(&self) -> Cow<'_, str> {
        "color_wheel_interaction_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/color-wheel").await?;

        // Dragging the thumb (at 0°, 3 o'clock, 87 pixels from the center) to below the center:
        // 90°.
        let wheel = input(&page, "test-cw-default").await?;
        let thumb = page.css("#test-cw-default .leptonic-ColorThumb").await?;
        driver
            .execute(
                "arguments[0].scrollIntoView({block: 'center'});",
                vec![thumb.to_json()?],
            )
            .await?;
        driver
            .action_chain()
            .click_and_hold_element(&thumb)
            .move_by_offset(-87, 87)
            .release()
            .perform()
            .await?;
        wait_for_last_log(&page, "end:90").await?;
        page.wait_for_focus_on(&wheel, "the wheel's input").await?;
        clear(&page).await?;

        // The `input` event (assistive technology sets the value), then the keyboard: the value
        // property follows the state.
        driver
            .execute(
                "arguments[0].value = '200'; \
                 arguments[0].dispatchEvent(new Event('input', {bubbles: true}));",
                vec![wheel.to_json()?],
            )
            .await?;
        wait_for_last_log(&page, "change:200").await?;
        wait_for_value(&wheel, "200").await?;
        assert_that!(attr(&wheel, "aria-valuetext").await?.unwrap_or_default())
            .starts_with("200°, ");
        wheel.focus().await?;
        page.send_keys_to_active(Key::Right).await?;
        wait_for_value(&wheel, "201").await?;

        // An RGB color: the wheel changes the hue of its HSL form and keeps the RGB type.
        let rgb = input(&page, "test-cw-rgb").await?;
        rgb.focus().await?;
        page.send_keys_to_active(Key::Right).await?;
        page.wait_for_text("test-cw-rgb-log", "rgb:FF0400").await?;
        wait_for_value(&rgb, "1").await?;
        // A gray has no hue in RGB: the wheel keeps the hue it set (the color stays gray).
        let gray = input(&page, "test-cw-gray").await?;
        gray.focus().await?;
        page.send_keys_to_active(Key::Right).await?;
        wait_for_value(&gray, "1").await?;
        page.send_keys_to_active(Key::Right).await?;
        wait_for_value(&gray, "2").await?;
        assert_that!(page.element("test-cw-rgb-log").await?.text().await?)
            .is_equal_to("rgb:FF0400".to_owned());

        // Track and thumb mounted again (inside a `<Show>`) render and work.
        let toggle = page.element("test-cw-toggle").await?;
        toggle.click().await?;
        page.wait_for_count("#test-cw-show input[type=range]", 0)
            .await?;
        toggle.click().await?;
        page.wait_for_count("#test-cw-show input[type=range]", 1)
            .await?;
        let shown = input(&page, "test-cw-show").await?;
        wait_for_value(&shown, "20").await?;
        shown.focus().await?;
        page.send_keys_to_active(Key::Right).await?;
        wait_for_value(&shown, "21").await?;

        page.expect_no_page_errors().await
    }
}

/// Waits until the input's `value` property is `expected`.
async fn wait_for_value(input: &WebElement, expected: &str) -> Result<(), Report> {
    let mut last = None;
    for _ in 0..100 {
        last = input.prop("value").await?;
        if last.as_deref() == Some(expected) {
            return Ok(());
        }
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }
    Err(rootcause::report!(
        "the input's value is {last:?}, not {expected:?}"
    ))
}

/// Waits until the log's last entry is `expected` (dragging logs many changes).
async fn wait_for_last_log(page: &Page<'_>, expected: &str) -> Result<(), Report> {
    let mut last = String::new();
    for _ in 0..100 {
        let log = page.element("test-cw-log").await?.text().await?;
        last = log.rsplit(',').next().unwrap_or_default().to_owned();
        if last == expected {
            return Ok(());
        }
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }
    Err(rootcause::report!(
        "the last log entry is {last:?}, not {expected:?}"
    ))
}

async fn input(page: &Page<'_>, id: &str) -> Result<WebElement, Report> {
    Ok(page
        .driver
        .find(By::Css(format!("#{id} input[type=range]")))
        .await?)
}

async fn attr(element: &WebElement, name: &str) -> Result<Option<String>, Report> {
    Ok(element.attr(name).await?)
}

async fn clear(page: &Page<'_>) -> Result<(), Report> {
    page.driver
        .execute("document.getElementById('test-cw-clear').click();", vec![])
        .await?;
    page.wait_for_text("test-cw-log", "").await
}
