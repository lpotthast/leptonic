pub mod button;
pub mod focus;
pub mod focus_manager;
pub mod focus_ring;
pub mod focus_scope;
pub mod focus_visible;
pub mod focus_within;
pub mod focusable;
pub mod has_tabbable_child;

use thirtyfour::prelude::*;

pub trait BaseActions {
    fn driver(&self) -> &WebDriver;
    fn base_url(&self) -> &str;

    async fn goto_path(&self, path: &str) -> anyhow::Result<()> {
        let url = format!("{}{path}", self.base_url());
        self.driver().goto(&url).await?;
        Ok(())
    }

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
