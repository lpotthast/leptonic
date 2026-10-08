// Upstream: react-aria-components/test/Menu.test.tsx @ 99e6102368
// Upstream: @adobe/react-spectrum/test/menu/Menu.test.js @ 99e6102368
// Upstream: @adobe/react-spectrum/test/menu/MenuTrigger.test.js @ 99e6102368
use std::{borrow::Cow, time::Duration};

use assertr::prelude::*;
use browser_test::{BrowserTest, async_trait, thirtyfour::prelude::*};
use rootcause::{Report, prelude::ResultExt};

use crate::pages::{ElementActions, Page, PageActions, role};

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
        cases!(
            closed_trigger(&page),
            open_menu_aria_structure(&page),
            keyboard_opening_focuses_first_or_last_item(&page),
            keyboard_navigation_skips_disabled_items(&page),
            keyboard_activation_closes_and_restores_focus(&page),
            type_ahead(&page),
            clicking_an_item(&page),
            mouse_opening_focuses_the_menu(&page),
            type_ahead_skips_disabled_items(&page),
            selection_menu(&page),
        );
        Ok(())
    }
}

/// The menu item with the text `action`.
async fn item(page: &Page<'_>, action: &str) -> Result<WebElement, Report> {
    page.element(role("menuitem").text(action)).await
}

/// Wait until the menu item `action` has focus.
async fn expect_focus(page: &Page<'_>, action: &str) -> Result<(), Report> {
    page.wait_for_focus(&item(page, action).await?).await
}

/// The fixture's log of performed actions, comma-separated.
async fn actions_log(page: &Page<'_>) -> Result<WebElement, Report> {
    page.element("#test-menu-actions").await
}

/// Focus the trigger with the keyboard and press `key` on it.
async fn open_with_key(page: &Page<'_>, key: impl Into<TypingData> + Send) -> Result<(), Report> {
    page.element("#test-menu-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&page.element(TRIGGER).await?).await?;
    page.send_keys(key).await?;
    page.element(MENU).await?;
    Ok(())
}

/// The menu is gone, the trigger says so and has focus again.
async fn expect_closed_with_focus_on_trigger(page: &Page<'_>) -> Result<(), Report> {
    page.wait_for_count(MENU, 0).await?;
    let trigger = page.element(TRIGGER).await?;
    trigger
        .wait_for_attr("aria-expanded", Some("false"))
        .await?;
    page.wait_for_focus(&trigger).await?;
    Ok(())
}

/// Escape closes the menu and restores focus to the trigger.
async fn close_with_escape(page: &Page<'_>) -> Result<(), Report> {
    page.send_keys(Key::Escape).await?;
    expect_closed_with_focus_on_trigger(page)
        .await
        .context("after pressing Escape")?;
    Ok(())
}

/// The closed trigger announces a menu, collapsed, controlling nothing.
async fn closed_trigger(page: &Page<'_>) -> Result<(), Report> {
    let trigger = page.element(TRIGGER).await?;
    assert_that!(trigger.attr("aria-haspopup").await?)
        .get_some()
        .is_equal_to("true");
    assert_that!(trigger.attr("aria-expanded").await?)
        .get_some()
        .is_equal_to("false");
    // `aria-controls` may only reference the menu while it exists (i.e. while open).
    assert_that!(trigger.attr("aria-controls").await?).is_none();
    assert_that!(page.count(MENU).await?).is_equal_to(0);
    Ok(())
}

/// The open menu is controlled and labelled by the trigger and lists the items, Paste disabled.
/// Escape closes it without an action.
async fn open_menu_aria_structure(page: &Page<'_>) -> Result<(), Report> {
    let trigger = page.element(TRIGGER).await?;
    open_with_key(page, Key::Down).await?;
    expect_focus(page, "Copy").await?;
    let menu = page.element(MENU).await?;
    assert_that!(trigger.attr("aria-expanded").await?)
        .get_some()
        .is_equal_to("true");
    let menu_id = menu.attr("id").await?;
    assert_that!(trigger.attr("aria-controls").await?).is_equal_to(menu_id);
    let trigger_id = trigger.attr("id").await?;
    assert_that!(menu.attr("aria-labelledby").await?).is_equal_to(trigger_id);

    assert_that!(menu.inner_texts("[role=menuitem]").await?).contains_exactly(ACTIONS);
    for action in ACTIONS {
        let disabled = item(page, action).await?.attr("aria-disabled").await?;
        if action == "Paste" {
            assert_that!(disabled).get_some().is_equal_to("true");
        } else {
            assert_that!(disabled)
                .with_detail_message(format!("menu item {action:?}"))
                .is_none();
        }
    }

    close_with_escape(page).await?;
    actions_log(page).await?.inner_text_stays("").await?;
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
            .context_with(|| format!("opening with {name}"))?;
        expect_focus(page, expected)
            .await
            .context_with(|| format!("after opening with {name}"))?;
        close_with_escape(page).await?;
    }
    actions_log(page).await?.inner_text_stays("").await?;
    Ok(())
}

/// Arrow keys, Home and End move focus, skipping the disabled item and wrapping.
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
        page.send_keys(key.clone()).await?;
        expect_focus(page, expected)
            .await
            .context_with(|| format!("after pressing {key:?}"))?;
    }
    close_with_escape(page).await?;
    Ok(())
}

/// Enter and Space on an item perform its action, close the menu and restore focus.
async fn keyboard_activation_closes_and_restores_focus(page: &Page<'_>) -> Result<(), Report> {
    let actions = actions_log(page).await?;
    open_with_key(page, Key::Down).await?;
    expect_focus(page, "Copy").await?;
    page.send_keys(Key::Down).await?;
    expect_focus(page, "Cut").await?;
    page.send_keys(Key::Enter).await?;
    actions
        .wait_for_inner_text("Cut")
        .await
        .context("after pressing Enter on Cut")?;
    expect_closed_with_focus_on_trigger(page)
        .await
        .context("after pressing Enter on Cut")?;

    open_with_key(page, Key::Down).await?;
    expect_focus(page, "Copy").await?;
    page.send_keys(" ").await?;
    actions
        .wait_for_inner_text("Cut,Copy")
        .await
        .context("after pressing Space on Copy")?;
    expect_closed_with_focus_on_trigger(page)
        .await
        .context("after pressing Space on Copy")?;
    Ok(())
}

/// Typing focuses the next item whose text starts with the typed characters.
async fn type_ahead(page: &Page<'_>) -> Result<(), Report> {
    open_with_key(page, Key::Down).await?;
    expect_focus(page, "Copy").await?;
    page.send_keys("d").await?;
    expect_focus(page, "Delete")
        .await
        .context("after typing \"d\"")?;

    // Characters typed in quick succession form one search string. Searching starts at the
    // focused item and wraps around. A real timer: let the previous search expire first.
    tokio::time::sleep(TYPE_AHEAD_RESET).await;
    page.send_keys("cu").await?;
    expect_focus(page, "Cut")
        .await
        .context("after typing \"cu\"")?;
    close_with_escape(page).await?;
    Ok(())
}

/// Clicking an item performs its action, closes the menu and restores focus to the trigger.
/// Clicking a disabled item does nothing.
async fn clicking_an_item(page: &Page<'_>) -> Result<(), Report> {
    let trigger = page.element(TRIGGER).await?;
    let actions = actions_log(page).await?;
    trigger.click().await?;
    page.element(MENU).await?;
    trigger.wait_for_attr("aria-expanded", Some("true")).await?;

    item(page, "Paste").await?.click().await?;
    actions.inner_text_stays("Cut,Copy").await?;
    assert_that!(page.count(MENU).await?)
        .with_detail_message("the menu stays open after clicking a disabled item")
        .is_equal_to(1);

    item(page, "Delete").await?.click().await?;
    actions.wait_for_inner_text("Cut,Copy,Delete").await?;
    expect_closed_with_focus_on_trigger(page)
        .await
        .context("after clicking Delete")?;
    Ok(())
}

/// react-aria opens the menu with a `null` focus strategy for mouse users, which focuses the menu
/// element itself rather than an item (`useSelectableCollection` with `autoFocus: true`).
async fn mouse_opening_focuses_the_menu(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/hooks/menu").await?;
    page.element(TRIGGER).await?.click().await?;
    page.wait_for_focus(&page.element(MENU).await?).await?;
    // ArrowDown then focuses the first item.
    page.send_keys(Key::Down).await?;
    expect_focus(page, "Copy").await?;
    Ok(())
}

/// Type-ahead only considers enabled items (react-aria `ListKeyboardDelegate.getKeyForSearch`
/// walks with `getNextKey`, which skips disabled keys): "p" only matches the disabled "Paste".
async fn type_ahead_skips_disabled_items(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/hooks/menu").await?;
    open_with_key(page, Key::Down).await?;
    let copy = item(page, "Copy").await?;
    page.wait_for_focus(&copy).await?;
    page.send_keys("p").await?;
    page.focus_stays(&copy).await?;
    Ok(())
}

/// A menu with multiple selection: `menuitemcheckbox` items with `aria-checked`, grouped into
/// labelled sections. Checking items by click or Space keeps the menu open; Enter closes it.
async fn selection_menu(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/hooks/menu").await?;
    let view = page.element(role("button").text("View")).await?;
    let selection = page.element("#test-menu-view-selection").await?;
    view.click().await?;
    let menu = page.element(MENU).await?;

    let items = menu.elements("[role=menuitemcheckbox]").await?;
    assert_that!(&items).has_length(3);
    for item in &items {
        assert_that!(item.attr("aria-checked").await?)
            .get_some()
            .is_equal_to("false");
    }
    let groups = menu.elements("[role=group]").await?;
    let mut headings = Vec::new();
    for group in &groups {
        headings.push(group.referenced_text("aria-labelledby").await?);
    }
    assert_that!(headings).contains_exactly(["Panels", "Zoom"]);

    let sidebar = page
        .element(role("menuitemcheckbox").text("Sidebar"))
        .await?;
    sidebar.click().await?;
    selection.wait_for_inner_text("Sidebar").await?;
    assert_that!(sidebar.attr("aria-checked").await?)
        .get_some()
        .is_equal_to("true");
    assert_that!(page.count(MENU).await?)
        .with_detail_message("clicking an item of a multiple selection menu keeps it open")
        .is_equal_to(1);

    page.send_keys(Key::Down).await?;
    page.wait_for_focus(
        &page
            .element(role("menuitemcheckbox").text("Toolbar"))
            .await?,
    )
    .await?;
    page.send_keys(" ").await?;
    selection.wait_for_inner_text("Sidebar,Toolbar").await?;
    assert_that!(page.count(MENU).await?)
        .with_detail_message("Space in a multiple selection menu keeps it open")
        .is_equal_to(1);

    page.send_keys(Key::Enter).await?;
    selection.wait_for_inner_text("Sidebar").await?;
    page.wait_for_count(MENU, 0).await?;
    page.wait_for_focus(&view).await?;
    Ok(())
}
