// No upstream: leptonic-only (the SSR test app's panic counter; react-aria renders no server).
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{BrowserTest, async_trait, thirtyfour::prelude::*};
use rootcause::Report;

use crate::pages::{ElementActions, Page, PageActions};

/// Runs last: fails if any page made the server panic while rendering, which the browser alone
/// would not notice (the page may still load, or hydrate over a broken server render).
pub struct ServerPanicTests {}

#[async_trait]
impl BrowserTest<str> for ServerPanicTests {
    fn name(&self) -> Cow<'_, str> {
        "server_did_not_panic".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        // A plain text page, not the app: `goto_path` would wait for hydration.
        driver
            .goto(&format!("{base_url}/__test/server-panics"))
            .await?;
        let panics = page.element("body").await?.inner_text().await?;
        assert_that!(panics)
            .with_detail_message("server panics while rendering pages")
            .is_equal_to("0");
        Ok(())
    }
}
