// Upstream: react-aria-components/test/Select.test.js @ 99e6102368
// Upstream: react-aria/test/select/HiddenSelect.test.tsx @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{Key, WebDriver, WebElement},
};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

const LISTBOX: &str = "[role=listbox]";

/// "should support multiple selection", "should support deselection if multiple selection is
/// enabled", "supports placeholder", and the open state bound to app state.
pub struct SelectMultipleTests {}

#[async_trait]
impl BrowserTest<str> for SelectMultipleTests {
    fn name(&self) -> Cow<'_, str> {
        "select_multiple_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/select-forms").await?;

        let trigger = trigger_in(&page, "#sf-multiple").await?;
        // "should support hover" on the trigger.
        page.driver
            .action_chain()
            .move_to_element_center(&trigger)
            .perform()
            .await?;
        page.wait_for_attr(&trigger, "data-hovered", Some("true"))
            .await?;
        page.driver
            .action_chain()
            .move_to_element_center(&page.css("h1").await?)
            .perform()
            .await?;
        page.wait_for_attr(&trigger, "data-hovered", None).await?;
        // The default placeholder.
        assert_that!(trigger.text().await?).is_equal_to("Select an item".to_owned());
        trigger.click().await?;
        page.wait_for_selector(LISTBOX).await?;
        assert_that!(
            page.css(LISTBOX)
                .await?
                .attr("aria-multiselectable")
                .await?
        )
        .is_equal_to(Some("true".to_owned()));
        assert_that!(page.count_matching("[role=listbox] [role=option]").await?).is_equal_to(3);
        page.by_role_and_text("option", "Cat")
            .await?
            .click()
            .await?;
        page.by_role_and_text("option", "Dog")
            .await?
            .click()
            .await?;
        page.wait_for_text("sf-multiple-changes", "[cat]|[cat,dog]")
            .await?;
        // Multiple selection keeps the popover open.
        assert_that!(page.count_matching(LISTBOX).await?).is_equal_to(1);
        wait_for!(
            "the trigger text",
            "Cat and Dog".to_owned(),
            trigger.text().await?
        );
        assert_that!(form_data(&page, "sf-multiple", "select").await?)
            .is_equal_to(vec!["cat".to_owned(), "dog".to_owned()]);
        // Deselecting.
        page.by_role_and_text("option", "Cat")
            .await?
            .click()
            .await?;
        page.wait_for_text("sf-multiple-changes", "[cat]|[cat,dog]|[dog]")
            .await?;
        page.wait_for_attr(
            &page.by_role_and_text("option", "Cat").await?,
            "aria-selected",
            Some("false"),
        )
        .await?;
        page.send_keys_to_active(Key::Escape).await?;
        page.wait_for_no_selector(LISTBOX).await?;

        // The open state bound to app state.
        page.click_element_with_id("sf-open-toggle").await?;
        page.wait_for_selector(LISTBOX).await?;
        page.wait_for_text("sf-open-state", "true").await?;
        let root = page.css("#sf-open .leptonic-Select").await?;
        assert_that!(root.attr("data-open").await?).is_equal_to(Some("true".to_owned()));
        page.send_keys_to_active(Key::Escape).await?;
        page.wait_for_no_selector(LISTBOX).await?;
        page.wait_for_text("sf-open-state", "false").await?;
        page.expect_no_page_errors().await
    }
}

/// "supports validation errors", "should not submit if required and selectedKey is null", "should
/// send disabled prop to the hidden field", the root's data attributes, and autofill (a `change`
/// of the hidden `<select>`).
pub struct SelectValidationTests {}

#[async_trait]
impl BrowserTest<str> for SelectValidationTests {
    fn name(&self) -> Cow<'_, str> {
        "select_validation_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/select-forms").await?;

        // Native validation: the hidden select is required, the error shows once checked.
        let root = page.css("#sf-required .leptonic-Select").await?;
        let trigger = trigger_in(&page, "#sf-required").await?;
        let select = page.css("#sf-required select").await?;
        assert_that!(select.attr("required").await?).is_some();
        assert_that!(trigger.attr("aria-describedby").await?).is_none();
        assert_that!(is_valid(&page, &select).await?).is_false();
        assert_that!(root.attr("data-invalid").await?).is_none();
        assert_that!(root.attr("data-required").await?).is_equal_to(Some("true".to_owned()));
        // The hidden select is labelled for autofill.
        assert_that!(
            page.count_matching("#sf-required [aria-hidden=true] label select")
                .await?
        )
        .is_equal_to(1);

        check_validity(&page, "sf-required").await?;
        page.wait_for_attr(&root, "data-invalid", Some("true"))
            .await?;
        page.wait_for_focus_on(&trigger, "the required select's trigger")
            .await?;
        assert_that!(described_by_text(&page, &trigger).await?).is_not_empty();
        // Focus within the select: `data-focused`, `data-focus-visible` (focused by the script,
        // after keyboard-free interaction: the modality decides).
        page.wait_for_attr(&root, "data-focused", Some("true"))
            .await?;
        trigger.click().await?;
        page.wait_for_selector(LISTBOX).await?;
        page.by_role_and_text("option", "Cat")
            .await?
            .click()
            .await?;
        page.wait_for_no_selector(LISTBOX).await?;
        page.wait_for_attr(&trigger, "aria-describedby", None)
            .await?;
        page.wait_for_attr(&root, "data-invalid", None).await?;

        // A required select without a value blocks submission.
        let trigger = trigger_in(&page, "#sf-submit").await?;
        assert_that!(trigger.text().await?).is_equal_to("Select an item".to_owned());
        trigger.click().await?;
        page.wait_for_selector(LISTBOX).await?;
        page.by_role_and_text("option", "Cat")
            .await?
            .click()
            .await?;
        wait_for!("the trigger text", "Cat".to_owned(), trigger.text().await?);
        page.click_element_with_id("sf-submit-button").await?;
        page.wait_for_text("sf-submits", "1").await?;
        page.click_element_with_id("sf-submit-clear").await?;
        wait_for!(
            "the trigger text",
            "Select an item".to_owned(),
            trigger.text().await?
        );
        page.click_element_with_id("sf-submit-button").await?;
        stays!(
            "the submissions",
            "1".to_owned(),
            page.read_text_of("sf-submits").await?
        );
        assert_that!(page.css("#sf-submit select").await?.prop("value").await?)
            .is_equal_to(Some(String::new()));

        // Disabled: the hidden select too, and the trigger doesn't open.
        let select = page.css("#sf-disabled select").await?;
        assert_that!(select.prop("disabled").await?).is_equal_to(Some("true".to_owned()));
        let trigger = trigger_in(&page, "#sf-disabled").await?;
        assert_that!(trigger.prop("disabled").await?).is_equal_to(Some("true".to_owned()));
        page.driver
            .execute("arguments[0].click();", vec![trigger.to_json()?])
            .await?;
        stays!("the open listboxes", 0, page.count_matching(LISTBOX).await?);

        // Autofill picks an option of the hidden select.
        let trigger = trigger_in(&page, "#sf-required").await?;
        page.driver
            .execute(
                "let select = document.querySelector('#sf-required select'); \
                 select.value = 'kangaroo'; \
                 select.dispatchEvent(new Event('change', {bubbles: true}));",
                vec![],
            )
            .await?;
        wait_for!(
            "the trigger text",
            "Kangaroo".to_owned(),
            trigger.text().await?
        );
        page.expect_no_page_errors().await
    }
}

/// "shouldn't allow the user to open the select if there are no items", "should support empty
/// state", "should support multiple selection form integration with many items" (hidden inputs
/// instead of a `<select>`, with validation and form reset).
pub struct SelectEmptyAndManyTests {}

#[async_trait]
impl BrowserTest<str> for SelectEmptyAndManyTests {
    fn name(&self) -> Cow<'_, str> {
        "select_empty_and_many_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/select-forms").await?;

        trigger_in(&page, "#sf-empty").await?.click().await?;
        stays!("the open listboxes", 0, page.count_matching(LISTBOX).await?);

        trigger_in(&page, "#sf-empty-allowed")
            .await?
            .click()
            .await?;
        page.wait_for_selector(LISTBOX).await?;
        let listbox = page.css(LISTBOX).await?;
        assert_that!(listbox.attr("data-empty").await?).is_equal_to(Some("true".to_owned()));
        assert_that!(
            page.css("[role=listbox] [role=option]")
                .await?
                .text()
                .await?
        )
        .is_equal_to("No results".to_owned());
        page.send_keys_to_active(Key::Escape).await?;
        page.wait_for_no_selector(LISTBOX).await?;

        // More than 300 options: hidden inputs; the first one is required (native validation).
        assert_that!(page.count_matching("#sf-many select").await?).is_equal_to(0);
        page.click_element_with_id("sf-many-submit").await?;
        let error = page.css("#sf-many .leptonic-FieldError").await?;
        assert_that!(error.text().await?).is_not_empty();
        assert_that!(page.read_text_of("sf-many-submits").await?).is_equal_to("0".to_owned());
        let trigger = trigger_in(&page, "#sf-many").await?;
        // Open with the keyboard, which focuses the first option (a pointer resting over the
        // popover would focus the option under it: `should_focus_on_hover`).
        page.driver
            .action_chain()
            .move_to_element_center(&page.css("h1").await?)
            .perform()
            .await?;
        page.driver
            .execute("arguments[0].focus();", vec![trigger.to_json()?])
            .await?;
        page.send_keys_to_active(Key::Down).await?;
        page.wait_for_selector(LISTBOX).await?;
        page.wait_for_active_text("item0").await?;
        page.send_keys_to_active(" ").await?;
        page.send_keys_to_active(Key::Down).await?;
        page.wait_for_active_text("item1").await?;
        page.send_keys_to_active(" ").await?;
        page.send_keys_to_active(Key::Escape).await?;
        page.wait_for_no_selector(LISTBOX).await?;
        wait_for!(
            "the trigger text",
            "item0 and item1".to_owned(),
            trigger.text().await?
        );
        assert_that!(form_data(&page, "sf-many", "many").await?)
            .is_equal_to(vec!["0".to_owned(), "1".to_owned()]);
        page.click_element_with_id("sf-many-submit").await?;
        page.wait_for_text("sf-many-submits", "1").await?;
        page.wait_for_no_selector("#sf-many .leptonic-FieldError")
            .await?;
        // Form reset works on the hidden inputs too.
        page.click_element_with_id("sf-many-reset").await?;
        wait_for!(
            "the submitted values",
            vec![String::new()],
            form_data(&page, "sf-many", "many").await?
        );
        wait_for!(
            "the trigger text",
            "Select an item".to_owned(),
            trigger.text().await?
        );
        page.expect_no_page_errors().await
    }
}

/// The select trigger inside `container`.
async fn trigger_in(page: &Page<'_>, container: &str) -> Result<WebElement, Report> {
    page.css(&format!("{container} [aria-haspopup=listbox]"))
        .await
}

/// The text of the elements describing `element`.
async fn described_by_text(page: &Page<'_>, element: &WebElement) -> Result<String, Report> {
    let ids = element.attr("aria-describedby").await?.unwrap_or_default();
    let mut texts = Vec::new();
    for id in ids.split_whitespace() {
        texts.push(page.element(id).await?.text().await?);
    }
    Ok(texts.join(" "))
}

async fn is_valid(page: &Page<'_>, element: &WebElement) -> Result<bool, Report> {
    Ok(page
        .driver
        .execute(
            "return arguments[0].validity.valid;",
            vec![element.to_json()?],
        )
        .await?
        .convert::<bool>()?)
}

async fn check_validity(page: &Page<'_>, form_id: &str) -> Result<(), Report> {
    page.driver
        .execute(
            "document.getElementById(arguments[0]).checkValidity();",
            vec![serde_json::Value::from(form_id)],
        )
        .await?;
    Ok(())
}

/// The values the form `form_id` submits under `name`.
async fn form_data(page: &Page<'_>, form_id: &str, name: &str) -> Result<Vec<String>, Report> {
    Ok(page
        .driver
        .execute(
            "return new FormData(document.getElementById(arguments[0])).getAll(arguments[1]);",
            vec![
                serde_json::Value::from(form_id),
                serde_json::Value::from(name),
            ],
        )
        .await?
        .convert::<Vec<String>>()?)
}
