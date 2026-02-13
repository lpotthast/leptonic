pub mod test_button;
pub mod test_focus;
pub mod test_focus_manager;
pub mod test_focus_ring;
pub mod test_focus_scope;
pub mod test_focus_visible;
pub mod test_focus_within;
pub mod test_focusable;
pub mod test_has_tabbable_child;

use thirtyfour::WebDriver;

#[async_trait::async_trait]
pub trait UiTest {
    fn name(&self) -> String;

    async fn run(&self, driver: &WebDriver, base_url: &str) -> anyhow::Result<()>;
}
