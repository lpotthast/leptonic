pub mod test_button;

use thirtyfour::WebDriver;

#[async_trait::async_trait]
pub trait UiTest {
    fn name(&self) -> String;

    async fn run(&self, driver: &WebDriver) -> anyhow::Result<()>;
}
