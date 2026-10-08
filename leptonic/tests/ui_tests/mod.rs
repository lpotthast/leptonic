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
pub mod test_localized_atoms;
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
pub mod test_table_tree;
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

use std::{borrow::Cow, panic::AssertUnwindSafe};

use browser_test::{
    BrowserTest, BrowserTests, ElementQueryWait, Parallelism, async_trait, thirtyfour::WebDriver,
};
use futures::FutureExt as _;
use rootcause::{Report, prelude::ResultExt};

use crate::{
    cases::{Case, CaseFn},
    pages::{Page, PageActions},
};

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
        // Register the cases of known, not yet fixed bugs here (and only here).
        return tests
            .case(test_virtual_list::rebuilt_component_spread_drops_the_old_handlers)
            .case(test_tree::expand_button)
            .tests;
    }
    tests
        .case(test_table_resizing::initial_widths)
        .case(test_table_resizing::resizing_each_column)
        .case(test_table_resizing::cannot_resize_below_the_min_width)
        .case(test_table_resizing::resizing_the_first_column_preserves_fr_ratios)
        .case(test_table_resizing::resizing_the_last_column_locks_the_columns_before_it)
        .case(test_table_resizing::on_resize_start_and_end_without_moving)
        .case(test_table_resizing::keyboard_resizing)
        .case(test_table_resizing::exiting_keyboard_resizing)
        .case(test_button::presses_and_props)
        .case(test_button::state_attributes)
        .case(test_button::pending)
        .case(test_button::pending_form_submission)
        .case(test_button::pending_labelled)
        .case(test_button::pending_trigger)
        .case(test_focus::basic_focus)
        .case(test_focus::tab_focus)
        .case(test_focus::focus_change_count)
        .case(test_focus::child_focus_does_not_trigger_parent)
        .case(test_focus::blur_when_disabled_while_focused)
        .case(test_focus_within::basic_focus_within)
        .case(test_focus_within::disabled)
        .case(test_focus_within::change_callback)
        .case(test_focus_within::tab_into_and_out_of_container)
        .case(test_focus_within::nested_focus_within)
        .case(test_focus_within::focus_outside_after_a_hidden_blur)
        .case(test_focus_within::removal_of_the_focused_child)
        .case(test_focus_within::disabling_the_focused_element)
        .case(test_focus_ring::basic_click_focus)
        .case(test_focus_ring::basic_tab_focus)
        .case(test_focus_ring::within_click_focus)
        .case(test_focus_ring::within_tab_focus)
        .case(test_focus_ring::modality_switch)
        .case(test_focus_ring::arrow_key_keyboard_modality)
        .case(test_focus_ring::disabled_focus_ring)
        .case(test_focus_ring::atom_text_input)
        .case(test_focus_safely::connected)
        .case(test_focus_safely::no_longer_connected)
        .case(test_focus_safely::svg)
        .case(test_scroll::scroll_parents)
        .case(test_scroll::scroll_into_view)
        .case(test_hover::target_is_the_hooked_element)
        .case(test_hover::no_hover_by_touch)
        .case(test_hover::hover_ends_when_disabled)
        .case(test_hover::hover_ends_when_the_element_is_removed)
        .case(test_hover::hoverable_atom)
        .case(test_move::responds_to_pointer_events)
        .case(test_move::ends_with_pointercancel)
        .case(test_move::ignores_right_clicks_and_taps)
        .case(test_move::ignores_additional_pointers)
        .case(test_move::doesnt_bubble_to_a_movable_parent)
        .case(test_move::responds_to_keys)
        .case(test_keyboard::handlers_and_propagation)
        .case(test_keyboard::repeats_and_composing)
        .case(test_keyboard::no_shortcuts_on_keyup)
        .case(test_color_area::input_props)
        .case(test_color_area::keyboard)
        .case(test_color_area::keyboard_steps)
        .case(test_color_area::press_and_drag)
        .case(test_color_area::disabled)
        .case(test_color_area::labelling)
        .case(test_color_area::forms)
        .case(test_color_area::hsv)
        .case(test_color_area::gradients)
        .case(test_color_area::right_to_left)
        .case(test_color_area::input_event)
        .case(test_color_area::thumb_without_alpha)
        .case(test_color_area::mounted_again)
        .case(test_color_field::defaults)
        .case(test_color_field::uncontrolled_state)
        .case(test_color_field::invalid_characters)
        .case(test_color_field::stepping)
        .case(test_color_field::mouse_wheel)
        .case(test_color_field::flags)
        .case(test_color_field::form_reset)
        .case(test_color_field::channel)
        .case(test_color_picker::shared_color)
        .case(test_color_picker::alpha)
        .case(test_color_slider::input_props)
        .case(test_color_slider::hue_value_text_and_label)
        .case(test_color_slider::keyboard)
        .case(test_color_slider::track_click)
        .case(test_color_slider::disabled)
        .case(test_color_slider::forms)
        .case(test_color_slider::default_label)
        .case(test_color_slider::drag_thumb)
        .case(test_color_slider::drag_thumb_vertical)
        .case(test_color_slider::drag_track_vertical)
        .case(test_color_slider::mounted_again)
        .case(test_color_swatch::swatches)
        .case(test_color_swatch::picker_default_value)
        .case(test_color_swatch::picker_keyboard)
        .case(test_color_swatch::picker_disabled_items)
        .case(test_color_swatch::swatch_in_item)
        .case(test_color_wheel::input_props)
        .case(test_color_wheel::keyboard)
        .case(test_color_wheel::ring_press)
        .case(test_color_wheel::disabled)
        .case(test_color_wheel::forms)
        .case(test_color_wheel::drag_thumb)
        .case(test_color_wheel::input_event)
        .case(test_color_wheel::rgb_colors)
        .case(test_color_wheel::mounted_again)
        .case(test_label_slots::no_labels)
        .case(test_label_slots::labels_added)
        .case(test_label_slots::labels_removed)
        .case(test_context_menu::right_click_requests_the_menu)
        .case(test_context_menu::without_a_handler_nothing_happens)
        .case(test_context_menu::ctrl_enter_is_mac_only)
        .case(test_interact_outside::pointer_events)
        .case(test_interact_outside::left_button_only)
        .case(test_interact_outside::disabled)
        .case(test_long_press::long_press)
        .case(test_long_press::cancelled_when_released_early)
        .case(test_long_press::cancels_other_press_events)
        .case(test_long_press::keeps_press_events_when_released_early)
        .case(test_long_press::custom_threshold)
        .case(test_long_press::accessibility_description)
        .case(test_long_press::prevents_context_menu_during_touch)
        .case(test_long_press::no_long_press_by_keyboard)
        .case(test_focusable_atoms::focusable_child)
        .case(test_focusable_atoms::disabled_and_excluded)
        .case(test_focusable_atoms::focusable_tooltip_trigger)
        .case(test_focusable_atoms::merged_props)
        .case(test_focusable_atoms::pressable_tooltip_trigger)
        .case(test_focusable_atoms::auto_focus)
        .case(test_focusable::tabindex_attributes)
        .case(test_focusable::keyboard_events)
        .case(test_focusable::tab_skip)
        .case(test_focusable::focus_handle)
        .case(test_focusable::dynamic_disabled_transition)
        .case(test_focus_manager::basic_navigation)
        .case(test_focus_manager::wrap_next)
        .case(test_focus_manager::wrap_prev)
        .case(test_focus_manager::nowrap_boundary_next)
        .case(test_focus_manager::nowrap_boundary_prev)
        .case(test_focus_manager::tabbable_skip)
        .case(test_focus_manager::nontabbable_include)
        .case(test_focus_manager::accept_filter)
        .case(test_focus_manager::radio_group_checked)
        .case(test_focus_manager::radio_group_none_checked)
        .case(test_focus_manager::radio_group_wrap_next)
        .case(test_focus_manager::radio_group_wrap_prev)
        .case(test_focus_manager::hidden_elements_skipped)
        .case(test_focus_manager::inert_elements_skipped)
        .case(test_focus_manager::focus_next_from_outside_scope)
        .case(test_focus_manager::focus_previous_from_outside_scope)
        .case(test_focus_visible::click_sets_pointer_modality)
        .case(test_focus_visible::tab_sets_keyboard_modality)
        .case(test_focus_visible::arrow_key_sets_keyboard_modality)
        .case(test_focus_visible::typing_on_non_text_input_sets_keyboard_modality)
        .case(test_focus_visible::typing_in_text_input_does_not_set_keyboard_modality)
        .case(test_focus_visible::typing_in_text_input_silently_updates_stored_modality)
        .case(test_focus_visible::pointer_after_keyboard)
        .case(test_focus_visible::escape_sets_keyboard_modality)
        .case(test_focus_visible::enter_sets_keyboard_modality)
        .case(test_focus_visible::space_sets_keyboard_modality)
        .case(test_focus_visible::focus_without_a_preceding_event_is_virtual)
        .case(test_focus_visible::programmatic_focus_keeps_the_modality)
        .case(test_focus_visible::window_refocus_keeps_the_modality)
        .case(test_focus_visible::safari_window_refocus_keeps_the_modality)
        .case(test_focus_visible::focus_moved_after_invalid_shows_focus)
        .case(test_has_tabbable_child::with_tabbable_child)
        .case(test_has_tabbable_child::child_removed_and_re_added)
        .case(test_has_tabbable_child::no_tabbable_children)
        .case(test_has_tabbable_child::deeply_nested_tabbable_child)
        .case(test_has_tabbable_child::child_disabled_attribute_change)
        .case(test_focus_scope::auto_focus)
        .case(test_focus_scope::tab_wrapping)
        .case(test_focus_scope::shift_tab_wrapping)
        .case(test_focus_scope::focus_restoration)
        .case(test_focus_scope::nested_scopes)
        .case(test_focus_scope::containment_blocks_escape)
        .case(test_focus_scope::outer_to_inner_navigation)
        .case(test_focus_scope::nested_restore_focuses_outermost)
        .case(test_focus_scope::restore_fallback)
        .case(test_focus_scope::dialog_from_menu)
        .case(test_focus_scope::restore_on_blur)
        .case(test_focus_scope::select_on_tab)
        .case(test_focus_scope::tab_outside_the_scope_is_native)
        .case(test_focus_scope::runtime_contain)
        .case(test_focus_scope::cancelled_restore)
        .case(test_focus_scope::tab_out_of_restoring_scope)
        .case(test_press::mouse_click_fires_all_events_in_order)
        .case(test_press::keyboard_enter_and_space_press)
        .case(test_press::releasing_outside_does_not_press)
        .case(test_press::disabled_element_ignores_presses)
        .case(test_press::becoming_disabled_cancels_active_press)
        .case(test_press::enter_on_checkbox_submits_form)
        .case(test_press::prevent_focus_on_press_keeps_the_focus)
        .case(test_press::nested_press_stops_by_default)
        .case(test_press::nested_press_propagates_when_continued)
        .case(test_press::keyboard_press_ends_when_key_up_is_stopped)
        .case(test_press::a_drag_inside_cancels_the_press)
        .case(test_press::a_child_stopping_the_click_cancels_the_press)
        .case(test_press::focus_moving_before_key_up_ends_without_press)
        .case(test_press::repeating_key_downs_are_ignored)
        .case(test_press::dragging_out_and_back_in)
        .case(test_press::cancel_on_pointer_exit)
        .case(test_press::pointer_cancel_cancels_the_press)
        .case(test_press::space_on_a_link_with_button_role)
        .case(test_press::double_press)
        .case(test_press::press_propagation_continue)
        .case(test_press::virtual_click)
        .case(test_press::removed_while_pressed)
        .case(test_press::meta_release_ends_held_key_presses)
        .case(test_use_button::attributes_depend_on_element_type)
        .case(test_use_button::native_and_custom_elements_press)
        .case(test_use_button::tab_order)
        .case(test_use_button::disabled_buttons)
        .case(test_use_button::form_submission)
        .case(test_use_button::hover_and_focus_visible)
        .case(test_menu_trigger::aria_attributes)
        .case(test_menu_trigger::mouse_press_opens_once)
        .case(test_menu_trigger::keyboard_opens_with_focus_strategy)
        .case(test_number_field::stepper_buttons)
        .case(test_number_field::click_steps_once_and_focuses_input)
        .case(test_number_field::holding_spins_until_the_limit)
        .case(test_number_field::keyboard)
        .case(test_number_field::steppers_are_not_tab_stops)
        .case(test_number_field::enter_commits_and_submits)
        .case(test_number_field_atoms::provides_slots)
        .case(test_number_field_atoms::hover_and_focus_visible_state)
        .case(test_number_field_atoms::read_only_state)
        .case(test_number_field_atoms::form_value)
        .case(test_number_field_atoms::validation_errors)
        .case(test_number_field_atoms::arrow_keys)
        .case(test_number_field_atoms::programmatic_clicks_on_steppers)
        .case(test_number_field_atoms::deleting_the_first_digit_before_a_group_separator)
        .case(test_number_field_atoms::typing_and_enter_commit)
        .case(test_number_field_atoms::no_grouping_characters_without_grouping)
        .case(test_number_field_atoms::no_grouping_characters_in_german)
        .case(test_number_field_atoms::scroll_wheel)
        .case(test_number_field_atoms::pasting_into_a_format)
        .case(test_number_field_atoms::rejected_values_keep_the_text)
        .case(test_number_field_atoms::server_errors_survive_an_unchanged_blur)
        .case(test_number_field_atoms::validate_commit_behavior)
        .case(test_number_field_atoms::validate_commit_behavior_and_enter_submit)
        .case(test_number_field_atoms::typed_values)
        .case(test_live_announcer::announcements)
        .case(test_live_announcer::clear)
        .case(test_live_announcer::timeout)
        .case(test_listbox::aria_structure)
        .case(test_listbox::keyboard_navigation_skips_disabled_items)
        .case(test_listbox::selection)
        .case(test_listbox::tab_in_and_out)
        .case(test_listbox::type_ahead)
        .case(test_listbox::select_all_and_clear)
        .case(test_listbox::shift_arrow_extends_selection)
        .case(test_listbox_features::sections_and_separators)
        .case(test_listbox_features::arrow_keys_cross_sections)
        .case(test_listbox_features::hover)
        .case(test_listbox_features::replace_selection_by_press)
        .case(test_listbox_features::replace_selection_by_keyboard)
        .case(test_listbox_features::actions_without_selection)
        .case(test_listbox_features::links)
        .case(test_listbox_features::links_with_single_selection)
        .case(test_listbox_features::arrow_keys_per_layout)
        .case(test_listbox_features::page_down_and_up)
        .case(test_listbox_features::disabled_selection_behavior)
        .case(test_listbox_features::empty_state)
        .case(test_listbox_features::removing_the_focused_option)
        .case(test_listbox_features::labels_follow_the_collection)
        .case(test_select::initial_state)
        .case(test_select::opening_focuses_the_selected_option)
        .case(test_select::escape_closes_and_restores_focus)
        .case(test_select::escape_after_opening_with_the_keyboard)
        .case(test_select::bound_select)
        .case(test_select::selecting_an_option)
        .case(test_select::trigger_keyboard)
        .case(test_select::labelling)
        .case(test_select::form_reset)
        .case(test_select_forms::trigger_hover_and_placeholder)
        .case(test_select_forms::multiple_selection)
        .case(test_select_forms::open_state_bound_to_app_state)
        .case(test_select_forms::native_validation)
        .case(test_select_forms::required_blocks_submission)
        .case(test_select_forms::disabled)
        .case(test_select_forms::autofill)
        .case(test_select_forms::no_items)
        .case(test_select_forms::empty_state)
        .case(test_select_forms::many_items_validation)
        .case(test_select_forms::many_items_selection_and_reset)
        .case(test_menu::closed_trigger)
        .case(test_menu::open_menu_aria_structure)
        .case(test_menu::keyboard_opening_focuses_first_or_last_item)
        .case(test_menu::keyboard_navigation_skips_disabled_items)
        .case(test_menu::keyboard_activation_closes_and_restores_focus)
        .case(test_menu::type_ahead)
        .case(test_menu::clicking_an_item)
        .case(test_menu::mouse_opening_focuses_the_menu)
        .case(test_menu::type_ahead_skips_disabled_items)
        .case(test_menu::selection_menu)
        .case(test_menu_atoms::menu_trigger)
        .case(test_menu_atoms::keyboard_opening)
        .case(test_menu_atoms::selection_menu)
        .case(test_menu_atoms::long_press_trigger)
        .case(test_menu_atoms::section_selection)
        .case(test_menu_atoms::close_on_select)
        .case(test_grid::aria_structure)
        .case(test_grid::row_focus_cell_focus)
        .case(test_grid::row_focus_child_focus)
        .case(test_grid::cell_focus_child_focus)
        .case(test_grid::cell_focus_cell_focus)
        .case(test_grid::restores_the_last_focused_child)
        .case(test_grid::focusing_a_child_from_outside_keeps_it)
        .case(test_grid::two_dimensional_navigation)
        .case(test_grid::row_selection)
        .case(test_grid::cell_focus_mode_selects_rows)
        .case(test_grid::cell_actions)
        .case(test_grid_list::aria_structure)
        .case(test_grid_list::tab_into_the_list_focuses_the_first_row)
        .case(test_grid_list::keyboard_navigation_skips_disabled_rows)
        .case(test_grid_list::tab_out_and_back_restores_the_focused_row)
        .case(test_grid_list::select_all_and_clear)
        .case(test_grid_list::space_toggles_selection)
        .case(test_grid_list::click_selection)
        .case(test_grid_list::disabled_row_is_marked)
        .case(test_grid_list::select_all_skips_disabled_row)
        .case(test_grid_list::shift_arrow_extends_selection)
        .case(test_grid_list::selection_announcements)
        .case(test_grid_list_features::arrows_cycle_through_children_and_row)
        .case(test_grid_list_features::arrows_are_mirrored_right_to_left)
        .case(test_grid_list_features::tab_walks_the_children)
        .case(test_grid_list_features::arrows_move_between_children_of_rows)
        .case(test_grid_list_features::text_input_keeps_its_keys)
        .case(test_grid_list_features::hover_on_rows_with_an_action)
        .case(test_grid_list_features::actions_without_selection)
        .case(test_grid_list_features::replace_selection_behavior)
        .case(test_grid_list_features::links_open_on_press)
        .case(test_grid_list_features::sections_and_descriptions)
        .case(test_table::aria_structure)
        .case(test_table::column_groups)
        .case(test_table::navigation_into_the_column_headers)
        .case(test_table::sorting)
        .case(test_table::select_all)
        .case(test_table::disabled_rows)
        .case(test_table::type_ahead)
        .case(test_table::removing_the_focused_row)
        .case(test_table::localized)
        .case(test_table_navigation::tab_from_a_cell_focuses_its_first_tabbable_child)
        .case(test_table_navigation::tab_from_a_cell_without_children_exits_the_table)
        .case(test_table_navigation::shift_tab_from_a_child_returns_to_the_cell)
        .case(test_table_navigation::keys_in_a_text_input_stay_there)
        .case(test_table_navigation::clicking_a_child_or_a_row)
        .case(test_table_navigation::child_focus_mode_in_tab_navigation)
        .case(test_table_navigation::arrow_navigation_through_cell_children)
        .case(test_table_navigation::arrow_navigation_with_cell_focus_mode)
        .case(test_table_navigation::right_to_left)
        .case(test_table_navigation::page_up_reaches_the_column_headers)
        .case(test_table_navigation::column_spans)
        .case(test_table_navigation::an_empty_table)
        .case(test_table_navigation::enter_on_a_button_that_is_not_the_first_child)
        .case(test_table_selection::replace_selection_with_the_mouse)
        .case(test_table_selection::replace_selection_in_single_mode)
        .case(test_table_selection::replace_selection_with_the_keyboard)
        .case(test_table_selection::escape_without_clearing)
        .case(test_table_selection::select_on_press_down_or_up)
        .case(test_table_selection::row_actions)
        .case(test_table_selection::changing_columns)
        .case(test_table_selection::hover_and_focus_states)
        .case(test_table_tree::renders_a_treegrid)
        .case(test_table_tree::expands_a_row_with_the_mouse)
        .case(test_table_tree::expands_a_row_with_the_keyboard)
        .case(test_table_tree::expands_a_row_with_the_keyboard_rtl)
        .case(test_table_tree::default_expanded_keys)
        .case(test_table_tree::controlled_expanded_keys)
        .case(test_table_tree::keyboard_navigation_of_flattened_rows)
        .case(test_table_tree::keyboard_navigation_of_cells)
        .case(test_table_tree::selection)
        .case(test_table_tree::type_ahead_searches_the_rows_shown)
        .case(test_table_tree::arrow_left_moves_from_a_child_row_to_its_parent)
        .case(test_table_tree::collapsing_from_outside_moves_focus_to_the_parent)
        .case(test_table_tree::leaf_rows_are_never_expanded)
        .case(test_tabs::aria_structure)
        .case(test_tabs::selection_by_press)
        .case(test_tabs::keyboard_navigation)
        .case(test_tabs::disabled_tab)
        .case(test_tabs::disabled_first_tab)
        .case(test_tabs::all_tabs_disabled)
        .case(test_tabs::vertical)
        .case(test_tabs::manual_activation)
        .case(test_tabs::rtl_vertical)
        .case(test_tabs::data_attributes)
        .case(test_tabs::force_mount)
        .case(test_tabs::tab_is_disabled)
        .case(test_tabs::controlled)
        .case(test_tabs::dynamic)
        .case(test_tabs::nested)
        .case(test_tabs::tab_panels)
        .case(test_calendar::structure)
        .case(test_calendar::selection_by_press)
        .case(test_calendar::keyboard_navigation)
        .case(test_calendar::previous_next_buttons)
        .case(test_calendar::min_max)
        .case(test_calendar::unavailable)
        .case(test_calendar::disabled)
        .case(test_calendar::read_only)
        .case(test_calendar::invalid)
        .case(test_calendar::two_months)
        .case(test_calendar::week_view)
        .case(test_calendar::day_view)
        .case(test_calendar::first_day_of_week)
        .case(test_calendar::labelled_by_another_element)
        .case(test_calendar::right_to_left)
        .case(test_calendar::setting_the_focused_date_keeps_the_focus)
        .case(test_calendar::range_by_press)
        .case(test_calendar::range_by_keyboard)
        .case(test_calendar::range_by_dragging)
        .case(test_calendar::range_committed_by_an_outside_press)
        .case(test_calendar::controlled_range_cleared)
        .case(test_calendar::unavailable_dates_depending_on_the_anchor)
        .case(test_calendar::range_unavailable)
        .case(test_calendar::range_by_touch_taps)
        .case(test_calendar::range_by_touch_dragging)
        .case(test_calendar::range_kept_when_a_touch_scrolls)
        .case(test_calendar::today_in_the_browsers_time_zone)
        .case(test_calendar::page_behavior_single)
        .case(test_calendar::two_weeks)
        .case(test_calendar::weeks_in_month)
        .case(test_calendar::held_arrow_keys)
        .case(test_calendar::changing_the_visible_duration)
        .case(test_calendar::month_and_year_pickers)
        .case(test_calendar::announcements)
        .case(test_calendar::commit_behaviors)
        .case(test_date_field::structure)
        .case(test_date_field::typing)
        .case(test_date_field::arrows_and_backspace)
        .case(test_date_field::invalid_date_committed_when_left)
        .case(test_date_field::form_reset)
        .case(test_date_field::min_validation)
        .case(test_date_field::disabled_and_read_only)
        .case(test_date_field::date_and_time)
        .case(test_date_field::zoned)
        .case(test_date_field::time_field)
        .case(test_date_field::date_picker)
        .case(test_date_field::date_range_picker)
        .case(test_date_field::group_states)
        .case(test_date_picker::close_on_select)
        .case(test_date_picker::disabled_picker)
        .case(test_date_picker::programmatic_value)
        .case(test_date_picker::required_picker)
        .case(test_date_picker::required_time_field)
        .case(test_date_picker::range_placeholder_times)
        .case(test_date_picker::enter_does_nothing)
        .case(test_date_picker::held_keys)
        .case(test_date_picker::deleting_a_partial_field)
        .case(test_date_picker::autofill)
        .case(test_date_picker::selection_while_elsewhere)
        .case(test_date_picker::german_order)
        .case(test_date_picker::twelve_hour_clocks)
        .case(test_date_picker::right_to_left)
        .case(test_date_picker::switching_to_right_to_left)
        .case(test_slider::labelled_group)
        .case(test_slider::fill)
        .case(test_slider::keyboard)
        .case(test_slider::track_click)
        .case(test_slider::dragging_state)
        .case(test_slider::two_thumbs)
        .case(test_slider::orientation)
        .case(test_slider::disabled_state)
        .case(test_slider::tooltips)
        .case(test_slider::closest_thumb_by_click)
        .case(test_slider::closest_thumb_by_drag)
        .case(test_slider::stacked_thumbs)
        .case(test_slider::many_stacked_thumbs)
        .case(test_slider::disabled_track)
        .case(test_slider::vertical_drag)
        .case(test_slider::right_to_left)
        .case(test_slider::keys)
        .case(test_slider::keys_vertical)
        .case(test_slider::repeated_page_keys)
        .case(test_slider::input_event)
        .case(test_slider::disabled_thumb)
        .case(test_slider::form_prop)
        .case(test_slider::thumb_labels)
        .case(test_slider::attributes)
        .case(test_slider::three_thumbs)
        .case(test_slider::controlled_thumbs)
        .case(test_slider::restricted_values)
        .case(test_slider::missing_value)
        .case(test_dnd::basic_drag_and_drop)
        .case(test_dnd::escape_cancels)
        .case(test_dnd::reorder_a_list)
        .case(test_dnd::native_basic_drag_and_drop)
        .case(test_dnd::tab_forward_skips_non_drop_targets)
        .case(test_dnd::tab_backward_skips_non_drop_targets)
        .case(test_dnd::prefers_an_ancestor_drop_target)
        .case(test_dnd::enter_on_the_drag_source_cancels)
        .case(test_dnd::ignores_drop_targets_in_hidden_trees)
        .case(test_dnd::a_removed_drop_target)
        .case(test_dnd::a_drop_target_hidden_during_the_drag)
        .case(test_dnd::an_added_drop_target_keeps_the_current_target)
        .case(test_dnd::a_hidden_drag_source_is_skipped)
        .case(test_dnd::escape_with_a_hidden_drag_source)
        .case(test_dnd::disabled_drag)
        .case(test_dnd::disabled_drop)
        .case(test_dnd::drop_operation_override)
        .case(test_dnd::allowed_drop_operations)
        .case(test_dnd::canceled_targets_are_hidden)
        .case(test_dnd::alt_enter_activates)
        .case(test_dnd::native_disabled)
        .case(test_dnd::navigating_with_focus_events_only)
        .case(test_dnd::hides_everything_but_drop_targets)
        .case(test_dnd::clicking_the_drag_source_cancels)
        .case(test_dnd::restores_focus_from_non_drop_targets)
        .case(test_dnd::ignores_clicks_not_from_screen_readers)
        .case(test_dnd_collection::basic_drag_and_drop)
        .case(test_dnd_collection::arrow_key_navigation)
        .case(test_dnd_collection::home_and_end)
        .case(test_dnd_collection::page_up_and_page_down)
        .case(test_dnd_collection::page_up_and_page_down_skip_invalid_targets)
        .case(test_dnd_collection::after_the_last_focused_item)
        .case(test_dnd_collection::after_the_selected_items)
        .case(test_dnd_collection::before_the_selected_items)
        .case(test_dnd_collection::on_the_first_selected_item)
        .case(test_dnd_collection::on_the_last_selected_item)
        .case(test_dnd_collection::native_basic_drag_and_drop)
        .case(test_dnd_collection::native_drop_on_an_item)
        .case(test_clipboard::copies)
        .case(test_clipboard::copies_only_when_focused)
        .case(test_clipboard::no_copy_without_items)
        .case(test_clipboard::cuts)
        .case(test_clipboard::cuts_only_when_focused)
        .case(test_clipboard::no_cut_without_items)
        .case(test_clipboard::no_cut_without_on_cut)
        .case(test_clipboard::pastes)
        .case(test_clipboard::pastes_only_when_focused)
        .case(test_clipboard::no_paste_without_on_paste)
        .case(test_clipboard::custom_types)
        .case(test_clipboard::multiple_items_of_a_custom_type)
        .case(test_clipboard::items_of_multiple_types)
        .case(test_clipboard::multiple_items_of_multiple_types)
        .case(test_clipboard::the_action_of_a_cut)
        .case(test_clipboard::the_action_of_a_copy)
        .case(test_clipboard_write::writes_text)
        .case(test_clipboard_write::writes_text_loaded_later)
        .case(test_clipboard_write::writes_nothing_without_text)
        .case(test_text_field::labelling)
        .case(test_text_field::typing_updates_the_state)
        .case(test_text_field::validation)
        .case(test_text_field::programmatic_changes_update_the_input)
        .case(test_text_field::form_reset_restores_the_default)
        .case(test_text_field_atoms::provides_slots_input)
        .case(test_text_field_atoms::provides_slots_textarea)
        .case(test_text_field_atoms::hover_state)
        .case(test_text_field_atoms::focus_visible_state)
        .case(test_text_field_atoms::read_only_and_required_state)
        .case(test_text_field_atoms::native_validation_errors_input)
        .case(test_text_field_atoms::native_validation_errors_textarea)
        .case(test_text_field_atoms::customized_validation_errors)
        .case(test_text_field_atoms::invalid_without_message_renders_no_error)
        .case(test_text_field_atoms::id_goes_on_the_input)
        .case(test_text_field_atoms::form_attribute)
        .case(test_text_field_atoms::server_validation_errors)
        .case(test_text_field_atoms::bound_values_keep_the_dom_in_sync)
        .case(test_text_field_atoms::form_validation_behavior)
        .case(test_search_field::provides_slots)
        .case(test_search_field::enter_submits)
        .case(test_search_field::escape_clears_once)
        .case(test_search_field::clear_button_clears_and_focuses_the_input)
        .case(test_search_field::enter_without_on_submit_submits_the_form)
        .case(test_search_field::validation_errors)
        .case(test_search_field::read_only)
        .case(test_search_field::form_attribute)
        .case(test_search_field::input_type)
        .case(test_combobox::aria_structure)
        .case(test_combobox::typing_filters_and_keyboard_selects)
        .case(test_combobox::escape_reverts_the_input)
        .case(test_combobox::button_shows_all_options_and_click_selects)
        .case(test_combobox::arrow_down_opens_with_the_selected_option_focused)
        .case(test_combobox::clearing_the_input_clears_the_value)
        .case(test_combobox::externally_changed_value_shows_in_the_input)
        .case(test_combobox::popover_in_a_modal_stays_interactive)
        .case(test_combobox_forms::select_an_option)
        .case(test_combobox_forms::custom_text_on_blur)
        .case(test_combobox_forms::escape_keeps_custom_text)
        .case(test_combobox_forms::enter_commits_custom_text)
        .case(test_combobox_forms::native_validation)
        .case(test_combobox_forms::aria_validation)
        .case(test_combobox_forms::multiple_selection)
        .case(test_combobox_forms::multiple_form_reset)
        .case(test_combobox_forms::required_with_multiple_selection)
        .case(test_combobox_forms::form_value)
        .case(test_combobox_forms::focus_trigger)
        .case(test_combobox_forms::manual_trigger)
        .case(test_combobox_forms::filtering_sections)
        .case(test_combobox_forms::disabled_option_is_skipped)
        .case(test_combobox_forms::enter_without_a_focused_option)
        .case(test_checkbox::selected_state)
        .case(test_checkbox::keyboard_and_focus_ring)
        .case(test_checkbox::virtual_label_click)
        .case(test_checkbox::hover)
        .case(test_checkbox::indeterminate_state)
        .case(test_checkbox::disabled_state)
        .case(test_checkbox::read_only_state)
        .case(test_checkbox::invalid_state)
        .case(test_checkbox::required_state)
        .case(test_checkbox::bound_state)
        .case(test_checkbox::bound_read_only_and_on_change)
        .case(test_checkbox::group)
        .case(test_checkbox::group_disabled_and_read_only)
        .case(test_checkbox::group_validation)
        .case(test_forms::form_reset)
        .case(test_forms::canceled_form_reset)
        .case(test_forms::implicit_submission_with_enter)
        .case(test_forms::right_to_left_arrow_keys)
        .case(test_forms::checkbox_group_realtime_validation)
        .case(test_forms::field_atoms)
        .case(test_radio_group::structure)
        .case(test_radio_group::tab_enters_and_leaves_the_group)
        .case(test_radio_group::selection_by_press)
        .case(test_radio_group::virtual_label_click)
        .case(test_radio_group::arrow_keys)
        .case(test_radio_group::selected_radio_is_the_tab_stop)
        .case(test_radio_group::skips_disabled_radios)
        .case(test_radio_group::horizontal)
        .case(test_radio_group::disabled_group)
        .case(test_radio_group::read_only_group)
        .case(test_radio_group::validation)
        .case(test_radio_group::controlled)
        .case(test_radio_group::label_context_stays_inside)
        .case(test_radio_group::typed_values)
        .case(test_switch::selected_state)
        .case(test_switch::keyboard)
        .case(test_switch::virtual_label_click)
        .case(test_switch::disabled_state)
        .case(test_switch::read_only_state)
        .case(test_switch::bound_state)
        .case(test_switch::bound_read_only)
        .case(test_toggle_button::toggle_button)
        .case(test_toggle_button::disabled_toggle_button)
        .case(test_toggle_button::single_selection)
        .case(test_toggle_button::multiple_selection)
        .case(test_toggle_button::horizontal_navigation)
        .case(test_toggle_button::tab_leaves_and_restores)
        .case(test_toggle_button::vertical_navigation)
        .case(test_toggle_button::disabled_group)
        .case(test_aria_hide_outside::hides_everything_but_the_target)
        .case(test_aria_hide_outside::hides_the_cells_of_a_hidden_row)
        .case(test_aria_hide_outside::nested_hides_restored_out_of_order)
        .case(test_aria_hide_outside::nested_hides_restored_in_order)
        .case(test_aria_hide_outside::hides_a_root_without_the_target)
        .case(test_aria_hide_outside::shows_overlays_registered_late)
        .case(test_aria_hide_outside::mutations)
        .case(test_aria_hide_outside::unhide_after_reorder)
        .case(test_dialog::dismiss_button_closes)
        .case(test_dialog::escape_closes)
        .case(test_dialog::alert_dialog)
        .case(test_dialog::keyboard_open_and_close_from_inside)
        .case(test_dialog::keyboard_open_and_escape)
        .case(test_dialog::nested_modals)
        .case(test_dialog::animated_modal)
        .case(test_dialog::auto_focus)
        .case(test_toolbar::structure)
        .case(test_toolbar::keyboard_navigation)
        .case(test_toolbar::tab_leaves_and_reenters)
        .case(test_toolbar::no_wrapping)
        .case(test_toolbar::vertical)
        .case(test_toolbar::right_to_left)
        .case(test_toolbar::right_to_left_vertical)
        .case(test_toolbar::aria_example_children)
        .case(test_progress_bar::renders)
        .case(test_progress_bar::follows_its_value)
        .case(test_progress_bar::custom_range)
        .case(test_progress_bar::empty_range)
        .case(test_progress_bar::indeterminate)
        .case(test_progress_bar::custom_text_value)
        .case(test_progress_bar::label_follows_the_rendered_label)
        .case(test_progress_bar::meter)
        .case(test_pressable::merges_with_the_childs_handlers)
        .case(test_pressable::makes_the_child_focusable)
        .case(test_pressable::disabled)
        .case(test_pressable::press_responder)
        .case(test_pressable::press_responder_warns_without_pressable)
        .case(test_spin_button::aria_props)
        .case(test_spin_button::disabled_and_read_only)
        .case(test_spin_button::keys_call_their_callbacks)
        .case(test_spin_button::read_only_and_disabled_ignore_keys)
        .case(test_spin_button::announces_value_changes)
        .case(test_submenu::supports_a_submenu_trigger)
        .case(test_submenu::supports_nested_submenu_triggers)
        .case(test_submenu::keyboard)
        .case(test_submenu::focusing_another_item_closes_the_submenu)
        .case(test_submenu::interacting_outside_closes_all)
        .case(test_submenu::context_menu)
        .case(test_submenu::subdialog)
        .case(test_submenu::subdialog_with_dialog)
        .case(test_submenu::right_to_left)
        .case(test_submenu::safe_triangle)
        .case(test_link::current_page)
        .case(test_link::new_tab)
        .case(test_link::trigger_props)
        .case(test_link::disabled)
        .case(test_link::state_attributes)
        .case(test_link::disabled_hook_anchor)
        .case(test_link::anchor_link)
        .case(test_link::replace)
        .case(test_link::client_side_navigation)
        .case(test_localized_atoms::search_field)
        .case(test_localized_atoms::number_field)
        .case(test_localized_atoms::tag)
        .case(test_localized_atoms::select)
        .case(test_breadcrumbs::current_item)
        .case(test_breadcrumbs::dynamic_collections)
        .case(test_breadcrumbs::disabled)
        .case(test_breadcrumbs::hooks)
        .case(test_breadcrumbs::press)
        .case(test_disclosure::trigger_controls_its_panel)
        .case(test_disclosure::adjacent_interactive_elements)
        .case(test_disclosure::toggles_by_press_and_enter)
        .case(test_disclosure::find_in_page_expands)
        .case(test_disclosure::nested_disclosures)
        .case(test_disclosure::one_expanded_at_a_time)
        .case(test_disclosure::multiple_expanded)
        .case(test_disclosure::panel_as_landmark)
        .case(test_disclosure::repeated_keydown_toggles_once)
        .case(test_disclosure::disabled_group)
        .case(test_disclosure::focus_ring)
        .case(test_disclosure::controlled)
        .case(test_disclosure::groups)
        .case(test_popover::trigger_controls_the_dialog)
        .case(test_popover::outside_click_closes)
        .case(test_popover::non_modal_contains_focus_with_a_dialog)
        .case(test_popover::trigger_names_an_untitled_dialog)
        .case(test_popover::standalone_popover_is_the_dialog)
        .case(test_popover::animated)
        .case(test_popover::scrolling)
        .case(test_popover::containment_per_opening)
        .case(test_popover::direction)
        .case(test_dismiss_button::default_label)
        .case(test_dismiss_button::aria_label)
        .case(test_dismiss_button::aria_labelledby)
        .case(test_dismiss_button::aria_labelledby_and_label)
        .case(test_dismiss_button::activating_dismisses)
        .case(test_overlay::dismissable)
        .case(test_overlay::not_dismissable)
        .case(test_overlay::keyboard_dismiss_disabled)
        .case(test_overlay::top_most_only)
        .case(test_overlay::nested_modals)
        .case(test_overlay_position::placed_above)
        .case(test_overlay_position::reopened_with_arrow)
        .case(test_overlay_position::flips_below)
        .case(test_overlay_position::reopened_unplaced)
        .case(test_context_menu_atoms::right_click_opens_at_the_pointer)
        .case(test_context_menu_atoms::escape_returns_focus_to_the_row)
        .case(test_context_menu_atoms::shift_f10_opens_it)
        .case(test_global_shortcuts::slash_outside_text_fields)
        .case(test_global_shortcuts::slash_in_text_field)
        .case(test_global_shortcuts::mod_k_anywhere)
        .case(test_global_shortcuts::shift_key)
        .case(test_global_shortcuts::later_binding_wins)
        .case(test_global_shortcuts::shortcut_keys)
        .case(test_landmark::navigation_order)
        .case(test_landmark::restores_last_focused)
        .case(test_landmark::alt_f6_to_main)
        .case(test_landmark::added_and_removed)
        .case(test_landmark::wrap_event)
        .case(test_landmark::label_updates)
        .case(test_landmark::nested_order)
        .case(test_landmark::controller)
        .case(test_landmark::duplicate_role_warnings)
        .case(test_toast::trigger_and_close)
        .case(test_toast::timeouts)
        .case(test_toast::keyboard_focus)
        .case(test_toast::programmatic_close)
        .case(test_toast::remaining_time_after_pause)
        .case(test_toast::one_at_a_time)
        .case(test_toast::focused_toast_after_new_toast)
        .case(test_tooltip::shows_on_hover)
        .case(test_tooltip::warm_tooltip_replaces_without_animation)
        .case(test_tooltip::shows_on_focus)
        .case(test_tooltip::close_on_press_disabled_and_close_delay)
        .case(test_tooltip::focus_trigger_mode)
        .case(test_tooltip::hide_on_scroll)
        .case(test_tag_group_atoms::default_classes_and_slots)
        .case(test_tag_group_atoms::label_context_ends_with_the_group)
        .case(test_tag_group_atoms::focus_ring)
        .case(test_tag_group_atoms::tabbing_to_remove_buttons)
        .case(test_tag_group_atoms::selection_state)
        .case(test_tag_group_atoms::empty_state)
        .case(test_tag_group_atoms::focus_moves_to_the_grid_when_no_tag_can_take_it)
        .case(test_virtual_list::follows_its_end)
        .case(test_virtual_list::appended_lines_come_into_view)
        .case(test_virtual_list::scroll_jumps_render_rows_in_order)
        .case(test_virtual_list::scrolling_away_stops_following)
        .case(test_virtual_list::selected_row_stays_rendered)
        .case(test_virtual_list::turning_following_on_scrolls_to_the_end)
        .case(test_virtual_list::rebuilt_views_drop_the_old_handlers)
        .case(test_virtual_list::follow_toggle_keeps_measured_sizes)
        .case(test_virtualizer::renders_the_visible_options)
        .case(test_virtualizer::scrolling_renders_other_options)
        .case(test_virtualizer::focused_option_scrolls_into_view)
        .case(test_virtualizer::log_stays_at_its_end)
        .case(test_virtualizer::plain_list_box_is_not_virtualized)
        .case(test_tag_group::aria_structure)
        .case(test_tag_group::keyboard_navigation)
        .case(test_tag_group::removing_with_the_keyboard_moves_focus_on)
        .case(test_tag_group::remove_button)
        .case(test_tag_group::removing_every_tag_focuses_the_group)
        .case(test_tree::aria_structure)
        .case(test_tree::keyboard_expansion)
        .case(test_tree::arrow_right_on_an_expanded_row_keeps_the_focus)
        .case(test_tree::pressing_a_parent_toggles_it)
        .case(test_tree::disabled_items_can_be_expanded_but_not_selected)
        .case(test_tree::disabled_items_cannot_be_used)
        .case(test_tree::right_to_left_expansion_keys)
        .case(test_tree::collapsing_the_parent_of_the_focused_row)
        .case(test_tree::an_item_getting_children)
        .case(test_visually_hidden::hides_element)
        .case(test_visually_hidden::unhides_focused_focusable)
        .case(test_visually_hidden::reactive_is_focusable)
        .case(test_separator::default_class)
        .case(test_separator::accessibility_props)
        .case(test_separator::orientation)
        .case(test_theme::context)
        .case(test_theme::switching)
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

    fn case(self, case: impl for<'a> CaseFn<'a>) -> Self {
        self.with(Case(case))
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

/// Runs a test, then checks what the page reported (panics, uncaught errors, console errors,
/// literal `attr:` attributes; see `PageActions::expect_no_page_errors`; `goto_path` checks the
/// page it leaves):
/// - the test passed: page errors fail it;
/// - the test failed: page errors are added to its failure, as they are often the cause (a panic
///   in an event handler shows as a wait that times out);
/// - an assertion panicked: page errors are logged, then the panic continues.
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

    fn fresh_session(&self) -> bool {
        self.0.fresh_session()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        let outcome = AssertUnwindSafe(self.0.run(driver, base_url))
            .catch_unwind()
            .await;
        let page_errors = page.expect_no_page_errors().await;
        match (outcome, page_errors) {
            (Ok(Ok(())), page_errors) => {
                page_errors.context("the page reported problems after the test passed")?;
                Ok(())
            }
            (Ok(Err(failure)), Ok(())) => Err(failure),
            (Ok(Err(failure)), Err(page_errors)) => Err(failure
                .context(format!(
                    "the page also reported problems, possibly the cause:\n{page_errors}"
                ))
                .into_dynamic()),
            (Err(panic), page_errors) => {
                if let Err(page_errors) = page_errors {
                    tracing::error!(
                        "The page reported problems, possibly the cause of the panic:\n{page_errors}"
                    );
                }
                std::panic::resume_unwind(panic)
            }
        }
    }
}
