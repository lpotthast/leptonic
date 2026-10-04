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

        Ok(())
    }
}

/// Select behavior that is known to be broken today. Run with `BROWSER_TEST_KNOWN_ISSUES=1`;
/// move a check into [`SelectTests`] once it is fixed.
pub struct SelectKnownIssues {}

#[async_trait]
impl BrowserTest<str> for SelectKnownIssues {
    fn name(&self) -> Cow<'_, str> {
        "select_known_issues".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/select").await?;

        // An uncontrolled select shows the option the user picked.
        page.css(TRIGGER).await?.click().await?;
        page.wait_for_selector("[role=listbox]").await?;
        page.by_role_and_text("option", "Durian")
            .await?
            .click()
            .await?;
        page.wait_for_text("test-sel-changes", "Durian").await?;
        assert_that!(page.css(TRIGGER).await?.text().await?).is_equal_to("Durian".to_owned());
        assert_that!(hidden_select_value(&page).await?).is_equal_to("Durian".to_owned());
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
    Ok(())
}

async fn escape_closes_and_restores_focus(page: &Page<'_>) -> Result<(), Report> {
    page.send_keys_to_active(Key::Escape).await?;
    page.wait_for_selector("[aria-haspopup=listbox][aria-expanded=false]")
        .await?;
    assert_that!(page.driver.find_all(By::Css("[role=listbox]")).await?.len()).is_equal_to(0);
    let trigger = page.css(TRIGGER).await?;
    assert_that!(page.driver.active_element().await? == trigger).is_true();
    // Nothing changed.
    assert_that!(trigger.text().await?).is_equal_to("Banana".to_owned());
    Ok(())
}
