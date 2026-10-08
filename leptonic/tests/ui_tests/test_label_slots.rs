// Upstream: react-aria/test/label/useLabel.test.js @ 99e6102368
// Upstream: react-aria/test/label/useField.test.js @ 99e6102368
// Upstream: react-aria-components/test/FieldError.test.js @ 99e6102368
//! The parts of a field named after its label follow whether the label is rendered: the
//! select's trigger, the combo box's button, the number field's steppers and the slider's thumb.
use browser_test::thirtyfour::prelude::*;
use rootcause::Report;

use crate::{
    pages::{Page, PageActions},
    polling::wait_for,
};

const PATH: &str = "/atoms/label-slots";

/// The fields: their id suffix, the part named after the label, the label's text.
const PARTS: [(&str, &str, &str); 4] = [
    ("select", "button", "Fruit"),
    ("combobox", "button", "Fruit"),
    ("number-field", "button", "Width"),
    ("slider", "input", "Volume"),
];

/// Without labels, no part is named after one.
pub async fn no_labels(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    expect_parts(page, false).await
}

/// Labels rendered later name the parts.
pub async fn labels_added(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    toggle_labels(page).await?;
    expect_parts(page, true).await
}

/// Labels removed after they were rendered no longer name the parts.
pub async fn labels_removed(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    toggle_labels(page).await?;
    expect_parts(page, true).await?;
    toggle_labels(page).await?;
    expect_parts(page, false).await
}

/// Render the labels if they aren't, remove them if they are.
async fn toggle_labels(page: &Page<'_>) -> Result<(), Report> {
    page.element("#test-label-slots-toggle")
        .await?
        .click()
        .await?;
    Ok(())
}

/// Every field's label is rendered if `shown`, and labels the field's part then. Otherwise every
/// reference of the part's `aria-labelledby` resolves, and none to an element with the label's
/// text. Label presence is applied in effects.
async fn expect_parts(page: &Page<'_>, shown: bool) -> Result<(), Report> {
    for (field, part, text) in PARTS {
        wait_for(format!("the id of {field}'s label"))
            .observing(|| label_id(page, field, text))
            .to_be(if shown { "rendered" } else { "gone" }, |id| {
                id.is_some() == shown
            })
            .await?;
        let part = page.element(format!("#test-ls-{field} {part}")).await?;
        if let Some(label_id) = label_id(page, field, text).await? {
            wait_for(format!("the aria-labelledby of {field}"))
                .observing(|| labelled_by(&part))
                .to_be(&format!("including the label {label_id:?}"), |ids| {
                    ids.split(' ').any(|id| id == label_id)
                })
                .await?;
        } else {
            wait_for(format!("the texts {field}'s aria-labelledby refers to"))
                .observing(|| async {
                    referenced_text_contents(page, &labelled_by(&part).await?).await
                })
                .to_be(
                    "existing elements, none with the label's text",
                    |referenced| {
                        referenced.iter().all(Option::is_some)
                            && !referenced
                                .iter()
                                .flatten()
                                .any(|referenced| referenced == text)
                    },
                )
                .await?;
        }
    }
    Ok(())
}

/// The id of the field's label, while it is rendered (the innermost element with its text: a
/// group around only the label has the same text).
async fn label_id(page: &Page<'_>, field: &str, text: &str) -> Result<Option<String>, Report> {
    let mut label = None;
    for element in page.elements(format!("#test-ls-{field} [id]")).await? {
        if element.prop("textContent").await?.as_deref() == Some(text) {
            label = element.id().await?;
        }
    }
    Ok(label)
}

/// The `aria-labelledby` of `part`, empty if it has none.
async fn labelled_by(part: &WebElement) -> Result<String, Report> {
    Ok(part.attr("aria-labelledby").await?.unwrap_or_default())
}

/// The text content of each element `ids` (space-separated) refers to; `None` for a missing one
/// (unlike `ElementActions::referenced_text`, which fails on it).
async fn referenced_text_contents(
    page: &Page<'_>,
    ids: &str,
) -> Result<Vec<Option<String>>, Report> {
    let mut texts = Vec::new();
    for id in ids.split(' ').filter(|id| !id.is_empty()) {
        let selector = format!("#{id}");
        texts.push(if page.count(&selector).await? == 0 {
            None
        } else {
            page.element(&selector).await?.prop("textContent").await?
        });
    }
    Ok(texts)
}
