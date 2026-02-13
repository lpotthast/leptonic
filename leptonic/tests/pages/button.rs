use thirtyfour::WebDriver;

use crate::pages::BaseActions;

pub struct ButtonPage<'d> {
    pub driver: &'d WebDriver,
    pub base_url: &'d str,
}

impl BaseActions for ButtonPage<'_> {
    fn driver(&self) -> &WebDriver {
        self.driver
    }

    fn base_url(&self) -> &str {
        self.base_url
    }
}

impl ButtonPage<'_> {
    pub async fn goto(&self) -> anyhow::Result<()> {
        tracing::info!("Navigating to button test page...");
        self.goto_path("/atoms/button").await?;
        Ok(())
    }

    pub async fn click_basic_button(&self) -> anyhow::Result<()> {
        self.click_element_with_id("test-button-basic").await
    }

    pub async fn read_basic_count(&self) -> anyhow::Result<u32> {
        let text = self.read_text_of("test-button-basic-count").await?;
        Ok(text.trim().parse()?)
    }

    pub async fn click_disabled_button(&self) -> anyhow::Result<()> {
        self.click_element_with_id("test-button-disabled").await
    }

    pub async fn read_disabled_count(&self) -> anyhow::Result<u32> {
        let text = self.read_text_of("test-button-disabled-count").await?;
        Ok(text.trim().parse()?)
    }
}
