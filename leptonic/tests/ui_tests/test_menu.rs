// Upstream: react-aria-components/test/Menu.test.tsx @ 99e6102368
// Upstream: @adobe/react-spectrum/test/menu/Menu.test.js @ 99e6102368
// Upstream: @adobe/react-spectrum/test/menu/MenuTrigger.test.js @ 99e6102368
use std::{borrow::Cow, time::Duration};

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{By, Key, TypingData, WebDriver, WebElement},
};
use leptos_browser_test::bail;
use rootcause::Report;

use crate::pages::{BaseActions, Page};

/// The menu trigger: the button that opens the menu.
const TRIGGER: &str = "button[aria-haspopup]";
const MENU: &str = "[role=menu]";
/// Longer than the type-ahead timeout (1s in react-aria), after which typing starts a new search.
const TYPE_AHEAD_RESET: Duration = Duration::from_millis(1100);
const ACTIONS: [&str; 4] = ["Copy", "Cut", "Paste", "Delete"];

/// Behavior of an action menu built from the menu hooks, asserted on the DOM/ARIA level so that
/// these tests keep passing while the collection hooks underneath are rewritten. Elements are
/// found by role and text, as users perceive them. Spec: react-aria-components `Menu.test.tsx`
/// and the React Spectrum `Menu`/`MenuTrigger` tests.
pub struct MenuTests {}

#[async_trait]
impl BrowserTest<str> for MenuTests {
    fn name(&self) -> Cow<'_, str> {
        "menu_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/hooks/menu").await?;

        closed_trigger(&page).await?;
        open_menu_aria_structure(&page).await?;
        keyboard_opening_focuses_first_or_last_item(&page).await?;
        keyboard_navigation_skips_disabled_items(&page).await?;
        keyboard_activation_closes_and_restores_focus(&page).await?;
        type_ahead(&page).await?;
        clicking_an_item(&page).await?;
        mouse_opening_focuses_the_menu(&page).await?;
        type_ahead_skips_disabled_items(&page).await?;
        selection_menu(&page).await?;

        Ok(())
    }
}

/// Type-ahead only considers enabled items (react-aria `ListKeyboardDelegate.getKeyForSearch`
/// walks with `getNextKey`, which skips disabled keys): "p" only matches the disabled "Paste".
async fn type_ahead_skips_disabled_items(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/hooks/menu").await?;
    open_with_key(page, Key::Down).await?;
    expect_focus(page, "Copy").await?;
    page.send_keys_to_active("p").await?;
    tokio::time::sleep(Duration::from_millis(300)).await;
    let active = page.describe_active_element().await?;
    if active != r#"<li> role=menuitem "Copy""# {
        bail!("typing \"p\" moved focus away from \"Copy\" to {active}");
    }
    Ok(())
}

/// react-aria opens the menu with a `null` focus strategy for mouse users, which focuses the menu
/// element itself rather than an item (`useSelectableCollection` with `autoFocus: true`).
async fn mouse_opening_focuses_the_menu(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/hooks/menu").await?;
    trigger(page).await?.click().await?;
    page.wait_for_selector(MENU).await?;
    page.wait_for_focus("menu", None).await?;
    // ArrowDown then focuses the first item.
    page.send_keys_to_active(Key::Down).await?;
    expect_focus(page, "Copy").await
}

async fn trigger(page: &Page<'_>) -> Result<WebElement, Report> {
    page.css(TRIGGER).await
}

async fn trigger_attr(page: &Page<'_>, name: &str) -> Result<Option<String>, Report> {
    Ok(trigger(page).await?.attr(name).await?)
}

async fn item(page: &Page<'_>, action: &str) -> Result<WebElement, Report> {
    page.by_role_and_text("menuitem", action).await
}

async fn expect_focus(page: &Page<'_>, action: &str) -> Result<(), Report> {
    page.wait_for_focus("menuitem", Some(action)).await
}

async fn expect_actions(page: &Page<'_>, expected: &str) -> Result<(), Report> {
    page.wait_for_text("test-menu-actions", expected).await
}

/// Focus the trigger with the keyboard and press `key` on it.
async fn open_with_key(page: &Page<'_>, key: impl Into<TypingData> + Send) -> Result<(), Report> {
    page.click_element_with_id("test-menu-before").await?;
    page.press_tab().await?;
    page.wait_for_focus_on(&trigger(page).await?, "the trigger")
        .await?;
    page.send_keys_to_active(key).await?;
    page.wait_for_selector(MENU).await
}

/// The menu is gone, the trigger says so and has focus again.
async fn expect_closed_with_focus_on_trigger(page: &Page<'_>) -> Result<(), Report> {
    page.wait_for_no_selector(MENU).await?;
    assert_that!(trigger_attr(page, "aria-expanded").await?).is_equal_to(Some("false".to_owned()));
    page.wait_for_focus_on(&trigger(page).await?, "the trigger")
        .await
}

async fn close_with_escape(page: &Page<'_>) -> Result<(), Report> {
    page.send_keys_to_active(Key::Escape).await?;
    expect_closed_with_focus_on_trigger(page)
        .await
        .map_err(|e| e.context("after pressing Escape").into_dynamic())
}

async fn closed_trigger(page: &Page<'_>) -> Result<(), Report> {
    assert_that!(trigger_attr(page, "aria-haspopup").await?).is_equal_to(Some("true".to_owned()));
    assert_that!(trigger_attr(page, "aria-expanded").await?).is_equal_to(Some("false".to_owned()));
    // `aria-controls` may only reference the menu while it exists (i.e. while open).
    assert_that!(trigger_attr(page, "aria-controls").await?).is_none();
    assert_that!(page.count_matching(MENU).await?).is_equal_to(0);
    Ok(())
}

async fn open_menu_aria_structure(page: &Page<'_>) -> Result<(), Report> {
    open_with_key(page, Key::Down).await?;
    expect_focus(page, "Copy").await?;
    let menu = page.css(MENU).await?;
    assert_that!(trigger_attr(page, "aria-expanded").await?).is_equal_to(Some("true".to_owned()));
    let menu_id = menu.attr("id").await?;
    assert_that!(trigger_attr(page, "aria-controls").await?).is_equal_to(menu_id);
    // The trigger labels the menu.
    let trigger_id = trigger_attr(page, "id").await?;
    assert_that!(menu.attr("aria-labelledby").await?).is_equal_to(trigger_id);

    let items = menu.find_all(By::Css("[role=menuitem]")).await?;
    assert_that!(items.len()).is_equal_to(ACTIONS.len());
    for action in ACTIONS {
        let disabled = item(page, action).await?.attr("aria-disabled").await?;
        if action == "Paste" {
            assert_that!(disabled).is_equal_to(Some("true".to_owned()));
        } else {
            assert_that!(disabled)
                .with_detail_message(format!("menu item {action:?}"))
                .is_none();
        }
    }

    // Escape closes the menu and restores focus to the trigger, without an action.
    close_with_escape(page).await?;
    assert_that!(page.read_text_of("test-menu-actions").await?).is_equal_to(String::new());
    Ok(())
}

/// ArrowDown, Enter and Space focus the first item; ArrowUp focuses the last item.
async fn keyboard_opening_focuses_first_or_last_item(page: &Page<'_>) -> Result<(), Report> {
    for (key, name, expected) in [
        (Key::Down.into(), "ArrowDown", "Copy"),
        (Key::Up.into(), "ArrowUp", "Delete"),
        (Key::Enter.into(), "Enter", "Copy"),
        (TypingData::from(" "), "Space", "Copy"),
    ] {
        open_with_key(page, key)
            .await
            .map_err(|e| e.context(format!("opening with {name}")).into_dynamic())?;
        expect_focus(page, expected).await.map_err(|e| {
            e.context(format!("after opening with {name}"))
                .into_dynamic()
        })?;
        close_with_escape(page).await?;
    }
    assert_that!(page.read_text_of("test-menu-actions").await?).is_equal_to(String::new());
    Ok(())
}

async fn keyboard_navigation_skips_disabled_items(page: &Page<'_>) -> Result<(), Report> {
    open_with_key(page, Key::Down).await?;
    expect_focus(page, "Copy").await?;
    for (key, expected) in [
        (Key::Down, "Cut"),
        (Key::Down, "Delete"),
        // The fixture enables `should_focus_wrap`.
        (Key::Down, "Copy"),
        (Key::Up, "Delete"),
        (Key::Up, "Cut"),
        (Key::End, "Delete"),
        (Key::Home, "Copy"),
    ] {
        page.send_keys_to_active(key.clone()).await?;
        expect_focus(page, expected)
            .await
            .map_err(|e| e.context(format!("after pressing {key:?}")).into_dynamic())?;
    }
    close_with_escape(page).await
}

/// Enter and Space on an item perform its action, close the menu and restore focus.
async fn keyboard_activation_closes_and_restores_focus(page: &Page<'_>) -> Result<(), Report> {
    open_with_key(page, Key::Down).await?;
    expect_focus(page, "Copy").await?;
    page.send_keys_to_active(Key::Down).await?;
    expect_focus(page, "Cut").await?;
    page.send_keys_to_active(Key::Enter).await?;
    expect_actions(page, "Cut")
        .await
        .map_err(|e| e.context("after pressing Enter on Cut").into_dynamic())?;
    expect_closed_with_focus_on_trigger(page)
        .await
        .map_err(|e| e.context("after pressing Enter on Cut").into_dynamic())?;

    open_with_key(page, Key::Down).await?;
    expect_focus(page, "Copy").await?;
    page.send_keys_to_active(" ").await?;
    expect_actions(page, "Cut,Copy")
        .await
        .map_err(|e| e.context("after pressing Space on Copy").into_dynamic())?;
    expect_closed_with_focus_on_trigger(page)
        .await
        .map_err(|e| e.context("after pressing Space on Copy").into_dynamic())
}

/// Typing focuses the next item whose text starts with the typed characters.
async fn type_ahead(page: &Page<'_>) -> Result<(), Report> {
    open_with_key(page, Key::Down).await?;
    expect_focus(page, "Copy").await?;
    page.send_keys_to_active("d").await?;
    expect_focus(page, "Delete")
        .await
        .map_err(|e| e.context("after typing \"d\"").into_dynamic())?;

    // Characters typed in quick succession form one search string. Searching starts at the
    // focused item and wraps around.
    tokio::time::sleep(TYPE_AHEAD_RESET).await;
    page.send_keys_to_active("cu").await?;
    expect_focus(page, "Cut")
        .await
        .map_err(|e| e.context("after typing \"cu\"").into_dynamic())?;
    close_with_escape(page).await
}

/// Clicking an item performs its action, closes the menu and restores focus to the trigger.
/// Clicking a disabled item does nothing.
async fn clicking_an_item(page: &Page<'_>) -> Result<(), Report> {
    trigger(page).await?.click().await?;
    page.wait_for_selector(MENU).await?;
    assert_that!(trigger_attr(page, "aria-expanded").await?).is_equal_to(Some("true".to_owned()));

    item(page, "Paste").await?.click().await?;
    stays!(
        "the text of #test-menu-actions",
        "Cut,Copy".to_owned(),
        page.read_text_of("test-menu-actions").await?
    );
    assert_that!(page.count_matching(MENU).await?)
        .with_detail_message("the menu stays open after clicking a disabled item")
        .is_equal_to(1);

    item(page, "Delete").await?.click().await?;
    expect_actions(page, "Cut,Copy,Delete").await?;
    expect_closed_with_focus_on_trigger(page)
        .await
        .map_err(|e| e.context("after clicking Delete").into_dynamic())
}

/// A menu with multiple selection: `menuitemcheckbox` items with `aria-checked`, grouped into
/// labelled sections. Checking items by click or Space keeps the menu open; Enter closes it.
async fn selection_menu(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/hooks/menu").await?;
    let view = page
        .driver
        .find(By::XPath("//button[text()='View']"))
        .await?;
    view.click().await?;
    page.wait_for_selector(MENU).await?;
    let menu = page.css(MENU).await?;

    let items = menu.find_all(By::Css("[role=menuitemcheckbox]")).await?;
    assert_that!(items.len()).is_equal_to(3);
    for item in &items {
        assert_that!(item.attr("aria-checked").await?).is_equal_to(Some("false".to_owned()));
    }
    let groups = menu.find_all(By::Css("[role=group]")).await?;
    assert_that!(groups.len()).is_equal_to(2);
    for (group, heading) in groups.iter().zip(["Panels", "Zoom"]) {
        let heading_id = group.attr("aria-labelledby").await?.unwrap_or_default();
        let heading_el = page.css(&format!("#{heading_id}")).await?;
        assert_that!(heading_el.text().await?).is_equal_to(heading.to_owned());
    }

    let checkbox = |name: &'static str| page.by_role_and_text("menuitemcheckbox", name);

    checkbox("Sidebar").await?.click().await?;
    page.wait_for_text("test-menu-view-selection", "Sidebar")
        .await?;
    assert_that!(checkbox("Sidebar").await?.attr("aria-checked").await?)
        .is_equal_to(Some("true".to_owned()));
    assert_that!(page.count_matching(MENU).await?)
        .with_detail_message("clicking an item of a multiple selection menu keeps it open")
        .is_equal_to(1);

    page.send_keys_to_active(Key::Down).await?;
    page.wait_for_focus("menuitemcheckbox", Some("Toolbar"))
        .await?;
    page.send_keys_to_active(" ").await?;
    page.wait_for_text("test-menu-view-selection", "Sidebar,Toolbar")
        .await?;
    assert_that!(page.count_matching(MENU).await?)
        .with_detail_message("Space in a multiple selection menu keeps it open")
        .is_equal_to(1);

    page.send_keys_to_active(Key::Enter).await?;
    page.wait_for_text("test-menu-view-selection", "Sidebar")
        .await?;
    page.wait_for_no_selector(MENU).await?;
    page.wait_for_focus_on(&view, "the View trigger").await
}
