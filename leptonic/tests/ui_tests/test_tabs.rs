// Upstream: react-aria-components/test/Tabs.test.js @ 99e6102368
//! Behavior of the tabs hooks (through the `Tabs` atoms): ARIA structure (tabs control their
//! panel, the panel is labelled by its tab), selection by press and by arrow keys (automatic and
//! manual activation, wrapping, vertical orientation, right to left), disabled tabs (also by
//! `Tab::is_disabled`), state as data attributes, force-mounted panels, a bound selected key,
//! added and removed tabs, and nested tabs.
//! Spec: react-aria-components `Tabs.test.js`.
use assertr::{matchers::eq, prelude::*};
use browser_test::{browser_test, thirtyfour::prelude::*};
use leptonic::AriaRole;
use rootcause::Report;

use crate::pages::{ElementActions, Page, role};

const PATH: &str = "/atoms/tabs";

async fn tablist(page: &Page<'_>, name: &str) -> Result<WebElement, Report> {
    page.element(format!("[role=tablist][aria-label='{name}']"))
        .await
}

async fn tabs(page: &Page<'_>, name: &str) -> Result<Vec<WebElement>, Report> {
    tablist(page, name).await?.elements("[role=tab]").await
}

/// The panel of the tabs `name` (the panel next to the tab list).
async fn panel(page: &Page<'_>, name: &str) -> Result<WebElement, Report> {
    tablist(page, name)
        .await?
        .parent()
        .await?
        .element(":scope > [role=tabpanel]")
        .await
}

async fn enter(page: &Page<'_>, name: &str) -> Result<(), Report> {
    page.element(format!("#test-tabs-{name}-before"))
        .await?
        .click()
        .await?;
    page.send_keys(Key::Tab).await?;
    Ok(())
}

/// The indices of the selected tabs of the tabs `name`.
async fn selected_tabs(page: &Page<'_>, name: &str) -> Result<Vec<usize>, Report> {
    let mut selected = Vec::new();
    for (i, tab) in tabs(page, name).await?.iter().enumerate() {
        if tab.attr("aria-selected").await?.as_deref() == Some("true") {
            selected.push(i);
        }
    }
    Ok(selected)
}

/// Wait until the tab at `index` is the selected one.
async fn expect_selected(page: &Page<'_>, name: &str, index: usize) -> Result<(), Report> {
    assert_that!(|| selected_tabs(page, name))
        .with_subject_name(format!("{name}: the selected tabs"))
        .eventually_ok()
        .matches(eq(vec![index]))
        .await;
    Ok(())
}

/// The tab list is horizontal with the first tab selected, and only the selected tab controls a
/// panel, the only one rendered, which that tab labels.
#[browser_test]
pub async fn aria_structure(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let list = tablist(page, "basic").await?;
    assert_that!(list)
        .has_attribute("aria-orientation")
        .await
        .is_equal_to("horizontal");
    let tabs = tabs(page, "basic").await?;
    assert_that!(tabs.as_slice()).has_length(3);
    expect_selected(page, "basic", 0).await?;
    // The selected tab controls the panel, which its tab labels.
    let panel = panel(page, "basic").await?;
    assert_that!(panel)
        .inner_text()
        .await
        .is_equal_to("Panel A");
    let panel_id = panel.attr("id").await?;
    assert_that!(tabs[0])
        .attribute("aria-controls")
        .await
        .is_equal_to(panel_id);
    assert_that!(tabs[1])
        .attribute("aria-controls")
        .await
        .is_none();
    let tab_id = tabs[0].attr("id").await?;
    assert_that!(panel)
        .attribute("aria-labelledby")
        .await
        .is_equal_to(tab_id);
    // Only the selected tab's panel is rendered.
    let panels = list
        .parent()
        .await?
        .elements(":scope > [role=tabpanel]")
        .await?;
    assert_that!(panels).has_length(1);
    Ok(())
}

/// Pressing a tab selects it and shows its panel, whose id follows the tab and which that tab
/// labels; pressing the first tab selects it again ("should support selected state", "should update
/// TabPanel ID when current tab is changed").
#[browser_test]
pub async fn selection_by_press(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    tabs(page, "basic").await?[1].click().await?;
    page.element("#test-tabs-basic-selection")
        .await?
        .wait_for_inner_text("b")
        .await?;
    expect_selected(page, "basic", 1).await?;
    let panel = panel(page, "basic").await?;
    assert_that!(panel)
        .inner_text()
        .await
        .is_equal_to("Panel B");
    assert_that!(panel)
        .has_attribute("id")
        .await
        .ends_with("-b");
    let tab_id = tabs(page, "basic").await?[1].id().await?;
    assert_that!(panel)
        .attribute("aria-labelledby")
        .await
        .is_equal_to(tab_id);

    tabs(page, "basic").await?[0].click().await?;
    page.element("#test-tabs-basic-selection")
        .await?
        .wait_for_inner_text("a")
        .await?;
    expect_selected(page, "basic", 0).await?;
    Ok(())
}

/// ArrowRight/Left select the next/previous tab, wrapping, and Home/End the first/last; ArrowDown
/// and ArrowUp do nothing in a horizontal tab list.
#[browser_test]
pub async fn keyboard_navigation(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    enter(page, "basic").await?;
    page.wait_for_focus(&tabs(page, "basic").await?[0]).await?;
    page.send_keys(Key::Right).await?;
    page.wait_for_focus(&tabs(page, "basic").await?[1]).await?;
    expect_selected(page, "basic", 1).await?;
    page.send_keys(Key::Right).await?;
    page.send_keys(Key::Right).await?;
    page.wait_for_focus(&tabs(page, "basic").await?[0]).await?;
    expect_selected(page, "basic", 0).await?;
    page.send_keys(Key::Left).await?;
    expect_selected(page, "basic", 2).await?;
    page.send_keys(Key::Home).await?;
    expect_selected(page, "basic", 0).await?;
    page.send_keys(Key::End).await?;
    expect_selected(page, "basic", 2).await?;
    // Down/Up don't move in a horizontal tab list: focus and selection stay.
    let last = tabs(page, "basic").await?.swap_remove(2);
    for key in [Key::Down, Key::Up] {
        page.send_keys(key).await?;
        page.focus_stays(&last, std::time::Duration::from_millis(100))
            .await?;
        assert_that!(|| selected_tabs(page, "basic"))
            .consistently_ok()
            .for_at_least(std::time::Duration::from_millis(100))
            .matches(eq(vec![2]))
            .await;
    }
    Ok(())
}

/// A disabled tab has `aria-disabled`, and the arrow keys skip it ("should support disabled state
/// on tab").
#[browser_test]
pub async fn disabled_tab(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let tabs = tabs(page, "disabled-tab").await?;
    assert_that!(tabs[1])
        .has_attribute("aria-disabled")
        .await
        .is_equal_to("true");
    enter(page, "disabled-tab").await?;
    page.wait_for_focus(&tabs[0]).await?;
    page.send_keys(Key::Right).await?;
    page.wait_for_focus(&tabs[2]).await?;
    Ok(())
}

/// Without a default, the first enabled tab is selected.
#[browser_test]
pub async fn disabled_first_tab(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let tabs = tabs(page, "first-disabled").await?;
    assert_that!(tabs[0])
        .has_attribute("aria-disabled")
        .await
        .is_equal_to("true");
    expect_selected(page, "first-disabled", 1).await?;
    enter(page, "first-disabled").await?;
    page.wait_for_focus(&tabs[1]).await?;
    page.send_keys(Key::Right).await?;
    page.wait_for_focus(&tabs[2]).await?;
    Ok(())
}

/// With all tabs disabled, the first is selected and Tab moves past the tabs to the panel
/// ("selects first tab if all tabs are disabled").
#[browser_test]
pub async fn all_tabs_disabled(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    expect_selected(page, "all-disabled", 0).await?;
    enter(page, "all-disabled").await?;
    let panel = panel(page, "all-disabled").await?;
    page.wait_for_focus(&panel).await?;
    Ok(())
}

/// A vertical tab list has `aria-orientation=vertical`; Down and Up select the next and previous
/// tab, and so do Right and Left ("should support orientation").
#[browser_test]
pub async fn vertical(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let list = tablist(page, "vertical").await?;
    assert_that!(list)
        .has_attribute("aria-orientation")
        .await
        .is_equal_to("vertical");
    enter(page, "vertical").await?;
    page.send_keys(Key::Down).await?;
    expect_selected(page, "vertical", 1).await?;
    page.send_keys(Key::Up).await?;
    expect_selected(page, "vertical", 0).await?;
    page.send_keys(Key::Right).await?;
    expect_selected(page, "vertical", 1).await?;
    page.send_keys(Key::Left).await?;
    expect_selected(page, "vertical", 0).await?;
    Ok(())
}

/// With manual activation, arrow keys only move focus and Enter selects the focused tab ("should
/// support keyboardActivation=manual").
#[browser_test]
pub async fn manual_activation(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    enter(page, "manual").await?;
    page.wait_for_focus(&tabs(page, "manual").await?[0]).await?;
    page.send_keys(Key::Right).await?;
    page.wait_for_focus(&tabs(page, "manual").await?[1]).await?;
    page.send_keys(Key::Right).await?;
    page.wait_for_focus(&tabs(page, "manual").await?[2]).await?;
    expect_selected(page, "manual", 0).await?;
    page.send_keys(Key::Enter).await?;
    expect_selected(page, "manual", 2).await?;
    page.element("#test-tabs-manual-selection")
        .await?
        .wait_for_inner_text("c")
        .await?;
    Ok(())
}

/// In a right-to-left vertical tab list, ArrowDown and ArrowLeft select the next tab, ArrowUp and
/// ArrowRight the previous ("allows user to change tab item selection via arrow keys with vertical
/// tabs (rtl)").
#[browser_test]
pub async fn rtl_vertical(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    enter(page, "rtl-vertical").await?;
    expect_selected(page, "rtl-vertical", 0).await?;
    page.send_keys(Key::Down).await?;
    expect_selected(page, "rtl-vertical", 1).await?;
    page.send_keys(Key::Up).await?;
    expect_selected(page, "rtl-vertical", 0).await?;
    page.send_keys(Key::Left).await?;
    expect_selected(page, "rtl-vertical", 1).await?;
    page.send_keys(Key::Right).await?;
    expect_selected(page, "rtl-vertical", 0).await?;
    Ok(())
}

/// Tabs show their selected, disabled, hovered (never when disabled), pressed and focused state as
/// data attributes, with a focus ring on tab and panel only after keyboard focus ("should support
/// render props", "should support hover", "should not show hover state when item is not
/// interactive", "should support press state", "should support focus ring").
#[browser_test]
pub async fn data_attributes(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let tabs = tabs(page, "disabled-tab").await?;
    tabs[0].click().await?;
    expect_selected(page, "disabled-tab", 0).await?;
    assert_that!(tabs[0])
        .has_attribute("data-selected")
        .await
        .is_equal_to("true");
    assert_that!(tabs[2])
        .attribute("data-selected")
        .await
        .is_none();
    assert_that!(tabs[1])
        .has_attribute("data-disabled")
        .await
        .is_equal_to("true");
    assert_that!(tabs[0])
        .has_attribute("class")
        .await
        .is_equal_to("leptonic-Tab");

    // Hover, but not on the disabled tab ("should not show hover state when item is not
    // interactive").
    page.low_level()
        .driver()
        .action_chain()
        .move_to_element_center(&tabs[2])
        .perform()
        .await?;
    tabs[2].wait_for_attr("data-hovered", Some("true")).await?;
    page.low_level()
        .driver()
        .action_chain()
        .move_to_element_center(&tabs[1])
        .perform()
        .await?;
    tabs[2].wait_for_attr("data-hovered", None).await?;
    tabs[1]
        .attr_stays("data-hovered", None, std::time::Duration::from_millis(100))
        .await?;

    // Press state.
    let held = tabs[2].press_and_hold().await?;
    tabs[2].wait_for_attr("data-pressed", Some("true")).await?;
    held.release().await?;
    tabs[2].wait_for_attr("data-pressed", None).await?;
    expect_selected(page, "disabled-tab", 2).await?;
    // Focused by the pointer: no focus ring; by the keyboard: a focus ring on the tab, and on
    // the panel when focus moves there.
    tabs[2].wait_for_attr("data-focused", Some("true")).await?;
    assert_that!(tabs[2])
        .attribute("data-focus-visible")
        .await
        .is_none();
    page.send_keys(Key::Left).await?;
    page.wait_for_focus(&tabs[0]).await?;
    tabs[0]
        .wait_for_attr("data-focus-visible", Some("true"))
        .await?;
    page.send_keys(Key::Tab).await?;
    let panel = panel(page, "disabled-tab").await?;
    page.wait_for_focus(&panel).await?;
    panel
        .wait_for_attr("data-focus-visible", Some("true"))
        .await?;
    assert_that!(tabs[0])
        .attribute("data-focus-visible")
        .await
        .is_none();
    Ok(())
}

/// The panels after the tab list `name` (with or without the tab panel role).
async fn all_panels(page: &Page<'_>, name: &str) -> Result<Vec<WebElement>, Report> {
    tablist(page, name)
        .await?
        .parent()
        .await?
        .elements(":scope > .leptonic-TabPanel")
        .await
}

/// With forced mounting, every panel is rendered, but the unselected ones are inert and have no tab
/// panel role, id or label ("should support shouldForceMount").
#[browser_test]
pub async fn force_mount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let expect_panels = |selected: usize| async move {
        let panels = all_panels(page, "force").await?;
        assert_that!(panels.as_slice()).has_length(3);
        for (i, panel) in panels.iter().enumerate() {
            if i == selected {
                panel.wait_for_attr("inert", None).await?;
                assert_that!(panel).attribute("data-inert").await.is_none();
                assert_that!(panel)
                    .has_attribute("role")
                    .await
                    .is_equal_to("tabpanel");
                assert_that!(panel).has_attribute("aria-labelledby").await;
                assert_that!(panel).has_attribute("id").await;
            } else {
                panel.wait_for_attr("data-inert", Some("true")).await?;
                assert_that!(panel).has_attribute("inert").await;
                for name in ["role", "id", "aria-labelledby", "tabindex"] {
                    assert_that!(panel)
                        .attribute(name)
                        .await
                        .with_detail_message(format!("panel {i}: {name}"))
                        .is_none();
                }
            }
        }
        Ok::<(), Report>(())
    };
    expect_panels(0).await?;
    tabs(page, "force").await?[1].click().await?;
    expect_selected(page, "force", 1).await?;
    expect_panels(1).await?;
    Ok(())
}

/// A tab disabled through `Tab::is_disabled` is skipped by the arrow keys and not selected by a
/// press, and a disabled first tab leaves the initial selection to the next one ("should support
/// isDisabled prop on tab", "finds the first non-disabled tab").
#[browser_test]
pub async fn tab_is_disabled(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let list = tabs(page, "tab-disabled").await?;
    assert_that!(list[1])
        .has_attribute("aria-disabled")
        .await
        .is_equal_to("true");
    enter(page, "tab-disabled").await?;
    page.wait_for_focus(&tabs(page, "tab-disabled").await?[0])
        .await?;
    page.send_keys(Key::Right).await?;
    page.wait_for_focus(&tabs(page, "tab-disabled").await?[2])
        .await?;
    // A press doesn't select it.
    list[1].click().await?;
    page.settle().await?;
    assert_that!(|| selected_tabs(page, "tab-disabled"))
        .consistently_ok()
        .for_at_least(std::time::Duration::from_millis(100))
        .matches(eq(vec![2]))
        .await;

    let first = tabs(page, "tab-first-disabled").await?;
    assert_that!(first[0])
        .has_attribute("aria-disabled")
        .await
        .is_equal_to("true");
    expect_selected(page, "tab-first-disabled", 1).await?;
    enter(page, "tab-first-disabled").await?;
    page.wait_for_focus(&tabs(page, "tab-first-disabled").await?[1])
        .await?;
    page.send_keys(Key::Right).await?;
    page.wait_for_focus(&tabs(page, "tab-first-disabled").await?[2])
        .await?;
    Ok(())
}

/// A bound selected key is shown even when its tab is disabled, is written when the user selects a
/// tab and is followed when the app changes it.
#[browser_test]
pub async fn controlled(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    expect_selected(page, "controlled", 1).await?;
    let list = tabs(page, "controlled").await?;
    assert_that!(list[1])
        .has_attribute("aria-disabled")
        .await
        .is_equal_to("true");
    list[0].click().await?;
    expect_selected(page, "controlled", 0).await?;
    page.element("#test-tabs-controlled-key")
        .await?
        .wait_for_inner_text("a")
        .await?;
    page.element("#test-tabs-controlled-selection")
        .await?
        .wait_for_inner_text("a")
        .await?;
    page.element("#test-tabs-controlled-select-c")
        .await?
        .click()
        .await?;
    expect_selected(page, "controlled", 2).await?;
    let panel = panel(page, "controlled").await?;
    assert_that!(panel)
        .inner_text()
        .await
        .is_equal_to("Panel C");
    Ok(())
}

async fn dynamic_tabs(page: &Page<'_>) -> Result<Vec<WebElement>, Report> {
    page.element("[role=tablist][aria-label='Dynamic tabs']")
        .await?
        .elements("[role=tab]")
        .await
}

/// When the app adds or removes tabs, the new or the new last tab is selected without reporting a
/// selection change, while pressing a tab, even the selected one, reports one ("can add tabs and
/// keep the current selected key").
#[browser_test]
pub async fn dynamic(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let list = dynamic_tabs(page).await?;
    assert_that!(list.as_slice()).has_length(3);
    let changes = page.element("#test-tabs-dynamic-changes").await?;
    list[0].click().await?;
    changes.wait_for_inner_text("1").await?;
    page.send_keys(Key::Right).await?;
    list[1].wait_for_attr("aria-selected", Some("true")).await?;
    changes.wait_for_inner_text("2").await?;

    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&page.element("#test-tabs-dynamic-add").await?)
        .await?;
    page.send_keys(Key::Enter).await?;
    page.wait_for_count("[role=tablist][aria-label='Dynamic tabs'] [role=tab]", 4)
        .await?;
    let list = dynamic_tabs(page).await?;
    list[3].wait_for_attr("aria-selected", Some("true")).await?;
    page.element("[role=tabpanel][aria-labelledby$='-4']")
        .await?
        .wait_for_inner_text("Tab body 4")
        .await?;

    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&page.element("#test-tabs-dynamic-remove").await?)
        .await?;
    page.send_keys(Key::Enter).await?;
    page.wait_for_count("[role=tablist][aria-label='Dynamic tabs'] [role=tab]", 3)
        .await?;
    let list = dynamic_tabs(page).await?;
    list[2].wait_for_attr("aria-selected", Some("true")).await?;
    changes
        .inner_text_stays("2", std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// Tabs nested in a panel keep their own tabs and selection, and go away when another outer tab is
/// selected ("supports nested tabs").
#[browser_test]
pub async fn nested(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let outer = tabs(page, "Outer").await?;
    assert_that!(outer.as_slice()).has_length(2);
    assert_that!(outer[0]).inner_text().await.is_equal_to("Foo");
    let inner = tabs(page, "Inner").await?;
    assert_that!(inner.as_slice()).has_length(2);
    assert_that!(inner[1]).inner_text().await.is_equal_to("Two");
    // The inner tabs are in the outer panel.
    let outer_panel = panel(page, "Outer").await?;
    assert_that!(
        outer_panel
            .elements("[role=tablist][aria-label=Inner] [role=tab]")
            .await?
    )
    .has_length(2);
    inner[1].click().await?;
    expect_selected(page, "Inner", 1).await?;
    expect_selected(page, "Outer", 0).await?;
    outer[1].click().await?;
    expect_selected(page, "Outer", 1).await?;
    page.wait_for_count("[role=tablist][aria-label=Inner]", 0)
        .await?;
    Ok(())
}

/// While the selection changes, `TabPanels` holds its height in pixels for its transition, then
/// returns to `auto` ("should detect block-size in transition for TabPanels").
#[browser_test]
pub async fn tab_panels(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    assert_that!(panel_height(page).await?).is_equal_to("auto");
    tabs(page, "Animated").await?[1].click().await?;
    // A pixel size, while animating.
    assert_that!(|| panel_height(page))
        .eventually_ok()
        .satisfies(|height| {
            height.ends_with("px");
        })
        .await;
    assert_that!(|| panel_height(page))
        .eventually_ok()
        .matches(eq("auto"))
        .await;
    Ok(())
}

/// The `--tab-panel-height` of the animated `TabPanels`.
async fn panel_height(page: &Page<'_>) -> Result<String, Report> {
    let panels = page.element(".test-tab-panels").await?;
    page.low_level()
        .eval(
            "return arguments[0].style.getPropertyValue('--tab-panel-height');",
            vec![panels.to_json()?],
        )
        .await
}

/// `Tabs::is_disabled` disables every tab (`aria-disabled`, `data-disabled`, the root's
/// `data-disabled`), so Tab moves past the tabs to the panel ("should support disabled state on all
/// tabs", "should not focus any tabs when isDisabled tabbing in for the first time").
#[browser_test]
pub async fn disabled_tabs(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    for tab in tabs(page, "all-tabs-disabled").await? {
        assert_that!(tab)
            .has_attribute("aria-disabled")
            .await
            .is_equal_to("true");
        assert_that!(tab)
            .has_attribute("data-disabled")
            .await
            .is_equal_to("true");
    }
    let root = tablist(page, "all-tabs-disabled").await?.parent().await?;
    assert_that!(root)
        .has_attribute("data-disabled")
        .await
        .is_equal_to("true");
    enter(page, "all-tabs-disabled").await?;
    page.wait_for_focus(&panel(page, "all-tabs-disabled").await?)
        .await?;
    Ok(())
}

/// A tab selected by default is selected and shown, and tabbing into the tab list focuses it
/// ("should focus the selected tab when tabbing in for the first time").
#[browser_test]
pub async fn default_selected_key(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    expect_selected(page, "default", 2).await?;
    assert_that!(panel(page, "default").await?)
        .inner_text()
        .await
        .is_equal_to("Panel Third");
    page.element("#test-tabs-default-before")
        .await?
        .click()
        .await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&tabs(page, "default").await?[2])
        .await?;
    Ok(())
}

/// Home and End skip disabled tabs: with the first tab disabled, Home selects the second
/// ("select first item via home key", TabsKeyboardDelegate `getFirstKey`).
#[browser_test]
pub async fn home_and_end_skip_disabled_tabs(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    enter(page, "first-disabled").await?;
    page.wait_for_focus(&tabs(page, "first-disabled").await?[1])
        .await?;
    page.send_keys(Key::End).await?;
    expect_selected(page, "first-disabled", 2).await?;
    page.send_keys(Key::Home).await?;
    expect_selected(page, "first-disabled", 1).await?;
    Ok(())
}

/// A panel with an `aria_label` is named by it and its tab: it labels itself first, then the tab
/// ("should support tabpanels with aria-labels").
#[browser_test]
pub async fn panel_aria_label(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let panel = panel(page, "labelled-panels").await?;
    let panel_id = panel.attr("id").await?.unwrap_or_default();
    let tab_id = tabs(page, "labelled-panels").await?[0]
        .attr("id")
        .await?
        .unwrap_or_default();
    assert_that!(panel)
        .has_attribute("aria-label")
        .await
        .is_equal_to("Panel A");
    assert_that!(panel)
        .has_attribute("aria-labelledby")
        .await
        .is_equal_to(format!("{panel_id} {tab_id}"));
    assert_that!(panel)
        .accessible_name()
        .await
        .is_equal_to("Panel A A");
    Ok(())
}

/// In a right-to-left horizontal tab list, ArrowLeft selects the next tab and ArrowRight the
/// previous ("allows user to change tab item select via arrow keys with horizontal tabs (rtl)").
#[browser_test]
pub async fn rtl_horizontal(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    enter(page, "rtl").await?;
    expect_selected(page, "rtl", 0).await?;
    page.send_keys(Key::Left).await?;
    expect_selected(page, "rtl", 1).await?;
    page.send_keys(Key::Right).await?;
    expect_selected(page, "rtl", 0).await?;
    page.send_keys(Key::Right).await?;
    expect_selected(page, "rtl", 2).await?;
    Ok(())
}

/// With manual activation, Space selects the focused tab too ("not select via left / right keys
/// if keyboardActivation is manual, select on enter / spacebar").
#[browser_test]
pub async fn manual_activation_with_space(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    enter(page, "manual").await?;
    page.wait_for_focus(&tabs(page, "manual").await?[0]).await?;
    page.send_keys(Key::Right).await?;
    page.wait_for_focus(&tabs(page, "manual").await?[1]).await?;
    expect_selected(page, "manual", 0).await?;
    page.send_keys(Key::Space).await?;
    expect_selected(page, "manual", 1).await?;
    Ok(())
}

/// A panel is a tab stop only without tabbable content: one holding an input has no `tabindex`,
/// one holding only a disabled input has `tabindex=0` ("tabpanel should have tabIndex=0 only when
/// there are no focusable elements").
#[browser_test]
pub async fn panel_tab_stop(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let list = tablist(page, "Tooltip and inputs").await?;
    let panel = panel(page, "Tooltip and inputs").await?;
    panel.wait_for_attr("tabindex", None).await?;
    list.element(role(AriaRole::Tab).text("Tooltip tab"))
        .await?
        .click()
        .await?;
    page.element("[role=tabpanel] input[aria-label='Disabled input panel']")
        .await?;
    let panel = self::panel(page, "Tooltip and inputs").await?;
    panel.wait_for_attr("tabindex", Some("0")).await?;
    Ok(())
}

/// A `TooltipTrigger` around a tab shows its tooltip when the tab is hovered, describing the tab
/// ("supports tooltips").
#[browser_test]
pub async fn tooltip_on_a_tab(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element(role(AriaRole::Tab).text("Plain"))
        .await?
        .hover()
        .await?;
    let tab = page
        .element(role(AriaRole::Tab).text("Tooltip tab"))
        .await?;
    tab.hover().await?;
    let tooltip = page.element(role(AriaRole::Tooltip).text("Test")).await?;
    let tooltip_id = tooltip.attr("id").await?.unwrap_or_default();
    tab.wait_for_attr("aria-describedby", Some(&tooltip_id))
        .await?;
    Ok(())
}

/// When the app selects a tab, it becomes the tab stop: `tabindex=0` on it, `-1` on the others
/// ("updates the tab index of the selected tab if programmatically changed").
#[browser_test]
pub async fn roving_tabindex_follows_the_app(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-tabs-controlled-select-c")
        .await?
        .click()
        .await?;
    let list = tabs(page, "controlled").await?;
    list[2].wait_for_attr("tabindex", Some("0")).await?;
    assert_that!(list[0])
        .has_attribute("tabindex")
        .await
        .is_equal_to("-1");
    Ok(())
}

/// The parts carry their default classes, and the root shows focus within (`data-focused`,
/// `data-focus-visible` after keyboard focus) and its orientation ("should render tabs with
/// default classes").
#[browser_test]
pub async fn root_state_and_default_classes(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let list = tablist(page, "basic").await?;
    let root = list.parent().await?;
    assert_that!(root)
        .has_attribute("class")
        .await
        .is_equal_to("leptonic-Tabs");
    assert_that!(root)
        .has_attribute("data-orientation")
        .await
        .is_equal_to("horizontal");
    assert_that!(list)
        .has_attribute("class")
        .await
        .is_equal_to("leptonic-TabList");
    assert_that!(panel(page, "basic").await?)
        .has_attribute("class")
        .await
        .is_equal_to("leptonic-TabPanel");
    assert_that!(root).attribute("data-focused").await.is_none();
    enter(page, "basic").await?;
    root.wait_for_attr("data-focused", Some("true")).await?;
    root.wait_for_attr("data-focus-visible", Some("true"))
        .await?;
    Ok(())
}
