//! The fixtures of the hooks' performance tests, one per hook (`/perf/hooks/<hook>`, see
//! [`super`]).

pub mod use_anchor_link;
pub mod use_auto_scroll;
pub mod use_breadcrumb_item;
pub mod use_breadcrumbs;
pub mod use_button;
pub mod use_calendar;
pub mod use_calendar_cell;
pub mod use_calendar_grid;
pub mod use_calendar_heading;
pub mod use_calendar_month_picker;
pub mod use_calendar_state;
pub mod use_calendar_year_picker;
pub mod use_checkbox;
pub mod use_checkbox_group;
pub mod use_checkbox_group_item;
pub mod use_checkbox_group_state;
pub mod use_clipboard;
pub mod use_close_on_scroll;
pub mod use_collection;
pub mod use_color_area;
pub mod use_color_area_state;
pub mod use_color_channel_field;
pub mod use_color_channel_field_state;
pub mod use_color_field;
pub mod use_color_field_state;
pub mod use_color_picker_state;
pub mod use_color_slider;
pub mod use_color_slider_state;
pub mod use_color_swatch;
pub mod use_color_wheel;
pub mod use_color_wheel_state;
pub mod use_combobox;
pub mod use_combobox_state;
pub mod use_contains_filter;
pub mod use_context_menu;
pub mod use_date_field;
pub mod use_date_field_state;
pub mod use_date_picker;
pub mod use_date_picker_group;
pub mod use_date_picker_state;
pub mod use_date_range_picker;
pub mod use_date_range_picker_state;
pub mod use_date_segment;
pub mod use_dialog;
pub mod use_disclosure;
pub mod use_disclosure_group_state;
pub mod use_disclosure_state;
pub mod use_drag;
pub mod use_drag_modality;
pub mod use_drag_session;
pub mod use_draggable_collection;
pub mod use_draggable_collection_state;
pub mod use_draggable_item;
pub mod use_drop;
pub mod use_drop_indicator;
pub mod use_droppable_collection;
pub mod use_droppable_collection_state;
pub mod use_droppable_item;
pub mod use_enter_animation;
pub mod use_exit_animation;
pub mod use_field;
pub mod use_focus;
pub mod use_focus_manager_context;
pub mod use_focus_ring;
pub mod use_focus_visible;
pub mod use_focus_within;
pub mod use_focusable;
pub mod use_form_reset;
pub mod use_form_validation;
pub mod use_form_validation_state;
pub mod use_formatted_text_field;
pub mod use_global_shortcuts;
pub mod use_grid;
pub mod use_grid_cell;
pub mod use_grid_keyboard_delegate;
pub mod use_grid_list;
pub mod use_grid_list_item;
pub mod use_grid_list_section;
pub mod use_grid_row;
pub mod use_grid_row_group;
pub mod use_grid_selection_announcement;
pub mod use_grid_selection_checkbox;
pub mod use_grid_state;
pub mod use_has_tabbable_child;
pub mod use_hidden_date_input;
pub mod use_hidden_select;
pub mod use_hide_outside;
pub mod use_highlight_selection_description;
pub mod use_hover;
pub mod use_interact_outside;
pub mod use_interaction_modality;
pub mod use_item_measurer;
pub mod use_keyboard;
pub mod use_label;
pub mod use_landmark;
pub mod use_link;
pub mod use_list_collection;
pub mod use_list_keyboard_delegate;
pub mod use_list_state;
pub mod use_list_state_view;
pub mod use_listbox;
pub mod use_listbox_section;
pub mod use_menu;
pub mod use_menu_item;
pub mod use_menu_section;
pub mod use_menu_trigger;
pub mod use_menu_trigger_state;
pub mod use_meter;
pub mod use_modal_backdrop;
pub mod use_move;
pub mod use_node;
pub mod use_node_aria_label;
pub mod use_number_field;
pub mod use_number_field_state;
pub mod use_option;
pub mod use_overlay;
pub mod use_overlay_focus_contain;
pub mod use_overlay_position;
pub mod use_overlay_trigger;
pub mod use_overlay_trigger_state;
pub mod use_popover;
pub mod use_press;
pub mod use_prevent_scroll;
pub mod use_progress_bar;
pub mod use_radio;
pub mod use_radio_group;
pub mod use_radio_group_state;
pub mod use_range_calendar;
pub mod use_range_calendar_state;
pub mod use_safely_mouse_to_submenu;
pub mod use_scroll_view;
pub mod use_scroll_wheel;
pub mod use_search_field;
pub mod use_select;
pub mod use_select_state;
pub mod use_selectable_collection;
pub mod use_selectable_item;
pub mod use_selectable_list;
pub mod use_separator;
pub mod use_single_select_list_state;
pub mod use_slider;
pub mod use_slider_marks;
pub mod use_slider_state;
pub mod use_slider_thumb;
pub mod use_spin_button;
pub mod use_submenu_trigger;
pub mod use_submenu_trigger_state;
pub mod use_switch;
pub mod use_tab;
pub mod use_tab_list;
pub mod use_tab_list_state;
pub mod use_tab_panel;
pub mod use_table;
pub mod use_table_cell;
pub mod use_table_column_header;
pub mod use_table_column_resize;
pub mod use_table_column_resize_state;
pub mod use_table_header_placeholder;
pub mod use_table_header_row;
pub mod use_table_keyboard_delegate;
pub mod use_table_row;
pub mod use_table_select_all_checkbox;
pub mod use_table_selection_checkbox;
pub mod use_table_state;
pub mod use_tag;
pub mod use_tag_group;
pub mod use_text_field;
pub mod use_text_field_state;
pub mod use_time_field;
pub mod use_time_field_state;
pub mod use_toast;
pub mod use_toast_region;
pub mod use_toast_state;
pub mod use_toggle;
pub mod use_toggle_button;
pub mod use_toggle_button_group;
pub mod use_toggle_button_group_item;
pub mod use_toggle_group_state;
pub mod use_toggle_state;
pub mod use_toolbar;
pub mod use_tooltip;
pub mod use_tooltip_trigger;
pub mod use_tooltip_trigger_state;
pub mod use_tree;
pub mod use_tree_expansion;
pub mod use_tree_item;
pub mod use_tree_state;
pub mod use_type_select;
pub mod use_virtual_drop;
pub mod use_virtualizer_item;
pub mod use_virtualizer_state;
pub mod use_visually_hidden;
pub mod use_wheel_stepping;

use leptos::prelude::*;

use crate::pages::Fixture;

/// The fixture of every hook's performance test.
pub const FIXTURES: &[Fixture] = &[
    Fixture {
        group: "perf/hooks",
        name: "use_anchor_link",
        title: "use_anchor_link performance",
        view: || view! { <use_anchor_link::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_auto_scroll",
        title: "use_auto_scroll performance",
        view: || view! { <use_auto_scroll::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_breadcrumb_item",
        title: "use_breadcrumb_item performance",
        view: || view! { <use_breadcrumb_item::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_breadcrumbs",
        title: "use_breadcrumbs performance",
        view: || view! { <use_breadcrumbs::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_button",
        title: "use_button performance",
        view: || view! { <use_button::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_calendar",
        title: "use_calendar performance",
        view: || view! { <use_calendar::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_calendar_cell",
        title: "use_calendar_cell performance",
        view: || view! { <use_calendar_cell::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_calendar_grid",
        title: "use_calendar_grid performance",
        view: || view! { <use_calendar_grid::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_calendar_heading",
        title: "use_calendar_heading performance",
        view: || view! { <use_calendar_heading::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_calendar_month_picker",
        title: "use_calendar_month_picker performance",
        view: || view! { <use_calendar_month_picker::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_calendar_state",
        title: "use_calendar_state performance",
        view: || view! { <use_calendar_state::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_calendar_year_picker",
        title: "use_calendar_year_picker performance",
        view: || view! { <use_calendar_year_picker::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_checkbox",
        title: "use_checkbox performance",
        view: || view! { <use_checkbox::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_checkbox_group",
        title: "use_checkbox_group performance",
        view: || view! { <use_checkbox_group::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_checkbox_group_item",
        title: "use_checkbox_group_item performance",
        view: || view! { <use_checkbox_group_item::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_checkbox_group_state",
        title: "use_checkbox_group_state performance",
        view: || view! { <use_checkbox_group_state::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_clipboard",
        title: "use_clipboard performance",
        view: || view! { <use_clipboard::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_close_on_scroll",
        title: "use_close_on_scroll performance",
        view: || view! { <use_close_on_scroll::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_collection",
        title: "use_collection performance",
        view: || view! { <use_collection::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_color_area",
        title: "use_color_area performance",
        view: || view! { <use_color_area::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_color_area_state",
        title: "use_color_area_state performance",
        view: || view! { <use_color_area_state::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_color_channel_field",
        title: "use_color_channel_field performance",
        view: || view! { <use_color_channel_field::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_color_channel_field_state",
        title: "use_color_channel_field_state performance",
        view: || view! { <use_color_channel_field_state::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_color_field",
        title: "use_color_field performance",
        view: || view! { <use_color_field::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_color_field_state",
        title: "use_color_field_state performance",
        view: || view! { <use_color_field_state::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_color_picker_state",
        title: "use_color_picker_state performance",
        view: || view! { <use_color_picker_state::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_color_slider",
        title: "use_color_slider performance",
        view: || view! { <use_color_slider::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_color_slider_state",
        title: "use_color_slider_state performance",
        view: || view! { <use_color_slider_state::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_color_swatch",
        title: "use_color_swatch performance",
        view: || view! { <use_color_swatch::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_color_wheel",
        title: "use_color_wheel performance",
        view: || view! { <use_color_wheel::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_color_wheel_state",
        title: "use_color_wheel_state performance",
        view: || view! { <use_color_wheel_state::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_combobox",
        title: "use_combobox performance",
        view: || view! { <use_combobox::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_combobox_state",
        title: "use_combobox_state performance",
        view: || view! { <use_combobox_state::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_contains_filter",
        title: "use_contains_filter performance",
        view: || view! { <use_contains_filter::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_context_menu",
        title: "use_context_menu performance",
        view: || view! { <use_context_menu::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_date_field",
        title: "use_date_field performance",
        view: || view! { <use_date_field::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_date_field_state",
        title: "use_date_field_state performance",
        view: || view! { <use_date_field_state::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_date_picker",
        title: "use_date_picker performance",
        view: || view! { <use_date_picker::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_date_picker_group",
        title: "use_date_picker_group performance",
        view: || view! { <use_date_picker_group::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_date_picker_state",
        title: "use_date_picker_state performance",
        view: || view! { <use_date_picker_state::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_date_range_picker",
        title: "use_date_range_picker performance",
        view: || view! { <use_date_range_picker::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_date_range_picker_state",
        title: "use_date_range_picker_state performance",
        view: || view! { <use_date_range_picker_state::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_date_segment",
        title: "use_date_segment performance",
        view: || view! { <use_date_segment::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_dialog",
        title: "use_dialog performance",
        view: || view! { <use_dialog::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_disclosure",
        title: "use_disclosure performance",
        view: || view! { <use_disclosure::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_disclosure_group_state",
        title: "use_disclosure_group_state performance",
        view: || view! { <use_disclosure_group_state::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_disclosure_state",
        title: "use_disclosure_state performance",
        view: || view! { <use_disclosure_state::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_drag",
        title: "use_drag performance",
        view: || view! { <use_drag::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_drag_modality",
        title: "use_drag_modality performance",
        view: || view! { <use_drag_modality::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_drag_session",
        title: "use_drag_session performance",
        view: || view! { <use_drag_session::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_draggable_collection",
        title: "use_draggable_collection performance",
        view: || view! { <use_draggable_collection::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_draggable_collection_state",
        title: "use_draggable_collection_state performance",
        view: || view! { <use_draggable_collection_state::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_draggable_item",
        title: "use_draggable_item performance",
        view: || view! { <use_draggable_item::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_drop",
        title: "use_drop performance",
        view: || view! { <use_drop::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_drop_indicator",
        title: "use_drop_indicator performance",
        view: || view! { <use_drop_indicator::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_droppable_collection",
        title: "use_droppable_collection performance",
        view: || view! { <use_droppable_collection::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_droppable_collection_state",
        title: "use_droppable_collection_state performance",
        view: || view! { <use_droppable_collection_state::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_droppable_item",
        title: "use_droppable_item performance",
        view: || view! { <use_droppable_item::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_enter_animation",
        title: "use_enter_animation performance",
        view: || view! { <use_enter_animation::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_exit_animation",
        title: "use_exit_animation performance",
        view: || view! { <use_exit_animation::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_field",
        title: "use_field performance",
        view: || view! { <use_field::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_focus",
        title: "use_focus performance",
        view: || view! { <use_focus::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_focus_manager_context",
        title: "use_focus_manager_context performance",
        view: || view! { <use_focus_manager_context::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_focus_ring",
        title: "use_focus_ring performance",
        view: || view! { <use_focus_ring::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_focus_visible",
        title: "use_focus_visible performance",
        view: || view! { <use_focus_visible::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_focus_within",
        title: "use_focus_within performance",
        view: || view! { <use_focus_within::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_focusable",
        title: "use_focusable performance",
        view: || view! { <use_focusable::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_form_reset",
        title: "use_form_reset performance",
        view: || view! { <use_form_reset::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_form_validation",
        title: "use_form_validation performance",
        view: || view! { <use_form_validation::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_form_validation_state",
        title: "use_form_validation_state performance",
        view: || view! { <use_form_validation_state::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_formatted_text_field",
        title: "use_formatted_text_field performance",
        view: || view! { <use_formatted_text_field::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_global_shortcuts",
        title: "use_global_shortcuts performance",
        view: || view! { <use_global_shortcuts::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_grid",
        title: "use_grid performance",
        view: || view! { <use_grid::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_grid_cell",
        title: "use_grid_cell performance",
        view: || view! { <use_grid_cell::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_grid_keyboard_delegate",
        title: "use_grid_keyboard_delegate performance",
        view: || view! { <use_grid_keyboard_delegate::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_grid_list",
        title: "use_grid_list performance",
        view: || view! { <use_grid_list::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_grid_list_item",
        title: "use_grid_list_item performance",
        view: || view! { <use_grid_list_item::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_grid_list_section",
        title: "use_grid_list_section performance",
        view: || view! { <use_grid_list_section::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_grid_row",
        title: "use_grid_row performance",
        view: || view! { <use_grid_row::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_grid_row_group",
        title: "use_grid_row_group performance",
        view: || view! { <use_grid_row_group::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_grid_selection_announcement",
        title: "use_grid_selection_announcement performance",
        view: || view! { <use_grid_selection_announcement::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_grid_selection_checkbox",
        title: "use_grid_selection_checkbox performance",
        view: || view! { <use_grid_selection_checkbox::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_grid_state",
        title: "use_grid_state performance",
        view: || view! { <use_grid_state::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_has_tabbable_child",
        title: "use_has_tabbable_child performance",
        view: || view! { <use_has_tabbable_child::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_hidden_date_input",
        title: "use_hidden_date_input performance",
        view: || view! { <use_hidden_date_input::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_hidden_select",
        title: "use_hidden_select performance",
        view: || view! { <use_hidden_select::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_hide_outside",
        title: "use_hide_outside performance",
        view: || view! { <use_hide_outside::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_highlight_selection_description",
        title: "use_highlight_selection_description performance",
        view: || view! { <use_highlight_selection_description::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_hover",
        title: "use_hover performance",
        view: || view! { <use_hover::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_interact_outside",
        title: "use_interact_outside performance",
        view: || view! { <use_interact_outside::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_interaction_modality",
        title: "use_interaction_modality performance",
        view: || view! { <use_interaction_modality::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_item_measurer",
        title: "use_item_measurer performance",
        view: || view! { <use_item_measurer::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_keyboard",
        title: "use_keyboard performance",
        view: || view! { <use_keyboard::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_label",
        title: "use_label performance",
        view: || view! { <use_label::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_landmark",
        title: "use_landmark performance",
        view: || view! { <use_landmark::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_link",
        title: "use_link performance",
        view: || view! { <use_link::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_list_collection",
        title: "use_list_collection performance",
        view: || view! { <use_list_collection::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_list_keyboard_delegate",
        title: "use_list_keyboard_delegate performance",
        view: || view! { <use_list_keyboard_delegate::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_list_state",
        title: "use_list_state performance",
        view: || view! { <use_list_state::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_list_state_view",
        title: "use_list_state_view performance",
        view: || view! { <use_list_state_view::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_listbox",
        title: "use_listbox performance",
        view: || view! { <use_listbox::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_listbox_section",
        title: "use_listbox_section performance",
        view: || view! { <use_listbox_section::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_menu",
        title: "use_menu performance",
        view: || view! { <use_menu::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_menu_item",
        title: "use_menu_item performance",
        view: || view! { <use_menu_item::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_menu_section",
        title: "use_menu_section performance",
        view: || view! { <use_menu_section::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_menu_trigger",
        title: "use_menu_trigger performance",
        view: || view! { <use_menu_trigger::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_menu_trigger_state",
        title: "use_menu_trigger_state performance",
        view: || view! { <use_menu_trigger_state::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_meter",
        title: "use_meter performance",
        view: || view! { <use_meter::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_modal_backdrop",
        title: "use_modal_backdrop performance",
        view: || view! { <use_modal_backdrop::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_move",
        title: "use_move performance",
        view: || view! { <use_move::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_node",
        title: "use_node performance",
        view: || view! { <use_node::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_node_aria_label",
        title: "use_node_aria_label performance",
        view: || view! { <use_node_aria_label::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_number_field",
        title: "use_number_field performance",
        view: || view! { <use_number_field::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_number_field_state",
        title: "use_number_field_state performance",
        view: || view! { <use_number_field_state::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_option",
        title: "use_option performance",
        view: || view! { <use_option::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_overlay",
        title: "use_overlay performance",
        view: || view! { <use_overlay::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_overlay_focus_contain",
        title: "use_overlay_focus_contain performance",
        view: || view! { <use_overlay_focus_contain::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_overlay_position",
        title: "use_overlay_position performance",
        view: || view! { <use_overlay_position::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_overlay_trigger",
        title: "use_overlay_trigger performance",
        view: || view! { <use_overlay_trigger::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_overlay_trigger_state",
        title: "use_overlay_trigger_state performance",
        view: || view! { <use_overlay_trigger_state::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_popover",
        title: "use_popover performance",
        view: || view! { <use_popover::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_press",
        title: "use_press performance",
        view: || view! { <use_press::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_prevent_scroll",
        title: "use_prevent_scroll performance",
        view: || view! { <use_prevent_scroll::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_progress_bar",
        title: "use_progress_bar performance",
        view: || view! { <use_progress_bar::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_radio",
        title: "use_radio performance",
        view: || view! { <use_radio::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_radio_group",
        title: "use_radio_group performance",
        view: || view! { <use_radio_group::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_radio_group_state",
        title: "use_radio_group_state performance",
        view: || view! { <use_radio_group_state::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_range_calendar",
        title: "use_range_calendar performance",
        view: || view! { <use_range_calendar::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_range_calendar_state",
        title: "use_range_calendar_state performance",
        view: || view! { <use_range_calendar_state::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_safely_mouse_to_submenu",
        title: "use_safely_mouse_to_submenu performance",
        view: || view! { <use_safely_mouse_to_submenu::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_scroll_view",
        title: "use_scroll_view performance",
        view: || view! { <use_scroll_view::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_scroll_wheel",
        title: "use_scroll_wheel performance",
        view: || view! { <use_scroll_wheel::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_search_field",
        title: "use_search_field performance",
        view: || view! { <use_search_field::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_select",
        title: "use_select performance",
        view: || view! { <use_select::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_select_state",
        title: "use_select_state performance",
        view: || view! { <use_select_state::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_selectable_collection",
        title: "use_selectable_collection performance",
        view: || view! { <use_selectable_collection::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_selectable_item",
        title: "use_selectable_item performance",
        view: || view! { <use_selectable_item::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_selectable_list",
        title: "use_selectable_list performance",
        view: || view! { <use_selectable_list::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_separator",
        title: "use_separator performance",
        view: || view! { <use_separator::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_single_select_list_state",
        title: "use_single_select_list_state performance",
        view: || view! { <use_single_select_list_state::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_slider",
        title: "use_slider performance",
        view: || view! { <use_slider::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_slider_marks",
        title: "use_slider_marks performance",
        view: || view! { <use_slider_marks::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_slider_state",
        title: "use_slider_state performance",
        view: || view! { <use_slider_state::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_slider_thumb",
        title: "use_slider_thumb performance",
        view: || view! { <use_slider_thumb::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_spin_button",
        title: "use_spin_button performance",
        view: || view! { <use_spin_button::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_submenu_trigger",
        title: "use_submenu_trigger performance",
        view: || view! { <use_submenu_trigger::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_submenu_trigger_state",
        title: "use_submenu_trigger_state performance",
        view: || view! { <use_submenu_trigger_state::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_switch",
        title: "use_switch performance",
        view: || view! { <use_switch::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_tab",
        title: "use_tab performance",
        view: || view! { <use_tab::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_tab_list",
        title: "use_tab_list performance",
        view: || view! { <use_tab_list::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_tab_list_state",
        title: "use_tab_list_state performance",
        view: || view! { <use_tab_list_state::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_tab_panel",
        title: "use_tab_panel performance",
        view: || view! { <use_tab_panel::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_table",
        title: "use_table performance",
        view: || view! { <use_table::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_table_cell",
        title: "use_table_cell performance",
        view: || view! { <use_table_cell::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_table_column_header",
        title: "use_table_column_header performance",
        view: || view! { <use_table_column_header::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_table_column_resize",
        title: "use_table_column_resize performance",
        view: || view! { <use_table_column_resize::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_table_column_resize_state",
        title: "use_table_column_resize_state performance",
        view: || view! { <use_table_column_resize_state::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_table_header_placeholder",
        title: "use_table_header_placeholder performance",
        view: || view! { <use_table_header_placeholder::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_table_header_row",
        title: "use_table_header_row performance",
        view: || view! { <use_table_header_row::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_table_keyboard_delegate",
        title: "use_table_keyboard_delegate performance",
        view: || view! { <use_table_keyboard_delegate::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_table_row",
        title: "use_table_row performance",
        view: || view! { <use_table_row::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_table_select_all_checkbox",
        title: "use_table_select_all_checkbox performance",
        view: || view! { <use_table_select_all_checkbox::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_table_selection_checkbox",
        title: "use_table_selection_checkbox performance",
        view: || view! { <use_table_selection_checkbox::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_table_state",
        title: "use_table_state performance",
        view: || view! { <use_table_state::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_tag",
        title: "use_tag performance",
        view: || view! { <use_tag::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_tag_group",
        title: "use_tag_group performance",
        view: || view! { <use_tag_group::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_text_field",
        title: "use_text_field performance",
        view: || view! { <use_text_field::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_text_field_state",
        title: "use_text_field_state performance",
        view: || view! { <use_text_field_state::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_time_field",
        title: "use_time_field performance",
        view: || view! { <use_time_field::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_time_field_state",
        title: "use_time_field_state performance",
        view: || view! { <use_time_field_state::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_toast",
        title: "use_toast performance",
        view: || view! { <use_toast::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_toast_region",
        title: "use_toast_region performance",
        view: || view! { <use_toast_region::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_toast_state",
        title: "use_toast_state performance",
        view: || view! { <use_toast_state::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_toggle",
        title: "use_toggle performance",
        view: || view! { <use_toggle::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_toggle_button",
        title: "use_toggle_button performance",
        view: || view! { <use_toggle_button::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_toggle_button_group",
        title: "use_toggle_button_group performance",
        view: || view! { <use_toggle_button_group::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_toggle_button_group_item",
        title: "use_toggle_button_group_item performance",
        view: || view! { <use_toggle_button_group_item::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_toggle_group_state",
        title: "use_toggle_group_state performance",
        view: || view! { <use_toggle_group_state::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_toggle_state",
        title: "use_toggle_state performance",
        view: || view! { <use_toggle_state::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_toolbar",
        title: "use_toolbar performance",
        view: || view! { <use_toolbar::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_tooltip",
        title: "use_tooltip performance",
        view: || view! { <use_tooltip::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_tooltip_trigger",
        title: "use_tooltip_trigger performance",
        view: || view! { <use_tooltip_trigger::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_tooltip_trigger_state",
        title: "use_tooltip_trigger_state performance",
        view: || view! { <use_tooltip_trigger_state::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_tree",
        title: "use_tree performance",
        view: || view! { <use_tree::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_tree_expansion",
        title: "use_tree_expansion performance",
        view: || view! { <use_tree_expansion::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_tree_item",
        title: "use_tree_item performance",
        view: || view! { <use_tree_item::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_tree_state",
        title: "use_tree_state performance",
        view: || view! { <use_tree_state::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_type_select",
        title: "use_type_select performance",
        view: || view! { <use_type_select::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_virtual_drop",
        title: "use_virtual_drop performance",
        view: || view! { <use_virtual_drop::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_virtualizer_item",
        title: "use_virtualizer_item performance",
        view: || view! { <use_virtualizer_item::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_virtualizer_state",
        title: "use_virtualizer_state performance",
        view: || view! { <use_virtualizer_state::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_visually_hidden",
        title: "use_visually_hidden performance",
        view: || view! { <use_visually_hidden::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/hooks",
        name: "use_wheel_stepping",
        title: "use_wheel_stepping performance",
        view: || view! { <use_wheel_stepping::PagePerf /> }.into_any(),
    },
];
