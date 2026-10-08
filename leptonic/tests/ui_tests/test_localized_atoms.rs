// No upstream: checks that the atoms' own texts come from the localized strings.
use std::borrow::Cow;

use browser_test::{BrowserTest, async_trait, thirtyfour::prelude::*};
use rootcause::Report;

use crate::pages::{ElementActions, Page, PageActions};

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
        cases!(
            search_field(&page),
            number_field(&page),
            tag(&page),
            select(&page),
        );
        Ok(())
    }
}

/// The SearchField's clear button is labelled in German.
async fn search_field(page: &Page<'_>) -> Result<(), Report> {
    page.element(".leptonic-SearchField button")
        .await?
        .wait_for_attr("aria-label", Some("Suche zurücksetzen"))
        .await?;
    Ok(())
}

/// The NumberField's role description and stepper buttons are labelled in German.
async fn number_field(page: &Page<'_>) -> Result<(), Report> {
    let number_field = page.element(".leptonic-NumberField").await?;
    number_field
        .element("input")
        .await?
        .wait_for_attr("aria-roledescription", Some("Nummernfeld"))
        .await?;
    number_field
        .element(".leptonic-NumberFieldIncrementButton")
        .await?
        .wait_for_attr("aria-label", Some("Menge erhöhen"))
        .await?;
    number_field
        .element(".leptonic-NumberFieldDecrementButton")
        .await?
        .wait_for_attr("aria-label", Some("Menge verringern"))
        .await?;
    Ok(())
}

/// The tag's remove button is labelled in German.
async fn tag(page: &Page<'_>) -> Result<(), Report> {
    page.element(".leptonic-TagRemoveButton")
        .await?
        .wait_for_attr("aria-label", Some("Entfernen"))
        .await?;
    Ok(())
}

/// The Select's placeholder is German.
async fn select(page: &Page<'_>) -> Result<(), Report> {
    page.element(".leptonic-SelectValue")
        .await?
        .wait_for_inner_text("Element wählen")
        .await?;
    Ok(())
}
