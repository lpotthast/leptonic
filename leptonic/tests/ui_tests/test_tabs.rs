// Upstream: react-aria-components/test/Tabs.test.js @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{By, Key, WebDriver, WebElement},
};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

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

        aria_structure(&page).await?;
        selection_by_press(&page).await?;
        keyboard_navigation(&page).await?;
        disabled_tab(&page).await?;
        disabled_first_tab(&page).await?;
        all_tabs_disabled(&page).await?;
        vertical(&page).await?;
        manual_activation(&page).await?;
        rtl_vertical(&page).await?;
        data_attributes(&page).await?;
        force_mount(&page).await?;
        tab_is_disabled(&page).await?;
        controlled(&page).await?;
        dynamic(&page).await?;
        nested(&page).await?;
        tab_panels(&page).await?;

        Ok(())
    }
}

async fn tablist(page: &Page<'_>, name: &str) -> Result<WebElement, Report> {
    page.css(&format!("[role=tablist][aria-label='{name}']"))
        .await
}

async fn tabs(page: &Page<'_>, name: &str) -> Result<Vec<WebElement>, Report> {
    Ok(tablist(page, name)
        .await?
        .find_all(By::Css("[role=tab]"))
        .await?)
}

/// The panel of the tabs `name` (the tab list's following sibling panel).
async fn panel(page: &Page<'_>, name: &str) -> Result<WebElement, Report> {
    tablist(page, name)
        .await?
        .find(By::XPath("following-sibling::*[@role='tabpanel']"))
        .await
        .map_err(Into::into)
}

async fn attr(element: &WebElement, name: &str) -> Result<Option<String>, Report> {
    Ok(element.attr(name).await?)
}

async fn enter(page: &Page<'_>, name: &str) -> Result<(), Report> {
    page.click_element_with_id(&format!("test-tabs-{name}-before"))
        .await?;
    page.press_tab().await
}

async fn expect_focus(page: &Page<'_>, name: &str, index: usize) -> Result<(), Report> {
    let tab = tabs(page, name).await?.swap_remove(index);
    page.wait_for_focus_on(&tab, &format!("{name}: tab {index}"))
        .await
}

/// Wait until the tab at `index` is the selected one.
async fn expect_selected(page: &Page<'_>, name: &str, index: usize) -> Result<(), Report> {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    loop {
        let mut selected = Vec::new();
        for tab in tabs(page, name).await? {
            selected.push(attr(&tab, "aria-selected").await?.as_deref() == Some("true"));
        }
        if selected.iter().position(|s| *s) == Some(index)
            && selected.iter().filter(|s| **s).count() == 1
        {
            return Ok(());
        }
        if std::time::Instant::now() > deadline {
            leptos_browser_test::bail!("{name}: expected tab {index} selected, got {selected:?}");
        }
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }
}

async fn aria_structure(page: &Page<'_>) -> Result<(), Report> {
    let list = tablist(page, "basic").await?;
    assert_that!(attr(&list, "aria-orientation").await?).is_equal_to(Some("horizontal".to_owned()));
    let tabs = tabs(page, "basic").await?;
    assert_that!(tabs.len()).is_equal_to(3);
    expect_selected(page, "basic", 0).await?;
    // The selected tab controls the panel, which its tab labels.
    let panel = panel(page, "basic").await?;
    assert_that!(panel.text().await?).is_equal_to("Panel A".to_owned());
    let panel_id = attr(&panel, "id").await?;
    assert_that!(attr(&tabs[0], "aria-controls").await?).is_equal_to(panel_id);
    assert_that!(attr(&tabs[1], "aria-controls").await?).is_none();
    let tab_id = attr(&tabs[0], "id").await?;
    assert_that!(attr(&panel, "aria-labelledby").await?).is_equal_to(tab_id);
    // Only the selected tab's panel is rendered.
    let panels = list
        .find_all(By::XPath("following-sibling::*[@role='tabpanel']"))
        .await?;
    assert_that!(panels.len()).is_equal_to(1);
    Ok(())
}

async fn selection_by_press(page: &Page<'_>) -> Result<(), Report> {
    tabs(page, "basic").await?[1].click().await?;
    page.wait_for_text("test-tabs-basic-selection", "b").await?;
    expect_selected(page, "basic", 1).await?;
    let panel = panel(page, "basic").await?;
    assert_that!(panel.text().await?).is_equal_to("Panel B".to_owned());
    let id = attr(&panel, "id").await?.unwrap_or_default();
    assert_that!(id.ends_with("-b")).is_true();
    let tab_id = attr(&tabs(page, "basic").await?[1], "id").await?;
    assert_that!(attr(&panel, "aria-labelledby").await?).is_equal_to(tab_id);

    tabs(page, "basic").await?[0].click().await?;
    page.wait_for_text("test-tabs-basic-selection", "a").await?;
    expect_selected(page, "basic", 0).await
}

/// Arrow keys select the next/previous tab (wrapping); Home/End the first/last.
async fn keyboard_navigation(page: &Page<'_>) -> Result<(), Report> {
    enter(page, "basic").await?;
    expect_focus(page, "basic", 0).await?;
    page.send_keys_to_active(Key::Right).await?;
    expect_focus(page, "basic", 1).await?;
    expect_selected(page, "basic", 1).await?;
    page.send_keys_to_active(Key::Right).await?;
    page.send_keys_to_active(Key::Right).await?;
    expect_focus(page, "basic", 0).await?;
    expect_selected(page, "basic", 0).await?;
    page.send_keys_to_active(Key::Left).await?;
    expect_selected(page, "basic", 2).await?;
    page.send_keys_to_active(Key::Home).await?;
    expect_selected(page, "basic", 0).await?;
    page.send_keys_to_active(Key::End).await?;
    expect_selected(page, "basic", 2).await?;
    // Down/Up don't move in a horizontal tab list.
    page.send_keys_to_active(Key::Down).await?;
    expect_focus(page, "basic", 2).await
}

async fn disabled_tab(page: &Page<'_>) -> Result<(), Report> {
    let tabs = tabs(page, "disabled-tab").await?;
    assert_that!(attr(&tabs[1], "aria-disabled").await?).is_equal_to(Some("true".to_owned()));
    enter(page, "disabled-tab").await?;
    expect_focus(page, "disabled-tab", 0).await?;
    page.send_keys_to_active(Key::Right).await?;
    expect_focus(page, "disabled-tab", 2).await
}

/// Without a default, the first enabled tab is selected.
async fn disabled_first_tab(page: &Page<'_>) -> Result<(), Report> {
    let tabs = tabs(page, "first-disabled").await?;
    assert_that!(attr(&tabs[0], "aria-disabled").await?).is_equal_to(Some("true".to_owned()));
    expect_selected(page, "first-disabled", 1).await?;
    enter(page, "first-disabled").await?;
    expect_focus(page, "first-disabled", 1).await?;
    page.send_keys_to_active(Key::Right).await?;
    expect_focus(page, "first-disabled", 2).await
}

/// With all tabs disabled, the first is selected, and Tab moves past the tabs to the panel.
async fn all_tabs_disabled(page: &Page<'_>) -> Result<(), Report> {
    expect_selected(page, "all-disabled", 0).await?;
    enter(page, "all-disabled").await?;
    let panel = panel(page, "all-disabled").await?;
    page.wait_for_focus_on(&panel, "the all-disabled panel")
        .await
}

async fn vertical(page: &Page<'_>) -> Result<(), Report> {
    let list = tablist(page, "vertical").await?;
    assert_that!(attr(&list, "aria-orientation").await?).is_equal_to(Some("vertical".to_owned()));
    enter(page, "vertical").await?;
    page.send_keys_to_active(Key::Down).await?;
    expect_selected(page, "vertical", 1).await?;
    page.send_keys_to_active(Key::Up).await?;
    expect_selected(page, "vertical", 0).await?;
    page.send_keys_to_active(Key::Right).await?;
    expect_selected(page, "vertical", 1).await?;
    page.send_keys_to_active(Key::Left).await?;
    expect_selected(page, "vertical", 0).await
}

/// With manual activation, arrow keys only move focus; Enter selects.
async fn manual_activation(page: &Page<'_>) -> Result<(), Report> {
    enter(page, "manual").await?;
    expect_focus(page, "manual", 0).await?;
    page.send_keys_to_active(Key::Right).await?;
    expect_focus(page, "manual", 1).await?;
    page.send_keys_to_active(Key::Right).await?;
    expect_focus(page, "manual", 2).await?;
    expect_selected(page, "manual", 0).await?;
    page.send_keys_to_active(Key::Enter).await?;
    expect_selected(page, "manual", 2).await?;
    page.wait_for_text("test-tabs-manual-selection", "c").await
}

/// "allows user to change tab item selection via arrow keys with vertical tabs (rtl)": up and down
/// move, and left/right follow the reading direction (left is next).
async fn rtl_vertical(page: &Page<'_>) -> Result<(), Report> {
    enter(page, "rtl-vertical").await?;
    expect_selected(page, "rtl-vertical", 0).await?;
    page.send_keys_to_active(Key::Down).await?;
    expect_selected(page, "rtl-vertical", 1).await?;
    page.send_keys_to_active(Key::Up).await?;
    expect_selected(page, "rtl-vertical", 0).await?;
    page.send_keys_to_active(Key::Left).await?;
    expect_selected(page, "rtl-vertical", 1).await?;
    page.send_keys_to_active(Key::Right).await?;
    expect_selected(page, "rtl-vertical", 0).await
}

/// "should support render props", "should support hover", "should support focus ring", "should
/// support press state", "should support disabled state on tab".
async fn data_attributes(page: &Page<'_>) -> Result<(), Report> {
    let tabs = tabs(page, "disabled-tab").await?;
    tabs[0].click().await?;
    expect_selected(page, "disabled-tab", 0).await?;
    assert_that!(attr(&tabs[0], "data-selected").await?).is_equal_to(Some("true".to_owned()));
    assert_that!(attr(&tabs[2], "data-selected").await?).is_none();
    assert_that!(attr(&tabs[1], "data-disabled").await?).is_equal_to(Some("true".to_owned()));
    assert_that!(attr(&tabs[0], "class").await?).is_equal_to(Some("leptonic-Tab".to_owned()));

    // Hover, but not on the disabled tab ("should not show hover state when item is not
    // interactive").
    page.driver
        .action_chain()
        .move_to_element_center(&tabs[2])
        .perform()
        .await?;
    page.wait_for_attr(&tabs[2], "data-hovered", Some("true"))
        .await?;
    page.driver
        .action_chain()
        .move_to_element_center(&tabs[1])
        .perform()
        .await?;
    page.wait_for_attr(&tabs[2], "data-hovered", None).await?;
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    assert_that!(attr(&tabs[1], "data-hovered").await?).is_none();

    // Press state.
    page.driver
        .action_chain()
        .click_and_hold_element(&tabs[2])
        .perform()
        .await?;
    page.wait_for_attr(&tabs[2], "data-pressed", Some("true"))
        .await?;
    page.driver.action_chain().release().perform().await?;
    page.wait_for_attr(&tabs[2], "data-pressed", None).await?;
    expect_selected(page, "disabled-tab", 2).await?;
    // Focused by the pointer: no focus ring; by the keyboard: a focus ring on the tab, and on
    // the panel when focus moves there.
    page.wait_for_attr(&tabs[2], "data-focused", Some("true"))
        .await?;
    assert_that!(attr(&tabs[2], "data-focus-visible").await?).is_none();
    page.send_keys_to_active(Key::Left).await?;
    expect_focus(page, "disabled-tab", 0).await?;
    page.wait_for_attr(&tabs[0], "data-focus-visible", Some("true"))
        .await?;
    page.press_tab().await?;
    let panel = panel(page, "disabled-tab").await?;
    page.wait_for_focus_on(&panel, "the disabled-tab panel")
        .await?;
    page.wait_for_attr(&panel, "data-focus-visible", Some("true"))
        .await?;
    assert_that!(attr(&tabs[0], "data-focus-visible").await?).is_none();
    Ok(())
}

/// The panels after the tab list `name` (with or without the tab panel role).
async fn all_panels(page: &Page<'_>, name: &str) -> Result<Vec<WebElement>, Report> {
    Ok(tablist(page, name)
        .await?
        .find_all(By::XPath(
            "following-sibling::*[contains(concat(' ', @class, ' '), ' leptonic-TabPanel ')]",
        ))
        .await?)
}

/// "should support shouldForceMount": every panel is rendered; unselected ones are inert and no
/// tab panels (react-aria-components drops their panel props).
async fn force_mount(page: &Page<'_>) -> Result<(), Report> {
    let expect_panels = |selected: usize| async move {
        let panels = all_panels(page, "force").await?;
        assert_that!(panels.len()).is_equal_to(3);
        for (i, panel) in panels.iter().enumerate() {
            if i == selected {
                page.wait_for_attr(panel, "inert", None).await?;
                assert_that!(attr(panel, "data-inert").await?).is_none();
                assert_that!(attr(panel, "role").await?).is_equal_to(Some("tabpanel".to_owned()));
                assert_that!(attr(panel, "aria-labelledby").await?).is_some();
                assert_that!(attr(panel, "id").await?).is_some();
            } else {
                page.wait_for_attr(panel, "data-inert", Some("true"))
                    .await?;
                assert_that!(attr(panel, "inert").await?).is_some();
                for name in ["role", "id", "aria-labelledby", "tabindex"] {
                    assert_that!(attr(panel, name).await?)
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
    expect_panels(1).await
}

/// "should support isDisabled prop on tab", "finds the first non-disabled tab" (disabled by the
/// `Tab`).
async fn tab_is_disabled(page: &Page<'_>) -> Result<(), Report> {
    let list = tabs(page, "tab-disabled").await?;
    assert_that!(attr(&list[1], "aria-disabled").await?).is_equal_to(Some("true".to_owned()));
    enter(page, "tab-disabled").await?;
    expect_focus(page, "tab-disabled", 0).await?;
    page.send_keys_to_active(Key::Right).await?;
    expect_focus(page, "tab-disabled", 2).await?;
    // A press doesn't select it.
    list[1].click().await?;
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    expect_selected(page, "tab-disabled", 2).await?;

    let first = tabs(page, "tab-first-disabled").await?;
    assert_that!(attr(&first[0], "aria-disabled").await?).is_equal_to(Some("true".to_owned()));
    expect_selected(page, "tab-first-disabled", 1).await?;
    enter(page, "tab-first-disabled").await?;
    expect_focus(page, "tab-first-disabled", 1).await?;
    page.send_keys_to_active(Key::Right).await?;
    expect_focus(page, "tab-first-disabled", 2).await
}

/// A bound selected key: shown, written on selection, followed when the app changes it; a
/// disabled bound key stays selected (as react-aria's controlled `selectedKey`).
async fn controlled(page: &Page<'_>) -> Result<(), Report> {
    expect_selected(page, "controlled", 1).await?;
    let list = tabs(page, "controlled").await?;
    assert_that!(attr(&list[1], "aria-disabled").await?).is_equal_to(Some("true".to_owned()));
    list[0].click().await?;
    expect_selected(page, "controlled", 0).await?;
    page.wait_for_text("test-tabs-controlled-key", "a").await?;
    page.wait_for_text("test-tabs-controlled-selection", "a")
        .await?;
    page.click_element_with_id("test-tabs-controlled-select-c")
        .await?;
    expect_selected(page, "controlled", 2).await?;
    let panel = panel(page, "controlled").await?;
    assert_that!(panel.text().await?).is_equal_to("Panel C".to_owned());
    Ok(())
}

async fn dynamic_tabs(page: &Page<'_>) -> Result<Vec<WebElement>, Report> {
    Ok(page
        .css("[role=tablist][aria-label='Dynamic tabs']")
        .await?
        .find_all(By::Css("[role=tab]"))
        .await?)
}

/// "can add tabs and keep the current selected key": tabs added and removed by the app, which
/// selects the new or the new last tab; the app's changes aren't reported as selections.
async fn dynamic(page: &Page<'_>) -> Result<(), Report> {
    let list = dynamic_tabs(page).await?;
    assert_that!(list.len()).is_equal_to(3);
    list[0].click().await?;
    page.send_keys_to_active(Key::Right).await?;
    page.wait_for_attr(&list[1], "aria-selected", Some("true"))
        .await?;
    page.wait_for_text("test-tabs-dynamic-changes", "1").await?;

    page.press_tab().await?;
    page.wait_for_active_id("test-tabs-dynamic-add").await?;
    page.send_keys_to_active(Key::Enter).await?;
    page.wait_for_count("[role=tablist][aria-label='Dynamic tabs'] [role=tab]", 4)
        .await?;
    let list = dynamic_tabs(page).await?;
    page.wait_for_attr(&list[3], "aria-selected", Some("true"))
        .await?;
    page.wait_for_selector_text("[role=tabpanel][aria-labelledby$='-4']", "Tab body 4")
        .await?;

    page.press_tab().await?;
    page.wait_for_active_id("test-tabs-dynamic-remove").await?;
    page.send_keys_to_active(Key::Enter).await?;
    page.wait_for_count("[role=tablist][aria-label='Dynamic tabs'] [role=tab]", 3)
        .await?;
    let list = dynamic_tabs(page).await?;
    page.wait_for_attr(&list[2], "aria-selected", Some("true"))
        .await?;
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    assert_that!(page.read_text_of("test-tabs-dynamic-changes").await?).is_equal_to("1".to_owned());
    Ok(())
}

/// "supports nested tabs": each tab list holds its own tabs.
async fn nested(page: &Page<'_>) -> Result<(), Report> {
    let outer = tabs(page, "Outer").await?;
    assert_that!(outer.len()).is_equal_to(2);
    assert_that!(outer[0].text().await?).is_equal_to("Foo".to_owned());
    let inner = tabs(page, "Inner").await?;
    assert_that!(inner.len()).is_equal_to(2);
    assert_that!(inner[1].text().await?).is_equal_to("Two".to_owned());
    // The inner tabs are in the outer panel.
    let outer_panel = panel(page, "Outer").await?;
    assert_that!(
        outer_panel
            .find_all(By::Css("[role=tablist][aria-label=Inner] [role=tab]"))
            .await?
            .len()
    )
    .is_equal_to(2);
    inner[1].click().await?;
    expect_selected(page, "Inner", 1).await?;
    expect_selected(page, "Outer", 0).await?;
    outer[1].click().await?;
    expect_selected(page, "Outer", 1).await?;
    page.wait_for_no_selector("[role=tablist][aria-label=Inner]")
        .await
}

/// `TabPanels` ("should detect block-size in transition for TabPanels"): while the selection
/// changes, its size variables hold pixel sizes (animated by its transition), then `auto` again.
async fn tab_panels(page: &Page<'_>) -> Result<(), Report> {
    let read = || async {
        let value: String = page
            .driver
            .execute(
                "return document.querySelector('.test-tab-panels').style.getPropertyValue('--tab-panel-height');",
                vec![],
            )
            .await?
            .convert()?;
        Ok::<String, Report>(value)
    };
    assert_that!(read().await?).is_equal_to("auto".to_owned());
    tabs(page, "Animated").await?[1].click().await?;
    let mut seen_px = false;
    for _ in 0..100 {
        let value = read().await?;
        if value.ends_with("px") {
            seen_px = true;
        } else if seen_px && value == "auto" {
            break;
        }
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }
    assert_that!(seen_px)
        .with_detail_message("--tab-panel-height held a pixel size while animating")
        .is_true();
    assert_that!(read().await?).is_equal_to("auto".to_owned());
    Ok(())
}
