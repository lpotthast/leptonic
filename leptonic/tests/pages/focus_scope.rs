use browser_test::thirtyfour::WebDriver;
use rootcause::Report;

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
    pub async fn goto(&self) -> Result<(), Report> {
        tracing::info!("Navigating to focus-scope atom test page...");
        self.goto_path("/atoms/focus-scope").await?;
        // The auto-focus scope takes focus once mounted.
        self.wait_for_active_id("test-fs-autofocus-btn-1").await?;
        Ok(())
    }

    // ---- Containment section ----

    pub async fn click_contain_btn_1(&self) -> Result<(), Report> {
        self.click_element_with_id("test-fs-contain-btn-1").await
    }

    // ---- Restore focus section ----

    pub async fn click_restore_toggle(&self) -> Result<(), Report> {
        self.click_element_with_id("test-fs-restore-toggle").await
    }

    // ---- Nested section ----

    pub async fn click_nested_outer_btn(&self) -> Result<(), Report> {
        self.click_element_with_id("test-fs-nested-outer-btn").await
    }

    pub async fn click_nested_inner_btn_1(&self) -> Result<(), Report> {
        self.click_element_with_id("test-fs-nested-inner-btn-1")
            .await
    }

    // ---- Nested restore section ----

    pub async fn click_nested_restore_trigger(&self) -> Result<(), Report> {
        self.click_element_with_id("test-fs-nested-restore-trigger")
            .await
    }

    // ---- Outside ----

    pub async fn click_outside(&self) -> Result<(), Report> {
        self.click_element_with_id("test-fs-outside").await
    }
}
