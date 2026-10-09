// No upstream: leptonic-only (the SSR test app's panic counter; react-aria renders no server).
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{BrowserTest, async_trait, thirtyfour::prelude::*};
use rootcause::Report;

use crate::pages::Page;

/// Runs last, also in filtered runs: fails if any page made the server panic while rendering, which
/// the browser alone would not notice (the page may still load, or hydrate over a broken server
/// render). The failure lists every panic: the request it happened in, where, and its message.
pub struct ServerPanicTests {}

#[async_trait]
impl<'page> BrowserTest<Page<'page>> for ServerPanicTests {
    fn name(&self) -> Cow<'_, str> {
        "server_did_not_panic".into()
    }

    async fn run(&self, driver: &WebDriver, page: &Page<'page>) -> Result<(), Report> {
        let base_url = page.base_url();
        // A plain text page, not the app: `goto_path` would wait for hydration. One panic per line.
        driver
            .goto(&format!("{base_url}/__test/server-panics"))
            .await?;
        assert_that!(driver.find(By::Css("body")).await?)
            .with_detail_message("server panics (request: location: message)")
            .inner_text()
            .await
            .derive_owned(|report| {
                report
                    .lines()
                    .filter(|line| !line.is_empty())
                    .collect::<Vec<_>>()
            })
            .is_empty();
        Ok(())
    }
}
