// No upstream: leptonic-only (the SSR test app's panic counter; react-aria renders no server).
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{By, WebDriver},
};
use rootcause::Report;

/// Runs last: fails if any page made the server panic while rendering, which the browser alone
/// would not notice (the page may still load, or hydrate over a broken server render).
pub struct ServerPanicTests {}

#[async_trait]
impl BrowserTest<str> for ServerPanicTests {
    fn name(&self) -> Cow<'_, str> {
        "server_did_not_panic".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        driver
            .goto(&format!("{base_url}/__test/server-panics"))
            .await?;
        let panics = driver.find(By::Tag("body")).await?.text().await?;
        assert_that!(panics.trim().to_owned()).is_equal_to("0".to_owned());
        Ok(())
    }
}
