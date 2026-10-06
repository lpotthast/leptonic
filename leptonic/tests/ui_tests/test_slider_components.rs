use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{By, WebDriver},
};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

/// The styled `Slider`'s theme: disabled sliders look disabled, keyboard focus rings the thumb,
/// and vertical sliders run bottom-up with their marks along the track.
pub struct SliderComponentsTests {}

#[async_trait]
impl BrowserTest<str> for SliderComponentsTests {
    fn name(&self) -> Cow<'_, str> {
        "slider_components_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/components/slider").await?;

        let disabled = page.css("#test-cslider-disabled .leptonic-slider").await?;
        assert_that!(disabled.css_value("opacity").await?).is_equal_to("0.5".to_owned());

        // Keyboard focus rings the thumb.
        page.element("test-cslider-before").await?.focus().await?;
        page.press_tab().await?;
        let thumb = page.css("#test-cslider-horizontal .thumb").await?;
        page.wait_for_attr(&thumb, "data-focus-visible", Some("true"))
            .await?;
        assert_that!(thumb.css_value("outline-style").await?).is_equal_to("solid".to_owned());

        // Vertical: taller than wide, the maximum's thumb at the top, marks from the bottom.
        let vertical = page.css("#test-cslider-vertical .leptonic-slider").await?;
        let rect = vertical.rect().await?;
        assert_that!(rect.height > rect.width * 2.0)
            .with_detail_message(format!("{rect:?}"))
            .is_true();
        let track = page
            .css("#test-cslider-vertical .track")
            .await?
            .rect()
            .await?;
        let thumb = page
            .css("#test-cslider-vertical .thumb")
            .await?
            .rect()
            .await?;
        let thumb_center = thumb.y + thumb.height / 2.0;
        assert_that!((thumb_center - track.y).abs() <= 1.0)
            .with_detail_message(format!("thumb {thumb:?}, track {track:?}"))
            .is_true();
        let marks = driver
            .find_all(By::Css("#test-cslider-vertical .mark"))
            .await?;
        assert_that!(marks.len()).is_equal_to(5);
        let lowest = marks[0].rect().await?;
        let highest = marks[4].rect().await?;
        assert_that!(lowest.y > highest.y + track.height / 2.0)
            .with_detail_message(format!("{lowest:?} {highest:?}"))
            .is_true();

        page.expect_no_page_errors().await
    }
}
