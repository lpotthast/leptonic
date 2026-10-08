use browser_test::thirtyfour::prelude::*;
use rootcause::Report;

use crate::pages::{Page, PageActions};

/// Actions on the focus manager hook fixture. Its ids all start with `test-fm-`; the helpers take
/// the rest (e.g. `focus-next`, `wrap-item-3`).
pub trait FocusManagerActions: PageActions {
    /// The element `#test-fm-{name}`: a focus manager control or an item.
    async fn item(&self, name: &str) -> Result<WebElement, Report> {
        self.element(format!("#test-fm-{name}")).await
    }

    /// Click `#test-fm-{name}`.
    async fn click(&self, name: &str) -> Result<(), Report> {
        self.item(name).await?.click().await?;
        Ok(())
    }

    /// Wait until `#test-fm-{name}` has focus.
    async fn expect_focus(&self, name: &str) -> Result<(), Report> {
        self.wait_for_focus(&self.item(name).await?).await
    }

    /// Negative check: `#test-fm-{name}` has focus and keeps it.
    async fn expect_focus_stays(&self, name: &str) -> Result<(), Report> {
        self.focus_stays(&self.item(name).await?).await
    }
}

impl FocusManagerActions for Page<'_> {}
