// No upstream: checks that the atoms' own texts come from the localized strings.
use std::borrow::Cow;

use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{By, WebDriver},
};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

/// Texts the atoms and their hooks provide themselves follow the locale (de-DE): the search
/// field's clear button, the number field's steppers and role description, a tag's remove
/// button and the select's placeholder.
pub struct LocalizedAtomTests {}

#[async_trait]
impl BrowserTest<str> for LocalizedAtomTests {
    fn name(&self) -> Cow<'_, str> {
        "localized_atom_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/localized").await?;

        let clear = page.css(".leptonic-SearchField button").await?;
        page.wait_for_attr(&clear, "aria-label", Some("Suche zurücksetzen"))
            .await?;

        let number_field = page.css(".leptonic-NumberField").await?;
        let input = number_field.find(By::Css("input")).await?;
        page.wait_for_attr(&input, "aria-roledescription", Some("Nummernfeld"))
            .await?;
        let increment = number_field
            .find(By::Css(".leptonic-NumberFieldIncrementButton"))
            .await?;
        page.wait_for_attr(&increment, "aria-label", Some("Menge erhöhen"))
            .await?;
        let decrement = number_field
            .find(By::Css(".leptonic-NumberFieldDecrementButton"))
            .await?;
        page.wait_for_attr(&decrement, "aria-label", Some("Menge verringern"))
            .await?;

        let remove = page.css(".leptonic-TagRemoveButton").await?;
        page.wait_for_attr(&remove, "aria-label", Some("Entfernen"))
            .await?;

        let value = page.css(".leptonic-SelectValue").await?;
        wait_for!(
            "the select's placeholder",
            "Element wählen",
            value.text().await?
        );
        page.expect_no_page_errors().await
    }
}
