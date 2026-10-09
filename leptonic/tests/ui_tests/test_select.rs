// Upstream: react-aria-components/test/Select.test.js @ 99e6102368
// Upstream: react-aria/test/select/HiddenSelect.test.tsx @ 99e6102368
//! Behavior of the `Select` atoms, asserted on the DOM/ARIA level. Elements are found by role and
//! text, as users perceive them.
use assertr::prelude::*;
use browser_test::{browser_test, thirtyfour::prelude::*};
use leptonic::AriaRole;
use rootcause::Report;

use crate::pages::{ElementActions, Page, css, role};

const PATH: &str = "/atoms/select";

/// The select trigger: the element that opens a listbox.
const TRIGGER: &str = "#test-sel-form [aria-haspopup=listbox]";

/// The hidden native `<select>` that carries the form value.
async fn hidden_select(page: &Page<'_>) -> Result<WebElement, Report> {
    page.element("#test-sel-form select").await
}

/// Focuses the trigger from the keyboard: a click on the button before it, then Tab.
async fn tab_to_trigger(page: &Page<'_>) -> Result<WebElement, Report> {
    let trigger = page.element(TRIGGER).await?;
    page.element("#test-sel-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&trigger).await?;
    Ok(trigger)
}

/// Initially the trigger shows the default value, the hidden native select (skipped by focus
/// walks) holds it, and the listbox is closed and not referenced by `aria-controls` ("should have
/// form value after initial render", "should always add a data attribute
/// data-react-aria-prevent-focus").
#[browser_test]
pub async fn initial_state(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let trigger = page.element(TRIGGER).await?;
    assert_that!(trigger)
        .inner_text()
        .await
        .is_equal_to("Banana");
    assert_that!(hidden_select(page).await?)
        .property("value")
        .await
        .get_some()
        .is_equal_to("Banana");
    // Focus walks skip the hidden select (HiddenSelect.test.tsx, "should always add a data
    // attribute data-react-aria-prevent-focus"; here `data-leptonic-prevent-focus`).
    assert_that!(
        page.count("#test-sel-form [data-leptonic-prevent-focus] select")
            .await?
    )
    .is_equal_to(1);
    assert_that!(trigger)
        .has_attribute("aria-expanded")
        .await
        .is_equal_to("false");
    // `aria-controls` may only reference the listbox while it exists (i.e. while open).
    assert_that!(trigger)
        .attribute("aria-controls")
        .await
        .is_none();
    Ok(())
}

/// Clicking the trigger opens a modal popover (a dialog named like its listbox, dismiss buttons
/// around the options, the rest of the page inert) and focuses the selected option.
#[browser_test]
pub async fn opening_focuses_the_selected_option(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let trigger = page.element(TRIGGER).await?;
    trigger.click().await?;
    let listbox = page.element("[role=listbox]").await?;
    trigger.wait_for_attr("aria-expanded", Some("true")).await?;
    let listbox_id = listbox.id().await?;
    assert_that!(trigger)
        .attribute("aria-controls")
        .await
        .is_equal_to(listbox_id);
    page.wait_for_focus(
        &page
            .element("[role=listbox]")
            .await?
            .element(role(AriaRole::Option).text("Banana"))
            .await?,
    )
    .await?;
    page.element("#test-sel-after:is([inert], [inert] *)")
        .await?;
    assert_that!(page.count("button[aria-label=Dismiss]").await?).is_equal_to(2);
    let listbox_labelledby = assert_that!(listbox)
        .has_attribute("aria-labelledby")
        .await
        .actual()
        .clone();
    let dialog = page.element("[role=dialog]:has([role=listbox])").await?;
    assert_that!(dialog)
        .has_attribute("aria-labelledby")
        .await
        .is_equal_to(listbox_labelledby);
    Ok(())
}

/// Escape closes the popover, leaves nothing inert, returns focus to the trigger and keeps the
/// value.
#[browser_test]
pub async fn escape_closes_and_restores_focus(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let trigger = page.element(TRIGGER).await?;
    trigger.click().await?;
    page.wait_for_focus(
        &page
            .element("[role=listbox]")
            .await?
            .element(role(AriaRole::Option).text("Banana"))
            .await?,
    )
    .await?;
    page.send_keys(Key::Escape).await?;
    trigger
        .wait_for_attr("aria-expanded", Some("false"))
        .await?;
    page.wait_for_count("[role=listbox]", 0).await?;
    page.wait_for_count("[inert]", 0).await?;
    page.wait_for_focus(&trigger).await?;
    assert_that!(trigger)
        .inner_text()
        .await
        .is_equal_to("Banana");
    Ok(())
}

/// Enter opens and focuses the selected option; Escape returns focus to the trigger.
#[browser_test]
pub async fn escape_after_opening_with_the_keyboard(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let trigger = tab_to_trigger(page).await?;
    page.send_keys(Key::Enter).await?;
    page.wait_for_focus(
        &page
            .element("[role=listbox]")
            .await?
            .element(role(AriaRole::Option).text("Banana"))
            .await?,
    )
    .await?;
    page.send_keys(Key::Escape).await?;
    page.wait_for_count("[role=listbox]", 0).await?;
    page.wait_for_focus(&trigger).await?;
    Ok(())
}

/// A select bound to app state returns focus to the trigger after Escape and after a keyboard
/// selection, which it writes to the state; selecting 25 again after the app reset it to 10 is
/// reported as a change.
#[browser_test]
pub async fn bound_select(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let bound = page
        .element("#test-sel-bound [aria-haspopup=listbox]")
        .await?;
    let value = page.element("#test-sel-bound-value").await?;
    let changes = page.element("#test-sel-bound-changes").await?;
    bound.click().await?;
    page.element("[role=option][aria-selected=true]:focus")
        .await?;
    page.send_keys(Key::Escape).await?;
    page.wait_for_count("[role=listbox]", 0).await?;
    page.wait_for_focus(&bound).await?;
    page.send_keys(Key::Enter).await?;
    page.element("[role=option][aria-selected=true]:focus")
        .await?;
    page.send_keys(Key::Down).await?;
    page.send_keys(Key::Enter).await?;
    value.wait_for_inner_text("25").await?;
    page.wait_for_focus(&bound).await?;
    changes.wait_for_inner_text("1").await?;

    page.element("#test-sel-bound-reset").await?.click().await?;
    value.wait_for_inner_text("10").await?;
    bound.click().await?;
    page.element("[role=option][aria-selected=true]:focus")
        .await?;
    page.send_keys(Key::Down).await?;
    page.send_keys(Key::Enter).await?;
    value.wait_for_inner_text("25").await?;
    changes.wait_for_inner_text("2").await?;
    Ok(())
}

/// Picking an option closes the popover, updates the trigger and the form value, and returns
/// focus to the trigger.
#[browser_test]
pub async fn selecting_an_option(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let trigger = page.element(TRIGGER).await?;
    trigger.click().await?;
    page.element("[role=listbox]")
        .await?
        .element(role(AriaRole::Option).text("Durian"))
        .await?
        .click()
        .await?;
    page.element("#test-sel-changes")
        .await?
        .wait_for_inner_text("Durian")
        .await?;
    trigger
        .wait_for_attr("aria-expanded", Some("false"))
        .await?;
    assert_that!(trigger)
        .inner_text()
        .await
        .is_equal_to("Durian");
    assert_that!(hidden_select(page).await?)
        .property("value")
        .await
        .get_some()
        .is_equal_to("Durian");
    page.wait_for_focus(&trigger).await?;
    Ok(())
}

/// On the closed trigger, Right and Left select the next and previous enabled option without
/// opening the popover, and typing selects an option by its text ("can select an option via
/// typeahead").
#[browser_test]
pub async fn trigger_keyboard(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let trigger = tab_to_trigger(page).await?;
    let changes = page.element("#test-sel-changes").await?;
    // From "Banana" over the disabled "Cherry" and back.
    page.send_keys(Key::Right).await?;
    changes.wait_for_inner_text("Durian").await?;
    page.send_keys(Key::Left).await?;
    changes.wait_for_inner_text("Durian | Banana").await?;
    trigger
        .attr_stays(
            "aria-expanded",
            Some("false"),
            std::time::Duration::from_millis(100),
        )
        .await?;

    page.send_keys("e").await?;
    changes
        .wait_for_inner_text("Durian | Banana | Elderberry")
        .await?;
    assert_that!(trigger)
        .inner_text()
        .await
        .is_equal_to("Elderberry");
    Ok(())
}

/// The trigger is labelled by its value and the label; clicking the label focuses the trigger.
#[browser_test]
pub async fn labelling(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let trigger = page.element(TRIGGER).await?;
    let label = page.element(css("span").text("Fruit")).await?;
    let label_id = label.id().await?.unwrap_or_default();
    let value_id = {
        let labelled_by = assert_that!(trigger).has_attribute("aria-labelledby").await;
        let ids = labelled_by
            .derive_owned(|value| value.split(' ').collect::<Vec<_>>())
            .has_length(2);
        ids.derive_owned(|ids| ids[1])
            .is_equal_to(label_id.as_str());
        ids.actual()[0].to_owned()
    };
    let value = page.element(format!("#{value_id}")).await?;
    assert_that!(value).inner_text().await.is_equal_to("Banana");

    page.element("#test-sel-before").await?.click().await?;
    label.click().await?;
    page.wait_for_focus(&trigger).await?;
    Ok(())
}

/// Resetting the form restores the default value.
#[browser_test]
pub async fn form_reset(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let trigger = page.element(TRIGGER).await?;
    trigger.click().await?;
    page.element("[role=listbox]")
        .await?
        .element(role(AriaRole::Option).text("Durian"))
        .await?
        .click()
        .await?;
    trigger.wait_for_inner_text("Durian").await?;
    page.element("#test-sel-form").await?.reset().await?;
    trigger.wait_for_inner_text("Banana").await?;
    assert_that!(hidden_select(page).await?)
        .property("value")
        .await
        .get_some()
        .is_equal_to("Banana");
    Ok(())
}

/// Leptonic's composition contract: each misplaced Select part warns and renders nothing,
/// even when other Select parents exist elsewhere on the page.
#[browser_test]
pub async fn parts_without_parent_warn_and_render_nothing(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-sel-mount-orphans")
        .await?
        .click()
        .await?;
    let orphans = page.element("#test-sel-orphans").await?;
    orphans.wait_for_attr("data-mounted", Some("true")).await?;
    for part in [
        "SelectTrigger",
        "SelectValue",
        "SelectPopover",
        "HiddenSelect",
    ] {
        crate::fixtures::take_warnings(page, &format!("{part}: not inside a Select"), 1).await?;
    }
    assert_that!(orphans.count("*").await?).is_equal_to(0);
    assert_that!(orphans).inner_text().await.is_empty();
    Ok(())
}
