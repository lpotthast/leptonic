// Upstream: react-aria-components/test/Select.test.js @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{By, Key, WebDriver},
};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

/// The select trigger: the element that opens a listbox.
const TRIGGER: &str = "[aria-haspopup=listbox]";

/// Behavior of the `Select` atoms, asserted on the DOM/ARIA level. Elements are found by role and
/// text, as users perceive them.
pub struct SelectTests {}

#[async_trait]
impl BrowserTest<str> for SelectTests {
    fn name(&self) -> Cow<'_, str> {
        "select_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/select").await?;

        initial_state(&page).await?;
        opening_focuses_the_selected_option(&page).await?;
        escape_closes_and_restores_focus(&page).await?;
        selecting_an_option(&page).await?;
        trigger_keyboard(&page).await?;
        labelling(&page).await?;
        form_reset(&page).await?;

        Ok(())
    }
}

async fn trigger_attr(page: &Page<'_>, name: &str) -> Result<Option<String>, Report> {
    Ok(page.css(TRIGGER).await?.attr(name).await?)
}

async fn hidden_select_value(page: &Page<'_>) -> Result<String, Report> {
    let select = page.css("#test-sel-form select").await?;
    Ok(select.prop("value").await?.unwrap_or_default())
}

async fn initial_state(page: &Page<'_>) -> Result<(), Report> {
    assert_that!(page.css(TRIGGER).await?.text().await?).is_equal_to("Banana".to_owned());
    assert_that!(hidden_select_value(page).await?).is_equal_to("Banana".to_owned());
    assert_that!(trigger_attr(page, "aria-expanded").await?).is_equal_to(Some("false".to_owned()));
    // `aria-controls` may only reference the listbox while it exists (i.e. while open).
    assert_that!(trigger_attr(page, "aria-controls").await?).is_none();
    Ok(())
}

async fn opening_focuses_the_selected_option(page: &Page<'_>) -> Result<(), Report> {
    page.css(TRIGGER).await?.click().await?;
    page.wait_for_selector("[role=listbox]").await?;
    assert_that!(trigger_attr(page, "aria-expanded").await?).is_equal_to(Some("true".to_owned()));
    let listbox_id = page.css("[role=listbox]").await?.id().await?;
    assert_that!(trigger_attr(page, "aria-controls").await?).is_equal_to(listbox_id);
    page.wait_for_selector("[role=option][aria-selected=true]:focus")
        .await?;
    page.wait_for_active_text("Banana").await?;
    // The popover is modal (react-aria-components): the rest of the page is inert, and screen
    // reader users get dismiss buttons around the options.
    page.wait_for_selector("#test-sel-after:is([inert], [inert] *)")
        .await?;
    assert_that!(page.count_matching("button[aria-label=Dismiss]").await?).is_equal_to(2);
    // The popover is a dialog named like its listbox (react-aria-components).
    let listbox_labelledby = page
        .css("[role=listbox]")
        .await?
        .attr("aria-labelledby")
        .await?;
    let dialog = page.css("[role=dialog]:has([role=listbox])").await?;
    assert_that!(listbox_labelledby.is_some()).is_true();
    assert_that!(dialog.attr("aria-labelledby").await?).is_equal_to(listbox_labelledby);
    Ok(())
}

async fn escape_closes_and_restores_focus(page: &Page<'_>) -> Result<(), Report> {
    page.send_keys_to_active(Key::Escape).await?;
    page.wait_for_selector("[aria-haspopup=listbox][aria-expanded=false]")
        .await?;
    assert_that!(page.count_matching("[role=listbox]").await?).is_equal_to(0);
    // Closed, nothing stays inert.
    page.wait_for_no_selector("[inert]").await?;
    let trigger = page.css(TRIGGER).await?;
    assert_that!(page.driver.active_element().await? == trigger).is_true();
    // Nothing changed.
    assert_that!(trigger.text().await?).is_equal_to("Banana".to_owned());

    // The same when opened with the keyboard (Enter focuses the selected option).
    page.send_keys_to_active(Key::Enter).await?;
    page.wait_for_selector("[role=option][aria-selected=true]:focus")
        .await?;
    page.send_keys_to_active(Key::Escape).await?;
    page.wait_for_no_selector("[role=listbox]").await?;
    page.wait_for_focus_on(&trigger, "select trigger after Escape")
        .await?;

    // A select bound to app state (`value`) restores focus as well.
    let bound = page.css("#test-sel-bound [aria-haspopup=listbox]").await?;
    bound.click().await?;
    page.wait_for_selector("[role=option][aria-selected=true]:focus")
        .await?;
    page.send_keys_to_active(Key::Escape).await?;
    page.wait_for_no_selector("[role=listbox]").await?;
    page.wait_for_focus_on(&bound, "bound select trigger after Escape")
        .await?;
    page.send_keys_to_active(Key::Enter).await?;
    page.wait_for_selector("[role=option][aria-selected=true]:focus")
        .await?;
    page.send_keys_to_active(Key::Down).await?;
    page.send_keys_to_active(Key::Enter).await?;
    page.wait_for_text("test-sel-bound-value", "25").await?;
    page.wait_for_focus_on(&bound, "bound select trigger after selecting")
        .await?;
    page.wait_for_text("test-sel-bound-changes", "1").await?;

    // After the app changed the value, picking the previous value again is a change.
    page.element("test-sel-bound-reset").await?.click().await?;
    page.wait_for_text("test-sel-bound-value", "10").await?;
    bound.click().await?;
    page.wait_for_selector("[role=option][aria-selected=true]:focus")
        .await?;
    page.send_keys_to_active(Key::Down).await?;
    page.send_keys_to_active(Key::Enter).await?;
    page.wait_for_text("test-sel-bound-value", "25").await?;
    page.wait_for_text("test-sel-bound-changes", "2").await?;
    Ok(())
}

/// Picking an option closes the popover, updates the trigger and the form value, and returns
/// focus to the trigger.
async fn selecting_an_option(page: &Page<'_>) -> Result<(), Report> {
    page.css(TRIGGER).await?.click().await?;
    page.wait_for_selector("[role=listbox]").await?;
    page.by_role_and_text("option", "Durian")
        .await?
        .click()
        .await?;
    page.wait_for_text("test-sel-changes", "Durian").await?;
    page.wait_for_selector("[aria-haspopup=listbox][aria-expanded=false]")
        .await?;
    assert_that!(page.css(TRIGGER).await?.text().await?).is_equal_to("Durian".to_owned());
    assert_that!(hidden_select_value(page).await?).is_equal_to("Durian".to_owned());
    let trigger = page.css(TRIGGER).await?;
    assert_that!(page.driver.active_element().await? == trigger).is_true();
    Ok(())
}

/// On the closed trigger, ArrowLeft/ArrowRight change the value (skipping disabled options)
/// and typing selects by text.
async fn trigger_keyboard(page: &Page<'_>) -> Result<(), Report> {
    page.send_keys_to_active(Key::Left).await?;
    // "Cherry" is disabled.
    page.wait_for_text("test-sel-changes", "Durian | Banana")
        .await?;
    page.send_keys_to_active(Key::Right).await?;
    page.wait_for_text("test-sel-changes", "Durian | Banana | Durian")
        .await?;
    assert_that!(trigger_attr(page, "aria-expanded").await?).is_equal_to(Some("false".to_owned()));

    page.send_keys_to_active("e").await?;
    page.wait_for_text("test-sel-changes", "Durian | Banana | Durian | Elderberry")
        .await?;
    assert_that!(page.css(TRIGGER).await?.text().await?).is_equal_to("Elderberry".to_owned());
    Ok(())
}

/// The trigger is labelled by its value and the label; clicking the label focuses the trigger.
async fn labelling(page: &Page<'_>) -> Result<(), Report> {
    let label = page
        .driver
        .find(By::XPath("//span[text()='Fruit']"))
        .await?;
    let label_id = label.id().await?.unwrap_or_default();
    let labelled_by = trigger_attr(page, "aria-labelledby")
        .await?
        .unwrap_or_default();
    let ids: Vec<&str> = labelled_by.split(' ').collect();
    assert_that!(ids.len()).is_equal_to(2);
    assert_that!(ids.contains(&label_id.as_str())).is_true();
    let value = page.css(&format!("#{}", ids[0])).await?;
    assert_that!(value.text().await?).is_equal_to("Elderberry".to_owned());

    page.css("#test-sel-before").await?.click().await?;
    label.click().await?;
    let trigger = page.css(TRIGGER).await?;
    assert_that!(page.driver.active_element().await? == trigger).is_true();
    Ok(())
}

/// Resetting the form restores the default value.
async fn form_reset(page: &Page<'_>) -> Result<(), Report> {
    page.driver
        .execute("document.getElementById('test-sel-form').reset()", vec![])
        .await?;
    page.wait_for_selector("[aria-haspopup=listbox]").await?;
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
    while page.css(TRIGGER).await?.text().await? != "Banana" {
        if std::time::Instant::now() > deadline {
            leptos_browser_test::bail!("the form reset did not restore \"Banana\"");
        }
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }
    assert_that!(hidden_select_value(page).await?).is_equal_to("Banana".to_owned());
    Ok(())
}
