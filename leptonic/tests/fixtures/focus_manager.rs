use browser_test::thirtyfour::prelude::*;
use rootcause::Report;

use crate::pages::Page;

/// Actions on the focus manager hook fixture. Its ids all start with `test-fm-`; the helpers take
/// the rest (e.g. `focus-next`, `wrap-item-3`).
pub struct FocusManagerActions<'a, 'd> {
    page: &'a Page<'d>,
}

impl<'a, 'd> FocusManagerActions<'a, 'd> {
    pub fn new(page: &'a Page<'d>) -> Self {
        Self { page }
    }
    /// The element `#test-fm-{name}`: a focus manager control or an item.
    pub async fn item(&self, name: &str) -> Result<WebElement, Report> {
        self.page.element(format!("#test-fm-{name}")).await
    }
}
