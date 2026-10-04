pub mod test_button;
pub mod test_focus;
pub mod test_focus_manager;
pub mod test_focus_ring;
pub mod test_focus_scope;
pub mod test_focus_visible;
pub mod test_focus_within;
pub mod test_focusable;
pub mod test_has_tabbable_child;
pub mod test_hydration_ids;
pub mod test_listbox;
pub mod test_live_announcer;
pub mod test_menu_trigger;
pub mod test_number_field;
pub mod test_press;
pub mod test_select;
pub mod test_server_panics;
pub mod test_use_button;

use browser_test::BrowserTests;

/// Every browser test. Register new tests here.
///
/// With `BROWSER_TEST_KNOWN_ISSUES=1`, only the checks for known, not yet fixed bugs run instead.
pub fn all() -> BrowserTests<str> {
    if std::env::var("BROWSER_TEST_KNOWN_ISSUES").is_ok_and(|v| v == "1") {
        return BrowserTests::new()
            .with(test_listbox::ListBoxKnownIssues {})
            .with(test_select::SelectKnownIssues {});
    }
    BrowserTests::new()
        .with(test_button::ButtonTests {})
        .with(test_focus::FocusTests {})
        .with(test_focus_within::FocusWithinTests {})
        .with(test_focus_ring::FocusRingTests {})
        .with(test_focusable::FocusableTests {})
        .with(test_focus_manager::FocusManagerTests {})
        .with(test_focus_visible::FocusVisibleTests {})
        .with(test_has_tabbable_child::HasTabbableChildTests {})
        .with(test_focus_scope::FocusScopeTests {})
        .with(test_press::PressTests {})
        .with(test_use_button::UseButtonTests {})
        .with(test_menu_trigger::MenuTriggerTests {})
        .with(test_number_field::NumberFieldTests {})
        .with(test_live_announcer::LiveAnnouncerTests {})
        .with(test_listbox::ListBoxTests {})
        .with(test_select::SelectTests {})
        .with(test_hydration_ids::HydrationIdTests {})
        // Keep last: checks that none of the pages above made the server panic.
        .with(test_server_panics::ServerPanicTests {})
}
