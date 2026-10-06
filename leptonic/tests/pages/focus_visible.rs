use browser_test::thirtyfour::{WebDriver, prelude::*};
use rootcause::Report;

use crate::pages::BaseActions;

pub struct FocusVisiblePage<'d> {
    pub driver: &'d WebDriver,
    pub base_url: &'d str,
}

impl BaseActions for FocusVisiblePage<'_> {
    fn driver(&self) -> &WebDriver {
        self.driver
    }

    fn base_url(&self) -> &str {
        self.base_url
    }
}

impl FocusVisiblePage<'_> {
    pub async fn goto(&self) -> Result<(), Report> {
        tracing::info!("Navigating to focus-visible hook test page...");
        self.goto_path("/hooks/focus-visible").await?;
        Ok(())
    }

    pub async fn click_before(&self) -> Result<(), Report> {
        self.click_element_with_id("test-fv-before").await
    }

    pub async fn click_target(&self) -> Result<(), Report> {
        self.click_element_with_id("test-fv-target").await
    }

    pub async fn click_text_input(&self) -> Result<(), Report> {
        self.click_element_with_id("test-fv-text-input").await
    }

    pub async fn tab_from_before_to_target(&self) -> Result<(), Report> {
        tracing::info!("Clicking 'before' button, then Tab to reach focus-visible target...");
        self.click_before().await?;
        let active = self.driver.active_element().await?;
        active.send_keys(Key::Tab).await?;
        Ok(())
    }

    pub async fn read_visible(&self) -> Result<bool, Report> {
        let text = self.read_text_of("test-fv-visible").await?;
        Ok(text.trim() == "true")
    }

    pub async fn read_modality(&self) -> Result<String, Report> {
        let text = self.read_text_of("test-fv-modality").await?;
        Ok(text.trim().to_string())
    }

    pub async fn read_stored_modality(&self) -> Result<String, Report> {
        let text = self.read_text_of("test-fv-stored-modality").await?;
        Ok(text.trim().to_string())
    }
}
