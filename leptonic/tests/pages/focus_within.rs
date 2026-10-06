use browser_test::thirtyfour::WebDriver;
use rootcause::Report;

use crate::pages::BaseActions;

pub struct FocusWithinPage<'d> {
    pub driver: &'d WebDriver,
    pub base_url: &'d str,
}

impl BaseActions for FocusWithinPage<'_> {
    fn driver(&self) -> &WebDriver {
        self.driver
    }

    fn base_url(&self) -> &str {
        self.base_url
    }
}

impl FocusWithinPage<'_> {
    pub async fn goto(&self) -> Result<(), Report> {
        tracing::info!("Navigating to focus-within hook test page...");
        self.goto_path("/hooks/focus-within").await?;
        Ok(())
    }

    pub async fn click_input_a(&self) -> Result<(), Report> {
        self.click_element_with_id("test-fw-input-a").await
    }

    pub async fn click_input_b(&self) -> Result<(), Report> {
        self.click_element_with_id("test-fw-input-b").await
    }

    pub async fn click_outside(&self) -> Result<(), Report> {
        self.click_element_with_id("test-fw-outside").await
    }

    pub async fn read_is_focus_within(&self) -> Result<bool, Report> {
        let text = self.read_text_of("test-fw-is-focus-within").await?;
        Ok(text.trim() == "true")
    }

    pub async fn read_focus_within_count(&self) -> Result<u32, Report> {
        let text = self.read_text_of("test-fw-focus-within-count").await?;
        Ok(text.trim().parse()?)
    }

    pub async fn read_blur_within_count(&self) -> Result<u32, Report> {
        let text = self.read_text_of("test-fw-blur-within-count").await?;
        Ok(text.trim().parse()?)
    }

    // ---- Disabled section ----

    pub async fn click_disabled_input(&self) -> Result<(), Report> {
        self.click_element_with_id("test-fw-disabled-input").await
    }

    pub async fn read_disabled_is_focus_within(&self) -> Result<bool, Report> {
        let text = self
            .read_text_of("test-fw-disabled-is-focus-within")
            .await?;
        Ok(text.trim() == "true")
    }

    pub async fn read_disabled_focus_count(&self) -> Result<u32, Report> {
        let text = self.read_text_of("test-fw-disabled-focus-count").await?;
        Ok(text.trim().parse()?)
    }

    // ---- Change callback section ----

    pub async fn click_change_input(&self) -> Result<(), Report> {
        self.click_element_with_id("test-fw-change-input").await
    }

    pub async fn read_change_value(&self) -> Result<bool, Report> {
        let text = self.read_text_of("test-fw-change-value").await?;
        Ok(text.trim() == "true")
    }

    pub async fn read_change_count(&self) -> Result<u32, Report> {
        let text = self.read_text_of("test-fw-change-count").await?;
        Ok(text.trim().parse()?)
    }

    pub async fn click_before(&self) -> Result<(), Report> {
        self.click_element_with_id("test-fw-before").await
    }

    // ---- Nested containers section ----

    pub async fn click_nested_input(&self) -> Result<(), Report> {
        self.click_element_with_id("test-fw-nested-input").await
    }

    pub async fn read_nested_outer_is_focus_within(&self) -> Result<bool, Report> {
        let text = self
            .read_text_of("test-fw-nested-outer-is-focus-within")
            .await?;
        Ok(text.trim() == "true")
    }

    pub async fn read_nested_inner_is_focus_within(&self) -> Result<bool, Report> {
        let text = self
            .read_text_of("test-fw-nested-inner-is-focus-within")
            .await?;
        Ok(text.trim() == "true")
    }
}
