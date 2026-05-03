use thirtyfour::{WebDriver, prelude::*};

use crate::pages::BaseActions;

pub struct FocusPage<'d> {
    pub driver: &'d WebDriver,
    pub base_url: &'d str,
}

impl BaseActions for FocusPage<'_> {
    fn driver(&self) -> &WebDriver {
        self.driver
    }

    fn base_url(&self) -> &str {
        self.base_url
    }
}

impl FocusPage<'_> {
    pub async fn goto(&self) -> anyhow::Result<()> {
        tracing::info!("Navigating to focus hook test page...");
        self.goto_path("/hooks/focus").await?;
        Ok(())
    }

    pub async fn click_target(&self) -> anyhow::Result<()> {
        self.click_element_with_id("test-focus-target").await
    }

    pub async fn click_elsewhere(&self) -> anyhow::Result<()> {
        self.click_element_with_id("test-focus-elsewhere").await
    }

    pub async fn click_disabled_target(&self) -> anyhow::Result<()> {
        self.click_element_with_id("test-focus-disabled").await
    }

    pub async fn read_focus_count(&self) -> anyhow::Result<u32> {
        let text = self.read_text_of("test-focus-count").await?;
        Ok(text.trim().parse()?)
    }

    pub async fn read_blur_count(&self) -> anyhow::Result<u32> {
        let text = self.read_text_of("test-blur-count").await?;
        Ok(text.trim().parse()?)
    }

    pub async fn read_is_focused(&self) -> anyhow::Result<bool> {
        let text = self.read_text_of("test-is-focused").await?;
        Ok(text.trim() == "true")
    }

    pub async fn read_disabled_focus_count(&self) -> anyhow::Result<u32> {
        let text = self.read_text_of("test-disabled-focus-count").await?;
        Ok(text.trim().parse()?)
    }

    pub async fn click_before(&self) -> anyhow::Result<()> {
        self.click_element_with_id("test-focus-before").await
    }

    pub async fn tab_from_before_to_target(&self) -> anyhow::Result<()> {
        tracing::info!("Clicking 'before' button, then Tab to reach focus target...");
        self.click_before().await?;
        let active = self.driver.active_element().await?;
        active.send_keys(Key::Tab).await?;
        Ok(())
    }

    pub async fn read_focus_change_count(&self) -> anyhow::Result<u32> {
        let text = self.read_text_of("test-focus-change-count").await?;
        Ok(text.trim().parse()?)
    }

    // ---- Child focus filtering section ----

    pub async fn click_child(&self) -> anyhow::Result<()> {
        self.click_element_with_id("test-focus-child").await
    }

    pub async fn click_parent(&self) -> anyhow::Result<()> {
        self.click_element_with_id("test-focus-parent").await
    }

    pub async fn read_parent_focus_count(&self) -> anyhow::Result<u32> {
        let text = self.read_text_of("test-focus-parent-focus-count").await?;
        Ok(text.trim().parse()?)
    }

    pub async fn read_parent_blur_count(&self) -> anyhow::Result<u32> {
        let text = self.read_text_of("test-focus-parent-blur-count").await?;
        Ok(text.trim().parse()?)
    }
}
