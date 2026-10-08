// Upstream: react-aria-components/test/Disclosure.test.js @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{BrowserTest, async_trait, thirtyfour::prelude::*};
use rootcause::Report;

use crate::pages::{ElementActions, Page, PageActions, SyntheticEvent, role};

/// The disclosure atoms: the trigger controls its panel (`aria-expanded`, `aria-controls`,
/// `aria-labelledby`), which is `hidden="until-found"` while collapsed; other buttons in the
/// disclosure don't toggle it; nested disclosures toggle independently; groups expand one or
/// several; a disabled group disables its triggers; `beforematch` (find in page) expands; the
/// focus ring within; controlled disclosures and groups; a group's `on_expanded_change`; nested
/// groups.
pub struct DisclosureTests {}

/// The panel controlled by the trigger `trigger`.
async fn panel_of(page: &Page<'_>, trigger: &WebElement) -> Result<WebElement, Report> {
    let id = trigger.attr("aria-controls").await?.unwrap_or_default();
    page.element(format!("#{id}")).await
}

async fn expect_expanded(
    page: &Page<'_>,
    trigger: &WebElement,
    expanded: bool,
) -> Result<(), Report> {
    trigger
        .wait_for_attr(
            "aria-expanded",
            Some(if expanded { "true" } else { "false" }),
        )
        .await?;
    let panel = panel_of(page, trigger).await?;
    // The `hidden` property, not the attribute: WebDriver reports boolean attributes as "true".
    panel
        .wait_for_prop("hidden", if expanded { "false" } else { "until-found" })
        .await?;
    Ok(())
}

#[async_trait]
impl BrowserTest<str> for DisclosureTests {
    fn name(&self) -> Cow<'_, str> {
        "disclosure_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/disclosure").await?;

        cases!(
            trigger_controls_its_panel(&page),
            adjacent_interactive_elements(&page),
            toggles_by_press_and_enter(&page),
            find_in_page_expands(&page),
            nested_disclosures(&page),
            one_expanded_at_a_time(&page),
            multiple_expanded(&page),
            panel_as_landmark(&page),
            repeated_keydown_toggles_once(&page),
            disabled_group(&page),
            focus_ring(&page),
            controlled(&page),
            groups(&page),
        );

        Ok(())
    }
}

/// The trigger controls its panel, a group labelled by the trigger, collapsed at first.
async fn trigger_controls_its_panel(page: &Page<'_>) -> Result<(), Report> {
    let trigger = page.element("#test-disc-trigger").await?;
    let panel = panel_of(page, &trigger).await?;
    assert_that!(panel.attr("role").await?)
        .get_some()
        .is_equal_to("group");
    assert_that!(panel.attr("aria-labelledby").await?)
        .get_some()
        .is_equal_to("test-disc-trigger");
    expect_expanded(page, &trigger, false).await?;
    Ok(())
}

/// "should support interactive elements adjacent to heading": the menu opens, the disclosure
/// stays collapsed.
async fn adjacent_interactive_elements(page: &Page<'_>) -> Result<(), Report> {
    let trigger = page.element("#test-disc-trigger").await?;
    page.element("#test-disc-menu-trigger")
        .await?
        .click()
        .await?;
    page.element("[role=menu]").await?;
    expect_expanded(page, &trigger, false).await?;
    page.send_keys(Key::Escape).await?;
    page.wait_for_count("[role=menu]", 0).await?;
    Ok(())
}

/// "should toggle expanded state when trigger is clicked"; Enter toggles once.
async fn toggles_by_press_and_enter(page: &Page<'_>) -> Result<(), Report> {
    let trigger = page.element("#test-disc-trigger").await?;
    let changes = page.element("#test-disc-changes").await?;
    trigger.click().await?;
    expect_expanded(page, &trigger, true).await?;
    trigger.click().await?;
    expect_expanded(page, &trigger, false).await?;
    changes.wait_for_inner_text("2").await?;

    page.send_keys(Key::Enter).await?;
    expect_expanded(page, &trigger, true).await?;
    changes.wait_for_inner_text("3").await?;
    trigger.click().await?;
    expect_expanded(page, &trigger, false).await?;
    Ok(())
}

/// Find in page (`beforematch`, which doesn't bubble) expands the collapsed panel.
async fn find_in_page_expands(page: &Page<'_>) -> Result<(), Report> {
    let trigger = page.element("#test-disc-trigger").await?;
    expect_expanded(page, &trigger, false).await?;
    panel_of(page, &trigger)
        .await?
        .dispatch(SyntheticEvent::plain("beforematch").with("bubbles", false))
        .await?;
    expect_expanded(page, &trigger, true).await?;
    Ok(())
}

/// "should support nested Disclosures".
async fn nested_disclosures(page: &Page<'_>) -> Result<(), Report> {
    let outer = page.element(role("button").text("Outer")).await?;
    let inner = page.element(role("button").text("Inner")).await?;
    outer.click().await?;
    expect_expanded(page, &outer, true).await?;
    expect_expanded(page, &inner, false).await?;
    inner.click().await?;
    expect_expanded(page, &inner, true).await?;
    expect_expanded(page, &outer, true).await?;
    inner.click().await?;
    expect_expanded(page, &inner, false).await?;
    Ok(())
}

/// "should only allow one Disclosure to be expanded at a time by default".
async fn one_expanded_at_a_time(page: &Page<'_>) -> Result<(), Report> {
    let a = page.element(role("button").text("Group A")).await?;
    let b = page.element(role("button").text("Group B")).await?;
    a.click().await?;
    expect_expanded(page, &a, true).await?;
    b.click().await?;
    expect_expanded(page, &b, true).await?;
    expect_expanded(page, &a, false).await?;
    Ok(())
}

/// "should allow multiple Disclosures to be expanded when allowsMultipleExpanded is true".
async fn multiple_expanded(page: &Page<'_>) -> Result<(), Report> {
    let c = page.element(role("button").text("Multi C")).await?;
    let d = page.element(role("button").text("Multi D")).await?;
    c.click().await?;
    d.click().await?;
    expect_expanded(page, &c, true).await?;
    expect_expanded(page, &d, true).await?;
    Ok(())
}

/// A panel as a landmark: the role the panel asks for, on the server and the client.
async fn panel_as_landmark(page: &Page<'_>) -> Result<(), Report> {
    let region = page.element(role("button").text("Region")).await?;
    assert_that!(panel_of(page, &region).await?.attr("role").await?)
        .get_some()
        .is_equal_to("region");
    // The script's promise is awaited by WebDriver.
    let server_html: String = page
        .eval("return fetch(location.href).then(r => r.text());", vec![])
        .await?;
    assert_that!(server_html.matches("role=\"region\"").count())
        .with_detail_message("panels with role=\"region\" in the server's HTML")
        .is_equal_to(1);
    Ok(())
}

/// "should not expand or collapse on repeat keydown events": Enter held down toggles once.
async fn repeated_keydown_toggles_once(page: &Page<'_>) -> Result<(), Report> {
    let region = page.element(role("button").text("Region")).await?;
    region.click().await?;
    expect_expanded(page, &region, true).await?;
    page.hold_key("Enter", 2).await?;
    expect_expanded(page, &region, false).await?;
    Ok(())
}

/// "should disable all Disclosures when DisclosureGroup is disabled".
async fn disabled_group(page: &Page<'_>) -> Result<(), Report> {
    let trigger = page.element(role("button").text("Disabled E")).await?;
    assert_that!(trigger.is_enabled().await?).is_false();
    assert_that!(trigger.attr("data-disabled").await?)
        .get_some()
        .is_equal_to("true");
    Ok(())
}

/// "should support focus ring": `data-focus-visible-within` while keyboard focus is within.
async fn focus_ring(page: &Page<'_>) -> Result<(), Report> {
    let disclosure = page.element("#test-disc-main").await?;
    let trigger = page.element("#test-disc-trigger").await?;
    trigger.click().await?;
    // Pointer focus: no ring.
    disclosure
        .attr_stays("data-focus-visible-within", None)
        .await?;
    page.send_keys(Key::Shift + Key::Tab).await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&trigger).await?;
    disclosure
        .wait_for_attr("data-focus-visible-within", Some("true"))
        .await?;
    // Past the menu button next to the heading, out of the disclosure.
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&page.element("#test-disc-menu-trigger").await?)
        .await?;
    page.send_keys(Key::Tab).await?;
    disclosure
        .wait_for_attr("data-focus-visible-within", None)
        .await?;
    Ok(())
}

/// "should support controlled isExpanded prop", "should expand a disabled disclosure via
/// isExpanded", "should not expand when beforematch event occurs if controlled and closed".
async fn controlled(page: &Page<'_>) -> Result<(), Report> {
    let disclosure = page.element(".test-disc-controlled").await?;
    let trigger = page.element(role("button").text("Controlled")).await?;
    assert_that!(disclosure.attr("data-expanded").await?)
        .get_some()
        .is_equal_to("true");
    expect_expanded(page, &trigger, true).await?;
    trigger.click().await?;
    page.element("#test-disc-controlled-changes")
        .await?
        .wait_for_inner_text("false")
        .await?;
    disclosure.attr_stays("data-expanded", Some("true")).await?;
    expect_expanded(page, &trigger, true).await?;

    let disabled = page.element(".test-disc-disabled-expanded").await?;
    let disabled_trigger = page
        .element(role("button").text("Disabled expanded"))
        .await?;
    assert_that!(disabled.attr("data-disabled").await?)
        .get_some()
        .is_equal_to("true");
    assert_that!(disabled_trigger.is_enabled().await?).is_false();
    expect_expanded(page, &disabled_trigger, true).await?;
    let disabled_panel = panel_of(page, &disabled_trigger).await?;
    assert_that!(disabled_panel.inner_text().await?).is_equal_to("Disabled expanded content");
    assert_that!(disabled_panel.is_displayed().await?).is_true();

    let closed = page
        .element(role("button").text("Closed controlled"))
        .await?;
    expect_expanded(page, &closed, false).await?;
    panel_of(page, &closed)
        .await?
        .dispatch(SyntheticEvent::plain("beforematch").with("bubbles", false))
        .await?;
    page.element("#test-disc-closed-requests")
        .await?
        .wait_for_inner_text("true")
        .await?;
    expect_expanded(page, &closed, false).await?;
    Ok(())
}

/// "should call onExpandedChange when a Disclosure is toggled", "should support controlled
/// expandedKeys prop", "should support nested DisclosureGroups".
async fn groups(page: &Page<'_>) -> Result<(), Report> {
    let keys = page.element("#test-disc-group-keys").await?;
    page.element(role("button").text("Report 1"))
        .await?
        .click()
        .await?;
    keys.wait_for_inner_text("report1").await?;
    page.element(role("button").text("Report 2"))
        .await?
        .click()
        .await?;
    keys.wait_for_inner_text("report2").await?;

    let controlled_1 = page.element(role("button").text("Controlled 1")).await?;
    let controlled_2 = page.element(role("button").text("Controlled 2")).await?;
    expect_expanded(page, &controlled_1, true).await?;
    expect_expanded(page, &controlled_2, false).await?;
    page.element("#test-disc-expand-2").await?.click().await?;
    expect_expanded(page, &controlled_1, false).await?;
    expect_expanded(page, &controlled_2, true).await?;

    let nested_1 = page.element(role("button").text("Nested 1")).await?;
    let nested_2 = page.element(role("button").text("Nested 2")).await?;
    expect_expanded(page, &nested_2, false).await?;
    nested_1.click().await?;
    expect_expanded(page, &nested_1, true).await?;
    expect_expanded(page, &nested_2, false).await?;
    nested_2.click().await?;
    expect_expanded(page, &nested_2, true).await?;
    expect_expanded(page, &nested_1, true).await?;
    nested_2.click().await?;
    expect_expanded(page, &nested_2, false).await?;
    expect_expanded(page, &nested_1, true).await?;
    Ok(())
}
