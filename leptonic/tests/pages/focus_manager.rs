use thirtyfour::WebDriver;

use crate::pages::BaseActions;

pub struct FocusManagerPage<'d> {
    pub driver: &'d WebDriver,
    pub base_url: &'d str,
}

impl BaseActions for FocusManagerPage<'_> {
    fn driver(&self) -> &WebDriver {
        self.driver
    }

    fn base_url(&self) -> &str {
        self.base_url
    }
}

#[allow(dead_code)]
impl FocusManagerPage<'_> {
    pub async fn goto(&self) -> anyhow::Result<()> {
        tracing::info!("Navigating to focus-manager hook test page...");
        self.goto_path("/hooks/focus-manager").await?;
        Ok(())
    }

    // ---- Basic navigation controls ----

    pub async fn click_focus_next(&self) -> anyhow::Result<()> {
        self.click_element_with_id("test-fm-focus-next").await
    }

    pub async fn click_focus_prev(&self) -> anyhow::Result<()> {
        self.click_element_with_id("test-fm-focus-prev").await
    }

    pub async fn click_focus_first(&self) -> anyhow::Result<()> {
        self.click_element_with_id("test-fm-focus-first").await
    }

    pub async fn click_focus_last(&self) -> anyhow::Result<()> {
        self.click_element_with_id("test-fm-focus-last").await
    }

    pub async fn get_active_element_id(&self) -> anyhow::Result<Option<String>> {
        let active = self.driver.active_element().await?;
        Ok(active.attr("id").await?)
    }

    // ---- Wrap section ----

    pub async fn click_wrap_item(&self, n: u32) -> anyhow::Result<()> {
        self.click_element_with_id(&format!("test-fm-wrap-item-{n}"))
            .await
    }

    pub async fn click_wrap_focus_next(&self) -> anyhow::Result<()> {
        self.click_element_with_id("test-fm-wrap-focus-next").await
    }

    pub async fn click_wrap_focus_prev(&self) -> anyhow::Result<()> {
        self.click_element_with_id("test-fm-wrap-focus-prev").await
    }

    pub async fn click_nowrap_focus_next(&self) -> anyhow::Result<()> {
        self.click_element_with_id("test-fm-nowrap-focus-next")
            .await
    }

    pub async fn click_nowrap_focus_prev(&self) -> anyhow::Result<()> {
        self.click_element_with_id("test-fm-nowrap-focus-prev")
            .await
    }

    // ---- Tabbable section ----

    pub async fn click_tabbable_item(&self, n: u32) -> anyhow::Result<()> {
        self.click_element_with_id(&format!("test-fm-tabbable-item-{n}"))
            .await
    }

    pub async fn click_tabbable_focus_next(&self) -> anyhow::Result<()> {
        self.click_element_with_id("test-fm-tabbable-focus-next")
            .await
    }

    pub async fn click_tabbable_focus_prev(&self) -> anyhow::Result<()> {
        self.click_element_with_id("test-fm-tabbable-focus-prev")
            .await
    }

    pub async fn click_nontabbable_focus_next(&self) -> anyhow::Result<()> {
        self.click_element_with_id("test-fm-nontabbable-focus-next")
            .await
    }

    pub async fn click_nontabbable_focus_prev(&self) -> anyhow::Result<()> {
        self.click_element_with_id("test-fm-nontabbable-focus-prev")
            .await
    }

    // ---- Accept filter section ----

    pub async fn click_accept_item(&self, n: u32) -> anyhow::Result<()> {
        self.click_element_with_id(&format!("test-fm-accept-item-{n}"))
            .await
    }

    pub async fn click_accept_focus_next(&self) -> anyhow::Result<()> {
        self.click_element_with_id("test-fm-accept-focus-next")
            .await
    }

    pub async fn click_accept_focus_prev(&self) -> anyhow::Result<()> {
        self.click_element_with_id("test-fm-accept-focus-prev")
            .await
    }

    // ---- Radio group (one checked) section ----

    pub async fn click_radio_item(&self, id: &str) -> anyhow::Result<()> {
        self.click_element_with_id(id).await
    }

    pub async fn click_radio_focus_next(&self) -> anyhow::Result<()> {
        self.click_element_with_id("test-fm-radio-focus-next").await
    }

    pub async fn click_radio_focus_prev(&self) -> anyhow::Result<()> {
        self.click_element_with_id("test-fm-radio-focus-prev").await
    }

    // ---- Radio group (wrap) section ----

    pub async fn click_radio_wrap_focus_next(&self) -> anyhow::Result<()> {
        self.click_element_with_id("test-fm-radio-wrap-focus-next")
            .await
    }

    pub async fn click_radio_wrap_focus_prev(&self) -> anyhow::Result<()> {
        self.click_element_with_id("test-fm-radio-wrap-focus-prev")
            .await
    }

    // ---- Radio group (none checked) section ----

    pub async fn click_radio_none_focus_next(&self) -> anyhow::Result<()> {
        self.click_element_with_id("test-fm-radio-none-focus-next")
            .await
    }

    pub async fn click_radio_none_focus_prev(&self) -> anyhow::Result<()> {
        self.click_element_with_id("test-fm-radio-none-focus-prev")
            .await
    }

    // ---- Visibility section ----

    pub async fn click_vis_item(&self, n: u32) -> anyhow::Result<()> {
        self.click_element_with_id(&format!("test-fm-vis-item-{n}"))
            .await
    }

    pub async fn click_vis_focus_next(&self) -> anyhow::Result<()> {
        self.click_element_with_id("test-fm-vis-focus-next").await
    }

    pub async fn click_vis_focus_prev(&self) -> anyhow::Result<()> {
        self.click_element_with_id("test-fm-vis-focus-prev").await
    }

    // ---- Inert section ----

    pub async fn click_inert_item(&self, n: u32) -> anyhow::Result<()> {
        self.click_element_with_id(&format!("test-fm-inert-item-{n}"))
            .await
    }

    pub async fn click_inert_focus_next(&self) -> anyhow::Result<()> {
        self.click_element_with_id("test-fm-inert-focus-next").await
    }

    pub async fn click_inert_focus_prev(&self) -> anyhow::Result<()> {
        self.click_element_with_id("test-fm-inert-focus-prev").await
    }

    // ---- Outside scope section ----

    pub async fn click_outside_external(&self) -> anyhow::Result<()> {
        self.click_element_with_id("test-fm-outside-external").await
    }

    pub async fn click_outside_focus_next(&self) -> anyhow::Result<()> {
        self.click_element_with_id("test-fm-outside-focus-next")
            .await
    }

    pub async fn click_outside_focus_prev(&self) -> anyhow::Result<()> {
        self.click_element_with_id("test-fm-outside-focus-prev")
            .await
    }
}
