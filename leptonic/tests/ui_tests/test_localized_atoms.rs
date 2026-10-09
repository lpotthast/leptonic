// No upstream: checks that the atoms' own texts come from the localized strings.
//! Texts the atoms and their hooks provide themselves follow the locale (de-DE): the search
//! field's clear button, the number field's steppers and role description, a tag's remove
//! button and the select's placeholder.
use browser_test::browser_test;
use rootcause::Report;

use crate::pages::{ElementActions, Page};

const PATH: &str = "/atoms/localized";

/// The SearchField's clear button is labelled in German.
#[browser_test]
pub async fn search_field(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element(".leptonic-SearchField button")
        .await?
        .wait_for_attr("aria-label", Some("Suche zurücksetzen"))
        .await?;
    Ok(())
}

/// The NumberField's role description and stepper buttons are labelled in German.
#[browser_test]
pub async fn number_field(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
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
#[browser_test]
pub async fn tag(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element(".leptonic-TagRemoveButton")
        .await?
        .wait_for_attr("aria-label", Some("Entfernen"))
        .await?;
    Ok(())
}

/// The Select's placeholder is German.
#[browser_test]
pub async fn select(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element(".leptonic-SelectValue")
        .await?
        .wait_for_inner_text("Element wählen")
        .await?;
    Ok(())
}
