// Upstream: react-aria-components/test/Disclosure.test.js @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{By, Key, WebDriver, WebElement},
};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

/// The disclosure atoms: the trigger controls its panel (`aria-expanded`, `aria-controls`,
/// `aria-labelledby`), which is `hidden="until-found"` while collapsed; other buttons in the
/// disclosure don't toggle it; nested disclosures toggle independently; groups expand one or
/// several; a disabled group disables its triggers; `beforematch` (find in page) expands; the
/// focus ring within; controlled disclosures and groups; a group's `on_expanded_change`; nested
/// groups.
pub struct DisclosureTests {}

async fn button(page: &Page<'_>, text: &str) -> Result<WebElement, Report> {
    Ok(page
        .driver
        .find(By::XPath(format!("//button[normalize-space(.)='{text}']")))
        .await?)
}

/// The panel controlled by the trigger `trigger`.
async fn panel_of(page: &Page<'_>, trigger: &WebElement) -> Result<WebElement, Report> {
    let id = trigger.attr("aria-controls").await?.unwrap_or_default();
    page.element(&id).await
}

async fn expect_expanded(
    page: &Page<'_>,
    trigger: &WebElement,
    expanded: bool,
) -> Result<(), Report> {
    page.wait_for_attr(
        trigger,
        "aria-expanded",
        Some(if expanded { "true" } else { "false" }),
    )
    .await?;
    let panel = panel_of(page, trigger).await?;
    // WebDriver reports boolean attributes as "true": read the attribute's value with a script.
    let expected = if expanded { "null" } else { "until-found" };
    for _ in 0..100 {
        let value: Option<String> = page
            .driver
            .execute(
                "return arguments[0].getAttribute('hidden');",
                vec![panel.to_json()?],
            )
            .await?
            .convert()?;
        if value.as_deref().unwrap_or("null") == expected {
            return Ok(());
        }
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }
    leptos_browser_test::bail!("the panel's `hidden` did not become {expected}")
}

#[async_trait]
impl BrowserTest<str> for DisclosureTests {
    fn name(&self) -> Cow<'_, str> {
        "disclosure_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/disclosure").await?;

        // "should toggle expanded state when trigger is clicked", with ARIA wiring.
        let trigger = page.element("test-disc-trigger").await?;
        let panel = panel_of(&page, &trigger).await?;
        assert_that!(panel.attr("role").await?).is_equal_to(Some("group".to_owned()));
        assert_that!(panel.attr("aria-labelledby").await?)
            .is_equal_to(Some("test-disc-trigger".to_owned()));
        expect_expanded(&page, &trigger, false).await?;

        // "should support interactive elements adjacent to heading": the menu opens, the
        // disclosure stays collapsed.
        page.click_element_with_id("test-disc-menu-trigger").await?;
        page.wait_for_selector("[role=menu]").await?;
        expect_expanded(&page, &trigger, false).await?;
        page.send_keys_to_active(Key::Escape).await?;
        page.wait_for_no_selector("[role=menu]").await?;

        trigger.click().await?;
        expect_expanded(&page, &trigger, true).await?;
        trigger.click().await?;
        expect_expanded(&page, &trigger, false).await?;
        page.wait_for_text("test-disc-changes", "2").await?;

        // Keyboard: Enter toggles once.
        page.send_keys_to_active(Key::Enter).await?;
        expect_expanded(&page, &trigger, true).await?;
        page.wait_for_text("test-disc-changes", "3").await?;
        trigger.click().await?;
        expect_expanded(&page, &trigger, false).await?;

        // Find in page (`beforematch`) expands the collapsed panel.
        page.driver
            .execute(
                "arguments[0].dispatchEvent(new Event('beforematch'));",
                vec![panel.to_json()?],
            )
            .await?;
        expect_expanded(&page, &trigger, true).await?;

        // "should support nested Disclosures".
        let outer = button(&page, "Outer").await?;
        let inner = button(&page, "Inner").await?;
        outer.click().await?;
        expect_expanded(&page, &outer, true).await?;
        expect_expanded(&page, &inner, false).await?;
        inner.click().await?;
        expect_expanded(&page, &inner, true).await?;
        expect_expanded(&page, &outer, true).await?;
        inner.click().await?;
        expect_expanded(&page, &inner, false).await?;

        // "should only allow one Disclosure to be expanded at a time by default".
        let group_a = button(&page, "Group A").await?;
        let group_b = button(&page, "Group B").await?;
        group_a.click().await?;
        expect_expanded(&page, &group_a, true).await?;
        group_b.click().await?;
        expect_expanded(&page, &group_b, true).await?;
        expect_expanded(&page, &group_a, false).await?;

        // "should allow multiple Disclosures to be expanded when allowsMultipleExpanded is true".
        let multi_c = button(&page, "Multi C").await?;
        let multi_d = button(&page, "Multi D").await?;
        multi_c.click().await?;
        multi_d.click().await?;
        expect_expanded(&page, &multi_c, true).await?;
        expect_expanded(&page, &multi_d, true).await?;

        // A panel as a landmark: the role the panel asks for, on the server and the client.
        let region = button(&page, "Region").await?;
        let region_panel = panel_of(&page, &region).await?;
        assert_that!(region_panel.attr("role").await?).is_equal_to(Some("region".to_owned()));
        let server_html: String = page
            .driver
            .execute_async(
                "const done = arguments[arguments.length - 1];
                 fetch(location.href).then(r => r.text()).then(done);",
                vec![],
            )
            .await?
            .convert()?;
        assert_that!(server_html.matches("role=\"region\"").count()).is_equal_to(1);
        // "should not expand or collapse on repeat keydown events".
        region.click().await?;
        expect_expanded(&page, &region, true).await?;
        page.driver
            .execute(
                "const el = document.activeElement;
                 for (let i = 0; i < 3; i++) {
                     el.dispatchEvent(new KeyboardEvent('keydown', {key: 'Enter', repeat: i > 0, bubbles: true}));
                 }
                 el.dispatchEvent(new KeyboardEvent('keyup', {key: 'Enter', bubbles: true}));",
                vec![],
            )
            .await?;
        expect_expanded(&page, &region, false).await?;

        // "should disable all Disclosures when DisclosureGroup is disabled".
        let disabled_e = button(&page, "Disabled E").await?;
        assert_that!(disabled_e.attr("disabled").await?).is_some();
        assert_that!(disabled_e.attr("data-disabled").await?).is_equal_to(Some("true".to_owned()));

        focus_ring(&page).await?;
        controlled(&page).await?;
        groups(&page).await?;

        page.expect_no_page_errors().await
    }
}

/// "should support focus ring": `data-focus-visible-within` while keyboard focus is within.
async fn focus_ring(page: &Page<'_>) -> Result<(), Report> {
    let disclosure = page.element("test-disc-main").await?;
    let trigger = page.element("test-disc-trigger").await?;
    trigger.click().await?;
    // Pointer focus: no ring.
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    assert_that!(disclosure.attr("data-focus-visible-within").await?).is_none();
    page.press_shift_tab().await?;
    page.press_tab().await?;
    page.wait_for_focus_on(&trigger, "the trigger").await?;
    page.wait_for_attr(&disclosure, "data-focus-visible-within", Some("true"))
        .await?;
    page.press_tab().await?;
    page.wait_for_attr(&disclosure, "data-focus-visible-within", None)
        .await
}

/// "should support controlled isExpanded prop", "should expand a disabled disclosure via
/// isExpanded", "should not expand when beforematch event occurs if controlled and closed".
async fn controlled(page: &Page<'_>) -> Result<(), Report> {
    let disclosure = page.css(".test-disc-controlled").await?;
    let trigger = button(page, "Controlled").await?;
    assert_that!(disclosure.attr("data-expanded").await?).is_equal_to(Some("true".to_owned()));
    expect_expanded(page, &trigger, true).await?;
    trigger.click().await?;
    page.wait_for_text("test-disc-controlled-changes", "false")
        .await?;
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    assert_that!(disclosure.attr("data-expanded").await?).is_equal_to(Some("true".to_owned()));
    expect_expanded(page, &trigger, true).await?;

    let disabled = page.css(".test-disc-disabled-expanded").await?;
    let disabled_trigger = button(page, "Disabled expanded").await?;
    assert_that!(disabled.attr("data-disabled").await?).is_equal_to(Some("true".to_owned()));
    assert_that!(disabled_trigger.attr("disabled").await?).is_some();
    expect_expanded(page, &disabled_trigger, true).await?;
    assert_that!(
        page.by_role_and_text("group", "Disabled expanded content")
            .await?
            .is_displayed()
            .await?
    )
    .is_true();

    let closed = button(page, "Closed controlled").await?;
    expect_expanded(page, &closed, false).await?;
    let panel = panel_of(page, &closed).await?;
    page.driver
        .execute(
            "arguments[0].dispatchEvent(new Event('beforematch'));",
            vec![panel.to_json()?],
        )
        .await?;
    page.wait_for_text("test-disc-closed-requests", "true")
        .await?;
    expect_expanded(page, &closed, false).await
}

/// "should call onExpandedChange when a Disclosure is toggled", "should support controlled
/// expandedKeys prop", "should support nested DisclosureGroups".
async fn groups(page: &Page<'_>) -> Result<(), Report> {
    button(page, "Report 1").await?.click().await?;
    page.wait_for_text("test-disc-group-keys", "report1")
        .await?;
    button(page, "Report 2").await?.click().await?;
    page.wait_for_text("test-disc-group-keys", "report2")
        .await?;

    let controlled_1 = button(page, "Controlled 1").await?;
    let controlled_2 = button(page, "Controlled 2").await?;
    expect_expanded(page, &controlled_1, true).await?;
    expect_expanded(page, &controlled_2, false).await?;
    page.click_element_with_id("test-disc-expand-2").await?;
    expect_expanded(page, &controlled_1, false).await?;
    expect_expanded(page, &controlled_2, true).await?;

    let nested_1 = button(page, "Nested 1").await?;
    let nested_2 = button(page, "Nested 2").await?;
    expect_expanded(page, &nested_2, false).await?;
    nested_1.click().await?;
    expect_expanded(page, &nested_1, true).await?;
    expect_expanded(page, &nested_2, false).await?;
    nested_2.click().await?;
    expect_expanded(page, &nested_2, true).await?;
    expect_expanded(page, &nested_1, true).await?;
    nested_2.click().await?;
    expect_expanded(page, &nested_2, false).await?;
    expect_expanded(page, &nested_1, true).await
}
