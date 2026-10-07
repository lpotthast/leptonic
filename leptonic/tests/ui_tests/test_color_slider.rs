// Upstream: @adobe/react-spectrum/test/color/ColorSlider.test.tsx @ 99e6102368
// Upstream: react-aria-components/test/ColorSlider.test.js @ 99e6102368
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
        // "should render a slider with default class": the track is the group, inside the root.
        let group = page.css("#test-cs-red [role=group]").await?;
        assert_that!(attr(&group, "class").await?.unwrap_or_default())
            .contains("leptonic-ColorSliderTrack");
        let root = page.css("#test-cs-red .leptonic-ColorSlider").await?;
        assert_that!(attr(&root, "role").await?).is_none();
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
        let track = page.css("#test-cs-red [role=group]").await?;
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

/// Dragging thumbs and tracks (react-spectrum's `ColorSlider.test.tsx`, "dragging the thumb
/// works", "... when vertical", "clicking and dragging on the track works when vertical"), the
/// `Label`'s default text, and parts that mount again.
pub struct ColorSliderDragTests {}

#[async_trait]
impl BrowserTest<str> for ColorSliderDragTests {
    fn name(&self) -> Cow<'_, str> {
        "color_slider_drag_tests".into()
    }

    #[allow(clippy::too_many_lines)]
    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/color-slider").await?;

        // "defaults to showing the channel as a label" (react-aria-components: the `Label`'s
        // default children); it labels the group.
        let label = page.css("#test-cs-label .leptonic-Label").await?;
        assert_that!(label.text().await?).is_equal_to("Green".to_owned());
        let group = page.css("#test-cs-label [role=group]").await?;
        let label_id = attr(&label, "id").await?.unwrap_or_default();
        assert_that!(attr(&group, "aria-labelledby").await?).is_equal_to(Some(label_id));

        // "dragging the thumb works": 80 of 200 pixels is 144°; no change on the press, the
        // input has focus.
        let drag = input(&page, "test-cs-drag").await?;
        let thumb = page.css("#test-cs-drag .leptonic-ColorThumb").await?;
        scroll_into_view(&page, &thumb).await?;
        driver
            .action_chain()
            .click_and_hold_element(&thumb)
            .perform()
            .await?;
        expect_active(&page, &drag).await?;
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        assert_that!(page.element("test-cs-hue-log").await?.text().await?)
            .is_equal_to(String::new());
        driver
            .action_chain()
            .move_by_offset(80, 0)
            .perform()
            .await?;
        wait_for_last_hue(&page, "change:144").await?;
        driver.action_chain().release().perform().await?;
        wait_for_last_hue(&page, "end:144").await?;
        expect_active(&page, &drag).await?;
        clear_hues(&page).await?;

        // "dragging the thumb works when vertical": upwards.
        let vertical = input(&page, "test-cs-vertical").await?;
        assert_that!(attr(&vertical, "aria-orientation").await?)
            .is_equal_to(Some("vertical".to_owned()));
        let thumb = page.css("#test-cs-vertical .leptonic-ColorThumb").await?;
        scroll_into_view(&page, &thumb).await?;
        driver
            .action_chain()
            .click_and_hold_element(&thumb)
            .move_by_offset(0, -80)
            .perform()
            .await?;
        wait_for_last_hue(&page, "change:144").await?;
        driver.action_chain().release().perform().await?;
        wait_for_last_hue(&page, "end:144").await?;
        clear_hues(&page).await?;

        // "clicking and dragging on the track works when vertical": the middle is 180° (±2:
        // WebDriver's integer center), 40 pixels up 72° more.
        let track = page.css("#test-cs-vertical [role=group]").await?;
        driver
            .action_chain()
            .move_to_element_center(&track)
            .click_and_hold()
            .perform()
            .await?;
        let pressed = wait_for_last_hue_near(&page, "change", 180.0).await?;
        expect_active(&page, &vertical).await?;
        driver
            .action_chain()
            .move_by_offset(0, -40)
            .perform()
            .await?;
        let dragged = wait_for_last_hue_near(&page, "change", pressed + 72.0).await?;
        driver.action_chain().release().perform().await?;
        wait_for_last_hue(&page, &format!("end:{dragged}")).await?;
        clear_hues(&page).await?;
        // The keyboard on a vertical slider: Up increases.
        page.send_keys_to_active(Key::Up).await?;
        wait_for_last_hue(&page, &format!("end:{}", dragged + 1.0)).await?;

        // Parts mounted again (inside a `<Show>`) render and work.
        let toggle = page.element("test-cs-toggle").await?;
        toggle.click().await?;
        page.wait_for_count("#test-cs-show input[type=range]", 0)
            .await?;
        toggle.click().await?;
        page.wait_for_count("#test-cs-show input[type=range]", 1)
            .await?;
        let shown = input(&page, "test-cs-show").await?;
        assert_that!(attr(&shown, "aria-valuetext").await?.unwrap_or_default()).starts_with("50, ");
        shown.focus().await?;
        page.send_keys_to_active(Key::Right).await?;
        page.wait_for_selector_text("#test-cs-show output", "51")
            .await?;

        page.expect_no_page_errors().await
    }
}

async fn scroll_into_view(page: &Page<'_>, element: &WebElement) -> Result<(), Report> {
    page.driver
        .execute(
            "arguments[0].scrollIntoView({block: 'center'});",
            vec![element.to_json()?],
        )
        .await?;
    Ok(())
}

/// Waits until the last entry of the hue log is `expected` (dragging logs a change per move).
async fn wait_for_last_hue(page: &Page<'_>, expected: &str) -> Result<(), Report> {
    let mut last = String::new();
    for _ in 0..100 {
        let log = page.element("test-cs-hue-log").await?.text().await?;
        last = log.rsplit(',').next().unwrap_or_default().to_owned();
        if last == expected {
            return Ok(());
        }
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }
    Err(rootcause::report!(
        "the last hue log entry is {last:?}, not {expected:?}"
    ))
}

/// Waits until the last hue log entry is `kind:<hue>` with the hue within 2° of `expected`;
/// returns the hue.
async fn wait_for_last_hue_near(page: &Page<'_>, kind: &str, expected: f64) -> Result<f64, Report> {
    let mut last = String::new();
    for _ in 0..100 {
        let log = page.element("test-cs-hue-log").await?.text().await?;
        last = log.rsplit(',').next().unwrap_or_default().to_owned();
        if let Some(hue) = last
            .strip_prefix(&format!("{kind}:"))
            .and_then(|hue| hue.parse::<f64>().ok())
            && (hue - expected).abs() <= 2.0
        {
            return Ok(hue);
        }
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }
    Err(rootcause::report!(
        "the last hue log entry is {last:?}, not {kind}:{expected} (±2)"
    ))
}

async fn clear_hues(page: &Page<'_>) -> Result<(), Report> {
    page.driver
        .execute(
            "document.getElementById('test-cs-hue-clear').click();",
            vec![],
        )
        .await?;
    page.wait_for_text("test-cs-hue-log", "").await
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
