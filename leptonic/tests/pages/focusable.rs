use thirtyfour::{TypingData, WebDriver, prelude::*};

use crate::pages::BaseActions;

pub struct FocusablePage<'d> {
    pub driver: &'d WebDriver,
    pub base_url: &'d str,
}

impl BaseActions for FocusablePage<'_> {
    fn driver(&self) -> &WebDriver {
        self.driver
    }

    fn base_url(&self) -> &str {
        self.base_url
    }
}

impl FocusablePage<'_> {
    pub async fn goto(&self) -> anyhow::Result<()> {
        tracing::info!("Navigating to focusable hook test page...");
        self.goto_path("/hooks/focusable").await?;
        Ok(())
    }

    pub async fn read_normal_tabindex(&self) -> anyhow::Result<Option<String>> {
        let el = self.driver.find(By::Id("test-fcbl-normal")).await?;
        Ok(el.attr("tabindex").await?)
    }

    pub async fn read_disabled_tabindex(&self) -> anyhow::Result<Option<String>> {
        let el = self.driver.find(By::Id("test-fcbl-disabled")).await?;
        Ok(el.attr("tabindex").await?)
    }

    pub async fn read_excluded_tabindex(&self) -> anyhow::Result<Option<String>> {
        let el = self.driver.find(By::Id("test-fcbl-excluded")).await?;
        Ok(el.attr("tabindex").await?)
    }

    pub async fn get_active_element_id(&self) -> anyhow::Result<Option<String>> {
        let active = self.driver.active_element().await?;
        Ok(active.attr("id").await?)
    }

    pub async fn click_normal(&self) -> anyhow::Result<()> {
        self.click_element_with_id("test-fcbl-normal").await
    }

    pub async fn send_key_to_active(&self, key: impl Into<TypingData>) -> anyhow::Result<()> {
        let active = self.driver.active_element().await?;
        active.send_keys(key).await?;
        Ok(())
    }

    pub async fn read_keydown_count(&self) -> anyhow::Result<u32> {
        let text = self.read_text_of("test-fcbl-keydown-count").await?;
        Ok(text.trim().parse()?)
    }

    pub async fn read_keyup_count(&self) -> anyhow::Result<u32> {
        let text = self.read_text_of("test-fcbl-keyup-count").await?;
        Ok(text.trim().parse()?)
    }

    pub async fn tab_from_active(&self) -> anyhow::Result<()> {
        let active = self.driver.active_element().await?;
        active.send_keys(Key::Tab).await?;
        Ok(())
    }

    pub async fn click_focus_btn(&self) -> anyhow::Result<()> {
        self.click_element_with_id("test-fcbl-focus-btn").await
    }

    // ---- Dynamic disabled transition section ----

    pub async fn read_dynamic_tabindex(&self) -> anyhow::Result<Option<String>> {
        let el = self.driver.find(By::Id("test-fcbl-dynamic")).await?;
        Ok(el.attr("tabindex").await?)
    }

    pub async fn click_dynamic_toggle(&self) -> anyhow::Result<()> {
        self.click_element_with_id("test-fcbl-dynamic-toggle").await
    }
}
