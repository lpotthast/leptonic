pub mod test_button;
pub mod test_checkbox;
pub mod test_combobox;
pub mod test_dialog;
pub mod test_dnd;
pub mod test_focus;
pub mod test_focus_manager;
pub mod test_focus_ring;
pub mod test_focus_scope;
pub mod test_focus_visible;
pub mod test_focus_within;
pub mod test_focusable;
pub mod test_grid;
pub mod test_grid_list;
pub mod test_has_tabbable_child;
pub mod test_hydration_ids;
pub mod test_listbox;
pub mod test_live_announcer;
pub mod test_menu;
pub mod test_menu_atoms;
pub mod test_menu_trigger;
pub mod test_number_field;
pub mod test_number_field_atom;
pub mod test_popover;
pub mod test_press;
pub mod test_radio_group;
pub mod test_search_field;
pub mod test_select;
pub mod test_server_panics;
pub mod test_slider;
pub mod test_switch;
pub mod test_table;
pub mod test_table_resizing;
pub mod test_tabs;
pub mod test_tag_group;
pub mod test_text_field;
pub mod test_text_field_atom;
pub mod test_text_field_components;
pub mod test_toggle_button;
pub mod test_tooltip;
pub mod test_tree;
pub mod test_use_button;

use std::borrow::Cow;

use browser_test::{
    BrowserTest, BrowserTests, ElementQueryWait, Parallelism, async_trait, thirtyfour::WebDriver,
};
use leptos_browser_test::Report;

use crate::pages::{BaseActions, Page};

/// Every browser test: the UI tests at `parallelism` at once, then the checks of the whole run
/// ([`after_all`]).
pub fn all(parallelism: Parallelism) -> BrowserTests<str> {
    BrowserTests::sequential()
        .with_group(ui_tests(BrowserTests::parallel(parallelism)))
        .with_group(after_all(
            BrowserTests::sequential().named("after all").run_always(),
        ))
}

/// The UI tests, each loading its own page. Register new tests here.
///
/// With `BROWSER_TEST_KNOWN_ISSUES=1`, only the checks for known, not yet fixed bugs run instead.
/// With `BROWSER_TEST_FILTER=<text>`, only the tests whose name contains `<text>` run.
fn ui_tests(group: BrowserTests<str>) -> BrowserTests<str> {
    let tests = Selected::new(group);
    if std::env::var("BROWSER_TEST_KNOWN_ISSUES").is_ok_and(|v| v == "1") {
        // No known issues at the moment. Register `*KnownIssues` tests here.
        return tests.tests;
    }
    tests
        // First: the slowest test (~16s), so that it overlaps with the others in parallel runs.
        .with(test_table_resizing::TableResizingTests {})
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
        .with(test_number_field_atom::NumberFieldAtomTests {})
        .with(test_live_announcer::LiveAnnouncerTests {})
        .with(test_listbox::ListBoxTests {})
        .with(test_select::SelectTests {})
        .with(test_menu::MenuTests {})
        .with(test_menu_atoms::MenuAtomTests {})
        .with(test_grid::GridTests {})
        .with(test_grid_list::GridListTests {})
        .with(test_table::TableTests {})
        .with(test_tabs::TabsTests {})
        .with(test_slider::SliderTests {})
        .with(test_dnd::DndTests {})
        .with(test_text_field::TextFieldTests {})
        .with(test_text_field_atom::TextFieldAtomTests {})
        .with(test_search_field::SearchFieldTests {})
        .with(test_text_field_components::TextFieldComponentTests {})
        .with(test_combobox::ComboBoxTests {})
        .with(test_checkbox::CheckboxTests {})
        .with(test_radio_group::RadioGroupTests {})
        .with(test_switch::SwitchTests {})
        .with(test_toggle_button::ToggleButtonTests {})
        .with(test_dialog::DialogTests {})
        .with(test_popover::PopoverTests {})
        .with(test_tooltip::TooltipTests {})
        .with(test_tag_group::TagGroupTests {})
        .with(test_tree::TreeTests {})
        .with(test_hydration_ids::HydrationIdTests {})
        .tests
}

/// The checks of the whole run, after the UI tests (they must not overlap with other tests). They
/// run even when a UI test failed.
fn after_all(group: BrowserTests<str>) -> BrowserTests<str> {
    let tests = Selected::new(group);
    if std::env::var("BROWSER_TEST_KNOWN_ISSUES").is_ok_and(|v| v == "1") {
        return tests.tests;
    }
    // Checks that none of the pages visited before made the server panic.
    tests.with(test_server_panics::ServerPanicTests {}).tests
}

/// The tests matching `BROWSER_TEST_FILTER`.
struct Selected {
    tests: BrowserTests<str>,
    filter: Option<String>,
}

impl Selected {
    fn new(tests: BrowserTests<str>) -> Self {
        Self {
            tests,
            filter: std::env::var("BROWSER_TEST_FILTER").ok(),
        }
    }

    fn with(mut self, test: impl BrowserTest<str> + 'static) -> Self {
        if self
            .filter
            .as_deref()
            .is_none_or(|filter| test.name().contains(filter))
        {
            self.tests = self.tests.with(CheckPageErrors(test));
        }
        self
    }
}

/// Runs a test, then fails it if the page reported uncaught errors (see
/// `BaseActions::expect_no_page_errors`; `goto_path` checks the page it leaves).
struct CheckPageErrors<T>(T);

#[async_trait]
impl<T: BrowserTest<str>> BrowserTest<str> for CheckPageErrors<T> {
    fn name(&self) -> Cow<'_, str> {
        self.0.name()
    }

    fn timeouts(&self) -> Option<browser_test::Timeouts> {
        self.0.timeouts()
    }

    fn element_query_wait(&self) -> Option<ElementQueryWait> {
        self.0.element_query_wait()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        self.0.run(driver, base_url).await?;
        Page { driver, base_url }.expect_no_page_errors().await
    }
}
