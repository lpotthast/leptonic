use std::{borrow::Cow, time::Duration};

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{By, WebDriver},
};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

pub struct LiveAnnouncerTests {}

#[async_trait]
impl BrowserTest<str> for LiveAnnouncerTests {
    fn name(&self) -> Cow<'_, str> {
        "live_announcer_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/hooks/live-announcer").await?;

        // Event handlers run without a reactive owner. Announcing must still work.
        page.click_element_with_id("test-la-polite").await?;
        page.wait_for_selector("[data-live-announcer] [aria-live=polite] div")
            .await?;
        assert_that!(log_text(&page, "polite").await?).is_equal_to("Polite hello".to_owned());

        page.click_element_with_id("test-la-assertive").await?;
        page.wait_for_selector("[data-live-announcer] [aria-live=assertive] div")
            .await?;
        assert_that!(log_text(&page, "assertive").await?).is_equal_to("Urgent hello".to_owned());

        // Exactly one announcer node, shared by all announcements.
        let announcers = page
            .driver
            .find_all(By::Css("[data-live-announcer]"))
            .await?;
        assert_that!(announcers.len()).is_equal_to(1);

        page.click_element_with_id("test-la-clear").await?;
        assert_that!(log_text(&page, "polite").await?).is_equal_to(String::new());
        assert_that!(log_text(&page, "assertive").await?).is_equal_to(String::new());

        // Announcements are removed after their timeout.
        page.click_element_with_id("test-la-short").await?;
        page.wait_for_selector("[data-live-announcer] [aria-live=polite] div")
            .await?;
        tokio::time::sleep(Duration::from_millis(600)).await;
        let entries = page
            .driver
            .find_all(By::Css("[data-live-announcer] [aria-live=polite] div"))
            .await?;
        assert_that!(entries.len()).is_equal_to(0);
        Ok(())
    }
}

/// Text content of a log region (it is visually hidden, so WebDriver's `text()` returns "").
async fn log_text(page: &Page<'_>, live: &str) -> Result<String, Report> {
    let log = page
        .driver
        .find(By::Css(format!("[data-live-announcer] [aria-live={live}]")))
        .await?;
    Ok(log.prop("textContent").await?.unwrap_or_default())
}
