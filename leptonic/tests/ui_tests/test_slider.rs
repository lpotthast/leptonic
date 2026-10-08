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

/// Waits until the input's `value` property is `expected` (it updates in an effect).
async fn wait_for_value(input: &WebElement, expected: &str) -> Result<(), Report> {
    let mut last = String::new();
    for _ in 0..100 {
        last = value(input).await?;
        if last == expected {
            return Ok(());
        }
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }
    Err(rootcause::report!(
        "the input's value is {last:?}, not {expected:?}"
    ))
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
        // The arrows of the other axis step too (useSliderThumb's `onMove`): up increments, down
        // decrements.
        page.send_keys_to_active(Key::Up).await?;
        page.wait_for_text("test-slider-volume-output", "40")
            .await?;
        page.send_keys_to_active(Key::Down).await?;
        page.wait_for_text("test-slider-volume-output", "35")
            .await?;
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
        page.wait_for_text("test-slider-volume-ends", "7").await?;

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
        wait_for_value(&vertical, "51").await?;
        // "can be moved with keys (vertical)", and the arrows of the other axis.
        page.send_keys_to_active(Key::Down).await?;
        wait_for_value(&vertical, "50").await?;
        page.send_keys_to_active(Key::Right).await?;
        wait_for_value(&vertical, "51").await?;
        page.send_keys_to_active(Key::Left).await?;
        wait_for_value(&vertical, "50").await?;

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

// -- Interactions (`/atoms/slider-interactions`) ------------------------------------------------

const INTERACTIONS: &str = "/atoms/slider-interactions";

/// The thumbs' range inputs of the slider named `name` (its `aria-label`).
async fn inputs_of(page: &Page<'_>, name: &str) -> Result<Vec<WebElement>, Report> {
    Ok(page
        .driver
        .find_all(By::Css(format!(
            "[role=group][aria-label='{name}'] input[type=range]"
        )))
        .await?)
}

/// The log of the slider named `name`.
async fn log_of(page: &Page<'_>, name: &str) -> Result<Vec<String>, Report> {
    let text = page.element(&format!("{name}-log")).await?.text().await?;
    Ok(text
        .split(';')
        .filter(|entry| !entry.is_empty())
        .map(str::to_owned)
        .collect())
}

/// Waits until the last log entry of the slider named `name` starting with `kind` (`change` or
/// `end`) is `kind:expected` (dragging logs a change per pointer move).
async fn wait_for_last(
    page: &Page<'_>,
    name: &str,
    kind: &str,
    expected: &str,
) -> Result<(), Report> {
    let mut last = None;
    for _ in 0..100 {
        last = log_of(page, name)
            .await?
            .into_iter()
            .rev()
            .find(|entry| entry.starts_with(&format!("{kind}:")));
        if last.as_deref() == Some(format!("{kind}:{expected}").as_str()) {
            return Ok(());
        }
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }
    Err(rootcause::report!(
        "the last {kind} of {name} is {last:?}, not {expected:?}"
    ))
}

/// The number of `end` entries in the log of the slider named `name`, after letting pending
/// changes settle.
async fn ends_of(page: &Page<'_>, name: &str) -> Result<usize, Report> {
    tokio::time::sleep(std::time::Duration::from_millis(150)).await;
    Ok(log_of(page, name)
        .await?
        .iter()
        .filter(|entry| entry.starts_with("end:"))
        .count())
}

/// Scrolls the track of the slider named `name` into view (pointer actions don't scroll) and
/// returns it.
async fn track_of(page: &Page<'_>, name: &str) -> Result<WebElement, Report> {
    let track = page.element(&format!("{name}-track")).await?;
    page.driver
        .execute(
            "arguments[0].scrollIntoView({block: 'center'});",
            vec![track.to_json()?],
        )
        .await?;
    Ok(track)
}

/// Presses (down and up) the 200 × 20 pixel track of `name` at `x` pixels from its start.
async fn click_track(page: &Page<'_>, name: &str, x: i64) -> Result<(), Report> {
    let track = track_of(page, name).await?;
    page.driver
        .action_chain()
        .move_to_element_with_offset(&track, x - 100, 0)
        .click()
        .perform()
        .await?;
    Ok(())
}

/// Sends `key` to the focused element as a synthetic `keydown` (for `repeat`, which WebDriver
/// can't send).
async fn dispatch_keydown(page: &Page<'_>, key: &str, repeat: bool) -> Result<(), Report> {
    page.driver
        .execute(
            "document.activeElement.dispatchEvent(new KeyboardEvent('keydown', \
             {key: arguments[0], repeat: arguments[1], bubbles: true, cancelable: true}));",
            vec![key.into(), repeat.into()],
        )
        .await?;
    Ok(())
}

/// Presses and drags on the track (react-aria's `useSlider.test.js`, "interactions on track using
/// pointerEvents"): the closest thumb moves to the press and follows the pointer, stacked thumbs,
/// disabled sliders, vertical and right-to-left tracks.
pub struct SliderTrackTests {}

#[async_trait]
impl BrowserTest<str> for SliderTrackTests {
    fn name(&self) -> Cow<'_, str> {
        "slider_track_tests".into()
    }

    #[allow(clippy::too_many_lines)]
    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path(INTERACTIONS).await?;

        // "should allow you to set value of closest thumb by clicking on track".
        click_track(&page, "track", 40).await?;
        page.wait_for_text("track-log", "change:[20, 80];end:[20, 80]")
            .await?;
        click_track(&page, "track", 180).await?;
        page.wait_for_text(
            "track-log",
            "change:[20, 80];end:[20, 80];change:[20, 90];end:[20, 90]",
        )
        .await?;

        // "... by dragging on track": changes while dragging, the end on release.
        let track = track_of(&page, "drag").await?;
        driver
            .action_chain()
            .move_to_element_with_offset(&track, 40 - 100, 0)
            .click_and_hold()
            .perform()
            .await?;
        wait_for_last(&page, "drag", "change", "[20, 80]").await?;
        driver
            .action_chain()
            .move_by_offset(20, 0)
            .perform()
            .await?;
        wait_for_last(&page, "drag", "change", "[30, 80]").await?;
        driver
            .action_chain()
            .move_by_offset(20, 0)
            .perform()
            .await?;
        wait_for_last(&page, "drag", "change", "[40, 80]").await?;
        assert_that!(ends_of(&page, "drag").await?).is_equal_to(0);
        driver.action_chain().release().perform().await?;
        wait_for_last(&page, "drag", "end", "[40, 80]").await?;
        assert_that!(ends_of(&page, "drag").await?).is_equal_to(1);

        // "... before thumbs when thumbs stacked", "... after thumbs when thumbs stacked".
        click_track(&page, "stacked-before", 40).await?;
        wait_for_last(&page, "stacked-before", "end", "[20, 40]").await?;
        click_track(&page, "stacked-after", 120).await?;
        wait_for_last(&page, "stacked-after", "end", "[40, 60]").await?;

        // "... before thumbs when many thumbs and stacked".
        click_track(&page, "many-before", 140).await?;
        wait_for_last(&page, "many-before", "end", "[25, 25, 50, 70, 75]").await?;
        click_track(&page, "many-before", 40).await?;
        wait_for_last(&page, "many-before", "end", "[20, 25, 50, 70, 75]").await?;
        // "... after thumbs when many thumbs and stacked".
        click_track(&page, "many-after", 160).await?;
        wait_for_last(&page, "many-after", "end", "[25, 25, 50, 75, 80]").await?;
        click_track(&page, "many-after", 60).await?;
        wait_for_last(&page, "many-after", "end", "[25, 30, 50, 75, 80]").await?;

        // "should not allow you to set value if disabled".
        let track = track_of(&page, "disabled").await?;
        driver
            .action_chain()
            .move_to_element_with_offset(&track, 40 - 100, 0)
            .click_and_hold()
            .move_by_offset(20, 0)
            .release()
            .perform()
            .await?;
        tokio::time::sleep(std::time::Duration::from_millis(150)).await;
        assert_that!(log_of(&page, "disabled").await?).is_empty();
        let disabled = inputs_of(&page, "disabled").await?;
        assert_that!(value(&disabled[0]).await?).is_equal_to("10".to_owned());

        // "... by dragging on track (vertical)": the value grows upwards.
        let track = track_of(&page, "vertical").await?;
        driver
            .action_chain()
            .move_to_element_with_offset(&track, 0, 160 - 100)
            .click_and_hold()
            .perform()
            .await?;
        wait_for_last(&page, "vertical", "change", "[20, 80]").await?;
        driver
            .action_chain()
            .move_by_offset(0, -20)
            .perform()
            .await?;
        wait_for_last(&page, "vertical", "change", "[30, 80]").await?;
        driver
            .action_chain()
            .move_by_offset(0, -20)
            .perform()
            .await?;
        wait_for_last(&page, "vertical", "change", "[40, 80]").await?;
        driver.action_chain().release().perform().await?;
        wait_for_last(&page, "vertical", "end", "[40, 80]").await?;

        // Right to left: the track starts on the right; the arrow keys follow the reading
        // direction (Left increases).
        click_track(&page, "rtl", 140).await?;
        wait_for_last(&page, "rtl", "end", "[30, 80]").await?;
        let thumb = page.css("#rtl-track .thumb").await?;
        let style = thumb
            .attr("style")
            .await?
            .unwrap_or_default()
            .replace(' ', "");
        assert_that!(style.as_str()).contains("left:70%");
        let rtl = inputs_of(&page, "rtl").await?;
        rtl[0].focus().await?;
        page.send_keys_to_active(Key::Left).await?;
        wait_for_value(&rtl[0], "31").await?;
        page.send_keys_to_active(Key::Right).await?;
        wait_for_value(&rtl[0], "30").await?;

        page.expect_no_page_errors().await
    }
}

/// The thumbs' keyboard (react-aria's `useSliderThumb.test.js`, "using KeyEvents"; react-aria-
/// components' `Slider.test.js`), labels and attributes.
pub struct SliderThumbTests {}

#[async_trait]
impl BrowserTest<str> for SliderThumbTests {
    fn name(&self) -> Cow<'_, str> {
        "slider_thumb_tests".into()
    }

    #[allow(clippy::too_many_lines)]
    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path(INTERACTIONS).await?;

        // "can be moved with keys": each key changes and ends.
        let keys = inputs_of(&page, "keys").await?;
        keys[0].focus().await?;
        page.send_keys_to_active(Key::Right).await?;
        page.wait_for_text("keys-log", "change:[11];end:[11]")
            .await?;
        page.send_keys_to_active(Key::Left).await?;
        page.wait_for_text("keys-log", "change:[11];end:[11];change:[10];end:[10]")
            .await?;
        // All four arrows step a horizontal slider: Up increases, Down decreases.
        page.send_keys_to_active(Key::Up).await?;
        wait_for_value(&keys[0], "11").await?;
        page.send_keys_to_active(Key::Down).await?;
        wait_for_value(&keys[0], "10").await?;

        // "can be moved with keys (vertical)": Right and Up increase, Down and Left decrease.
        let vertical = inputs_of(&page, "keys-vertical").await?;
        vertical[0].focus().await?;
        page.send_keys_to_active(Key::Right).await?;
        wait_for_last(&page, "keys-vertical", "change", "[11]").await?;
        page.send_keys_to_active(Key::Up).await?;
        wait_for_last(&page, "keys-vertical", "change", "[12]").await?;
        page.send_keys_to_active(Key::Down).await?;
        wait_for_last(&page, "keys-vertical", "change", "[11]").await?;
        page.send_keys_to_active(Key::Left).await?;
        wait_for_last(&page, "keys-vertical", "change", "[10]").await?;
        wait_for_value(&vertical[0], "10").await?;

        // "should support repeat keydown events when holding Page Up/Page Down".
        let paged = inputs_of(&page, "page").await?;
        paged[0].focus().await?;
        for repeat in [false, true, true] {
            dispatch_keydown(&page, "PageUp", repeat).await?;
        }
        wait_for_value(&paged[0], "50").await?;
        for repeat in [false, true, true] {
            dispatch_keydown(&page, "PageDown", repeat).await?;
        }
        wait_for_value(&paged[0], "20").await?;

        // The `input` event (assistive technology sets the value): the state follows, and the
        // property keeps following the state afterwards.
        page.driver
            .execute(
                "arguments[0].value = '42'; \
                 arguments[0].dispatchEvent(new Event('input', {bubbles: true}));",
                vec![keys[0].to_json()?],
            )
            .await?;
        wait_for_last(&page, "keys", "change", "[42]").await?;
        keys[0].focus().await?;
        page.send_keys_to_active(Key::Right).await?;
        wait_for_value(&keys[0], "43").await?;

        // A disabled thumb: not changed by track presses near it, its input disabled.
        let thumb_disabled = inputs_of(&page, "thumb-disabled").await?;
        assert_that!(thumb_disabled[0].attr("disabled").await?).is_none();
        assert_that!(thumb_disabled[1].attr("disabled").await?).is_some();
        click_track(&page, "thumb-disabled", 180).await?;
        stays!(
            "the disabled thumb's value",
            "80".to_owned(),
            value(&thumb_disabled[1]).await?
        );
        click_track(&page, "thumb-disabled", 40).await?;
        wait_for_value(&thumb_disabled[0], "20").await?;

        // "should support form prop".
        let form = inputs_of(&page, "form").await?;
        assert_that!(form[0].attr("form").await?).is_equal_to(Some("test-form".to_owned()));
        assert_that!(form[0].attr("name").await?).is_equal_to(Some("volume".to_owned()));

        // Labels ("should have the right labels with Slider thumb label", "... thumb
        // aria-label"): a thumb's own `Label` labels its input (`for`), before the slider's.
        let group = page.element("labels").await?;
        let slider_label = group.attr("aria-labelledby").await?.unwrap_or_default();
        let labels = page
            .driver
            .find_all(By::Css("[role=group]#labels input[type=range]"))
            .await?;
        let min_id = labels[0].attr("id").await?.unwrap_or_default();
        let min_label = page.css("#labels .leptonic-SliderThumb label").await?;
        assert_that!(min_label.text().await?).is_equal_to("Min".to_owned());
        assert_that!(min_label.attr("for").await?).is_equal_to(Some(min_id));
        let min_label_id = min_label.attr("id").await?.unwrap_or_default();
        page.wait_for_attr(
            &labels[0],
            "aria-labelledby",
            Some(&format!("{min_label_id} {slider_label}")),
        )
        .await?;
        let max_id = labels[1].attr("id").await?.unwrap_or_default();
        assert_that!(labels[1].attr("aria-label").await?).is_equal_to(Some("Max".to_owned()));
        page.wait_for_attr(
            &labels[1],
            "aria-labelledby",
            Some(&format!("{max_id} {slider_label}")),
        )
        .await?;
        // The slider's label is a different element with a different id.
        assert_that!(slider_label.as_str()).is_not_equal_to(min_label_id.as_str());
        let price = page.element(&slider_label).await?;
        assert_that!(price.text().await?).is_equal_to("Price".to_owned());

        // Attributes: exact decimals (no `f64` noise), required, invalid, error message and
        // details.
        let attributes = inputs_of(&page, "attributes").await?;
        let input = &attributes[0];
        assert_that!(input.attr("step").await?).is_equal_to(Some("0.1".to_owned()));
        assert_that!(input.attr("value").await?).is_equal_to(Some("0.1".to_owned()));
        assert_that!(input.attr("aria-required").await?).is_equal_to(Some("true".to_owned()));
        assert_that!(input.attr("aria-invalid").await?).is_equal_to(Some("true".to_owned()));
        assert_that!(input.attr("aria-errormessage").await?)
            .is_equal_to(Some("attributes-error".to_owned()));
        assert_that!(input.attr("aria-details").await?)
            .is_equal_to(Some("attributes-details".to_owned()));
        input.focus().await?;
        page.send_keys_to_active(Key::Right).await?;
        wait_for_value(input, "0.2").await?;
        assert_that!(input.attr("value").await?).is_equal_to(Some("0.2".to_owned()));

        page.expect_no_page_errors().await
    }
}

/// Several thumbs (react-aria-components' `Slider.test.js`): three thumbs with an output, thumbs
/// bound to app state, bound values out of the range, a thumb without a value.
pub struct SliderMultipleThumbsTests {}

#[async_trait]
impl BrowserTest<str> for SliderMultipleThumbsTests {
    fn name(&self) -> Cow<'_, str> {
        "slider_multiple_thumbs_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path(INTERACTIONS).await?;

        // "should support three thumbs".
        let three = inputs_of(&page, "three").await?;
        assert_that!(three.len()).is_equal_to(3);
        for (input, expected) in three.iter().zip(["30", "60", "80"]) {
            assert_that!(value(input).await?).is_equal_to(expected.to_owned());
        }
        page.wait_for_text("three-output", "30, 60, 80").await?;

        // "should support multiple thumbs (controlled)".
        let controlled = inputs_of(&page, "controlled").await?;
        assert_that!(value(&controlled[0]).await?).is_equal_to("30".to_owned());
        assert_that!(value(&controlled[1]).await?).is_equal_to("60".to_owned());
        page.element("controlled-reset").await?.click().await?;
        wait_for_value(&controlled[0], "0").await?;
        wait_for_value(&controlled[1], "100").await?;
        // From the element before the slider (react-aria-components' test: from the body).
        page.element("controlled-before").await?.focus().await?;
        page.press_tab().await?;
        page.wait_for_focus_on(&controlled[0], "the first thumb")
            .await?;
        for _ in 0..3 {
            page.send_keys_to_active(Key::Right).await?;
        }
        wait_for_value(&controlled[0], "3").await?;
        assert_that!(value(&controlled[1]).await?).is_equal_to("100".to_owned());
        page.element("controlled-reset").await?.click().await?;
        wait_for_value(&controlled[0], "0").await?;
        page.element("controlled-before").await?.focus().await?;
        page.press_tab().await?;
        page.press_tab().await?;
        page.wait_for_focus_on(&controlled[1], "the second thumb")
            .await?;
        for _ in 0..3 {
            page.send_keys_to_active(Key::Left).await?;
        }
        wait_for_value(&controlled[1], "97").await?;
        assert_that!(value(&controlled[0]).await?).is_equal_to("0".to_owned());

        // Bound values out of the range render within it (react-stately's `restrictValues`).
        let restricted = inputs_of(&page, "restricted").await?;
        assert_that!(value(&restricted[0]).await?).is_equal_to("0".to_owned());
        assert_that!(value(&restricted[1]).await?).is_equal_to("100".to_owned());
        page.wait_for_text("restricted-output", "0 \u{2013} 100")
            .await?;

        // A thumb without a value renders (at the minimum) instead of panicking.
        let missing = inputs_of(&page, "missing").await?;
        assert_that!(missing.len()).is_equal_to(2);
        assert_that!(value(&missing[0]).await?).is_equal_to("30".to_owned());

        page.expect_no_page_errors().await
    }
}
