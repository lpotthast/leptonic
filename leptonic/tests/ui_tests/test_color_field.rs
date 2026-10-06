// Upstream: @adobe/react-spectrum/test/color/ColorField.test.js @ 99e6102368
// Upstream: react-aria/test/color/useColorField.test.js @ 99e6102368
use std::{borrow::Cow, time::Duration};

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{By, Key, WebDriver, WebElement},
};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

/// The `ColorField`/`ColorChannelField` atoms: hex text committed on blur (cleared, typed, invalid
/// characters rejected, incomplete text reverted), stepping by keys and the wheel within
/// #000000–#FFFFFF, flags, forms, and channel fields as number fields.
pub struct ColorFieldTests {}

#[async_trait]
impl BrowserTest<str> for ColorFieldTests {
    fn name(&self) -> Cow<'_, str> {
        "color_field_tests".into()
    }

    #[allow(clippy::too_many_lines)]
    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/color-field").await?;

        // "handles defaults": a text box without spin button values, labelled by its `Label`.
        let primary = input(&page, "test-cf-primary").await?;
        assert_that!(attr(&primary, "type").await?).is_equal_to(Some("text".to_owned()));
        assert_that!(attr(&primary, "autocomplete").await?).is_equal_to(Some("off".to_owned()));
        assert_that!(attr(&primary, "spellcheck").await?).is_equal_to(Some("false".to_owned()));
        assert_that!(attr(&primary, "role").await?).is_none();
        assert_that!(attr(&primary, "aria-valuenow").await?).is_none();
        assert_that!(attr(&primary, "aria-valuetext").await?).is_none();
        assert_that!(value(&page, &primary).await?).is_equal_to("#AABBCC".to_owned());
        let labelled_by = attr(&primary, "aria-labelledby").await?.unwrap_or_default();
        assert_that!(page.element(&labelled_by).await?.text().await?)
            .is_equal_to("Primary Color".to_owned());

        // "should handle uncontrolled state".
        primary.focus().await?;
        page.send_keys_to_active(Key::Control + "a").await?;
        page.send_keys_to_active(Key::Backspace).await?;
        blur(&page).await?;
        page.wait_for_text("test-cf-log", "primary:none").await?;
        assert_that!(value(&page, &primary).await?).is_equal_to(String::new());
        primary.focus().await?;
        page.send_keys_to_active("cbacba").await?;
        blur(&page).await?;
        page.wait_for_text("test-cf-log", "primary:none,primary:CBACBA")
            .await?;
        assert_that!(value(&page, &primary).await?).is_equal_to("#CBACBA".to_owned());
        clear(&page).await?;

        // "should disallow invalid characters and revert back to last valid value if left
        // incomplete".
        let empty = input(&page, "test-cf-empty").await?;
        empty.focus().await?;
        page.send_keys_to_active("abc").await?;
        blur(&page).await?;
        page.wait_for_text("test-cf-log", "empty:AABBCC").await?;
        assert_that!(value(&page, &empty).await?).is_equal_to("#AABBCC".to_owned());
        empty.focus().await?;
        page.send_keys_to_active(Key::Control + "a").await?;
        page.send_keys_to_active(Key::Backspace).await?;
        page.send_keys_to_active("abcxyz8b").await?;
        assert_that!(value(&page, &empty).await?).is_equal_to("abc8b".to_owned());
        blur(&page).await?;
        assert_that!(value(&page, &empty).await?).is_equal_to("#AABBCC".to_owned());
        assert_that!(page.element("test-cf-log").await?.text().await?)
            .is_equal_to("empty:AABBCC".to_owned());
        clear(&page).await?;

        // "increment with arrow up key", "decrement with arrow down key", "not increment beyond
        // max value", "decrement to min value".
        let max = input(&page, "test-cf-max").await?;
        max.focus().await?;
        page.send_keys_to_active(Key::Up).await?;
        page.wait_for_text("test-cf-log", "max:FFFFFF").await?;
        page.send_keys_to_active(Key::Up).await?;
        page.send_keys_to_active(Key::Down).await?;
        page.wait_for_text("test-cf-log", "max:FFFFFF,max:FFFFFE")
            .await?;
        page.send_keys_to_active(Key::Home).await?;
        page.wait_for_text("test-cf-log", "max:FFFFFF,max:FFFFFE,max:000000")
            .await?;
        assert_that!(value(&page, &max).await?).is_equal_to("#000000".to_owned());
        clear(&page).await?;

        // "increment with mouse wheel" (while focused).
        driver
            .execute(
                "arguments[0].dispatchEvent(new WheelEvent('wheel', \
                 { deltaY: 10, bubbles: true, cancelable: true }));",
                vec![max.to_json()?],
            )
            .await?;
        page.wait_for_text("test-cf-log", "max:000001").await?;
        clear(&page).await?;

        // "should be readonly", "should be required".
        let flags = input(&page, "test-cf-flags").await?;
        assert_that!(attr(&flags, "readonly").await?.is_some()).is_true();
        // Native validation (the default): `required`, not `aria-required` (react-aria).
        assert_that!(attr(&flags, "required").await?.is_some()).is_true();
        assert_that!(attr(&flags, "aria-required").await?).is_none();

        // "supports form reset", with the name on a hidden input.
        let form = input(&page, "test-cf-form").await?;
        let hidden = page.css("#test-cf-form input[type=hidden]").await?;
        assert_that!(attr(&hidden, "name").await?).is_equal_to(Some("color".to_owned()));
        form.focus().await?;
        page.send_keys_to_active(Key::Up).await?;
        page.wait_until_value(&form, "#123457").await?;
        page.element("test-cf-reset").await?.click().await?;
        page.wait_until_value(&form, "#123456").await?;

        // "should support the channel prop": a number field of the channel, named after it.
        let hue = input(&page, "test-cf-hue").await?;
        assert_that!(value(&page, &hue).await?).is_equal_to("10°".to_owned());
        assert_that!(attr(&hue, "aria-label").await?).is_equal_to(Some("Hue".to_owned()));
        hue.focus().await?;
        page.send_keys_to_active(Key::Up).await?;
        page.wait_for_text("test-cf-log", "hue:11").await?;
        page.wait_until_value(&hue, "11°").await?;
        let saturation = input(&page, "test-cf-saturation").await?;
        assert_that!(value(&page, &saturation).await?).is_equal_to("50%".to_owned());

        page.expect_no_page_errors().await
    }
}

async fn input(page: &Page<'_>, id: &str) -> Result<WebElement, Report> {
    Ok(page
        .driver
        .find(By::Css(format!("#{id} input:not([type=hidden])")))
        .await?)
}

async fn attr(element: &WebElement, name: &str) -> Result<Option<String>, Report> {
    Ok(element.attr(name).await?)
}

async fn value(page: &Page<'_>, input: &WebElement) -> Result<String, Report> {
    Ok(page
        .driver
        .execute("return arguments[0].value;", vec![input.to_json()?])
        .await?
        .json()
        .as_str()
        .unwrap_or_default()
        .to_owned())
}

/// Moves focus to the button before the fields (committing the focused field).
async fn blur(page: &Page<'_>) -> Result<(), Report> {
    page.element("test-cf-before").await?.focus().await?;
    tokio::time::sleep(Duration::from_millis(50)).await;
    Ok(())
}

async fn clear(page: &Page<'_>) -> Result<(), Report> {
    page.driver
        .execute("document.getElementById('test-cf-clear').click();", vec![])
        .await?;
    page.wait_for_text("test-cf-log", "").await
}

trait WaitForValue {
    async fn wait_until_value(&self, input: &WebElement, expected: &str) -> Result<(), Report>;
}

impl WaitForValue for Page<'_> {
    /// Waits until the input's value (the property) is `expected`.
    async fn wait_until_value(&self, input: &WebElement, expected: &str) -> Result<(), Report> {
        let mut last = String::new();
        for _ in 0..50 {
            last = value(self, input).await?;
            if last == expected {
                return Ok(());
            }
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
        Err(rootcause::report!(
            "the value is {last:?}, not {expected:?}"
        ))
    }
}
