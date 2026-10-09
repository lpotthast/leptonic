// Upstream: react-aria/test/menu/useMenuTrigger.test.js @ 99e6102368
//! `useMenuTrigger`: the trigger's ARIA attributes, opening on mouse down (once), and the focus
//! strategy of keyboard opening.
use assertr::{matchers::eq, prelude::*};
use browser_test::{browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::pages::{ElementActions, Page, PointerKind, PointerType, SyntheticEvent};

const PATH: &str = "/hooks/menu-trigger";

const TRIGGER: &str = "[data-testid=trigger]";

/// The closed trigger has `aria-haspopup="true"`, `aria-expanded="false"` and a generated id
/// ("should return default props for menu and menu trigger").
#[browser_test]
pub async fn aria_attributes(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let trigger = page.element(TRIGGER).await?;
    assert_that!(trigger)
        .has_attribute("aria-haspopup")
        .await
        .is_equal_to("true");
    assert_that!(trigger)
        .has_attribute("aria-expanded")
        .await
        .is_equal_to("false");
    assert_that!(trigger)
        .has_attribute("id")
        .await
        .starts_with("menu-trigger-");
    Ok(())
}

/// A mouse click opens the menu on press start without the click closing it again, with no focus
/// strategy so the menu itself is focused ("returns a onPress for the menuTrigger").
#[browser_test]
pub async fn mouse_press_opens_once(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let trigger = page.element(TRIGGER).await?;
    let is_open = page.element("#test-mt-is-open").await?;
    trigger.click().await?;
    is_open.wait_for_inner_text("true").await?;
    is_open
        .inner_text_stays("true", std::time::Duration::from_millis(100))
        .await?;
    assert_that!(trigger)
        .has_attribute("aria-expanded")
        .await
        .is_equal_to("true");
    assert_that!(page.element("#test-mt-strategy").await?)
        .inner_text()
        .await
        .is_equal_to("none");
    Ok(())
}

/// Focuses the trigger by Tab, presses `key` and checks that the menu opens with the focus
/// strategy `expected` and stays open.
async fn opens_by_key(
    page: &Page<'_>,
    key: impl Into<TypingData> + Send,
    expected: &str,
) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let trigger = page.element(TRIGGER).await?;
    let is_open = page.element("#test-mt-is-open").await?;
    page.element("#test-mt-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&trigger).await?;
    page.send_keys(key).await?;
    is_open.wait_for_inner_text("true").await?;
    page.element("#test-mt-strategy")
        .await?
        .wait_for_inner_text(expected)
        .await?;
    is_open
        .inner_text_stays("true", std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// ArrowDown opens the menu with its first item focused.
#[browser_test]
pub async fn arrow_down_opens_on_the_first_item(page: &Page<'_>) -> Result<(), Report> {
    opens_by_key(page, Key::Down, "first").await
}

/// ArrowUp opens the menu with its last item focused.
#[browser_test]
pub async fn arrow_up_opens_on_the_last_item(page: &Page<'_>) -> Result<(), Report> {
    opens_by_key(page, Key::Up, "last").await
}

/// Enter opens the menu with its first item focused.
#[browser_test]
pub async fn enter_opens_on_the_first_item(page: &Page<'_>) -> Result<(), Report> {
    opens_by_key(page, Key::Enter, "first").await
}

/// Space opens the menu with its first item focused.
#[browser_test]
pub async fn space_opens_on_the_first_item(page: &Page<'_>) -> Result<(), Report> {
    opens_by_key(page, " ", "first").await
}

/// A disabled trigger opens the menu neither by a mouse nor by a touch press ("doesn't toggle the
/// menu if isDisabled").
#[browser_test]
pub async fn disabled_trigger_doesnt_open(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let trigger = page.element("[data-testid=disabled-trigger]").await?;
    trigger.click().await?;
    for kind in [PointerKind::Down, PointerKind::Up] {
        trigger
            .dispatch(SyntheticEvent::pointer(kind).pointer_type(PointerType::Touch))
            .await?;
    }
    let is_open = page.element("#test-mt-disabled-is-open").await?;
    page.settle().await?;
    is_open
        .inner_text_stays("false", std::time::Duration::from_millis(100))
        .await?;
    assert_that!(
        page.element("#test-mt-disabled-strategy")
            .await?
            .inner_text()
            .await?
    )
    .is_equal_to("none");
    Ok(())
}

/// A long press trigger opens the menu with its first item focused once the press is held past
/// the long press threshold; a short press doesn't open it.
#[browser_test]
pub async fn long_press_opens_on_the_first_item(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let trigger = page.element("[data-testid=long-press-trigger]").await?;
    let is_open = page.element("#test-mt-long-is-open").await?;
    trigger.click().await?;
    // Past the long press threshold (500 ms).
    page.settle().await?;
    assert_that!(|| is_open.inner_text())
        .consistently_ok()
        .for_at_least(std::time::Duration::from_millis(600))
        .matches(eq("false".to_owned()))
        .await;
    let held = trigger.press_and_hold().await?;
    is_open.wait_for_inner_text("true").await?;
    held.release().await?;
    page.element("#test-mt-long-strategy")
        .await?
        .wait_for_inner_text("first")
        .await?;
    is_open
        .inner_text_stays("true", std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}
