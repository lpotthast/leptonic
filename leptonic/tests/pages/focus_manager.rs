use browser_test::thirtyfour::prelude::*;
use rootcause::Report;

use crate::pages::PageActions;

/// The focus manager hook page. Its ids all start with `test-fm-`; the helpers take the rest
/// (e.g. `focus-next`, `wrap-item-3`).
pub struct FocusManagerPage<'d> {
    pub driver: &'d WebDriver,
    pub base_url: &'d str,
}

impl PageActions for FocusManagerPage<'_> {
    fn driver(&self) -> &WebDriver {
        self.driver
    }

    fn base_url(&self) -> &str {
        self.base_url
    }
}

impl FocusManagerPage<'_> {
    pub async fn goto(&self) -> Result<(), Report> {
        self.goto_path("/hooks/focus-manager").await
    }

    /// The element `#test-fm-{name}`: a focus manager control or an item.
    pub async fn item(&self, name: &str) -> Result<WebElement, Report> {
        self.element(format!("#test-fm-{name}")).await
    }

    /// Click `#test-fm-{name}`.
    pub async fn click(&self, name: &str) -> Result<(), Report> {
        self.item(name).await?.click().await?;
        Ok(())
    }

    /// Wait until `#test-fm-{name}` has focus.
    pub async fn expect_focus(&self, name: &str) -> Result<(), Report> {
        self.wait_for_focus(&self.item(name).await?).await
    }

    /// Negative check: `#test-fm-{name}` has focus and keeps it.
    pub async fn expect_focus_stays(&self, name: &str) -> Result<(), Report> {
        self.focus_stays(&self.item(name).await?).await
    }
}
