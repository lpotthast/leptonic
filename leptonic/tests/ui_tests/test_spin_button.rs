// Upstream: react-aria/test/spinbutton/useSpinButton.test.js @ 99e6102368
//! `use_spin_button`: the spin button role and ARIA props, disabled and read-only spin buttons,
//! the keys calling their callbacks (PageUp/PageDown falling back to a single step), and
//! announcements of value changes while focused, with a real minus sign.
use assertr::prelude::*;
use browser_test::thirtyfour::prelude::*;
use rootcause::{Report, prelude::ResultExt};

use crate::pages::{ElementActions, Page, PageActions};

const PATH: &str = "/hooks/spin-button";

/// The spin button named `label`.
async fn spin_button(page: &Page<'_>, label: &str) -> Result<WebElement, Report> {
    page.element(format!("[aria-label='{label}']")).await
}

/// The fixture's log of the last callback called.
async fn log(page: &Page<'_>) -> Result<WebElement, Report> {
    page.element("#test-sb-log").await
}

/// Focus the spin button named `label` and press `key`.
async fn press(page: &Page<'_>, label: &str, key: Key) -> Result<(), Report> {
    spin_button(page, label).await?.focus().await?;
    page.send_keys(key).await?;
    Ok(())
}

/// "should have role="spinbutton" and aria props".
pub async fn aria_props(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let pages = spin_button(page, "Pages").await?;
    assert_that!(pages.attr("role").await?)
        .get_some()
        .is_equal_to("spinbutton");
    assert_that!(pages.attr("aria-valuenow").await?)
        .get_some()
        .is_equal_to("2");
    assert_that!(pages.attr("aria-valuetext").await?)
        .get_some()
        .is_equal_to("2");
    assert_that!(pages.attr("aria-valuemin").await?)
        .get_some()
        .is_equal_to("1");
    assert_that!(pages.attr("aria-valuemax").await?)
        .get_some()
        .is_equal_to("5");
    assert_that!(pages.attr("aria-disabled").await?).is_none();
    assert_that!(pages.attr("aria-readonly").await?).is_none();
    // Without a value, the spin button reads "Empty".
    let empty = spin_button(page, "Empty").await?;
    assert_that!(empty.attr("aria-valuetext").await?)
        .get_some()
        .is_equal_to("Empty");
    Ok(())
}

/// "should have aria-disabled if isDisabled is set" / "aria-readonly if isReadOnly".
pub async fn disabled_and_read_only(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let disabled = spin_button(page, "Disabled").await?;
    assert_that!(disabled.attr("aria-disabled").await?)
        .get_some()
        .is_equal_to("true");
    let read_only = spin_button(page, "Read only").await?;
    assert_that!(read_only.attr("aria-readonly").await?)
        .get_some()
        .is_equal_to("true");
    // The returned steppers are disabled with the spin button (and while read-only).
    for (label, enabled) in [("Pages", true), ("Disabled", false), ("Read only", false)] {
        let up = spin_button(page, &format!("{label} up")).await?;
        assert_that!(up.is_enabled().await?)
            .with_detail_message(format!("the stepper of {label}"))
            .is_equal_to(enabled);
    }
    Ok(())
}

/// The keys call their callbacks; "should fall back to onIncrement/onDecrement on page up/down".
pub async fn keys_call_their_callbacks(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    for (label, key, expected) in [
        ("Pages", Key::Up, "increment"),
        ("Pages", Key::Down, "decrement"),
        ("Pages", Key::PageUp, "increment_page"),
        ("Pages", Key::PageDown, "decrement_page"),
        ("Pages", Key::Home, "decrement_to_min"),
        ("Pages", Key::End, "increment_to_max"),
        ("No pages", Key::PageUp, "increment"),
        ("No pages", Key::PageDown, "decrement"),
    ] {
        press(page, label, key.clone()).await?;
        log(page)
            .await?
            .wait_for_inner_text(expected)
            .await
            .context_with(|| format!("after {key:?} on {label}"))?;
    }
    Ok(())
}

/// Read-only and disabled spin buttons ignore the keys.
pub async fn read_only_and_disabled_ignore_keys(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    press(page, "Pages", Key::Up).await?;
    log(page).await?.wait_for_inner_text("increment").await?;
    press(page, "Read only", Key::Down).await?;
    press(page, "Disabled", Key::Down).await?;
    log(page).await?.inner_text_stays("increment").await?;
    Ok(())
}

/// "should announce on value change while focused", with "should substitute a minus sign for
/// hyphen in the textValue for negative values".
pub async fn announces_value_changes(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let temperature = spin_button(page, "Temperature").await?;
    assert_that!(temperature.attr("aria-valuetext").await?)
        .get_some()
        .is_equal_to("\u{2212}5 \u{b0}C");
    press(page, "Temperature", Key::Up).await?;
    log(page).await?.wait_for_inner_text("increment").await?;
    let assertive = page
        .element("[data-live-announcer] [aria-live=assertive]")
        .await?;
    assertive.wait_for_inner_text("\u{2212}4 \u{b0}C").await?;
    Ok(())
}
