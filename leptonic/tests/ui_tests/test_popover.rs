// Upstream: react-aria-components/test/Popover.test.js @ 99e6102368
// Upstream: react-aria-components/test/Dialog.test.js @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{BrowserTest, async_trait, thirtyfour::prelude::*};
use rootcause::Report;

use crate::pages::{ElementActions, Page, PageActions, SyntheticEvent, role};

/// `DialogTrigger` + `Popover` atoms: the trigger opens a dialog in the popover and controls it,
/// focus moves into the dialog and back, Escape and outside clicks close it, presses inside don't
/// toggle the trigger, and a non-modal popover closes when focus moves out (it contains the focus
/// only while a dialog is inside, decided per opening). A popover keeps the direction of the
/// subtree its trigger is in.
pub struct PopoverTests {}

#[async_trait]
impl BrowserTest<str> for PopoverTests {
    fn name(&self) -> Cow<'_, str> {
        "popover_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/popover").await?;

        cases!(
            trigger_controls_the_dialog(&page),
            outside_click_closes(&page),
            non_modal_contains_focus_with_a_dialog(&page),
            trigger_names_an_untitled_dialog(&page),
            standalone_popover_is_the_dialog(&page),
            animated(&page),
            scrolling(&page),
            containment_per_opening(&page),
            direction(&page),
        );

        Ok(())
    }
}

const DIALOG: &str = "[role=dialog]";

/// Works with a dialog: the trigger controls the dialog, which its title names (Dialog.test.js)
/// and which takes the focus. Pressing a button inside doesn't toggle the popover through the
/// trigger; Escape closes it and focus returns to the trigger.
async fn trigger_controls_the_dialog(page: &Page<'_>) -> Result<(), Report> {
    let trigger = page.element("#test-popover-trigger").await?;
    assert_that!(trigger.attr("aria-expanded").await?)
        .get_some()
        .is_equal_to("false");
    assert_that!(page.count(DIALOG).await?).is_equal_to(0);

    trigger.click().await?;
    let dialog = page.element(DIALOG).await?;
    let dialog_id = dialog.id().await?;
    assert_that!(trigger.attr("aria-controls").await?).is_equal_to(dialog_id);
    assert_that!(trigger.attr("aria-expanded").await?)
        .get_some()
        .is_equal_to("true");
    page.element("[data-placement]").await?;
    let title_id = dialog.element("h2").await?.id().await?;
    assert_that!(dialog.attr("aria-labelledby").await?).is_equal_to(title_id);
    page.wait_for_focus(&dialog).await?;

    page.element("#test-popover-inner").await?.click().await?;
    page.element("#test-popover-inner-presses")
        .await?
        .wait_for_inner_text("1")
        .await?;
    assert_that!(page.count(DIALOG).await?).is_equal_to(1);

    page.send_keys(Key::Escape).await?;
    page.wait_for_count(DIALOG, 0).await?;
    page.wait_for_focus(&trigger).await?;
    trigger
        .wait_for_attr("aria-expanded", Some("false"))
        .await?;
    Ok(())
}

/// A click outside a modal popover (on its underlay, which covers the page) closes it.
async fn outside_click_closes(page: &Page<'_>) -> Result<(), Report> {
    let trigger = page.element("#test-popover-trigger").await?;
    trigger.click().await?;
    page.element(DIALOG).await?;
    page.driver
        .action_chain()
        .move_to(5, 5)
        .click()
        .perform()
        .await?;
    page.wait_for_count(DIALOG, 0).await?;
    page.wait_for_focus(&trigger).await?;
    Ok(())
}

/// A non-modal popover leaves the page usable and closes when focus moves out. With a dialog
/// inside, it contains focus (react-aria's `useOverlayFocusContain`): Tab wraps around instead of
/// leaving (which would close it).
async fn non_modal_contains_focus_with_a_dialog(page: &Page<'_>) -> Result<(), Report> {
    page.element("#test-popover-non-modal-trigger")
        .await?
        .click()
        .await?;
    let info = page.element("[role=dialog][aria-label=Info]").await?;
    page.wait_for_focus(&info).await?;
    let first = page.element("#test-popover-non-modal-first").await?;
    let second = page.element("#test-popover-non-modal-second").await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&first).await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&second).await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&first).await?;

    let outside = page.element("#test-popover-outside").await?;
    outside.click().await?;
    page.wait_for_count(DIALOG, 0).await?;
    page.wait_for_focus(&outside).await?;
    Ok(())
}

/// "should get default aria label from trigger": without a title, the trigger names the dialog;
/// the trigger gets an id for it.
async fn trigger_names_an_untitled_dialog(page: &Page<'_>) -> Result<(), Report> {
    let untitled = page.element(role("button").text("Untitled")).await?;
    untitled.click().await?;
    let dialog = page.element(DIALOG).await?;
    let untitled_id = untitled.id().await?;
    assert_that!(untitled_id.as_deref())
        .get_some()
        .is_not_empty();
    assert_that!(dialog.attr("aria-labelledby").await?).is_equal_to(untitled_id);
    page.send_keys(Key::Escape).await?;
    page.wait_for_count(DIALOG, 0).await?;
    Ok(())
}

/// "applies overlay id to standalone popover", "should handle focus": without a dialog inside,
/// the modal popover is the dialog: the trigger controls it, it is focused and named by the
/// trigger, and focus returns to the trigger when it closes.
async fn standalone_popover_is_the_dialog(page: &Page<'_>) -> Result<(), Report> {
    let standalone = page.element("#test-popover-standalone-trigger").await?;
    standalone.click().await?;
    let dialog = page.element(DIALOG).await?;
    let dialog_id = dialog.id().await?;
    assert_that!(standalone.attr("aria-controls").await?).is_equal_to(dialog_id);
    assert_that!(dialog.attr("aria-labelledby").await?)
        .get_some()
        .is_equal_to("test-popover-standalone-trigger");
    assert_that!(dialog.inner_text().await?).is_equal_to("Standalone content");
    page.wait_for_focus(&dialog).await?;
    page.send_keys(Key::Escape).await?;
    page.wait_for_count(DIALOG, 0).await?;
    page.wait_for_focus(&standalone).await?;
    Ok(())
}

/// "supports isEntering and isExiting props", with CSS animations: entering while its animation
/// runs, then exiting (and still rendered) until the exit animation ended.
async fn animated(page: &Page<'_>) -> Result<(), Report> {
    let trigger = page.element("#test-popover-animated-trigger").await?;
    trigger.click().await?;
    page.element(".test-animated-popover[data-entering]")
        .await?;
    page.element(".test-animated-popover:not([data-entering])")
        .await?;
    page.send_keys(Key::Escape).await?;
    page.element(".test-animated-popover[data-exiting]").await?;
    page.wait_for_count(".test-animated-popover", 0).await?;
    page.wait_for_focus(&trigger).await?;
    Ok(())
}

/// useOverlayPosition.test.tsx: a non-modal popover stays open when an adjacent region scrolls
/// ("should not close the overlay when an adjacent scrollable region scrolls"), and closes when
/// the page scrolls ("should close the overlay when the body scrolls"). `scroll` events don't
/// bubble.
async fn scrolling(page: &Page<'_>) -> Result<(), Report> {
    page.element("#test-popover-non-modal-trigger")
        .await?
        .click()
        .await?;
    page.element(DIALOG).await?;
    page.element("#test-popover-adjacent-scroll")
        .await?
        .dispatch(SyntheticEvent::plain("scroll").with("bubbles", false))
        .await?;
    page.count_stays(DIALOG, 1).await?;
    page.element("body")
        .await?
        .dispatch(SyntheticEvent::plain("scroll").with("bubbles", false))
        .await?;
    page.wait_for_count(DIALOG, 0).await?;
    Ok(())
}

/// A non-modal popover contains the focus while a dialog is inside; reopened without one, it
/// doesn't (the containment starts over with each opening): Shift+Tab leaves it, which closes it.
async fn containment_per_opening(page: &Page<'_>) -> Result<(), Report> {
    let trigger = page.element("#test-popover-toggled-trigger").await?;
    trigger.click().await?;
    page.element("[role=dialog][aria-label=Toggled]").await?;
    page.element("#test-popover-toggled-first")
        .await?
        .focus()
        .await?;
    // Contained: Shift+Tab wraps around.
    page.send_keys(Key::Shift + Key::Tab).await?;
    page.wait_for_focus(&page.element("#test-popover-toggled-second").await?)
        .await?;
    page.send_keys(Key::Escape).await?;
    page.wait_for_count("#test-popover-toggled-first", 0)
        .await?;

    // Without the dialog.
    page.element("#test-popover-with-dialog")
        .await?
        .click()
        .await?;
    trigger.click().await?;
    page.element("#test-popover-toggled-second").await?;
    assert_that!(page.count(DIALOG).await?).is_equal_to(0);
    let first = page.element("#test-popover-toggled-first").await?;
    first.focus().await?;
    page.wait_for_focus(&first).await?;
    // Not contained: Shift+Tab moves to the page before the popover, which closes it.
    page.send_keys(Key::Shift + Key::Tab).await?;
    page.wait_for_count("#test-popover-toggled-second", 0)
        .await?;
    Ok(())
}

/// The portalled popover renders `dir` from its trigger's locale (react-aria-components).
async fn direction(page: &Page<'_>) -> Result<(), Report> {
    page.element("#test-popover-rtl-trigger")
        .await?
        .click()
        .await?;
    let popover = page.element(".test-popover-rtl").await?;
    assert_that!(popover.attr("dir").await?)
        .get_some()
        .is_equal_to("rtl");
    page.send_keys(Key::Escape).await?;
    page.wait_for_count(".test-popover-rtl", 0).await?;

    page.element("#test-popover-trigger").await?.click().await?;
    let popover = page.element(".leptonic-Popover").await?;
    assert_that!(popover.attr("dir").await?)
        .get_some()
        .is_equal_to("ltr");
    page.send_keys(Key::Escape).await?;
    page.wait_for_count(".leptonic-Popover", 0).await?;
    Ok(())
}
