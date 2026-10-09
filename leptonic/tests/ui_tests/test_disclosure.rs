// Upstream: react-aria-components/test/Disclosure.test.js @ 99e6102368
// Upstream: react-aria/test/disclosure/useDisclosure.test.ts @ 99e6102368
//! The disclosure atoms: the trigger controls its panel (`aria-expanded`, `aria-controls`,
//! `aria-labelledby`), which is `hidden="until-found"` while collapsed; other buttons in the
//! disclosure don't toggle it; nested disclosures toggle independently; groups expand one or
//! several; a disabled group disables its triggers; `beforematch` (find in page) expands; the
//! focus ring within; controlled disclosures and groups; a group's `on_expanded_change`; nested
//! groups.
use assertr::prelude::*;
use browser_test::{browser_test, thirtyfour::prelude::*};
use leptonic::AriaRole;
use rootcause::Report;

use crate::pages::{ElementActions, EventKind, Page, SyntheticEvent, css, role};

const PATH: &str = "/atoms/disclosure";

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

/// The trigger controls its panel, a group labelled by the trigger, collapsed at first.
#[browser_test]
pub async fn trigger_controls_its_panel(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let trigger = page.element("#test-disc-trigger").await?;
    let panel = panel_of(page, &trigger).await?;
    assert_that!(panel)
        .has_attribute("role")
        .await
        .is_equal_to("group");
    assert_that!(panel)
        .has_attribute("aria-labelledby")
        .await
        .is_equal_to("test-disc-trigger");
    expect_expanded(page, &trigger, false).await?;
    Ok(())
}

/// Pressing a menu button next to the trigger opens the menu and leaves the disclosure collapsed
/// ("should support interactive elements adjacent to heading").
#[browser_test]
pub async fn adjacent_interactive_elements(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
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

/// Clicking the trigger expands and collapses the disclosure, and Enter on it toggles it once
/// ("should toggle expanded state when trigger is clicked").
#[browser_test]
pub async fn toggles_by_press_and_enter(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
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

/// Find in page (a `beforematch` event on the panel) expands the collapsed disclosure ("should
/// expand when beforematch event occurs").
#[browser_test]
pub async fn find_in_page_expands(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let trigger = page.element("#test-disc-trigger").await?;
    expect_expanded(page, &trigger, false).await?;
    panel_of(page, &trigger)
        .await?
        .dispatch(SyntheticEvent::plain(EventKind::BeforeMatch).bubbles(false))
        .await?;
    expect_expanded(page, &trigger, true).await?;
    Ok(())
}

/// Nested disclosures toggle independently: expanding and collapsing the inner one leaves the
/// outer one expanded ("should support nested Disclosures").
#[browser_test]
pub async fn nested_disclosures(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let outer = page.element(role(AriaRole::Button).text("Outer")).await?;
    // The inner trigger is deliberately inspected before its parent panel is expanded.
    let inner = panel_of(page, &outer)
        .await?
        .element(css("button").text("Inner"))
        .await?;
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

/// In a group, expanding a disclosure collapses the expanded one ("should only allow one
/// Disclosure to be expanded at a time by default").
#[browser_test]
pub async fn one_expanded_at_a_time(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let a = page.element(role(AriaRole::Button).text("Group A")).await?;
    let b = page.element(role(AriaRole::Button).text("Group B")).await?;
    a.click().await?;
    expect_expanded(page, &a, true).await?;
    b.click().await?;
    expect_expanded(page, &b, true).await?;
    expect_expanded(page, &a, false).await?;
    Ok(())
}

/// A group allowing multiple expanded disclosures keeps both expanded ("should allow multiple
/// Disclosures to be expanded when allowsMultipleExpanded is true").
#[browser_test]
pub async fn multiple_expanded(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let c = page.element(role(AriaRole::Button).text("Multi C")).await?;
    let d = page.element(role(AriaRole::Button).text("Multi D")).await?;
    c.click().await?;
    d.click().await?;
    expect_expanded(page, &c, true).await?;
    expect_expanded(page, &d, true).await?;
    Ok(())
}

/// A panel asking for the region role has it, both in the server's HTML and after hydration.
#[browser_test]
pub async fn panel_as_landmark(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let region = page.element(role(AriaRole::Button).text("Region")).await?;
    assert_that!(panel_of(page, &region).await?)
        .has_attribute("role")
        .await
        .is_equal_to("region");
    // The script's promise is awaited by WebDriver.
    let server_html: String = page
        .low_level()
        .eval("return fetch(location.href).then(r => r.text());", vec![])
        .await?;
    assert_that!(server_html.matches("role=\"region\"").count())
        .with_detail_message("panels with role=\"region\" in the server's HTML")
        .is_equal_to(1);
    Ok(())
}

/// Holding Enter on the trigger toggles the disclosure once, not once per repeated keydown
/// ("should not expand or collapse on repeat keydown events").
#[browser_test]
pub async fn repeated_keydown_toggles_once(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let trigger = page.element("#test-disc-trigger").await?;
    let changes = page.element("#test-disc-changes").await?;
    // Focused by the keyboard: Shift+Tab from the menu button next to it.
    page.element("#test-disc-menu-trigger")
        .await?
        .focus()
        .await?;
    page.send_keys(Key::Shift + Key::Tab).await?;
    page.wait_for_focus(&trigger).await?;
    expect_expanded(page, &trigger, false).await?;

    page.hold_key("Enter", 3).await?;
    expect_expanded(page, &trigger, true).await?;
    changes.wait_for_inner_text("1").await?;
    changes
        .inner_text_stays("1", std::time::Duration::from_millis(100))
        .await?;

    page.hold_key("Enter", 3).await?;
    expect_expanded(page, &trigger, false).await?;
    changes.wait_for_inner_text("2").await?;
    changes
        .inner_text_stays("2", std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// A disabled group disables the triggers of its disclosures ("should disable all Disclosures
/// when DisclosureGroup is disabled").
#[browser_test]
pub async fn disabled_group(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let trigger = page
        .element(role(AriaRole::Button).text("Disabled E"))
        .await?;
    assert_that!(trigger).enabled().await.is_false();
    assert_that!(trigger)
        .has_attribute("data-disabled")
        .await
        .is_equal_to("true");
    Ok(())
}

/// The disclosure shows `data-focus-visible-within` while its trigger has keyboard focus, not
/// after a click, and drops it once focus leaves ("should support focus ring").
#[browser_test]
pub async fn focus_ring(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let disclosure = page.element("#test-disc-main").await?;
    let trigger = page.element("#test-disc-trigger").await?;
    trigger.click().await?;
    // Pointer focus: no ring.
    disclosure
        .attr_stays(
            "data-focus-visible-within",
            None,
            std::time::Duration::from_millis(100),
        )
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

/// Pressing the trigger of a controlled, expanded disclosure only requests collapsing it; the
/// disclosure stays expanded ("should support controlled isExpanded prop").
#[browser_test]
pub async fn controlled(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let disclosure = page.element(".test-disc-controlled").await?;
    let trigger = page
        .element(role(AriaRole::Button).text("Controlled"))
        .await?;
    assert_that!(disclosure)
        .has_attribute("data-expanded")
        .await
        .is_equal_to("true");
    expect_expanded(page, &trigger, true).await?;
    trigger.click().await?;
    page.element("#test-disc-controlled-changes")
        .await?
        .wait_for_inner_text("false")
        .await?;
    disclosure
        .attr_stays(
            "data-expanded",
            Some("true"),
            std::time::Duration::from_millis(100),
        )
        .await?;
    expect_expanded(page, &trigger, true).await?;
    Ok(())
}

/// A disabled disclosure expanded by its controlled state shows its panel while its trigger stays
/// disabled ("should expand a disabled disclosure via isExpanded").
#[browser_test]
pub async fn disabled_expanded(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let disabled = page.element(".test-disc-disabled-expanded").await?;
    let trigger = page
        .element(role(AriaRole::Button).text("Disabled expanded"))
        .await?;
    assert_that!(disabled)
        .has_attribute("data-disabled")
        .await
        .is_equal_to("true");
    assert_that!(trigger).enabled().await.is_false();
    expect_expanded(page, &trigger, true).await?;
    let panel = panel_of(page, &trigger).await?;
    assert_that!(panel)
        .inner_text()
        .await
        .is_equal_to("Disabled expanded content");
    assert_that!(panel).displayed().await.is_true();
    Ok(())
}

/// Find in page on a controlled, collapsed disclosure only requests the expansion; the panel
/// stays collapsed ("should not expand when beforematch event occurs if controlled and closed").
#[browser_test]
pub async fn find_in_page_controlled_closed(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let closed = page
        .element(role(AriaRole::Button).text("Closed controlled"))
        .await?;
    expect_expanded(page, &closed, false).await?;
    let panel = panel_of(page, &closed).await?;
    panel
        .dispatch(SyntheticEvent::plain(EventKind::BeforeMatch).bubbles(false))
        .await?;
    let requests = page.element("#test-disc-closed-requests").await?;
    requests.wait_for_inner_text("true").await?;
    requests
        .inner_text_stays("true", std::time::Duration::from_millis(100))
        .await?;
    closed
        .attr_stays(
            "aria-expanded",
            Some("false"),
            std::time::Duration::from_millis(100),
        )
        .await?;
    panel
        .prop_stays(
            "hidden",
            "until-found",
            std::time::Duration::from_millis(100),
        )
        .await?;
    Ok(())
}

/// Expanding a disclosure of a group reports the group's expanded keys to `on_expanded_change`
/// ("should call onExpandedChange when a Disclosure is toggled").
#[browser_test]
pub async fn group_on_expanded_change(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let keys = page.element("#test-disc-group-keys").await?;
    page.element(role(AriaRole::Button).text("Group A"))
        .await?
        .click()
        .await?;
    keys.wait_for_inner_text("a").await?;
    page.element(role(AriaRole::Button).text("Group B"))
        .await?
        .click()
        .await?;
    keys.wait_for_inner_text("b").await?;
    Ok(())
}

/// A group's controlled expanded keys decide which disclosure is expanded, also when changed from
/// outside ("should support controlled expandedKeys prop").
#[browser_test]
pub async fn group_controlled(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let controlled_1 = page
        .element(role(AriaRole::Button).text("Controlled 1"))
        .await?;
    let controlled_2 = page
        .element(role(AriaRole::Button).text("Controlled 2"))
        .await?;
    expect_expanded(page, &controlled_1, true).await?;
    expect_expanded(page, &controlled_2, false).await?;
    page.element("#test-disc-expand-2").await?.click().await?;
    expect_expanded(page, &controlled_1, false).await?;
    expect_expanded(page, &controlled_2, true).await?;
    Ok(())
}

/// A disclosure in a group nested in another group toggles without collapsing the outer group's
/// expanded disclosure ("should support nested DisclosureGroups").
#[browser_test]
pub async fn nested_groups(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let nested_1 = page
        .element(role(AriaRole::Button).text("Nested 1"))
        .await?;
    // Hidden descendants have no computed accessibility role until their panel opens.
    let nested_2 = panel_of(page, &nested_1)
        .await?
        .element(css("button").text("Nested 2"))
        .await?;
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

/// A panel rendered again while its disclosure stays expanded shows its content (it isn't left
/// `hidden`), and one rendered again while collapsed stays hidden.
#[browser_test]
pub async fn remounted_panel(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let trigger = page
        .element(role(AriaRole::Button).text("Remounted"))
        .await?;
    let toggle = page.element("#test-disc-remount-toggle").await?;
    trigger.click().await?;
    expect_expanded(page, &trigger, true).await?;

    toggle.click().await?;
    page.wait_for_count(css("p").text("Remounted content"), 0)
        .await?;
    toggle.click().await?;
    expect_expanded(page, &trigger, true).await?;
    assert_that!(page.element(css("p").text("Remounted content")).await?)
        .displayed()
        .await
        .is_true();

    trigger.click().await?;
    expect_expanded(page, &trigger, false).await?;
    toggle.click().await?;
    page.wait_for_count(css("p").text("Remounted content"), 0)
        .await?;
    toggle.click().await?;
    expect_expanded(page, &trigger, false).await?;
    Ok(())
}
