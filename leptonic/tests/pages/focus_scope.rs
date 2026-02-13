use std::time::Duration;

use thirtyfour::{prelude::*, WebDriver};

use crate::pages::BaseActions;

pub struct FocusScopePage<'d> {
    pub driver: &'d WebDriver,
    pub base_url: &'d str,
}

impl BaseActions for FocusScopePage<'_> {
    fn driver(&self) -> &WebDriver {
        self.driver
    }

    fn base_url(&self) -> &str {
        self.base_url
    }
}

impl FocusScopePage<'_> {
    pub async fn goto(&self) -> anyhow::Result<()> {
        tracing::info!("Navigating to focus-scope atom test page...");
        self.goto_path("/atoms/focus-scope").await?;
        // Wait for auto-focus effects to settle.
        tokio::time::sleep(Duration::from_millis(200)).await;
        Ok(())
    }

    pub async fn get_active_element_id(&self) -> anyhow::Result<Option<String>> {
        let active = self.driver.active_element().await?;
        Ok(active.attr("id").await?)
    }

    // ---- Containment section ----

    pub async fn click_contain_btn_1(&self) -> anyhow::Result<()> {
        self.click_element_with_id("test-fs-contain-btn-1").await
    }

    // ---- Restore focus section ----

    pub async fn click_restore_toggle(&self) -> anyhow::Result<()> {
        self.click_element_with_id("test-fs-restore-toggle").await
    }

    // ---- Nested section ----

    pub async fn click_nested_outer_btn(&self) -> anyhow::Result<()> {
        self.click_element_with_id("test-fs-nested-outer-btn").await
    }

    pub async fn click_nested_inner_btn_1(&self) -> anyhow::Result<()> {
        self.click_element_with_id("test-fs-nested-inner-btn-1")
            .await
    }

    pub async fn tab_from_active(&self) -> anyhow::Result<()> {
        let active = self.driver.active_element().await?;
        active.send_keys(Key::Tab).await?;
        Ok(())
    }

    // ---- Nested restore section ----

    pub async fn click_nested_restore_trigger(&self) -> anyhow::Result<()> {
        self.click_element_with_id("test-fs-nested-restore-trigger")
            .await
    }

    // ---- Outside ----

    pub async fn click_outside(&self) -> anyhow::Result<()> {
        self.click_element_with_id("test-fs-outside").await
    }
}
