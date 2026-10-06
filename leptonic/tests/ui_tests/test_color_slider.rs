// Upstream: @adobe/react-spectrum/test/color/ColorSlider.test.tsx @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{By, Key, WebDriver, WebElement},
};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

/// The `ColorSlider` atoms: input attributes, value text and labelling, keyboard steps, a press
/// on the track, disabled sliders, forms.
pub struct ColorSliderTests {}

#[async_trait]
impl BrowserTest<str> for ColorSliderTests {
    fn name(&self) -> Cow<'_, str> {
        "color_slider_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/color-slider").await?;

        // "sets input props"; the channel names a slider without labels.
        let red = input(&page, "test-cs-red").await?;
        assert_that!(attr(&red, "type").await?).is_equal_to(Some("range".to_owned()));
        assert_that!(attr(&red, "min").await?).is_equal_to(Some("0".to_owned()));
        assert_that!(attr(&red, "max").await?).is_equal_to(Some("255".to_owned()));
        assert_that!(attr(&red, "step").await?).is_equal_to(Some("1".to_owned()));
        // "sets input props": the value and the color's name.
        assert_that!(attr(&red, "aria-valuetext").await?).is_equal_to(Some("0, black".to_owned()));
        // "sets a default aria-label when label={null}": on the group, which labels the input.
        let group = page.css("#test-cs-red [role=group]").await?;
        assert_that!(attr(&group, "aria-label").await?).is_equal_to(Some("Red".to_owned()));
        let group_id = attr(&group, "id").await?.unwrap_or_default();
        assert_that!(attr(&red, "aria-labelledby").await?).is_equal_to(Some(group_id));
        assert_that!(attr(&red, "aria-label").await?).is_none();
        assert_that!(page.css("#test-cs-red output").await?.text().await?)
            .is_equal_to("0".to_owned());

        // "sets aria-valuetext to formatted value" (with the hue's name); a `Label` names it.
        let hue = input(&page, "test-cs-hue").await?;
        assert_that!(attr(&hue, "max").await?).is_equal_to(Some("360".to_owned()));
        assert_that!(attr(&hue, "aria-valuetext").await?)
            .is_equal_to(Some("10°, red orange".to_owned()));
        assert_that!(attr(&hue, "aria-label").await?).is_none();
        // "clicking on label should focus input".
        page.css("#test-cs-hue [id^=label]").await?.click().await?;
        expect_active(&page, &hue).await?;

        // "keyboard events": steps, pages, Home/End.
        page.element("test-cs-before").await?.focus().await?;
        page.press_tab().await?;
        expect_active(&page, &red).await?;
        for (key, expected) in [
            (Key::Right, "010000"),
            (Key::PageUp, "120000"),
            (Key::Home, "000000"),
            (Key::End, "FF0000"),
        ] {
            page.send_keys_to_active(key).await?;
            page.wait_for_text("test-cs-log", &format!("change:{expected},end:{expected}"))
                .await?;
            clear(&page).await?;
        }

        // "clicking and dragging on the track works": a quarter along (±1, sub-pixel positions).
        let track = page.css("#test-cs-red [role=group] > div").await?;
        driver
            .execute(
                "arguments[0].scrollIntoView({block: 'center'});",
                vec![track.to_json()?],
            )
            .await?;
        driver
            .action_chain()
            .move_to_element_with_offset(&track, -50, 0)
            .click()
            .perform()
            .await?;
        let mut value = 0.0;
        for _ in 0..50 {
            value = number(&page, &red).await?;
            if (value - 64.0).abs() <= 1.0 {
                break;
            }
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }
        assert_that!((value - 64.0).abs() <= 1.0)
            .with_detail_message(value.to_string())
            .is_true();

        // "disabled".
        let disabled = input(&page, "test-cs-disabled").await?;
        assert_that!(attr(&disabled, "disabled").await?.is_some()).is_true();
        page.element("test-cs-a").await?.focus().await?;
        page.press_tab().await?;
        page.wait_for_active_id("test-cs-b").await?;

        // "supports form name", "supports form reset".
        let form = input(&page, "test-cs-form").await?;
        assert_that!(attr(&form, "name").await?).is_equal_to(Some("red".to_owned()));
        assert_that!(number(&page, &form).await?).is_equal_to(127.0);
        form.focus().await?;
        page.send_keys_to_active(Key::Right).await?;
        page.wait_for_attr(&form, "aria-valuetext", Some("128, dark vibrant red"))
            .await?;
        page.element("test-cs-reset").await?.click().await?;
        page.wait_for_attr(&form, "aria-valuetext", Some("127, dark vibrant red"))
            .await?;

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

async fn number(page: &Page<'_>, input: &WebElement) -> Result<f64, Report> {
    Ok(page
        .driver
        .execute("return Number(arguments[0].value);", vec![input.to_json()?])
        .await?
        .json()
        .as_f64()
        .unwrap_or_default())
}

async fn expect_active(page: &Page<'_>, element: &WebElement) -> Result<(), Report> {
    let id = attr(element, "id").await?.unwrap_or_default();
    page.wait_for_active_id(&id).await
}

async fn clear(page: &Page<'_>) -> Result<(), Report> {
    page.driver
        .execute("document.getElementById('test-cs-clear').click();", vec![])
        .await?;
    page.wait_for_text("test-cs-log", "").await
}
