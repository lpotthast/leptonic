// Upstream: @adobe/react-spectrum/test/color/ColorArea.test.tsx @ 99e6102368
// Upstream: react-aria-components/test/ColorArea.test.js @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{Key, WebDriver, WebElement},
};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

/// The `ColorArea`/`ColorThumb` atoms: the hidden inputs' attributes and labelling, keyboard
/// steps (arrows, Shift, PageUp/PageDown, Home/End), pressing and dragging, disabled areas,
/// forms.
pub struct ColorAreaTests {}

#[async_trait]
impl BrowserTest<str> for ColorAreaTests {
    fn name(&self) -> Cow<'_, str> {
        "color_area_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/color-area").await?;

        // "sets input props".
        let (x, y) = inputs(&page, "test-ca-default").await?;
        for input in [&x, &y] {
            assert_that!(attr(input, "type").await?).is_equal_to(Some("range".to_owned()));
            assert_that!(attr(input, "aria-label").await?)
                .is_equal_to(Some("Color picker".to_owned()));
            assert_that!(attr(input, "min").await?).is_equal_to(Some("0".to_owned()));
            assert_that!(attr(input, "max").await?).is_equal_to(Some("255".to_owned()));
            assert_that!(attr(input, "step").await?).is_equal_to(Some("1".to_owned()));
        }
        assert_that!(attr(&x, "aria-valuetext").await?).is_equal_to(Some(
            "Red: 255, Green: 0, Blue: 255, light vibrant magenta".to_owned(),
        ));
        assert_that!(attr(&y, "aria-valuetext").await?).is_equal_to(Some(
            "Green: 0, Red: 255, Blue: 255, light vibrant magenta".to_owned(),
        ));
        assert_that!(attr(&x, "tabindex").await?).is_none();
        assert_that!(attr(&y, "tabindex").await?).is_equal_to(Some("-1".to_owned()));
        assert_that!(attr(&y, "aria-hidden").await?).is_equal_to(Some("true".to_owned()));

        // Keyboard: "left/right", "up/down".
        page.element("test-ca-before").await?.focus().await?;
        page.press_tab().await?;
        expect_active(&page, &x).await?;
        page.send_keys_to_active(Key::Left).await?;
        page.wait_for_text("test-ca-log", "change:FE00FF,end:FE00FF")
            .await?;
        assert_that!(attr(&x, "aria-valuetext").await?)
            .is_equal_to(Some("Red: 254, light vibrant magenta".to_owned()));
        page.send_keys_to_active(Key::Right).await?;
        page.wait_for_text(
            "test-ca-log",
            "change:FE00FF,end:FE00FF,change:FF00FF,end:FF00FF",
        )
        .await?;
        clear(&page).await?;
        page.send_keys_to_active(Key::Up).await?;
        page.wait_for_text("test-ca-log", "change:FF01FF,end:FF01FF")
            .await?;
        // The input of the axis that moved has the focus, both are revealed.
        expect_active(&page, &y).await?;
        assert_that!(attr(&x, "aria-hidden").await?).is_none();
        clear(&page).await?;

        // "shiftleft/shiftright", "shiftup/shiftdown", "pageup/pagedown", "home/end".
        let (shift_x, _) = inputs(&page, "test-ca-shift").await?;
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
            page.send_keys_to_active(keys).await?;
            page.wait_for_text("test-ca-log", &format!("change:{expected},end:{expected}"))
                .await?;
            clear(&page).await?;
        }

        // "clicking on the area chooses the color at that point", then dragging the thumb.
        let area = page.css("#test-ca-default [role=group]").await?;
        // Pointer actions don't scroll: keep the area in view.
        driver
            .execute(
                "arguments[0].scrollIntoView({block: 'center'});",
                vec![area.to_json()?],
            )
            .await?;
        driver
            .action_chain()
            .move_to_element_with_offset(&area, -50, 50)
            .click()
            .perform()
            .await?;
        // The point a quarter in from the left and bottom (±1: sub-pixel positions).
        wait_for_last_log(&page, "end:").await?;
        expect_channels(&page, &x, &y, (64.0, 64.0)).await?;
        expect_active(&page, &x).await?;
        clear(&page).await?;
        let thumb = page.css("#test-ca-default [role=presentation]").await?;
        driver
            .action_chain()
            .click_and_hold_element(&thumb)
            .move_by_offset(100, -100)
            .release()
            .perform()
            .await?;
        wait_for_last_log(&page, "end:").await?;
        expect_channels(&page, &x, &y, (192.0, 192.0)).await?;
        clear(&page).await?;

        // "disabled": not focusable, no events.
        let (disabled_x, disabled_y) = inputs(&page, "test-ca-disabled").await?;
        assert_that!(attr(&disabled_x, "disabled").await?.is_some()).is_true();
        assert_that!(attr(&disabled_y, "disabled").await?.is_some()).is_true();
        page.element("test-ca-a").await?.focus().await?;
        page.press_tab().await?;
        page.wait_for_active_id("test-ca-b").await?;

        // Labelling: "should support a custom aria-label", "... aria-labelledby".
        let (label_x, label_y) = inputs(&page, "test-ca-label").await?;
        for input in [&label_x, &label_y] {
            assert_that!(attr(input, "aria-label").await?)
                .is_equal_to(Some("Color hue, Color picker".to_owned()));
            assert_that!(attr(input, "aria-labelledby").await?).is_none();
        }
        let group = page.css("#test-ca-label [role=group]").await?;
        assert_that!(attr(&group, "aria-label").await?)
            .is_equal_to(Some("Color hue, Color picker".to_owned()));
        let (lb_x, lb_y) = inputs(&page, "test-ca-labelledby").await?;
        for input in [&lb_x, &lb_y] {
            let id = attr(input, "id").await?.unwrap_or_default();
            assert_that!(attr(input, "aria-labelledby").await?)
                .is_equal_to(Some(format!("{id} test-ca-label-id")));
        }
        let group = page.css("#test-ca-labelledby [role=group]").await?;
        assert_that!(attr(&group, "aria-labelledby").await?)
            .is_equal_to(Some("test-ca-label-id".to_owned()));
        assert_that!(attr(&group, "aria-label").await?).is_none();

        // "supports form name", "supports form reset".
        let (form_x, form_y) = inputs(&page, "test-ca-form").await?;
        assert_that!(attr(&form_x, "name").await?).is_equal_to(Some("red".to_owned()));
        assert_that!(attr(&form_y, "name").await?).is_equal_to(Some("green".to_owned()));
        form_x.focus().await?;
        page.send_keys_to_active(Key::Right).await?;
        page.wait_for_attr(
            &form_x,
            "aria-valuetext",
            Some("Red: 11, very dark grayish cyan blue"),
        )
        .await?;
        page.element("test-ca-reset").await?.click().await?;
        // Focus left the area: the full text again.
        page.wait_for_attr(
            &form_x,
            "aria-valuetext",
            Some("Red: 10, Green: 20, Blue: 30, very dark grayish cyan blue"),
        )
        .await?;

        page.expect_no_page_errors().await
    }
}

/// HSV and HSL areas (their gradients' layer order with swapped axes, percentages), right to
/// left, the inputs' `value` property and `input` event (assistive technology), the thumb's
/// color without alpha, and a thumb that mounts again.
pub struct ColorAreaSpacesTests {}

#[async_trait]
impl BrowserTest<str> for ColorAreaSpacesTests {
    fn name(&self) -> Cow<'_, str> {
        "color_area_spaces_tests".into()
    }

    #[allow(clippy::too_many_lines)]
    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/color-area").await?;

        // An HSV area: saturation and brightness from 0 to 1, formatted as percentages.
        let (x, y) = inputs(&page, "test-ca-hsv").await?;
        for input in [&x, &y] {
            assert_that!(attr(input, "min").await?).is_equal_to(Some("0".to_owned()));
            assert_that!(attr(input, "max").await?).is_equal_to(Some("1".to_owned()));
            assert_that!(attr(input, "step").await?).is_equal_to(Some("0.01".to_owned()));
        }
        assert_that!(attr(&x, "aria-valuetext").await?.unwrap_or_default())
            .starts_with("Saturation: 50%, Brightness: 50%, Hue: 0°, ");
        x.focus().await?;
        page.send_keys_to_active(Key::Right).await?;
        wait_for_value(&x, "0.51").await?;
        assert_that!(attr(&x, "aria-valuetext").await?.unwrap_or_default())
            .starts_with("Saturation: 51%, ");

        // Gradients: the space's later channel on top (react-aria's `useColorAreaGradient`),
        // whichever axis it is on.
        let layer = gradient_layers(&page, "test-ca-hsv").await?;
        assert_that!(layer[0].as_str()).contains("rgb(0, 0, 0), rgba(0, 0, 0, 0)");
        let layer = gradient_layers(&page, "test-ca-hsv-swapped").await?;
        assert_that!(layer[0].as_str()).contains("rgb(0, 0, 0), rgba(0, 0, 0, 0)");
        assert_that!(layer[0].contains("to right") || layer[0].contains("90deg")).is_true();
        assert_that!(layer[1].contains("to top") || layer[1].contains("0deg")).is_true();
        let layer = gradient_layers(&page, "test-ca-hsl-swapped").await?;
        assert_that!(layer[0].as_str())
            .contains("rgb(0, 0, 0), rgba(0, 0, 0, 0), rgb(255, 255, 255)");

        // Right to left: x grows to the left (a press a quarter in from the left is 75%), and
        // ArrowLeft increases it.
        let (rtl_x, rtl_y) = inputs(&page, "test-ca-rtl").await?;
        let area = page.css("#test-ca-rtl [role=group]").await?;
        driver
            .execute(
                "arguments[0].scrollIntoView({block: 'center'});",
                vec![area.to_json()?],
            )
            .await?;
        driver
            .action_chain()
            .move_to_element_with_offset(&area, -50, 50)
            .click()
            .perform()
            .await?;
        let mut channels = (0.0, 0.0);
        for _ in 0..50 {
            channels = (number(&page, &rtl_x).await?, number(&page, &rtl_y).await?);
            if (channels.0 - 191.0).abs() <= 1.0 && (channels.1 - 64.0).abs() <= 1.0 {
                break;
            }
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }
        expect_channels(&page, &rtl_x, &rtl_y, (191.0, 64.0)).await?;
        let before = number(&page, &rtl_x).await?;
        rtl_x.focus().await?;
        page.send_keys_to_active(Key::Left).await?;
        wait_for_value(&rtl_x, &(before + 1.0).to_string()).await?;

        // The `input` event (assistive technology sets the value), then the keyboard: the value
        // property follows the state.
        let (input_x, _) = inputs(&page, "test-ca-input").await?;
        driver
            .execute(
                "arguments[0].value = '100'; \
                 arguments[0].dispatchEvent(new Event('input', {bubbles: true}));",
                vec![input_x.to_json()?],
            )
            .await?;
        page.wait_for_text("test-ca-input-log", "input:640000")
            .await?;
        input_x.focus().await?;
        page.send_keys_to_active(Key::Right).await?;
        wait_for_value(&input_x, "101").await?;

        // The thumb shows the color without its alpha (react-aria's `getDisplayColor`).
        let thumb = page.css("#test-ca-alpha .leptonic-ColorThumb").await?;
        let background = thumb.css_value("background-color").await?;
        assert_that!(background == "rgb(255, 0, 255)" || background == "rgba(255, 0, 255, 1)")
            .with_detail_message(background)
            .is_true();

        // A thumb mounted again (inside a `<Show>`) renders and works.
        let toggle = page.element("test-ca-toggle").await?;
        toggle.click().await?;
        page.wait_for_count("#test-ca-show input[type=range]", 0)
            .await?;
        toggle.click().await?;
        page.wait_for_count("#test-ca-show input[type=range]", 2)
            .await?;
        let (shown_x, _) = inputs(&page, "test-ca-show").await?;
        shown_x.focus().await?;
        page.send_keys_to_active(Key::Right).await?;
        wait_for_value(&shown_x, "11").await?;

        page.expect_no_page_errors().await
    }
}

/// The layers of the computed background of the area in `#id`.
async fn gradient_layers(page: &Page<'_>, id: &str) -> Result<Vec<String>, Report> {
    let area = page.css(&format!("#{id} [role=group]")).await?;
    let background = page
        .driver
        .execute(
            "return getComputedStyle(arguments[0]).backgroundImage;",
            vec![area.to_json()?],
        )
        .await?
        .json()
        .as_str()
        .unwrap_or_default()
        .to_owned();
    Ok(background
        .split(", linear-gradient(")
        .map(str::to_owned)
        .collect())
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

async fn number(page: &Page<'_>, input: &WebElement) -> Result<f64, Report> {
    Ok(page
        .driver
        .execute("return Number(arguments[0].value);", vec![input.to_json()?])
        .await?
        .json()
        .as_f64()
        .unwrap_or_default())
}

async fn inputs(page: &Page<'_>, id: &str) -> Result<(WebElement, WebElement), Report> {
    let mut found = page
        .driver
        .find_all(browser_test::thirtyfour::By::Css(format!(
            "#{id} input[type=range]"
        )))
        .await?
        .into_iter();
    let x = found.next().expect("x input");
    let y = found.next().expect("y input");
    Ok((x, y))
}

async fn attr(element: &WebElement, name: &str) -> Result<Option<String>, Report> {
    Ok(element.attr(name).await?)
}

async fn expect_active(page: &Page<'_>, element: &WebElement) -> Result<(), Report> {
    let id = attr(element, "id").await?.unwrap_or_default();
    page.wait_for_active_id(&id).await
}

async fn clear(page: &Page<'_>) -> Result<(), Report> {
    page.driver
        .execute("document.getElementById('test-ca-clear').click();", vec![])
        .await?;
    page.wait_for_text("test-ca-log", "").await
}

/// Waits until the log's last entry starts with `expected` (dragging logs many changes).
async fn wait_for_last_log(page: &Page<'_>, expected: &str) -> Result<(), Report> {
    let mut last = String::new();
    for _ in 0..50 {
        let log = page.element("test-ca-log").await?.text().await?;
        last = log.rsplit(',').next().unwrap_or_default().to_owned();
        if last.starts_with(expected) {
            return Ok(());
        }
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
    Err(rootcause::report!(
        "the last log entry is {last:?}, not {expected:?}"
    ))
}

/// The inputs' values are the expected channel values, ±1.
async fn expect_channels(
    page: &Page<'_>,
    x: &WebElement,
    y: &WebElement,
    (expected_x, expected_y): (f64, f64),
) -> Result<(), Report> {
    for (input, expected) in [(x, expected_x), (y, expected_y)] {
        let value = page
            .driver
            .execute("return Number(arguments[0].value);", vec![input.to_json()?])
            .await?
            .json()
            .as_f64()
            .unwrap_or_default();
        assert_that!((value - expected).abs() <= 1.0)
            .with_detail_message(format!("{value} vs. {expected}"))
            .is_true();
    }
    Ok(())
}
