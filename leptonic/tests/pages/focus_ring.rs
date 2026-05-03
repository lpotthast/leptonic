use thirtyfour::{TypingData, WebDriver, prelude::*};

use crate::pages::BaseActions;

pub struct FocusRingPage<'d> {
    pub driver: &'d WebDriver,
    pub base_url: &'d str,
}

impl BaseActions for FocusRingPage<'_> {
    fn driver(&self) -> &WebDriver {
        self.driver
    }

    fn base_url(&self) -> &str {
        self.base_url
    }
}

impl FocusRingPage<'_> {
    pub async fn goto(&self) -> anyhow::Result<()> {
        tracing::info!("Navigating to focus-ring hook test page...");
        self.goto_path("/hooks/focus-ring").await?;
        Ok(())
    }

    pub async fn click_target(&self) -> anyhow::Result<()> {
        self.click_element_with_id("test-fr-target").await
    }

    pub async fn click_elsewhere(&self) -> anyhow::Result<()> {
        self.click_element_with_id("test-fr-elsewhere").await
    }

    pub async fn click_before(&self) -> anyhow::Result<()> {
        self.click_element_with_id("test-fr-before").await
    }

    pub async fn tab_from_before_to_target(&self) -> anyhow::Result<()> {
        tracing::info!("Clicking 'before' button, then Tab to reach focus-ring target...");
        self.click_before().await?;
        let active = self.driver.active_element().await?;
        active.send_keys(Key::Tab).await?;
        Ok(())
    }

    pub async fn read_is_focus_visible(&self) -> anyhow::Result<bool> {
        let text = self.read_text_of("test-fr-is-focus-visible").await?;
        Ok(text.trim() == "true")
    }

    pub async fn read_is_focused(&self) -> anyhow::Result<bool> {
        let text = self.read_text_of("test-fr-is-focused").await?;
        Ok(text.trim() == "true")
    }

    pub async fn read_data_focus_visible_attr(&self) -> anyhow::Result<Option<String>> {
        let el = self.driver.find(By::Id("test-fr-target")).await?;
        Ok(el.attr("data-focus-visible").await?)
    }

    // ---- Within section ----

    pub async fn click_within_child_1(&self) -> anyhow::Result<()> {
        self.click_element_with_id("test-fr-within-child-1").await
    }

    pub async fn click_within_elsewhere(&self) -> anyhow::Result<()> {
        self.click_element_with_id("test-fr-within-elsewhere").await
    }

    pub async fn click_within_before(&self) -> anyhow::Result<()> {
        self.click_element_with_id("test-fr-within-before").await
    }

    pub async fn tab_from_within_before_to_child(&self) -> anyhow::Result<()> {
        tracing::info!(
            "Clicking 'within-before' button, then Tab to reach within container child..."
        );
        self.click_within_before().await?;
        let active = self.driver.active_element().await?;
        active.send_keys(Key::Tab).await?;
        Ok(())
    }

    pub async fn read_within_is_focus_visible(&self) -> anyhow::Result<bool> {
        let text = self.read_text_of("test-fr-within-is-focus-visible").await?;
        Ok(text.trim() == "true")
    }

    pub async fn read_within_is_focused(&self) -> anyhow::Result<bool> {
        let text = self.read_text_of("test-fr-within-is-focused").await?;
        Ok(text.trim() == "true")
    }

    pub async fn send_key_to_active(&self, key: impl Into<TypingData>) -> anyhow::Result<()> {
        let active = self.driver.active_element().await?;
        active.send_keys(key).await?;
        Ok(())
    }

    // ---- Disabled section ----

    pub async fn click_disabled_target(&self) -> anyhow::Result<()> {
        self.click_element_with_id("test-fr-disabled-target").await
    }

    pub async fn read_disabled_is_focused(&self) -> anyhow::Result<bool> {
        let text = self.read_text_of("test-fr-disabled-is-focused").await?;
        Ok(text.trim() == "true")
    }

    pub async fn read_disabled_is_focus_visible(&self) -> anyhow::Result<bool> {
        let text = self
            .read_text_of("test-fr-disabled-is-focus-visible")
            .await?;
        Ok(text.trim() == "true")
    }
}
