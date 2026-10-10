// No upstream: `use_global_shortcuts` is a leptonic addition.
//! `use_global_shortcuts`: `Mod+K` works anywhere, also in a text field (and the browser's default
//! is prevented); a bare `/` works only outside text fields, where it types instead; `?` matches
//! although typing it takes Shift; a later binding of `Mod+K` wins while it exists. The
//! `ShortcutKeys` atom shows `Mod+K` as "Ctrl" (read "Control"), "+", "K" off Apple platforms.
use assertr::prelude::*;
use browser_test::browser_test;
use rootcause::Report;

use crate::pages::{ElementActions, Page, Platform};

const PATH: &str = "/hooks/global-shortcuts";

/// Typing `/` outside a text field focuses the filter (marked with `aria-keyshortcuts="/"`)
/// without typing the slash into it.
#[browser_test]
pub async fn slash_outside_text_fields(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-gs-before").await?.click().await?;
    page.send_keys("/").await?;
    let filter = page.element("#test-gs-filter input").await?;
    page.wait_for_focus(&filter).await?;
    assert_that!(filter)
        .has_attribute("aria-keyshortcuts")
        .await
        .is_equal_to("/");
    assert_that!(filter)
        .property("value")
        .await
        .some()
        .is_equal_to("");
    Ok(())
}

/// Typing `/` in a text field types it and keeps focus there.
#[browser_test]
pub async fn slash_in_text_field(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let other = page.element("#test-gs-other").await?;
    other.click().await?;
    page.send_keys("a/b").await?;
    other.wait_for_prop("value", "a/b").await?;
    page.focus_stays(&other, std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// Mod+K (Ctrl, Command on a Mac) triggers its shortcut with nothing focused and with a button
/// focused.
#[browser_test]
pub async fn mod_k_anywhere(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let palette = page.element("#test-gs-palette").await?;
    page.send_keys(page.primary_modifier().await? + "k").await?;
    palette.wait_for_inner_text("1").await?;
    page.element("#test-gs-before").await?.click().await?;
    page.send_keys(page.primary_modifier().await? + "k").await?;
    palette.wait_for_inner_text("2").await?;
    Ok(())
}

/// Typing `?`, which takes Shift, triggers the `?` shortcut defined without Shift.
#[browser_test]
pub async fn shift_key(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.send_keys("?").await?;
    page.element("#test-gs-help")
        .await?
        .wait_for_inner_text("1")
        .await?;
    Ok(())
}

/// While a later binding of Mod+K is mounted only it is triggered, and once it unmounts the
/// earlier binding is triggered again.
#[browser_test]
pub async fn later_binding_wins(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let toggle = page.element("#test-gs-nested-toggle").await?;
    let nested = page.element("#test-gs-nested").await?;
    let palette = page.element("#test-gs-palette").await?;
    toggle.click().await?;
    page.element("#test-gs-nested-shown").await?;
    page.send_keys(page.primary_modifier().await? + "k").await?;
    nested.wait_for_inner_text("1").await?;
    palette
        .inner_text_stays("0", std::time::Duration::from_millis(100))
        .await?;

    toggle.click().await?;
    page.wait_for_count("#test-gs-nested-shown", 0).await?;
    page.send_keys(page.primary_modifier().await? + "k").await?;
    palette.wait_for_inner_text("1").await?;
    nested
        .inner_text_stays("1", std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// Off Apple platforms, the `ShortcutKeys` atom shows `Mod+K` as "Ctrl" (read "Control"), "+",
/// "K" in a `dir="ltr"` element, and literal keys as given.
#[browser_test]
pub async fn shortcut_keys(page: &Page<'_>) -> Result<(), Report> {
    page.emulate_platform(Platform::Linux).await?;
    page.goto_path(PATH).await?;
    let keys = page.element("#test-gs-keys").await?;
    assert_that!(keys.inner_texts("kbd").await?).contains_exactly(["Ctrl\nControl", "K"]);
    assert_that!(keys.inner_texts("[data-separator]").await?).contains_exactly(["+"]);
    assert_that!(keys)
        .has_attribute("dir")
        .await
        .is_equal_to("ltr");

    let literal = page.element("#test-gs-literal").await?;
    assert_that!(literal.inner_texts("kbd").await?).contains_exactly(["⌘\nCommand", "X"]);
    assert_that!(literal.inner_texts("[data-separator]").await?).contains_exactly(["+"]);
    Ok(())
}
