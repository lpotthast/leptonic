// Upstream: @adobe/react-spectrum/test/color/ColorSlider.test.tsx @ 99e6102368
// Upstream: react-aria-components/test/ColorSlider.test.js @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{BrowserTest, async_trait, thirtyfour::prelude::*};
use rootcause::{Report, report};

use crate::{
    pages::{ElementActions, Page, PageActions},
    polling::wait_for,
};

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

        cases!(
            input_props(&page),
            hue_value_text_and_label(&page),
            keyboard(&page),
            track_click(&page),
            disabled(&page),
            forms(&page),
        );

        Ok(())
    }
}

/// Dragging thumbs and tracks (react-spectrum's `ColorSlider.test.tsx`, "dragging the thumb
/// works", "... when vertical", "clicking and dragging on the track works when vertical"), the
/// `Label`'s default text, and parts that mount again.
pub struct ColorSliderDragTests {}

#[async_trait]
impl BrowserTest<str> for ColorSliderDragTests {
    fn name(&self) -> Cow<'_, str> {
        "color_slider_drag_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/color-slider").await?;

        cases!(
            default_label(&page),
            drag_thumb(&page),
            drag_thumb_vertical(&page),
            drag_track_vertical(&page),
            mounted_again(&page),
        );

        Ok(())
    }
}

/// The range input of the slider `#id`.
async fn input(page: &Page<'_>, id: &str) -> Result<WebElement, Report> {
    page.element(format!("#{id} input[type=range]")).await
}

/// The value of a range input as a number.
async fn number(input: &WebElement) -> Result<f64, Report> {
    Ok(input.value().await?.unwrap_or_default().parse()?)
}

/// The change log of the RGB sliders: `change:<hex>` and `end:<hex>` entries, comma-separated.
async fn log(page: &Page<'_>) -> Result<WebElement, Report> {
    page.element("#test-cs-log").await
}

/// The change log of the hue sliders: `change:<hue>` and `end:<hue>` entries.
async fn hue_log(page: &Page<'_>) -> Result<WebElement, Report> {
    page.element("#test-cs-hue-log").await
}

/// Empties the change log (a script click, which leaves focus where it is).
async fn clear(page: &Page<'_>) -> Result<(), Report> {
    page.element("#test-cs-clear")
        .await?
        .virtual_click()
        .await?;
    log(page).await?.wait_for_inner_text("").await?;
    Ok(())
}

/// Empties the hue log (a script click, which leaves focus where it is).
async fn clear_hues(page: &Page<'_>) -> Result<(), Report> {
    page.element("#test-cs-hue-clear")
        .await?
        .virtual_click()
        .await?;
    hue_log(page).await?.wait_for_inner_text("").await?;
    Ok(())
}

/// The last entry of the hue log (dragging logs a change per move).
async fn last_hue(page: &Page<'_>) -> Result<String, Report> {
    let log = hue_log(page).await?.inner_text().await?;
    Ok(log.rsplit(',').next().unwrap_or_default().to_owned())
}

/// Waits until the last entry of the hue log is `expected`.
async fn wait_for_last_hue(page: &Page<'_>, expected: &str) -> Result<(), Report> {
    wait_for("the last entry of the hue log")
        .observing(|| last_hue(page))
        .to_be_equal_to(expected)
        .await?;
    Ok(())
}

/// The hue of a hue log entry `<kind>:<hue>`.
fn hue_of(entry: &str, kind: &str) -> Option<f64> {
    entry.strip_prefix(kind)?.strip_prefix(':')?.parse().ok()
}

/// Waits until the last hue log entry is `kind:<hue>` with the hue within 2° of `expected`;
/// returns the hue.
async fn wait_for_last_hue_near(page: &Page<'_>, kind: &str, expected: f64) -> Result<f64, Report> {
    wait_for("the last entry of the hue log")
        .observing(|| last_hue(page))
        .to_be(&format!("{kind}:{expected} (±2)"), |entry| {
            hue_of(entry, kind).is_some_and(|hue| (hue - expected).abs() <= 2.0)
        })
        .await?;
    let entry = last_hue(page).await?;
    hue_of(&entry, kind).ok_or_else(|| report!("the last hue log entry changed to {entry:?}"))
}

/// "sets input props"; the channel names a slider without labels.
async fn input_props(page: &Page<'_>) -> Result<(), Report> {
    let red = input(page, "test-cs-red").await?;
    assert_that!(red.attr("type").await?)
        .get_some()
        .is_equal_to("range");
    assert_that!(red.attr("min").await?)
        .get_some()
        .is_equal_to("0");
    assert_that!(red.attr("max").await?)
        .get_some()
        .is_equal_to("255");
    assert_that!(red.attr("step").await?)
        .get_some()
        .is_equal_to("1");
    // "sets input props": the value and the color's name.
    assert_that!(red.attr("aria-valuetext").await?)
        .get_some()
        .is_equal_to("0, black");
    // "sets a default aria-label when label={null}": on the group, which labels the input.
    // "should render a slider with default class": the track is the group, inside the root.
    let group = page.element("#test-cs-red [role=group]").await?;
    assert_that!(group.attr("class").await?)
        .get_some()
        .contains("leptonic-ColorSliderTrack");
    let root = page.element("#test-cs-red .leptonic-ColorSlider").await?;
    assert_that!(root.attr("role").await?).is_none();
    assert_that!(group.attr("aria-label").await?)
        .get_some()
        .is_equal_to("Red");
    let group_id = group.id().await?;
    assert_that!(red.attr("aria-labelledby").await?).is_equal_to(group_id);
    assert_that!(red.attr("aria-label").await?).is_none();
    let output = page.element("#test-cs-red output").await?;
    assert_that!(output.inner_text().await?).is_equal_to("0");
    Ok(())
}

/// "sets aria-valuetext to formatted value" (with the hue's name); a `Label` names it; "clicking
/// on label should focus input".
async fn hue_value_text_and_label(page: &Page<'_>) -> Result<(), Report> {
    let hue = input(page, "test-cs-hue").await?;
    assert_that!(hue.attr("max").await?)
        .get_some()
        .is_equal_to("360");
    assert_that!(hue.attr("aria-valuetext").await?)
        .get_some()
        .is_equal_to("10°, red orange");
    assert_that!(hue.attr("aria-label").await?).is_none();
    page.element("#test-cs-hue [id^=label]")
        .await?
        .click()
        .await?;
    page.wait_for_focus(&hue).await?;
    Ok(())
}

/// "keyboard events": steps, pages, Home/End.
async fn keyboard(page: &Page<'_>) -> Result<(), Report> {
    let red = input(page, "test-cs-red").await?;
    page.element("#test-cs-before").await?.focus().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&red).await?;
    let log = log(page).await?;
    for (key, expected) in [
        (Key::Right, "010000"),
        (Key::PageUp, "120000"),
        (Key::Home, "000000"),
        (Key::End, "FF0000"),
    ] {
        page.send_keys(key).await?;
        log.wait_for_inner_text(&format!("change:{expected},end:{expected}"))
            .await?;
        clear(page).await?;
    }
    Ok(())
}

/// "clicking and dragging on the track works": a quarter along (±1, sub-pixel positions).
async fn track_click(page: &Page<'_>) -> Result<(), Report> {
    let red = input(page, "test-cs-red").await?;
    let track = page.element("#test-cs-red [role=group]").await?;
    track.scroll_into_view().await?;
    page.driver
        .action_chain()
        .move_to_element_with_offset(&track, -50, 0)
        .click()
        .perform()
        .await?;
    wait_for("the red value")
        .observing(|| number(&red))
        .to_be("64 (±1)", |red| (red - 64.0).abs() <= 1.0)
        .await?;
    Ok(())
}

/// "disabled".
async fn disabled(page: &Page<'_>) -> Result<(), Report> {
    let disabled = input(page, "test-cs-disabled").await?;
    assert_that!(disabled.is_enabled().await?).is_false();
    page.element("#test-cs-a").await?.focus().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&page.element("#test-cs-b").await?)
        .await?;
    Ok(())
}

/// "supports form name", "supports form reset".
async fn forms(page: &Page<'_>) -> Result<(), Report> {
    let form = input(page, "test-cs-form").await?;
    assert_that!(form.attr("name").await?)
        .get_some()
        .is_equal_to("red");
    assert_that!(number(&form).await?).is_equal_to(127.0);
    form.focus().await?;
    page.send_keys(Key::Right).await?;
    form.wait_for_attr("aria-valuetext", Some("128, dark vibrant red"))
        .await?;
    page.element("#test-cs-reset").await?.click().await?;
    form.wait_for_attr("aria-valuetext", Some("127, dark vibrant red"))
        .await?;
    Ok(())
}

/// "defaults to showing the channel as a label" (react-aria-components: the `Label`'s default
/// children); it labels the group.
async fn default_label(page: &Page<'_>) -> Result<(), Report> {
    let label = page.element("#test-cs-label .leptonic-Label").await?;
    assert_that!(label.inner_text().await?).is_equal_to("Green");
    let group = page.element("#test-cs-label [role=group]").await?;
    let label_id = label.id().await?;
    assert_that!(group.attr("aria-labelledby").await?).is_equal_to(label_id);
    Ok(())
}

/// "dragging the thumb works": 80 of 200 pixels is 144°; no change on the press, the input has
/// focus.
async fn drag_thumb(page: &Page<'_>) -> Result<(), Report> {
    let drag = input(page, "test-cs-drag").await?;
    let thumb = page.element("#test-cs-drag .leptonic-ColorThumb").await?;
    thumb.scroll_into_view().await?;
    page.driver
        .action_chain()
        .click_and_hold_element(&thumb)
        .perform()
        .await?;
    page.wait_for_focus(&drag).await?;
    hue_log(page).await?.inner_text_stays("").await?;
    page.driver
        .action_chain()
        .move_by_offset(80, 0)
        .perform()
        .await?;
    wait_for_last_hue(page, "change:144").await?;
    page.driver.action_chain().release().perform().await?;
    wait_for_last_hue(page, "end:144").await?;
    page.wait_for_focus(&drag).await?;
    clear_hues(page).await?;
    Ok(())
}

/// "dragging the thumb works when vertical": upwards.
async fn drag_thumb_vertical(page: &Page<'_>) -> Result<(), Report> {
    let vertical = input(page, "test-cs-vertical").await?;
    assert_that!(vertical.attr("aria-orientation").await?)
        .get_some()
        .is_equal_to("vertical");
    let thumb = page
        .element("#test-cs-vertical .leptonic-ColorThumb")
        .await?;
    thumb.scroll_into_view().await?;
    page.driver
        .action_chain()
        .click_and_hold_element(&thumb)
        .move_by_offset(0, -80)
        .perform()
        .await?;
    wait_for_last_hue(page, "change:144").await?;
    page.driver.action_chain().release().perform().await?;
    wait_for_last_hue(page, "end:144").await?;
    clear_hues(page).await?;
    Ok(())
}

/// "clicking and dragging on the track works when vertical": the middle is 180° (±2:
/// WebDriver's integer center), 40 pixels up 72° more. The keyboard on a vertical slider: Up
/// increases.
async fn drag_track_vertical(page: &Page<'_>) -> Result<(), Report> {
    let vertical = input(page, "test-cs-vertical").await?;
    let track = page.element("#test-cs-vertical [role=group]").await?;
    page.driver
        .action_chain()
        .move_to_element_center(&track)
        .click_and_hold()
        .perform()
        .await?;
    let pressed = wait_for_last_hue_near(page, "change", 180.0).await?;
    page.wait_for_focus(&vertical).await?;
    page.driver
        .action_chain()
        .move_by_offset(0, -40)
        .perform()
        .await?;
    let dragged = wait_for_last_hue_near(page, "change", pressed + 72.0).await?;
    page.driver.action_chain().release().perform().await?;
    wait_for_last_hue(page, &format!("end:{dragged}")).await?;
    clear_hues(page).await?;
    page.send_keys(Key::Up).await?;
    wait_for_last_hue(page, &format!("end:{}", dragged + 1.0)).await?;
    Ok(())
}

/// Parts mounted again (inside a `<Show>`) render and work.
async fn mounted_again(page: &Page<'_>) -> Result<(), Report> {
    let toggle = page.element("#test-cs-toggle").await?;
    toggle.click().await?;
    page.wait_for_count("#test-cs-show input[type=range]", 0)
        .await?;
    toggle.click().await?;
    page.wait_for_count("#test-cs-show input[type=range]", 1)
        .await?;
    let shown = input(page, "test-cs-show").await?;
    assert_that!(shown.attr("aria-valuetext").await?)
        .get_some()
        .starts_with("50, ");
    shown.focus().await?;
    page.send_keys(Key::Right).await?;
    page.element("#test-cs-show output")
        .await?
        .wait_for_inner_text("51")
        .await?;
    Ok(())
}
