// Upstream: react-aria-components/test/Tabs.test.js @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{BrowserTest, async_trait, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    pages::{ElementActions, Page, PageActions, xpath},
    polling::{expect, wait_for},
};

/// Behavior of the tabs hooks (through the `Tabs` atoms): ARIA structure (tabs control their
/// panel, the panel is labelled by its tab), selection by press and by arrow keys (automatic and
/// manual activation, wrapping, vertical orientation, right to left), disabled tabs (also by
/// `Tab::is_disabled`), state as data attributes, force-mounted panels, a bound selected key,
/// added and removed tabs, and nested tabs.
/// Spec: react-aria-components `Tabs.test.js`.
pub struct TabsTests {}

#[async_trait]
impl BrowserTest<str> for TabsTests {
    fn name(&self) -> Cow<'_, str> {
        "tabs_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/tabs").await?;

        cases!(
            aria_structure(&page),
            selection_by_press(&page),
            keyboard_navigation(&page),
            disabled_tab(&page),
            disabled_first_tab(&page),
            all_tabs_disabled(&page),
            vertical(&page),
            manual_activation(&page),
            rtl_vertical(&page),
            data_attributes(&page),
            force_mount(&page),
            tab_is_disabled(&page),
            controlled(&page),
            dynamic(&page),
            nested(&page),
            tab_panels(&page),
        );

        Ok(())
    }
}

async fn tablist(page: &Page<'_>, name: &str) -> Result<WebElement, Report> {
    page.element(format!("[role=tablist][aria-label='{name}']"))
        .await
}

async fn tabs(page: &Page<'_>, name: &str) -> Result<Vec<WebElement>, Report> {
    tablist(page, name).await?.elements("[role=tab]").await
}

/// The panel of the tabs `name` (the tab list's following sibling panel).
async fn panel(page: &Page<'_>, name: &str) -> Result<WebElement, Report> {
    tablist(page, name)
        .await?
        .element(xpath("following-sibling::*[@role='tabpanel']"))
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

async fn expect_focus(page: &Page<'_>, name: &str, index: usize) -> Result<(), Report> {
    let tab = tabs(page, name).await?.swap_remove(index);
    page.wait_for_focus(&tab).await?;
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
    wait_for(format!("{name}: the selected tabs"))
        .observing(|| selected_tabs(page, name))
        .to_be_equal_to(vec![index])
        .await?;
    Ok(())
}

async fn aria_structure(page: &Page<'_>) -> Result<(), Report> {
    let list = tablist(page, "basic").await?;
    assert_that!(list.attr("aria-orientation").await?)
        .get_some()
        .is_equal_to("horizontal");
    let tabs = tabs(page, "basic").await?;
    assert_that!(tabs.as_slice()).has_length(3);
    expect_selected(page, "basic", 0).await?;
    // The selected tab controls the panel, which its tab labels.
    let panel = panel(page, "basic").await?;
    assert_that!(panel.inner_text().await?).is_equal_to("Panel A");
    let panel_id = panel.attr("id").await?;
    assert_that!(tabs[0].attr("aria-controls").await?).is_equal_to(panel_id);
    assert_that!(tabs[1].attr("aria-controls").await?).is_none();
    let tab_id = tabs[0].attr("id").await?;
    assert_that!(panel.attr("aria-labelledby").await?).is_equal_to(tab_id);
    // Only the selected tab's panel is rendered.
    let panels = list
        .elements(xpath("following-sibling::*[@role='tabpanel']"))
        .await?;
    assert_that!(panels).has_length(1);
    Ok(())
}

async fn selection_by_press(page: &Page<'_>) -> Result<(), Report> {
    tabs(page, "basic").await?[1].click().await?;
    page.element("#test-tabs-basic-selection")
        .await?
        .wait_for_inner_text("b")
        .await?;
    expect_selected(page, "basic", 1).await?;
    let panel = panel(page, "basic").await?;
    assert_that!(panel.inner_text().await?).is_equal_to("Panel B");
    assert_that!(panel.id().await?).get_some().ends_with("-b");
    let tab_id = tabs(page, "basic").await?[1].id().await?;
    assert_that!(panel.attr("aria-labelledby").await?).is_equal_to(tab_id);

    tabs(page, "basic").await?[0].click().await?;
    page.element("#test-tabs-basic-selection")
        .await?
        .wait_for_inner_text("a")
        .await?;
    expect_selected(page, "basic", 0).await?;
    Ok(())
}

/// Arrow keys select the next/previous tab (wrapping); Home/End the first/last.
async fn keyboard_navigation(page: &Page<'_>) -> Result<(), Report> {
    enter(page, "basic").await?;
    expect_focus(page, "basic", 0).await?;
    page.send_keys(Key::Right).await?;
    expect_focus(page, "basic", 1).await?;
    expect_selected(page, "basic", 1).await?;
    page.send_keys(Key::Right).await?;
    page.send_keys(Key::Right).await?;
    expect_focus(page, "basic", 0).await?;
    expect_selected(page, "basic", 0).await?;
    page.send_keys(Key::Left).await?;
    expect_selected(page, "basic", 2).await?;
    page.send_keys(Key::Home).await?;
    expect_selected(page, "basic", 0).await?;
    page.send_keys(Key::End).await?;
    expect_selected(page, "basic", 2).await?;
    // Down/Up don't move in a horizontal tab list.
    page.send_keys(Key::Down).await?;
    expect_focus(page, "basic", 2).await?;
    Ok(())
}

async fn disabled_tab(page: &Page<'_>) -> Result<(), Report> {
    let tabs = tabs(page, "disabled-tab").await?;
    assert_that!(tabs[1].attr("aria-disabled").await?)
        .get_some()
        .is_equal_to("true");
    enter(page, "disabled-tab").await?;
    expect_focus(page, "disabled-tab", 0).await?;
    page.send_keys(Key::Right).await?;
    expect_focus(page, "disabled-tab", 2).await?;
    Ok(())
}

/// Without a default, the first enabled tab is selected.
async fn disabled_first_tab(page: &Page<'_>) -> Result<(), Report> {
    let tabs = tabs(page, "first-disabled").await?;
    assert_that!(tabs[0].attr("aria-disabled").await?)
        .get_some()
        .is_equal_to("true");
    expect_selected(page, "first-disabled", 1).await?;
    enter(page, "first-disabled").await?;
    expect_focus(page, "first-disabled", 1).await?;
    page.send_keys(Key::Right).await?;
    expect_focus(page, "first-disabled", 2).await?;
    Ok(())
}

/// With all tabs disabled, the first is selected, and Tab moves past the tabs to the panel.
async fn all_tabs_disabled(page: &Page<'_>) -> Result<(), Report> {
    expect_selected(page, "all-disabled", 0).await?;
    enter(page, "all-disabled").await?;
    let panel = panel(page, "all-disabled").await?;
    page.wait_for_focus(&panel).await?;
    Ok(())
}

async fn vertical(page: &Page<'_>) -> Result<(), Report> {
    let list = tablist(page, "vertical").await?;
    assert_that!(list.attr("aria-orientation").await?)
        .get_some()
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

/// With manual activation, arrow keys only move focus; Enter selects.
async fn manual_activation(page: &Page<'_>) -> Result<(), Report> {
    enter(page, "manual").await?;
    expect_focus(page, "manual", 0).await?;
    page.send_keys(Key::Right).await?;
    expect_focus(page, "manual", 1).await?;
    page.send_keys(Key::Right).await?;
    expect_focus(page, "manual", 2).await?;
    expect_selected(page, "manual", 0).await?;
    page.send_keys(Key::Enter).await?;
    expect_selected(page, "manual", 2).await?;
    page.element("#test-tabs-manual-selection")
        .await?
        .wait_for_inner_text("c")
        .await?;
    Ok(())
}

/// "allows user to change tab item selection via arrow keys with vertical tabs (rtl)": up and down
/// move, and left/right follow the reading direction (left is next).
async fn rtl_vertical(page: &Page<'_>) -> Result<(), Report> {
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

/// "should support render props", "should support hover", "should support focus ring", "should
/// support press state", "should support disabled state on tab".
async fn data_attributes(page: &Page<'_>) -> Result<(), Report> {
    let tabs = tabs(page, "disabled-tab").await?;
    tabs[0].click().await?;
    expect_selected(page, "disabled-tab", 0).await?;
    assert_that!(tabs[0].attr("data-selected").await?)
        .get_some()
        .is_equal_to("true");
    assert_that!(tabs[2].attr("data-selected").await?).is_none();
    assert_that!(tabs[1].attr("data-disabled").await?)
        .get_some()
        .is_equal_to("true");
    assert_that!(tabs[0].attr("class").await?)
        .get_some()
        .is_equal_to("leptonic-Tab");

    // Hover, but not on the disabled tab ("should not show hover state when item is not
    // interactive").
    page.driver
        .action_chain()
        .move_to_element_center(&tabs[2])
        .perform()
        .await?;
    tabs[2].wait_for_attr("data-hovered", Some("true")).await?;
    page.driver
        .action_chain()
        .move_to_element_center(&tabs[1])
        .perform()
        .await?;
    tabs[2].wait_for_attr("data-hovered", None).await?;
    tabs[1].attr_stays("data-hovered", None).await?;

    // Press state.
    page.driver
        .action_chain()
        .click_and_hold_element(&tabs[2])
        .perform()
        .await?;
    tabs[2].wait_for_attr("data-pressed", Some("true")).await?;
    page.driver.action_chain().release().perform().await?;
    tabs[2].wait_for_attr("data-pressed", None).await?;
    expect_selected(page, "disabled-tab", 2).await?;
    // Focused by the pointer: no focus ring; by the keyboard: a focus ring on the tab, and on
    // the panel when focus moves there.
    tabs[2].wait_for_attr("data-focused", Some("true")).await?;
    assert_that!(tabs[2].attr("data-focus-visible").await?).is_none();
    page.send_keys(Key::Left).await?;
    expect_focus(page, "disabled-tab", 0).await?;
    tabs[0]
        .wait_for_attr("data-focus-visible", Some("true"))
        .await?;
    page.send_keys(Key::Tab).await?;
    let panel = panel(page, "disabled-tab").await?;
    page.wait_for_focus(&panel).await?;
    panel
        .wait_for_attr("data-focus-visible", Some("true"))
        .await?;
    assert_that!(tabs[0].attr("data-focus-visible").await?).is_none();
    Ok(())
}

/// The panels after the tab list `name` (with or without the tab panel role).
async fn all_panels(page: &Page<'_>, name: &str) -> Result<Vec<WebElement>, Report> {
    tablist(page, name)
        .await?
        .elements(xpath(
            "following-sibling::*[contains(concat(' ', @class, ' '), ' leptonic-TabPanel ')]",
        ))
        .await
}

/// "should support shouldForceMount": every panel is rendered; unselected ones are inert and no
/// tab panels (react-aria-components drops their panel props).
async fn force_mount(page: &Page<'_>) -> Result<(), Report> {
    let expect_panels = |selected: usize| async move {
        let panels = all_panels(page, "force").await?;
        assert_that!(panels.as_slice()).has_length(3);
        for (i, panel) in panels.iter().enumerate() {
            if i == selected {
                panel.wait_for_attr("inert", None).await?;
                assert_that!(panel.attr("data-inert").await?).is_none();
                assert_that!(panel.attr("role").await?)
                    .get_some()
                    .is_equal_to("tabpanel");
                assert_that!(panel.attr("aria-labelledby").await?).is_some();
                assert_that!(panel.attr("id").await?).is_some();
            } else {
                panel.wait_for_attr("data-inert", Some("true")).await?;
                assert_that!(panel.attr("inert").await?).is_some();
                for name in ["role", "id", "aria-labelledby", "tabindex"] {
                    assert_that!(panel.attr(name).await?)
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

/// "should support isDisabled prop on tab", "finds the first non-disabled tab" (disabled by the
/// `Tab`).
async fn tab_is_disabled(page: &Page<'_>) -> Result<(), Report> {
    let list = tabs(page, "tab-disabled").await?;
    assert_that!(list[1].attr("aria-disabled").await?)
        .get_some()
        .is_equal_to("true");
    enter(page, "tab-disabled").await?;
    expect_focus(page, "tab-disabled", 0).await?;
    page.send_keys(Key::Right).await?;
    expect_focus(page, "tab-disabled", 2).await?;
    // A press doesn't select it.
    list[1].click().await?;
    expect("tab-disabled: the selected tabs")
        .observing(|| selected_tabs(page, "tab-disabled"))
        .to_stay_equal_to(vec![2])
        .await?;

    let first = tabs(page, "tab-first-disabled").await?;
    assert_that!(first[0].attr("aria-disabled").await?)
        .get_some()
        .is_equal_to("true");
    expect_selected(page, "tab-first-disabled", 1).await?;
    enter(page, "tab-first-disabled").await?;
    expect_focus(page, "tab-first-disabled", 1).await?;
    page.send_keys(Key::Right).await?;
    expect_focus(page, "tab-first-disabled", 2).await?;
    Ok(())
}

/// A bound selected key: shown, written on selection, followed when the app changes it; a
/// disabled bound key stays selected (as react-aria's controlled `selectedKey`).
async fn controlled(page: &Page<'_>) -> Result<(), Report> {
    expect_selected(page, "controlled", 1).await?;
    let list = tabs(page, "controlled").await?;
    assert_that!(list[1].attr("aria-disabled").await?)
        .get_some()
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
    assert_that!(panel.inner_text().await?).is_equal_to("Panel C");
    Ok(())
}

async fn dynamic_tabs(page: &Page<'_>) -> Result<Vec<WebElement>, Report> {
    page.element("[role=tablist][aria-label='Dynamic tabs']")
        .await?
        .elements("[role=tab]")
        .await
}

/// "can add tabs and keep the current selected key": tabs added and removed by the app, which
/// selects the new or the new last tab; the app's changes aren't reported as selections. Pressing
/// the selected tab reports it again (`useSingleSelectListState`: "Always fire
/// onSelectionChange, even if the key is the same").
async fn dynamic(page: &Page<'_>) -> Result<(), Report> {
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
    changes.inner_text_stays("2").await?;
    Ok(())
}

/// "supports nested tabs": each tab list holds its own tabs.
async fn nested(page: &Page<'_>) -> Result<(), Report> {
    let outer = tabs(page, "Outer").await?;
    assert_that!(outer.as_slice()).has_length(2);
    assert_that!(outer[0].inner_text().await?).is_equal_to("Foo");
    let inner = tabs(page, "Inner").await?;
    assert_that!(inner.as_slice()).has_length(2);
    assert_that!(inner[1].inner_text().await?).is_equal_to("Two");
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

/// `TabPanels` ("should detect block-size in transition for TabPanels"): while the selection
/// changes, its size variables hold pixel sizes (animated by its transition), then `auto` again.
async fn tab_panels(page: &Page<'_>) -> Result<(), Report> {
    assert_that!(panel_height(page).await?).is_equal_to("auto");
    tabs(page, "Animated").await?[1].click().await?;
    wait_for("--tab-panel-height")
        .observing(|| panel_height(page))
        .to_be("a pixel size (while animating)", |height| {
            height.ends_with("px")
        })
        .await?;
    wait_for("--tab-panel-height")
        .observing(|| panel_height(page))
        .to_be_equal_to("auto")
        .await?;
    Ok(())
}

/// The `--tab-panel-height` of the animated `TabPanels`.
async fn panel_height(page: &Page<'_>) -> Result<String, Report> {
    let panels = page.element(".test-tab-panels").await?;
    page.eval(
        "return arguments[0].style.getPropertyValue('--tab-panel-height');",
        vec![panels.to_json()?],
    )
    .await
}
