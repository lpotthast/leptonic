use thirtyfour::WebDriver;

use crate::pages::BaseActions;

pub struct HasTabbableChildPage<'d> {
    pub driver: &'d WebDriver,
    pub base_url: &'d str,
}

impl BaseActions for HasTabbableChildPage<'_> {
    fn driver(&self) -> &WebDriver {
        self.driver
    }

    fn base_url(&self) -> &str {
        self.base_url
    }
}

impl HasTabbableChildPage<'_> {
    pub async fn goto(&self) -> anyhow::Result<()> {
        tracing::info!("Navigating to has-tabbable-child hook test page...");
        self.goto_path("/hooks/has-tabbable-child").await?;
        Ok(())
    }

    pub async fn read_result(&self) -> anyhow::Result<bool> {
        let text = self.read_text_of("test-htc-result").await?;
        Ok(text.trim() == "true")
    }

    pub async fn click_toggle(&self) -> anyhow::Result<()> {
        self.click_element_with_id("test-htc-toggle").await
    }

    pub async fn read_none_result(&self) -> anyhow::Result<bool> {
        let text = self.read_text_of("test-htc-none-result").await?;
        Ok(text.trim() == "true")
    }

    pub async fn read_disabled_result(&self) -> anyhow::Result<bool> {
        let text = self.read_text_of("test-htc-disabled-result").await?;
        Ok(text.trim() == "true")
    }

    // ---- Deeply nested section ----

    pub async fn read_nested_result(&self) -> anyhow::Result<bool> {
        let text = self.read_text_of("test-htc-nested-result").await?;
        Ok(text.trim() == "true")
    }

    // ---- Attribute mutation section ----

    pub async fn read_attr_result(&self) -> anyhow::Result<bool> {
        let text = self.read_text_of("test-htc-attr-result").await?;
        Ok(text.trim() == "true")
    }

    pub async fn click_attr_toggle(&self) -> anyhow::Result<()> {
        self.click_element_with_id("test-htc-attr-toggle").await
    }
}
