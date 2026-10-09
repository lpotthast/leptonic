// Upstream: react-aria-components/test/Menu.test.tsx @ 99e6102368
// Upstream: @adobe/react-spectrum/test/menu/Menu.test.js @ 99e6102368
// Upstream: @adobe/react-spectrum/test/menu/MenuTrigger.test.js @ 99e6102368
//! Behavior of an action menu built from the menu hooks, asserted on the DOM/ARIA level so that
//! these tests keep passing while the collection hooks underneath are rewritten. Elements are
//! found by role and text, as users perceive them. Spec: react-aria-components `Menu.test.tsx`
//! and the React Spectrum `Menu`/`MenuTrigger` tests.
use assertr::prelude::*;
use browser_test::{browser_test, thirtyfour::prelude::*};
use leptonic::AriaRole;
use rootcause::{Report, prelude::ResultExt};

use crate::pages::{ElementActions, Page, css, role};

const PATH: &str = "/hooks/menu";

const MENU: &str = "[role=menu]";
const ACTIONS: [&str; 4] = ["Copy", "Cut", "Paste", "Delete"];

/// The menu item with the text `action`.
async fn item(page: &Page<'_>, action: &str) -> Result<WebElement, Report> {
    page.element(role(AriaRole::Menuitem).text(action)).await
}

/// The fixture's log of performed actions, comma-separated.
async fn actions_log(page: &Page<'_>) -> Result<WebElement, Report> {
    page.element("#test-menu-actions").await
}

/// Focus the trigger with the keyboard and press `key` on it.
async fn open_with_key(page: &Page<'_>, key: impl Into<TypingData> + Send) -> Result<(), Report> {
    page.element("#test-menu-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(
        &page
            .element(css("button[aria-haspopup]").text("Actions"))
            .await?,
    )
    .await?;
    page.send_keys(key).await?;
    page.element(MENU).await?;
    Ok(())
}

/// The menu is gone, the trigger says so and has focus again.
async fn expect_closed_with_focus_on_trigger(page: &Page<'_>) -> Result<(), Report> {
    page.wait_for_count(MENU, 0).await?;
    let trigger = page
        .element(css("button[aria-haspopup]").text("Actions"))
        .await?;
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

/// The closed trigger has `aria-haspopup="true"` and `aria-expanded="false"` but no
/// `aria-controls`, and no menu is rendered.
#[browser_test]
pub async fn closed_trigger(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let trigger = page
        .element(css("button[aria-haspopup]").text("Actions"))
        .await?;
    assert_that!(trigger)
        .has_attribute("aria-haspopup")
        .await
        .is_equal_to("true");
    assert_that!(trigger)
        .has_attribute("aria-expanded")
        .await
        .is_equal_to("false");
    // `aria-controls` may only reference the menu while it exists (i.e. while open).
    assert_that!(trigger)
        .attribute("aria-controls")
        .await
        .is_none();
    assert_that!(page.count(MENU).await?).is_equal_to(0);
    Ok(())
}

/// The open menu is controlled and labelled by the expanded trigger and lists the items, only the
/// disabled one with `aria-disabled`. Escape closes it without performing an action.
#[browser_test]
pub async fn open_menu_aria_structure(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let trigger = page
        .element(css("button[aria-haspopup]").text("Actions"))
        .await?;
    open_with_key(page, Key::Down).await?;
    page.wait_for_focus(&item(page, "Copy").await?).await?;
    let menu = page.element(MENU).await?;
    assert_that!(trigger)
        .has_attribute("aria-expanded")
        .await
        .is_equal_to("true");
    let menu_id = menu.attr("id").await?;
    assert_that!(trigger)
        .attribute("aria-controls")
        .await
        .is_equal_to(menu_id);
    let trigger_id = trigger.attr("id").await?;
    assert_that!(menu)
        .attribute("aria-labelledby")
        .await
        .is_equal_to(trigger_id);

    assert_that!(menu.inner_texts("[role=menuitem]").await?).contains_exactly(ACTIONS);
    for action in ACTIONS {
        let item = item(page, action).await?;
        if action == "Paste" {
            assert_that!(item)
                .has_attribute("aria-disabled")
                .await
                .is_equal_to("true");
        } else {
            assert_that!(item)
                .with_detail_message(format!("menu item {action:?}"))
                .attribute("aria-disabled")
                .await
                .is_none();
        }
    }

    close_with_escape(page).await?;
    actions_log(page)
        .await?
        .inner_text_stays("", std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// Opening the menu with ArrowDown, Enter or Space focuses its first item, opening it with ArrowUp
/// its last item.
#[browser_test]
pub async fn keyboard_opening_focuses_first_or_last_item(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    for (key, name, expected) in [
        (Key::Down.into(), "ArrowDown", "Copy"),
        (Key::Up.into(), "ArrowUp", "Delete"),
        (Key::Enter.into(), "Enter", "Copy"),
        (TypingData::from(" "), "Space", "Copy"),
    ] {
        open_with_key(page, key)
            .await
            .context_with(|| format!("opening with {name}"))?;
        page.wait_for_focus(&item(page, expected).await?)
            .await
            .context_with(|| format!("after opening with {name}"))?;
        close_with_escape(page).await?;
    }
    actions_log(page)
        .await?
        .inner_text_stays("", std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// Arrow keys, Home and End move the focus, skipping the disabled item and wrapping at the ends
/// ("$Name wraps focus from first to last/last to first item if up/down arrow is pressed if
/// shouldFocusWrap is true").
#[browser_test]
pub async fn keyboard_navigation_skips_disabled_items(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    open_with_key(page, Key::Down).await?;
    page.wait_for_focus(&item(page, "Copy").await?).await?;
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
        page.wait_for_focus(&item(page, expected).await?)
            .await
            .context_with(|| format!("after pressing {key:?}"))?;
    }
    close_with_escape(page).await?;
    Ok(())
}

/// Enter and Space on an item perform its action, close the menu and return the focus to the
/// trigger.
#[browser_test]
pub async fn keyboard_activation_closes_and_restores_focus(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let actions = actions_log(page).await?;
    open_with_key(page, Key::Down).await?;
    page.wait_for_focus(&item(page, "Copy").await?).await?;
    page.send_keys(Key::Down).await?;
    page.wait_for_focus(&item(page, "Cut").await?).await?;
    page.send_keys(Key::Enter).await?;
    actions
        .wait_for_inner_text("Cut")
        .await
        .context("after pressing Enter on Cut")?;
    expect_closed_with_focus_on_trigger(page)
        .await
        .context("after pressing Enter on Cut")?;

    open_with_key(page, Key::Down).await?;
    page.wait_for_focus(&item(page, "Copy").await?).await?;
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

/// Typing focuses the next item whose text starts with the typed characters, searching from the
/// focused item and wrapping around ("$Name supports focusing items by typing letters in rapid
/// succession", "$Name wraps around when no items past the current one match").
#[browser_test]
pub async fn type_ahead(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    open_with_key(page, Key::Down).await?;
    page.wait_for_focus(&item(page, "Copy").await?).await?;
    page.send_keys("d").await?;
    page.wait_for_focus(&item(page, "Delete").await?)
        .await
        .context("after typing \"d\"")?;

    // "dx" matches nothing: the search ends (react-aria's `useTypeSelect`). Characters typed in
    // quick succession form one search string; searching starts at the focused item and wraps
    // around.
    page.send_keys("x").await?;
    page.focus_stays(
        &item(page, "Delete").await?,
        std::time::Duration::from_millis(100),
    )
    .await?;
    page.send_keys("cu").await?;
    page.wait_for_focus(&item(page, "Cut").await?)
        .await
        .context("after typing \"cu\"")?;
    close_with_escape(page).await?;
    Ok(())
}

/// Clicking an item performs its action, closes the menu and returns the focus to the trigger,
/// while clicking a disabled item does nothing ("$Name closes on menu item selection if toggled by
/// mouse click").
#[browser_test]
pub async fn clicking_an_item(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let trigger = page
        .element(css("button[aria-haspopup]").text("Actions"))
        .await?;
    let actions = actions_log(page).await?;
    trigger.click().await?;
    page.element(MENU).await?;
    trigger.wait_for_attr("aria-expanded", Some("true")).await?;

    item(page, "Paste").await?.click().await?;
    actions
        .inner_text_stays("", std::time::Duration::from_millis(100))
        .await?;
    assert_that!(page.count(MENU).await?)
        .with_detail_message("the menu stays open after clicking a disabled item")
        .is_equal_to(1);

    item(page, "Delete").await?.click().await?;
    actions.wait_for_inner_text("Delete").await?;
    expect_closed_with_focus_on_trigger(page)
        .await
        .context("after clicking Delete")?;
    Ok(())
}

/// Opening the menu with the mouse focuses the menu itself rather than an item, and ArrowDown then
/// focuses the first item.
#[browser_test]
pub async fn mouse_opening_focuses_the_menu(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element(css("button[aria-haspopup]").text("Actions"))
        .await?
        .click()
        .await?;
    page.wait_for_focus(&page.element(MENU).await?).await?;
    // ArrowDown then focuses the first item.
    page.send_keys(Key::Down).await?;
    page.wait_for_focus(&item(page, "Copy").await?).await?;
    Ok(())
}

/// Typing the first letter of only a disabled item ("p" for Paste) leaves the focus where it is.
#[browser_test]
pub async fn type_ahead_skips_disabled_items(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    open_with_key(page, Key::Down).await?;
    let copy = item(page, "Copy").await?;
    page.wait_for_focus(&copy).await?;
    page.send_keys("p").await?;
    page.focus_stays(&copy, std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// A multiple-selection menu has unchecked `menuitemcheckbox` items in labelled sections. Clicking
/// or Space toggles an item and keeps the menu open, Enter toggles and closes it ("$Name supports
/// selecting multiple items").
#[browser_test]
pub async fn selection_menu(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let view = page.element(role(AriaRole::Button).text("View")).await?;
    let selection = page.element("#test-menu-view-selection").await?;
    view.click().await?;
    let menu = page.element(MENU).await?;

    let items = menu.elements("[role=menuitemcheckbox]").await?;
    assert_that!(&items).has_length(3);
    for item in &items {
        assert_that!(item)
            .has_attribute("aria-checked")
            .await
            .is_equal_to("false");
    }
    let groups = menu.elements("[role=group]").await?;
    let mut headings = Vec::new();
    for group in &groups {
        headings.push(group.accessible_name().await?);
    }
    assert_that!(headings).contains_exactly(["Panels", "Zoom"]);

    let sidebar = page
        .element(role(AriaRole::Menuitemcheckbox).text("Sidebar"))
        .await?;
    sidebar.click().await?;
    selection.wait_for_inner_text("Sidebar").await?;
    assert_that!(sidebar)
        .has_attribute("aria-checked")
        .await
        .is_equal_to("true");
    assert_that!(page.count(MENU).await?)
        .with_detail_message("clicking an item of a multiple selection menu keeps it open")
        .is_equal_to(1);

    page.send_keys(Key::Down).await?;
    page.wait_for_focus(
        &page
            .element(role(AriaRole::Menuitemcheckbox).text("Toolbar"))
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
