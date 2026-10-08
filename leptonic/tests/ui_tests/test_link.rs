// Upstream: react-aria-components/test/Link.test.js @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{Key, WebDriver},
};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

/// The link atom: `aria-current` by route (prefix or exact), client-side navigation, `replace`,
/// `target`/`rel` for new tabs, a disabled link (no `href`, `aria-disabled`, not followed, no
/// presses), the props of a surrounding trigger, hover/focus/press state, Enter, a disabled
/// `use_link` anchor, and `AnchorLink` (scrolls, sets the hash without a history entry, leaves
/// modified clicks to the browser).
pub struct LinkTests {}

#[async_trait]
impl BrowserTest<str> for LinkTests {
    fn name(&self) -> Cow<'_, str> {
        "link_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/link").await?;

        // The current page by route.
        assert_that!(page.attr_of("test-link-self", "aria-current").await?)
            .is_equal_to(Some("page".to_owned()));
        assert_that!(page.attr_of("test-link-prefix", "aria-current").await?)
            .is_equal_to(Some("page".to_owned()));
        assert_that!(page.attr_of("test-link-exact", "aria-current").await?).is_none();

        // A new tab gets `noopener`.
        assert_that!(page.attr_of("test-link-external", "target").await?)
            .is_equal_to(Some("_blank".to_owned()));
        assert_that!(page.attr_of("test-link-external", "rel").await?)
            .is_equal_to(Some("noopener".to_owned()));

        // A link as a trigger gets the trigger's props.
        // react-aria's menu triggers: `aria-haspopup="true"`.
        assert_that!(page.attr_of("test-link-menu", "aria-haspopup").await?)
            .is_equal_to(Some("true".to_owned()));
        assert_that!(page.attr_of("test-link-menu", "aria-expanded").await?)
            .is_equal_to(Some("false".to_owned()));

        // "should support disabled state", "should not navigate if disabled".
        page.css(".test-link-disableable").await?.click().await?;
        page.wait_for_text("test-link-presses", "1").await?;
        page.click_element_with_id("test-link-toggle-disabled")
            .await?;
        page.wait_for_selector(
            "span.test-link-disableable[role=link][aria-disabled=true][data-disabled]",
        )
        .await?;
        assert_that!(
            page.css(".test-link-disableable")
                .await?
                .attr("href")
                .await?
        )
        .is_none();
        page.css(".test-link-disableable").await?.click().await?;
        assert_that!(page.read_text_of("test-link-presses").await?).is_equal_to("1".to_owned());
        page.click_element_with_id("test-link-toggle-disabled")
            .await?;
        page.wait_for_selector("a.test-link-disableable[href]")
            .await?;

        state_attributes(&page).await?;
        disabled_hook_anchor(&page).await?;
        anchor_link(&page).await?;

        // `replace`: no history entry.
        let history_length = || async {
            let len: u64 = page
                .driver
                .execute("return history.length;", vec![])
                .await?
                .convert()?;
            Ok::<u64, Report>(len)
        };
        let before = history_length().await?;
        page.click_element_with_id("test-link-replace").await?;
        page.wait_for_selector("body[data-hydrated]").await?;
        let search: String = page
            .driver
            .execute("return location.search;", vec![])
            .await?
            .convert()?;
        assert_that!(search).is_equal_to("?replaced".to_owned());
        assert_that!(history_length().await?).is_equal_to(before);

        // Client-side navigation: the page isn't reloaded.
        page.driver
            .execute("window.__testLinkNoReload = true;", vec![])
            .await?;
        page.click_element_with_id("test-link-toolbar").await?;
        page.wait_for_selector("#test-page-atom-toolbar").await?;
        let kept: bool = page
            .driver
            .execute("return window.__testLinkNoReload === true;", vec![])
            .await?
            .convert()?;
        assert_that!(kept)
            .with_detail_message("the link navigated on the client")
            .is_true();

        page.expect_no_page_errors().await
    }
}

/// "should support hover", "should support focus ring", "should support press state"; Enter
/// presses a focused link once.
async fn state_attributes(page: &Page<'_>) -> Result<(), Report> {
    let link = page.css("a.test-link-disableable").await?;
    for name in ["data-hovered", "data-pressed", "data-focus-visible"] {
        assert_that!(link.attr(name).await?).is_none();
    }
    page.driver
        .action_chain()
        .move_to_element_center(&link)
        .perform()
        .await?;
    page.wait_for_attr(&link, "data-hovered", Some("true"))
        .await?;
    let presses = page.read_u32("test-link-presses").await?;
    page.driver
        .action_chain()
        .click_and_hold_element(&link)
        .perform()
        .await?;
    page.wait_for_attr(&link, "data-pressed", Some("true"))
        .await?;
    page.driver.action_chain().release().perform().await?;
    page.wait_for_attr(&link, "data-pressed", None).await?;
    let presses = presses + 1;
    page.wait_for_text("test-link-presses", &presses.to_string())
        .await?;
    // Pointer focus shows no focus ring.
    page.wait_for_attr(&link, "data-focused", Some("true"))
        .await?;
    assert_that!(link.attr("data-focus-visible").await?).is_none();
    page.driver
        .action_chain()
        .move_to_element_center(&page.element("test-link-toggle-disabled").await?)
        .perform()
        .await?;
    page.wait_for_attr(&link, "data-hovered", None).await?;

    // Keyboard focus shows it; Enter presses once.
    page.press_shift_tab().await?;
    page.press_tab().await?;
    page.wait_for_attr(&link, "data-focus-visible", Some("true"))
        .await?;
    page.send_keys_to_active(Key::Enter).await?;
    page.wait_for_text("test-link-presses", &(presses + 1).to_string())
        .await?;
    stays!(
        "the text of #test-link-presses",
        (presses + 1).to_string(),
        page.read_text_of("test-link-presses").await?
    );
    page.press_tab().await?;
    page.wait_for_attr(&link, "data-focus-visible", None).await
}

/// A disabled anchor has no `href`, so it keeps the link role explicitly (useLink.test.js
/// "handles isDisabled").
async fn disabled_hook_anchor(page: &Page<'_>) -> Result<(), Report> {
    let link = page.element("test-link-hook-disabled").await?;
    assert_that!(link.attr("href").await?).is_none();
    assert_that!(link.attr("role").await?).is_equal_to(Some("link".to_owned()));
    assert_that!(link.attr("aria-disabled").await?).is_equal_to(Some("true".to_owned()));
    assert_that!(link.attr("tabindex").await?).is_none();
    Ok(())
}

async fn anchor_link(page: &Page<'_>) -> Result<(), Report> {
    let history_length = || async {
        let len: u64 = page
            .driver
            .execute("return history.length;", vec![])
            .await?
            .convert()?;
        Ok::<u64, Report>(len)
    };
    let script = |code: &'static str| async move {
        let value = page.driver.execute(code, vec![]).await?;
        Ok::<serde_json::Value, Report>(value.json().clone())
    };

    // A modified click is the browser's: no scrolling, no hash, the default action not prevented.
    let hash = script("return location.hash;").await?;
    let prevented = script(
        "const link = document.getElementById('test-link-anchor');
         let prevented = null;
         const record = e => { prevented = e.defaultPrevented; e.preventDefault(); };
         window.addEventListener('click', record, {once: true});
         link.dispatchEvent(new MouseEvent('click', {bubbles: true, cancelable: true, ctrlKey: true}));
         return prevented;",
    )
    .await?;
    assert_that!(prevented).is_equal_to(serde_json::Value::Bool(false));
    page.wait_for_text("test-link-anchor-presses", "1").await?;
    assert_that!(script("return location.hash;").await?).is_equal_to(hash);
    assert_that!(script("return window.scrollY;").await?.as_f64()).is_equal_to(Some(0.0));

    // A press scrolls the target into view and sets the hash, without a history entry.
    let before = history_length().await?;
    page.click_element_with_id("test-link-anchor").await?;
    page.wait_for_text("test-link-anchor-presses", "2").await?;
    assert_that!(script("return location.hash;").await?).is_equal_to(serde_json::Value::String(
        "#test-link-anchor-target".to_owned(),
    ));
    assert_that!(history_length().await?).is_equal_to(before);
    let in_view = script(
        "const r = document.getElementById('test-link-anchor-target').getBoundingClientRect();
         return window.scrollY > 0 && r.top >= 0 && r.top < window.innerHeight;",
    )
    .await?;
    assert_that!(in_view).is_equal_to(serde_json::Value::Bool(true));
    script("window.scrollTo(0, 0); return null;").await?;
    Ok(())
}
