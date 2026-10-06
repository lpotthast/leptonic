use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{Key, WebDriver},
};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

/// The styled select components: a visible label names the trigger, the selection is bound to
/// app state both ways, `is_disabled` and `aria_label` reach the trigger, `name` the hidden form
/// element.
pub struct SelectComponentsTests {}

#[async_trait]
impl BrowserTest<str> for SelectComponentsTests {
    fn name(&self) -> Cow<'_, str> {
        "select_components_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/components/select").await?;

        // Labelled by its visible label.
        let trigger = page.css("#test-csel-single button").await?;
        let labelled_by = trigger.attr("aria-labelledby").await?.unwrap_or_default();
        let label_named = driver
            .execute(
                &format!(
                    "return '{labelled_by}'.split(' ').some(id => \
                     document.getElementById(id)?.textContent === 'Fruit');"
                ),
                vec![],
            )
            .await?;
        assert_that!(label_named.json().as_bool()).is_equal_to(Some(true));
        assert_that!(trigger.text().await?).contains("Banana");

        // Selecting writes the app state; the app state changes the selection.
        // (Opening it used to panic: whitespace in a class name.)
        trigger.click().await?;
        page.wait_for_selector("[role=option]").await?;
        page.by_role_and_text("option", "Apple")
            .await?
            .click()
            .await?;
        page.wait_for_text("test-csel-fruit", "Apple").await?;
        page.driver
            .execute(
                "document.getElementById('test-csel-set-cherry').click();",
                vec![],
            )
            .await?;
        page.wait_for_text("test-csel-fruit", "Cherry").await?;
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        assert_that!(page.css("#test-csel-single button").await?.text().await?).contains("Cherry");

        // Disabled, named by `aria_label`.
        let optional = page.css("#test-csel-optional button").await?;
        assert_that!(optional.attr("disabled").await?.is_some()).is_true();
        assert_that!(optional.attr("aria-label").await?)
            .is_equal_to(Some("Optional fruit".to_owned()));

        // The hidden form element carries `name`.
        page.css("#test-csel-form [name=fruits]").await?;

        // The clear button is a button of its own (not inside the trigger), pressed by keyboard.
        let clear = page
            .css("#test-csel-clearable button[aria-label='Clear selection']")
            .await?;
        clear.focus().await?;
        page.send_keys_to_active(Key::Enter).await?;
        page.wait_for_text("test-csel-clearable-value", "").await?;
        page.wait_for_no_selector("#test-csel-clearable button[aria-label='Clear selection']")
            .await?;

        // The chips' dismiss buttons are buttons named after their option, reached with Tab and
        // pressed by keyboard, without opening the popover.
        let remove_apple = page
            .css("#test-csel-form button[aria-label='Remove Apple']")
            .await?;
        assert_that!(remove_apple.tag_name().await?).is_equal_to("button".to_owned());
        assert_that!(page.count_matching("#test-csel-form button button").await?).is_equal_to(0);
        remove_apple.focus().await?;
        page.send_keys_to_active(Key::Enter).await?;
        page.wait_for_text("test-csel-many", "Cherry").await?;
        assert_that!(page.count_matching("[role=listbox]").await?).is_equal_to(0);
        page.css("#test-csel-form button[aria-label='Remove Cherry']")
            .await?
            .focus()
            .await?;
        page.send_keys_to_active(Key::Space).await?;
        page.wait_for_text("test-csel-many", "").await?;

        page.expect_no_page_errors().await
    }
}
