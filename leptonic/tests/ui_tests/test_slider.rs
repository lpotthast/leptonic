// Upstream: react-aria-components/test/Slider.test.js @ 99e6102368
// Upstream: react-aria/test/slider/useSliderThumb.test.js @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{By, Key, WebDriver, WebElement},
};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

/// The slider atoms: a labelled group of range inputs (min, max, step, value, value text), the
/// output, keyboard changes (arrows, Shift+arrows, PageUp/PageDown, Home/End) ending in
/// `on_change_end`, thumbs bounded by their neighbors, track presses, dragging, vertical and
/// disabled sliders, the fill, and value tooltips.
pub struct SliderTests {}

async fn input_of(page: &Page<'_>, slider: &str, index: usize) -> Result<WebElement, Report> {
    let inputs = page
        .driver
        .find_all(By::Css(format!("{slider} input[type=range]")))
        .await?;
    Ok(inputs[index].clone())
}

async fn value(input: &WebElement) -> Result<String, Report> {
    Ok(input.prop("value").await?.unwrap_or_default())
}

#[async_trait]
impl BrowserTest<str> for SliderTests {
    fn name(&self) -> Cow<'_, str> {
        "slider_tests".into()
    }

    #[allow(clippy::too_many_lines)]
    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/slider").await?;

        // A labelled group; the thumb's input is named by the label and has the range's
        // attributes.
        let group = page.element("test-slider-volume").await?;
        assert_that!(group.attr("role").await?).is_equal_to(Some("group".to_owned()));
        let label_id = group.attr("aria-labelledby").await?.unwrap_or_default();
        let label = page.element(&label_id).await?;
        assert_that!(label.text().await?).is_equal_to("Volume".to_owned());
        let volume = input_of(&page, "#test-slider-volume", 0).await?;
        assert_that!(volume.attr("min").await?).is_equal_to(Some("0".to_owned()));
        assert_that!(volume.attr("max").await?).is_equal_to(Some("100".to_owned()));
        assert_that!(volume.attr("step").await?).is_equal_to(Some("5".to_owned()));
        assert_that!(value(&volume).await?).is_equal_to("30".to_owned());
        assert_that!(volume.attr("aria-valuetext").await?).is_equal_to(Some("30".to_owned()));
        assert_that!(volume.attr("aria-labelledby").await?.unwrap_or_default())
            .contains(label_id.as_str());
        let output = page.element("test-slider-volume-output").await?;
        assert_that!(output.text().await?).is_equal_to("30".to_owned());
        let volume_id = volume.attr("id").await?.unwrap_or_default();
        assert_that!(output.attr("for").await?).is_equal_to(Some(volume_id.clone()));

        // The fill runs from the offset (50) to the thumb (30).
        let fill = page.element("test-slider-volume-fill").await?;
        assert_that!(fill.css_value("width").await?).is_equal_to("40px".to_owned());

        // Clicking the label focuses the thumb; the keyboard changes the value by the step,
        // Shift by a page, PageUp/PageDown by a page, Home/End to the ends. Each change ends.
        label.click().await?;
        page.wait_for_active_id(&volume_id).await?;
        page.send_keys_to_active(Key::Right).await?;
        page.wait_for_text("test-slider-volume-output", "35")
            .await?;
        page.wait_for_text("test-slider-volume-ends", "1").await?;
        page.send_keys_to_active(Key::Shift + Key::Right).await?;
        page.wait_for_text("test-slider-volume-output", "45")
            .await?;
        page.send_keys_to_active(Key::PageDown).await?;
        page.wait_for_text("test-slider-volume-output", "35")
            .await?;
        page.send_keys_to_active(Key::End).await?;
        page.wait_for_text("test-slider-volume-output", "100")
            .await?;
        page.send_keys_to_active(Key::Home).await?;
        page.wait_for_text("test-slider-volume-output", "0").await?;
        page.wait_for_text("test-slider-volume-ends", "5").await?;

        // "should support clicking on the track to move the thumb".
        let track = page.element("test-slider-volume-track").await?;
        page.driver
            .action_chain()
            .move_to_element_with_offset(&track, 50, 0)
            .click()
            .perform()
            .await?;
        page.wait_for_text("test-slider-volume-output", "75")
            .await?;

        // "should support dragging state": the thumb follows the pointer.
        let thumb = page.css("#test-slider-volume .test-slider-thumb").await?;
        page.driver
            .action_chain()
            .click_and_hold_element(&thumb)
            .move_by_offset(-50, 0)
            .perform()
            .await?;
        page.wait_for_selector("#test-slider-volume .test-slider-thumb[data-dragging]")
            .await?;
        page.driver.action_chain().release().perform().await?;
        page.wait_for_text("test-slider-volume-output", "50")
            .await?;
        page.wait_for_no_selector("#test-slider-volume .test-slider-thumb[data-dragging]")
            .await?;

        // "should support two thumbs": each is bounded by the other; the output shows both.
        let price = page.css("[role=group][aria-label=Price]").await?;
        let minimum = input_of(&page, "[aria-label=Price]", 0).await?;
        let maximum = input_of(&page, "[aria-label=Price]", 1).await?;
        assert_that!(minimum.attr("max").await?).is_equal_to(Some("80".to_owned()));
        assert_that!(maximum.attr("min").await?).is_equal_to(Some("20".to_owned()));
        page.wait_for_text("test-slider-price-output", "20 \u{2013} 80")
            .await?;
        minimum.focus().await?;
        page.send_keys_to_active(Key::End).await?;
        page.wait_for_text("test-slider-price", "[80, 80]").await?;
        assert_that!(price.attr("data-disabled").await?).is_none();

        // "should support orientation": vertical arrows.
        let vertical = input_of(&page, "[aria-label=Vertical]", 0).await?;
        assert_that!(vertical.attr("aria-orientation").await?)
            .is_equal_to(Some("vertical".to_owned()));
        vertical.focus().await?;
        page.send_keys_to_active(Key::Up).await?;
        assert_that!(value(&vertical).await?).is_equal_to("51".to_owned());

        // "should support disabled state".
        let disabled = page.css("[role=group][aria-label=Disabled]").await?;
        assert_that!(disabled.attr("data-disabled").await?).is_some();
        let disabled_input = input_of(&page, "[aria-label=Disabled]", 0).await?;
        assert_that!(disabled_input.attr("disabled").await?).is_some();

        // Tooltips: always visible, or while hovered.
        let always = page.css("[aria-label=Always] .tooltip").await?;
        assert_that!(always.attr("data-visible").await?).is_some();
        assert_that!(always.text().await?).is_equal_to("30".to_owned());
        let hover_thumb = page
            .css("[aria-label='On hover'] .test-slider-thumb")
            .await?;
        hover_thumb.scroll_into_view().await?;
        page.driver
            .action_chain()
            .move_to_element_center(&hover_thumb)
            .perform()
            .await?;
        page.wait_for_selector("[aria-label='On hover'] .tooltip[data-visible]")
            .await?;

        page.expect_no_page_errors().await
    }
}
