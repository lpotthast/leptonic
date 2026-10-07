// Upstream: react-aria/test/label/useLabel.test.js @ 99e6102368
// Upstream: react-aria/test/label/useField.test.js @ 99e6102368
// Upstream: react-aria-components/test/FieldError.test.js @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{BrowserTest, async_trait, thirtyfour::WebDriver};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

/// The parts of a field named after its label follow whether the label is rendered: the
/// select's trigger, the combo box's button, the number field's steppers and the slider's thumb.
pub struct LabelSlotsTests {}

const PARTS: [(&str, &str, &str); 4] = [
    ("select", "button", "Fruit"),
    ("combobox", "button", "Fruit"),
    ("number-field", "button", "Width"),
    ("slider", "input", "Volume"),
];

#[async_trait]
impl BrowserTest<str> for LabelSlotsTests {
    fn name(&self) -> Cow<'_, str> {
        "label_slots_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/label-slots").await?;

        for shown in [false, true, false] {
            if shown {
                page.element("test-label-slots-toggle")
                    .await?
                    .click()
                    .await?;
            }
            for (field, part, text) in PARTS {
                let label_id = label_id(driver, field, text).await?;
                assert_that!(label_id.is_some()).is_equal_to(shown);
                let labelled_by = labelled_by(driver, field, part).await?;
                match &label_id {
                    Some(label_id) => {
                        assert_that!(labelled_by.split(' ').any(|id| id == label_id))
                            .with_detail_message(format!("{field}: {labelled_by:?}"))
                            .is_true();
                    }
                    None => {
                        // Every reference resolves, and none to an element with the label's text.
                        let referenced = referenced_texts(driver, &labelled_by).await?;
                        assert_that!(referenced.iter().all(Option::is_some))
                            .with_detail_message(format!("{field}: {labelled_by:?}"))
                            .is_true();
                        assert_that!(
                            referenced
                                .iter()
                                .flatten()
                                .any(|referenced| referenced == text)
                        )
                        .with_detail_message(format!("{field}: {labelled_by:?}"))
                        .is_false();
                    }
                }
            }
            if shown {
                page.element("test-label-slots-toggle")
                    .await?
                    .click()
                    .await?;
            }
        }

        page.expect_no_page_errors().await
    }
}

/// The id of the field's label, while it is rendered (the innermost element with its text: a
/// group around only the label has the same text).
async fn label_id(driver: &WebDriver, field: &str, text: &str) -> Result<Option<String>, Report> {
    let id = driver
        .execute(
            &format!(
                "const label = [...document.querySelectorAll('#test-ls-{field} [id]')]
                     .filter(el => el.textContent === '{text}')
                     .pop();
                 return label ? label.id : null;"
            ),
            vec![],
        )
        .await?;
    Ok(id.json().as_str().map(str::to_owned))
}

/// The `aria-labelledby` of the field's first `part` element, once the page settled.
async fn labelled_by(driver: &WebDriver, field: &str, part: &str) -> Result<String, Report> {
    // Label presence is applied in effects.
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    let value = driver
        .execute(
            &format!(
                "return document.querySelector('#test-ls-{field} {part}')
                     .getAttribute('aria-labelledby') ?? '';"
            ),
            vec![],
        )
        .await?;
    Ok(value.json().as_str().unwrap_or_default().to_owned())
}

/// The text content of each element `ids` (space-separated) refers to; `None` for a missing one.
async fn referenced_texts(driver: &WebDriver, ids: &str) -> Result<Vec<Option<String>>, Report> {
    let texts = driver
        .execute(
            "return arguments[0].split(' ').filter(Boolean) \
                 .map(id => document.getElementById(id)?.textContent ?? null);",
            vec![serde_json::Value::from(ids)],
        )
        .await?;
    Ok(texts.convert()?)
}
