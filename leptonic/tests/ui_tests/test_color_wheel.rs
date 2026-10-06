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
        page.wait_for_attr(&form, "value", Some("11")).await?;
        page.element("test-cw-reset").await?.click().await?;
        page.wait_for_attr(&form, "value", Some("10")).await?;

        page.expect_no_page_errors().await
    }
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
