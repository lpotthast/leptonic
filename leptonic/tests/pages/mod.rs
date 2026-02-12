pub mod button;

use thirtyfour::prelude::*;

pub trait BaseActions {
    fn driver(&self) -> &WebDriver;

    async fn click_element_with_id(&self, id: &str) -> anyhow::Result<()> {
        tracing::info!("Click element with id '{id}'.");
        let element = self.driver().find(By::Id(id)).await?;
        element.click().await?;
        Ok(())
    }

    async fn read_text_of(&self, id: &str) -> anyhow::Result<String> {
        let el = self.driver().find(By::Id(id)).await?;
        Ok(el.text().await?)
    }
}
