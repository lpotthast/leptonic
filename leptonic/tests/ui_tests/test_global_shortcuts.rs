// No upstream: `use_global_shortcuts` is a leptonic addition.
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{BrowserTest, async_trait, thirtyfour::prelude::*};
use rootcause::Report;

use crate::pages::{ElementActions, Page, PageActions};

/// `use_global_shortcuts`: `Mod+K` works anywhere, also in a text field (and the browser's default
/// is prevented); a bare `/` works only outside text fields, where it types instead; `?` matches
/// although typing it takes Shift; a later binding of `Mod+K` wins while it exists. The
/// `ShortcutKeys` atom shows `Mod+K` as "Ctrl" (read "Control"), "+", "K" off Apple platforms.
pub struct GlobalShortcutsTests {}

#[async_trait]
impl BrowserTest<str> for GlobalShortcutsTests {
    fn name(&self) -> Cow<'_, str> {
        "global_shortcuts_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/hooks/global-shortcuts").await?;
        cases!(
            slash_outside_text_fields(&page),
            slash_in_text_field(&page),
            mod_k_anywhere(&page),
            shift_key(&page),
            later_binding_wins(&page),
            shortcut_keys(&page),
        );
        Ok(())
    }
}

/// `/` outside a text field focuses the filter (and isn't typed).
async fn slash_outside_text_fields(page: &Page<'_>) -> Result<(), Report> {
    page.element("#test-gs-before").await?.click().await?;
    page.send_keys("/").await?;
    let filter = page.element("#test-gs-filter input").await?;
    page.wait_for_focus(&filter).await?;
    assert_that!(filter.attr("aria-keyshortcuts").await?)
        .get_some()
        .is_equal_to("/");
    assert_that!(filter.value().await?)
        .get_some()
        .is_equal_to("");
    Ok(())
}

/// In a text field, `/` is typed.
async fn slash_in_text_field(page: &Page<'_>) -> Result<(), Report> {
    let other = page.element("#test-gs-other").await?;
    other.click().await?;
    page.send_keys("a/b").await?;
    other.wait_for_prop("value", "a/b").await?;
    page.focus_stays(&other).await?;
    Ok(())
}

/// Mod+K works anywhere, also while typing.
async fn mod_k_anywhere(page: &Page<'_>) -> Result<(), Report> {
    let palette = page.element("#test-gs-palette").await?;
    page.send_keys(Key::Control + "k").await?;
    palette.wait_for_inner_text("1").await?;
    page.element("#test-gs-before").await?.click().await?;
    page.send_keys(Key::Control + "k").await?;
    palette.wait_for_inner_text("2").await?;
    Ok(())
}

/// `?` takes Shift to type: the shortcut without Shift still matches.
async fn shift_key(page: &Page<'_>) -> Result<(), Report> {
    page.send_keys("?").await?;
    page.element("#test-gs-help")
        .await?
        .wait_for_inner_text("1")
        .await?;
    Ok(())
}

/// A later binding wins while it exists.
async fn later_binding_wins(page: &Page<'_>) -> Result<(), Report> {
    let toggle = page.element("#test-gs-nested-toggle").await?;
    let nested = page.element("#test-gs-nested").await?;
    let palette = page.element("#test-gs-palette").await?;
    toggle.click().await?;
    page.element("#test-gs-nested-shown").await?;
    page.send_keys(Key::Control + "k").await?;
    nested.wait_for_inner_text("1").await?;
    palette.inner_text_stays("2").await?;

    toggle.click().await?;
    page.wait_for_count("#test-gs-nested-shown", 0).await?;
    page.send_keys(Key::Control + "k").await?;
    palette.wait_for_inner_text("3").await?;
    nested.inner_text_stays("1").await?;
    Ok(())
}

/// The `ShortcutKeys` atom: "Ctrl" shown, "Control" read (visually hidden); left to right in
/// right-to-left text too (react-aria-components' `Keyboard`). Literal keys are shown as given,
/// on every platform.
async fn shortcut_keys(page: &Page<'_>) -> Result<(), Report> {
    let keys = page.element("#test-gs-keys").await?;
    assert_that!(keys.inner_texts("kbd").await?).contains_exactly(["Ctrl\nControl", "K"]);
    assert_that!(keys.inner_texts("[data-separator]").await?).contains_exactly(["+"]);
    assert_that!(keys.attr("dir").await?)
        .get_some()
        .is_equal_to("ltr");

    let literal = page.element("#test-gs-literal").await?;
    assert_that!(literal.inner_texts("kbd").await?).contains_exactly(["⌘\nCommand", "X"]);
    assert_that!(literal.inner_texts("[data-separator]").await?).contains_exactly(["+"]);
    Ok(())
}
