#[macro_use]
mod polling;
pub mod test_aria_hide_outside;
pub mod test_breadcrumbs;
pub mod test_button;
pub mod test_calendar;
pub mod test_checkbox;
pub mod test_clipboard;
pub mod test_clipboard_write;
pub mod test_color_area;
pub mod test_color_field;
pub mod test_color_picker;
pub mod test_color_slider;
pub mod test_color_swatch;
pub mod test_color_wheel;
pub mod test_combobox;
pub mod test_combobox_forms;
pub mod test_context_menu;
pub mod test_context_menu_atoms;
pub mod test_date_field;
pub mod test_date_picker;
pub mod test_dialog;
pub mod test_disclosure;
pub mod test_dismiss_button;
pub mod test_dnd;
pub mod test_dnd_collection;
pub mod test_focus;
pub mod test_focus_manager;
pub mod test_focus_ring;
pub mod test_focus_safely;
pub mod test_focus_scope;
pub mod test_focus_visible;
pub mod test_focus_within;
pub mod test_focusable;
pub mod test_focusable_atoms;
pub mod test_forms;
pub mod test_global_shortcuts;
pub mod test_grid;
pub mod test_grid_list;
pub mod test_grid_list_features;
pub mod test_has_tabbable_child;
pub mod test_hover;
pub mod test_hydration_ids;
pub mod test_interact_outside;
pub mod test_keyboard;
pub mod test_label_slots;
pub mod test_landmark;
pub mod test_link;
pub mod test_listbox;
pub mod test_listbox_features;
pub mod test_live_announcer;
pub mod test_long_press;
pub mod test_menu;
pub mod test_menu_atoms;
pub mod test_menu_trigger;
pub mod test_move;
pub mod test_number_field;
pub mod test_number_field_atoms;
pub mod test_overlay;
pub mod test_overlay_position;
pub mod test_popover;
pub mod test_press;
pub mod test_pressable;
pub mod test_progress_bar;
pub mod test_radio_group;
pub mod test_scroll;
pub mod test_search_field;
pub mod test_select;
pub mod test_select_forms;
pub mod test_separator;
pub mod test_server_panics;
pub mod test_slider;
pub mod test_spin_button;
pub mod test_submenu;
pub mod test_switch;
pub mod test_table;
pub mod test_table_navigation;
pub mod test_table_resizing;
pub mod test_table_selection;
pub mod test_tabs;
pub mod test_tag_group;
pub mod test_tag_group_atoms;
pub mod test_text_field;
pub mod test_text_field_atoms;
pub mod test_theme;
pub mod test_toast;
pub mod test_toggle_button;
pub mod test_toolbar;
pub mod test_tooltip;
pub mod test_tree;
pub mod test_use_button;
pub mod test_virtual_list;
pub mod test_virtualizer;
pub mod test_visually_hidden;

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
/// With `BROWSER_TEST_FILTER=<text>`, only the tests whose name contains `<text>` run (several
/// texts separated by commas: any of them).
fn ui_tests(group: BrowserTests<str>) -> BrowserTests<str> {
    let tests = Selected::new(group);
    if std::env::var("BROWSER_TEST_KNOWN_ISSUES").is_ok_and(|v| v == "1") {
        // Register `*KnownIssues` tests here.
        return tests
            .with(test_virtual_list::ComponentSpreadRebuildKnownIssues {})
            .tests;
    }
    tests
        // First: the slowest test (~16s), so that it overlaps with the others in parallel runs.
        .with(test_table_resizing::TableResizingTests {})
        .with(test_button::ButtonTests {})
        .with(test_focus::FocusTests {})
        .with(test_focus_within::FocusWithinTests {})
        .with(test_focus_ring::FocusRingTests {})
        .with(test_focus_safely::FocusSafelyTests {})
        .with(test_scroll::ScrollTests {})
        .with(test_hover::HoverTests {})
        .with(test_move::MoveTests {})
        .with(test_keyboard::KeyboardTests {})
        .with(test_color_area::ColorAreaTests {})
        .with(test_color_area::ColorAreaSpacesTests {})
        .with(test_color_field::ColorFieldTests {})
        .with(test_color_picker::ColorPickerTests {})
        .with(test_color_slider::ColorSliderTests {})
        .with(test_color_slider::ColorSliderDragTests {})
        .with(test_color_swatch::ColorSwatchTests {})
        .with(test_color_wheel::ColorWheelTests {})
        .with(test_color_wheel::ColorWheelInteractionTests {})
        .with(test_label_slots::LabelSlotsTests {})
        .with(test_context_menu::ContextMenuTests {})
        .with(test_interact_outside::InteractOutsideTests {})
        .with(test_long_press::LongPressTests {})
        .with(test_focusable_atoms::FocusableAtomTests {})
        .with(test_focusable::FocusableTests {})
        .with(test_focus_manager::FocusManagerTests {})
        .with(test_focus_visible::FocusVisibleTests {})
        .with(test_has_tabbable_child::HasTabbableChildTests {})
        .with(test_focus_scope::FocusScopeTests {})
        .with(test_press::PressTests {})
        .with(test_press::PressMacTests {})
        .with(test_use_button::UseButtonTests {})
        .with(test_menu_trigger::MenuTriggerTests {})
        .with(test_number_field::NumberFieldTests {})
        .with(test_number_field_atoms::NumberFieldAtomTests {})
        .with(test_live_announcer::LiveAnnouncerTests {})
        .with(test_listbox::ListBoxTests {})
        .with(test_listbox_features::ListBoxSectionsTests {})
        .with(test_listbox_features::ListBoxReplaceSelectionTests {})
        .with(test_listbox_features::ListBoxActionsAndLinksTests {})
        .with(test_listbox_features::ListBoxLayoutTests {})
        .with(test_listbox_features::ListBoxDisabledAndEmptyTests {})
        .with(test_select::SelectTests {})
        .with(test_select_forms::SelectMultipleTests {})
        .with(test_select_forms::SelectValidationTests {})
        .with(test_select_forms::SelectEmptyAndManyTests {})
        .with(test_menu::MenuTests {})
        .with(test_menu_atoms::MenuAtomTests {})
        .with(test_grid::GridTests {})
        .with(test_grid_list::GridListTests {})
        .with(test_grid_list_features::GridListChildNavigationTests {})
        .with(test_grid_list_features::GridListActionsTests {})
        .with(test_table::TableTests {})
        .with(test_table_navigation::TableNavigationTests {})
        .with(test_table_selection::TableSelectionTests {})
        .with(test_tabs::TabsTests {})
        .with(test_calendar::CalendarTests {})
        .with(test_calendar::RangeCalendarTouchTests {})
        .with(test_calendar::CalendarTodayTests {})
        .with(test_calendar::CalendarViewTests {})
        .with(test_date_field::DateFieldTests {})
        .with(test_date_picker::DatePickerTests {})
        .with(test_slider::SliderTests {})
        .with(test_slider::SliderTrackTests {})
        .with(test_slider::SliderThumbTests {})
        .with(test_slider::SliderMultipleThumbsTests {})
        .with(test_dnd::DndTests {})
        .with(test_dnd::DndKeyboardNavigationTests {})
        .with(test_dnd::DndChangingTargetsTests {})
        .with(test_dnd::DndOperationsTests {})
        .with(test_dnd::DndScreenReaderTests {})
        .with(test_dnd_collection::DndCollectionTests {})
        .with(test_dnd_collection::DndCollectionTargetTests {})
        .with(test_clipboard::ClipboardTests {})
        .with(test_clipboard_write::ClipboardWriteTests {})
        .with(test_text_field::TextFieldTests {})
        .with(test_text_field_atoms::TextFieldAtomTests {})
        .with(test_search_field::SearchFieldTests {})
        .with(test_combobox::ComboBoxTests {})
        .with(test_combobox_forms::ComboBoxCustomValueTests {})
        .with(test_combobox_forms::ComboBoxValidationTests {})
        .with(test_combobox_forms::ComboBoxMultipleTests {})
        .with(test_combobox_forms::ComboBoxMenuTriggerTests {})
        .with(test_combobox_forms::ComboBoxSectionsTests {})
        .with(test_checkbox::CheckboxTests {})
        .with(test_forms::FormsTests {})
        .with(test_radio_group::RadioGroupTests {})
        .with(test_switch::SwitchTests {})
        .with(test_toggle_button::ToggleButtonTests {})
        .with(test_aria_hide_outside::AriaHideOutsideTests {})
        .with(test_dialog::DialogTests {})
        .with(test_toolbar::ToolbarTests {})
        .with(test_progress_bar::ProgressBarTests {})
        .with(test_pressable::PressableTests {})
        .with(test_spin_button::SpinButtonTests {})
        .with(test_submenu::SubmenuTests {})
        .with(test_link::LinkTests {})
        .with(test_breadcrumbs::BreadcrumbsTests {})
        .with(test_disclosure::DisclosureTests {})
        .with(test_popover::PopoverTests {})
        .with(test_dismiss_button::DismissButtonTests {})
        .with(test_overlay::OverlayTests {})
        .with(test_overlay_position::OverlayPositionTests {})
        .with(test_context_menu_atoms::ContextMenuAtomsTests {})
        .with(test_global_shortcuts::GlobalShortcutsTests {})
        .with(test_landmark::LandmarkTests {})
        .with(test_toast::ToastTests {})
        .with(test_tooltip::TooltipTests {})
        .with(test_tag_group_atoms::TagGroupAtomTests {})
        .with(test_virtual_list::VirtualListTests {})
        .with(test_virtual_list::VirtualListRebuildTests {})
        .with(test_virtual_list::VirtualListFollowToggleTests {})
        .with(test_virtualizer::VirtualizerTests {})
        .with(test_tag_group::TagGroupTests {})
        .with(test_tree::TreeTests {})
        .with(test_tree::TreeCasesTests {})
        .with(test_visually_hidden::VisuallyHiddenTests {})
        .with(test_separator::SeparatorTests {})
        .with(test_theme::ThemeTests {})
        .with(test_hydration_ids::HydrationIdTests {
            shard: 0,
            shards: 4,
        })
        .with(test_hydration_ids::HydrationIdTests {
            shard: 1,
            shards: 4,
        })
        .with(test_hydration_ids::HydrationIdTests {
            shard: 2,
            shards: 4,
        })
        .with(test_hydration_ids::HydrationIdTests {
            shard: 3,
            shards: 4,
        })
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
            .is_none_or(|filter| filter.split(',').any(|part| test.name().contains(part)))
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
