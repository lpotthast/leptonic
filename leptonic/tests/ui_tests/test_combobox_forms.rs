// Upstream: react-aria-components/test/ComboBox.test.js @ 99e6102368
// Upstream: react-stately/test/combobox/useComboBoxState.test.js @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{By, Key, WebDriver, WebElement},
};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

const LISTBOX: &str = "[role=listbox]";

/// The combo boxes of `/atoms/combobox-forms` with custom values: committing typed text on blur,
/// Enter and Escape keeps it and clears the selection (react-stately's `commitCustomValue`), and
/// the text is submitted with the form.
pub struct ComboBoxCustomValueTests {}

#[async_trait]
impl BrowserTest<str> for ComboBoxCustomValueTests {
    fn name(&self) -> Cow<'_, str> {
        "combobox_custom_value_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/combobox-forms").await?;
        let input = input_in(&page, "#cbf-custom").await?;

        // Select Kangaroo.
        input.click().await?;
        input.send_keys("Kan").await?;
        expect_options(&page, &["Kangaroo"]).await?;
        input.send_keys(Key::Down).await?;
        input.send_keys(Key::Enter).await?;
        page.wait_for_text("cbf-custom-changes", "[3]").await?;
        wait_for!(
            "the input value",
            "Kangaroo".to_owned(),
            input_value(&input).await?
        );

        // Typed text matching no option is kept when focus leaves, and clears the selection.
        input.send_keys(Key::Control + "a").await?;
        input.send_keys("Wombat").await?;
        page.wait_for_no_selector(LISTBOX).await?;
        page.press_tab().await?;
        page.wait_for_text("cbf-custom-changes", "[3]|[]").await?;
        stays!(
            "the input text",
            "Wombat".to_owned(),
            input_value(&input).await?
        );
        // The form submits the text (`allows_custom_value` submits the text, not the key).
        assert_that!(form_data(&page, "cbf-custom", "animal").await?)
            .is_equal_to(vec!["Wombat".to_owned()]);

        // Escape without a selection keeps the custom text (react-stately `revert`).
        input.click().await?;
        input.send_keys("x").await?;
        input.send_keys(Key::Escape).await?;
        stays!(
            "the input text",
            "Wombatx".to_owned(),
            input_value(&input).await?
        );

        // Enter commits custom text too; the (empty) value doesn't change again.
        input.send_keys(Key::Control + "a").await?;
        input.send_keys("Emu").await?;
        input.send_keys(Key::Enter).await?;
        stays!(
            "the input text",
            "Emu".to_owned(),
            input_value(&input).await?
        );
        assert_that!(page.read_text_of("cbf-custom-changes").await?)
            .is_equal_to("[3]|[]".to_owned());
        page.expect_no_page_errors().await
    }
}

/// "should support validation errors" (native validation reaching the combo box's input), and
/// ARIA validation with `validate`.
pub struct ComboBoxValidationTests {}

#[async_trait]
impl BrowserTest<str> for ComboBoxValidationTests {
    fn name(&self) -> Cow<'_, str> {
        "combobox_validation_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/combobox-forms").await?;

        // Native: `required` on the input, the error once the form is checked.
        let input = input_in(&page, "#cbf-required").await?;
        let root = page.css("#cbf-required .leptonic-ComboBox").await?;
        assert_that!(input.attr("required").await?).is_some();
        assert_that!(input.attr("aria-required").await?).is_none();
        assert_that!(input.attr("aria-describedby").await?).is_none();
        assert_that!(is_valid(&page, &input).await?).is_false();
        assert_that!(root.attr("data-invalid").await?).is_none();
        assert_that!(root.attr("data-required").await?).is_equal_to(Some("true".to_owned()));

        check_validity(&page, "cbf-required").await?;
        page.wait_for_focus_on(&input, "the required combo box's input")
            .await?;
        page.wait_for_attr(&root, "data-invalid", Some("true"))
            .await?;
        assert_that!(described_by_text(&page, &input).await?).is_not_empty();

        input.send_keys("C").await?;
        page.by_role_and_text("option", "Cat")
            .await?
            .click()
            .await?;
        wait_for!(
            "the input value",
            "Cat".to_owned(),
            input_value(&input).await?
        );
        assert_that!(is_valid(&page, &input).await?).is_true();
        // The error stays until the value is committed (focus leaves).
        assert_that!(input.attr("aria-describedby").await?).is_some();
        page.press_tab().await?;
        page.wait_for_attr(&input, "aria-describedby", None).await?;
        page.wait_for_attr(&root, "data-invalid", None).await?;

        // ARIA: `validate` runs on the value, its message shows right away.
        let input = input_in(&page, "#cbf-validate").await?;
        input.click().await?;
        input.send_keys("Do").await?;
        page.by_role_and_text("option", "Dog")
            .await?
            .click()
            .await?;
        wait_for!(
            "the input value",
            "Dog".to_owned(),
            input_value(&input).await?
        );
        page.wait_for_attr(&input, "aria-invalid", Some("true"))
            .await?;
        wait_for!(
            "the error",
            "Dogs are not allowed".to_owned(),
            described_by_text(&page, &input).await?
        );
        input.send_keys(Key::Control + "a").await?;
        input.send_keys("Ca").await?;
        page.by_role_and_text("option", "Cat")
            .await?
            .click()
            .await?;
        page.wait_for_attr(&input, "aria-invalid", None).await?;
        page.expect_no_page_errors().await
    }
}

/// "should support multiple selection", "should support deselection if multiple selection is
/// enabled", "should support isRequired with multiple selection", "should support formValue".
pub struct ComboBoxMultipleTests {}

#[async_trait]
impl BrowserTest<str> for ComboBoxMultipleTests {
    fn name(&self) -> Cow<'_, str> {
        "combobox_multiple_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/combobox-forms").await?;

        // Multiple selection: the popover stays open, the input stays empty.
        let input = input_in(&page, "#cbf-multiple").await?;
        open_with_button(&page, "#cbf-multiple").await?;
        let listbox = page.css(LISTBOX).await?;
        assert_that!(listbox.attr("aria-multiselectable").await?)
            .is_equal_to(Some("true".to_owned()));
        expect_options(&page, &["Cat", "Dog", "Kangaroo"]).await?;
        page.by_role_and_text("option", "Cat")
            .await?
            .click()
            .await?;
        page.wait_for_attr(
            &page.by_role_and_text("option", "Cat").await?,
            "aria-selected",
            Some("true"),
        )
        .await?;
        page.by_role_and_text("option", "Dog")
            .await?
            .click()
            .await?;
        page.wait_for_text("cbf-multiple-changes", "[1]|[1,2]")
            .await?;
        assert_that!(page.count_matching(LISTBOX).await?).is_equal_to(1);
        assert_that!(input_value(&input).await?).is_equal_to(String::new());
        assert_that!(form_data(&page, "cbf-multiple", "animals").await?)
            .is_equal_to(vec!["1".to_owned(), "2".to_owned()]);
        // Deselecting.
        page.by_role_and_text("option", "Cat")
            .await?
            .click()
            .await?;
        page.wait_for_text("cbf-multiple-changes", "[1]|[1,2]|[2]")
            .await?;
        input.send_keys(Key::Escape).await?;
        page.wait_for_no_selector(LISTBOX).await?;
        // Form reset restores the (empty) default.
        page.click_element_with_id("cbf-multiple-reset").await?;
        wait_for!(
            "the submitted animals",
            vec![String::new()],
            form_data(&page, "cbf-multiple", "animals").await?
        );

        // Required with multiple selection: required only while nothing is selected.
        let input = input_in(&page, "#cbf-multiple-required").await?;
        let root = page
            .css("#cbf-multiple-required .leptonic-ComboBox")
            .await?;
        assert_that!(input.attr("required").await?).is_some();
        assert_that!(is_valid(&page, &input).await?).is_false();
        check_validity(&page, "cbf-multiple-required").await?;
        page.wait_for_attr(&root, "data-invalid", Some("true"))
            .await?;
        open_with_button(&page, "#cbf-multiple-required").await?;
        page.by_role_and_text("option", "Cat")
            .await?
            .click()
            .await?;
        blur(&page).await?;
        page.wait_for_attr(&input, "required", None).await?;
        assert_that!(is_valid(&page, &input).await?).is_true();
        page.wait_for_attr(&root, "data-invalid", None).await?;
        assert_that!(hidden_values(&page, "#cbf-multiple-required").await?)
            .is_equal_to(vec![("required-animals".to_owned(), "1".to_owned())]);
        open_with_button(&page, "#cbf-multiple-required").await?;
        page.by_role_and_text("option", "Cat")
            .await?
            .click()
            .await?;
        blur(&page).await?;
        wait_for!(
            "the input to be required again",
            true,
            input.attr("required").await?.is_some()
        );
        assert_that!(is_valid(&page, &input).await?).is_false();
        assert_that!(hidden_values(&page, "#cbf-multiple-required").await?)
            .is_equal_to(vec![("required-animals".to_owned(), String::new())]);

        // `form_value`: the key in a hidden input (the input has no name), or the text.
        let input = input_in(&page, "#cbf-key").await?;
        assert_that!(input.attr("name").await?).is_none();
        assert_that!(input_value(&input).await?).is_equal_to("Dog".to_owned());
        assert_that!(hidden_values(&page, "#cbf-key").await?)
            .is_equal_to(vec![("key-animal".to_owned(), "2".to_owned())]);
        let input = input_in(&page, "#cbf-text").await?;
        assert_that!(input.attr("name").await?).is_equal_to(Some("text-animal".to_owned()));
        assert_that!(hidden_values(&page, "#cbf-text").await?).is_empty();
        assert_that!(form_data(&page, "cbf-text", "text-animal").await?)
            .is_equal_to(vec!["Dog".to_owned()]);
        page.expect_no_page_errors().await
    }
}

/// `menuTrigger` focus and manual (react-stately's `useComboBoxState`: `open` with a trigger,
/// showing all items), and `on_open_change` with what opened the popover.
pub struct ComboBoxMenuTriggerTests {}

#[async_trait]
impl BrowserTest<str> for ComboBoxMenuTriggerTests {
    fn name(&self) -> Cow<'_, str> {
        "combobox_menu_trigger_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/combobox-forms").await?;

        // Focus opens the popover with all options, though the input holds "Do".
        let input = input_in(&page, "#cbf-focus").await?;
        input.click().await?;
        page.wait_for_selector(LISTBOX).await?;
        expect_options(&page, &["Cat", "Dog", "Kangaroo"]).await?;
        page.wait_for_text("cbf-focus-open", "true:Some(Focus)")
            .await?;
        // Typing filters again.
        input.send_keys("g").await?;
        expect_options(&page, &["Dog"]).await?;
        input.send_keys(Key::Escape).await?;
        page.wait_for_no_selector(LISTBOX).await?;
        page.wait_for_text("cbf-focus-open", "true:Some(Focus)|false:None")
            .await?;

        // Manual: typing doesn't open, ArrowDown does (with all options).
        let input = input_in(&page, "#cbf-manual").await?;
        input.click().await?;
        input.send_keys("a").await?;
        stays!("the open listboxes", 0, page.count_matching(LISTBOX).await?);
        input.send_keys(Key::Down).await?;
        page.wait_for_selector(LISTBOX).await?;
        expect_options(&page, &["Cat", "Dog", "Kangaroo"]).await?;
        page.wait_for_text("cbf-manual-open", "true:Some(Manual)")
            .await?;
        input.send_keys(Key::Escape).await?;
        page.wait_for_no_selector(LISTBOX).await?;
        page.expect_no_page_errors().await
    }
}

/// "should support filtering sections", disabled keys, Enter without a focused option, and the
/// option count announcement.
pub struct ComboBoxSectionsTests {}

#[async_trait]
impl BrowserTest<str> for ComboBoxSectionsTests {
    fn name(&self) -> Cow<'_, str> {
        "combobox_sections_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/combobox-forms").await?;

        let input = input_in(&page, "#cbf-sections").await?;
        input.click().await?;
        input.send_keys("o").await?;
        expect_options(&page, &["Dog", "Owl", "Parrot"]).await?;
        // One group per section left, named by its heading; sections without matches are gone.
        let groups = page
            .driver
            .find_all(By::Css("[role=listbox] [role=group]"))
            .await?;
        let mut names = Vec::new();
        for group in &groups {
            let id = group.attr("aria-labelledby").await?.unwrap_or_default();
            names.push(page.element(&id).await?.text().await?);
        }
        assert_that!(names).is_equal_to(vec!["Animals".to_owned(), "Birds".to_owned()]);
        input.send_keys("w").await?;
        expect_options(&page, &["Owl"]).await?;
        assert_that!(page.count_matching("[role=listbox] [role=group]").await?).is_equal_to(1);
        // The disabled option is skipped.
        input.send_keys(Key::Backspace).await?;
        expect_options(&page, &["Dog", "Owl", "Parrot"]).await?;
        assert_that!(
            page.by_role_and_text("option", "Dog")
                .await?
                .attr("aria-disabled")
                .await?
        )
        .is_equal_to(Some("true".to_owned()));
        input.send_keys(Key::Down).await?;
        wait_for!(
            "the active descendant",
            "Owl".to_owned(),
            active_descendant_text(&page, &input).await?
        );
        input.send_keys(Key::Enter).await?;
        wait_for!(
            "the input value",
            "Owl".to_owned(),
            input_value(&input).await?
        );
        page.wait_for_no_selector(LISTBOX).await?;

        // Enter with the popover closed submits the form.
        let input = input_in(&page, "#cbf-submit").await?;
        input.click().await?;
        input.send_keys(Key::Enter).await?;
        page.wait_for_text("cbf-submits", "1").await?;
        // Enter with the popover open but no focused option only commits (reverting the text).
        input.send_keys("Ca").await?;
        page.wait_for_selector(LISTBOX).await?;
        // The number of options is announced when the popover opens without a focused option.
        page.wait_for_selector_text(
            "[data-live-announcer] [aria-live=assertive]",
            "1 option available.",
        )
        .await?;
        input.send_keys(Key::Enter).await?;
        page.wait_for_no_selector(LISTBOX).await?;
        wait_for!("the input value", String::new(), input_value(&input).await?);
        stays!(
            "the submissions",
            "1".to_owned(),
            page.read_text_of("cbf-submits").await?
        );
        page.expect_no_page_errors().await
    }
}

/// The combo box input inside `container`.
async fn input_in(page: &Page<'_>, container: &str) -> Result<WebElement, Report> {
    page.css(&format!("{container} [role=combobox]")).await
}

async fn input_value(input: &WebElement) -> Result<String, Report> {
    Ok(input.prop("value").await?.unwrap_or_default())
}

async fn option_texts(page: &Page<'_>) -> Result<Vec<String>, Report> {
    let mut texts = Vec::new();
    for option in page
        .driver
        .find_all(By::Css("[role=listbox] [role=option]"))
        .await?
    {
        texts.push(option.text().await?);
    }
    Ok(texts)
}

/// Waits until the open listbox shows exactly `expected`.
async fn expect_options(page: &Page<'_>, expected: &[&str]) -> Result<(), Report> {
    let expected: Vec<String> = expected.iter().map(|&text| text.to_owned()).collect();
    wait_for!("the options", expected, option_texts(page).await?);
    Ok(())
}

/// Opens the popover of the combo box in `container` with its button.
async fn open_with_button(page: &Page<'_>, container: &str) -> Result<(), Report> {
    page.css(&format!("{container} button[aria-haspopup]"))
        .await?
        .click()
        .await?;
    page.wait_for_selector(LISTBOX).await
}

/// The text of the input's active descendant.
async fn active_descendant_text(page: &Page<'_>, input: &WebElement) -> Result<String, Report> {
    match input.attr("aria-activedescendant").await? {
        Some(id) if !id.is_empty() => Ok(page.element(&id).await?.text().await?),
        _ => Ok(String::new()),
    }
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

/// Takes focus away from the active element.
async fn blur(page: &Page<'_>) -> Result<(), Report> {
    page.driver
        .execute("document.activeElement.blur();", vec![])
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

/// The names and values of the hidden inputs inside `container`.
async fn hidden_values(page: &Page<'_>, container: &str) -> Result<Vec<(String, String)>, Report> {
    let mut values = Vec::new();
    for input in page
        .driver
        .find_all(By::Css(format!("{container} input[type=hidden]")))
        .await?
    {
        values.push((
            input.attr("name").await?.unwrap_or_default(),
            input.prop("value").await?.unwrap_or_default(),
        ));
    }
    Ok(values)
}
