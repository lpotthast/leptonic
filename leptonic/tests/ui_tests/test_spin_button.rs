// Upstream: react-aria/test/spinbutton/useSpinButton.test.js @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{Key, WebDriver, WebElement},
};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

/// `use_spin_button`: the spin button role and ARIA props, disabled and read-only spin buttons,
/// the keys calling their callbacks (PageUp/PageDown falling back to a single step), and
/// announcements of value changes while focused, with a real minus sign.
pub struct SpinButtonTests {}

async fn attr(element: &WebElement, name: &str) -> Result<Option<String>, Report> {
    Ok(element.attr(name).await?)
}

/// Focus the spin button named `label`, press `key` and wait for `expected` in the log.
async fn press(page: &Page<'_>, label: &str, key: Key, expected: &str) -> Result<(), Report> {
    page.css(&format!("[aria-label='{label}']"))
        .await?
        .focus()
        .await?;
    page.send_keys_to_active(key).await?;
    page.wait_for_text("test-sb-log", expected).await
}

#[async_trait]
impl BrowserTest<str> for SpinButtonTests {
    fn name(&self) -> Cow<'_, str> {
        "spin_button_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/hooks/spin-button").await?;

        // "should have role="spinbutton" and aria props".
        let pages = page.css("[aria-label=Pages]").await?;
        assert_that!(attr(&pages, "role").await?).is_equal_to(Some("spinbutton".to_owned()));
        assert_that!(attr(&pages, "aria-valuenow").await?).is_equal_to(Some("2".to_owned()));
        assert_that!(attr(&pages, "aria-valuetext").await?).is_equal_to(Some("2".to_owned()));
        assert_that!(attr(&pages, "aria-valuemin").await?).is_equal_to(Some("1".to_owned()));
        assert_that!(attr(&pages, "aria-valuemax").await?).is_equal_to(Some("5".to_owned()));
        assert_that!(attr(&pages, "aria-disabled").await?).is_none();
        assert_that!(attr(&pages, "aria-readonly").await?).is_none();

        // "should have aria-disabled if isDisabled is set" / "aria-readonly if isReadOnly".
        let disabled = page.css("[aria-label=Disabled]").await?;
        assert_that!(attr(&disabled, "aria-disabled").await?).is_equal_to(Some("true".to_owned()));
        // The returned steppers are disabled with the spin button (and while read-only).
        for (label, disabled) in [("Pages", false), ("Disabled", true), ("Read only", true)] {
            let up = page.css(&format!("[aria-label='{label} up']")).await?;
            assert_that!(attr(&up, "disabled").await?.is_some()).is_equal_to(disabled);
        }
        // Without a value, the spin button reads "Empty".
        let empty = page.css("[aria-label=Empty]").await?;
        assert_that!(attr(&empty, "aria-valuetext").await?).is_equal_to(Some("Empty".to_owned()));
        let read_only = page.css("[aria-label='Read only']").await?;
        assert_that!(attr(&read_only, "aria-readonly").await?).is_equal_to(Some("true".to_owned()));

        // The keys call their callbacks.
        press(&page, "Pages", Key::Up, "increment").await?;
        press(&page, "Pages", Key::Down, "decrement").await?;
        press(&page, "Pages", Key::PageUp, "increment_page").await?;
        press(&page, "Pages", Key::PageDown, "decrement_page").await?;
        press(&page, "Pages", Key::Home, "decrement_to_min").await?;
        press(&page, "Pages", Key::End, "increment_to_max").await?;
        // "should fall back to onIncrement/onDecrement on page up/down".
        press(&page, "No pages", Key::PageUp, "increment").await?;
        press(&page, "No pages", Key::PageDown, "decrement").await?;
        // Read-only and disabled spin buttons ignore the keys.
        press(&page, "Pages", Key::Up, "increment").await?;
        page.css("[aria-label='Read only']").await?.focus().await?;
        page.send_keys_to_active(Key::Down).await?;
        page.css("[aria-label=Disabled]").await?.focus().await?;
        page.send_keys_to_active(Key::Down).await?;
        assert_that!(page.element("test-sb-log").await?.text().await?)
            .is_equal_to("increment".to_owned());

        // "should announce on value change while focused", with "should substitute a minus sign
        // for hyphen in the textValue for negative values".
        let temperature = page.css("[aria-label=Temperature]").await?;
        assert_that!(attr(&temperature, "aria-valuetext").await?)
            .is_equal_to(Some("\u{2212}5 \u{b0}C".to_owned()));
        press(&page, "Temperature", Key::Up, "increment").await?;
        page.wait_for_selector("[data-live-announcer] [aria-live=assertive] div")
            .await?;
        let announced = page
            .css("[data-live-announcer] [aria-live=assertive]")
            .await?
            .text()
            .await?;
        assert_that!(announced).contains("\u{2212}4 \u{b0}C");

        page.expect_no_page_errors().await
    }
}
