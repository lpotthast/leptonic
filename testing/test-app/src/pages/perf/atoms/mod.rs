//! The fixtures of the atoms' performance tests, one per atom (`/perf/atoms/<atom>`, see
//! [`super`]).

pub mod anchor_link;
pub mod breadcrumb;
pub mod breadcrumbs;
pub mod button;
pub mod calendar;
pub mod calendar_cell;
pub mod calendar_cell_button;
pub mod calendar_error_message;
pub mod calendar_grid;
pub mod calendar_grid_body;
pub mod calendar_grid_header;
pub mod calendar_header_cell;
pub mod calendar_header_row;
pub mod calendar_heading;
pub mod calendar_month_picker;
pub mod calendar_next_button;
pub mod calendar_previous_button;
pub mod calendar_week;
pub mod calendar_year_picker;
pub mod checkbox_button;
pub mod checkbox_field;
pub mod checkbox_group;
pub mod clear_press_responder;
pub mod color_area;
pub mod color_channel_field;
pub mod color_field;
pub mod color_picker;
pub mod color_slider;
pub mod color_slider_output;
pub mod color_slider_track;
pub mod color_swatch;
pub mod color_swatch_picker;
pub mod color_swatch_picker_item;
pub mod color_swatch_picker_items;
pub mod color_thumb;
pub mod color_wheel;
pub mod color_wheel_track;
pub mod combo_box;
pub mod combo_box_button;
pub mod combo_box_popover;
pub mod combo_box_value;
pub mod context_menu_trigger;
pub mod date_field;
pub mod date_input;
pub mod date_picker;
pub mod date_picker_button;
pub mod date_picker_group;
pub mod date_range_picker;
pub mod date_segment;
pub mod description;
pub mod dialog;
pub mod dialog_description;
pub mod dialog_title;
pub mod dialog_trigger;
pub mod disclosure;
pub mod disclosure_group;
pub mod disclosure_panel;
pub mod disclosure_trigger;
pub mod dismiss_button;
pub mod field_error;
pub mod focus_manager_provider;
pub mod focus_scope;
pub mod focusable;
pub mod form;
pub mod grid;
pub mod grid_cell;
pub mod grid_list;
pub mod grid_list_header;
pub mod grid_list_item;
pub mod grid_list_item_description;
pub mod grid_list_section;
pub mod grid_row;
pub mod grid_row_group;
pub mod hidden_select;
pub mod hoverable;
pub mod input;
pub mod keys;
pub mod label;
pub mod link;
pub mod list_box;
pub mod list_box_item;
pub mod list_box_item_description;
pub mod list_box_item_label;
pub mod list_box_items;
pub mod list_box_section;
pub mod list_box_section_heading;
pub mod menu;
pub mod menu_item;
pub mod menu_item_description;
pub mod menu_item_label;
pub mod menu_item_shortcut;
pub mod menu_items;
pub mod menu_section;
pub mod menu_trigger;
pub mod meter;
pub mod meter_fill;
pub mod meter_value_text;
pub mod modal_backdrop;
pub mod modal_content;
pub mod number_field;
pub mod number_field_decrement_button;
pub mod number_field_group;
pub mod number_field_increment_button;
pub mod overlay_arrow;
pub mod popover;
pub mod press_responder;
pub mod pressable;
pub mod progress_bar;
pub mod progress_bar_fill;
pub mod progress_bar_value_text;
pub mod radio_button;
pub mod radio_field;
pub mod radio_group;
pub mod range_calendar;
pub mod resizable_table_container;
pub mod search_field;
pub mod search_field_clear_button;
pub mod select;
pub mod select_popover;
pub mod select_trigger;
pub mod select_value;
pub mod separator;
pub mod shortcut_keys;
pub mod slider;
pub mod slider_fill;
pub mod slider_mark;
pub mod slider_marks;
pub mod slider_output;
pub mod slider_thumb;
pub mod slider_thumb_tooltip;
pub mod slider_track;
pub mod submenu_trigger;
pub mod switch_button;
pub mod switch_field;
pub mod tab;
pub mod tab_list;
pub mod tab_panel;
pub mod tab_panels;
pub mod table;
pub mod table_body;
pub mod table_cell;
pub mod table_expand_button;
pub mod table_header;
pub mod table_row;
pub mod tabs;
pub mod tag;
pub mod tag_group;
pub mod tag_items;
pub mod tag_list;
pub mod tag_remove_button;
pub mod text_area;
pub mod text_field;
pub mod theme_provider;
pub mod time_field;
pub mod toast;
pub mod toast_close_button;
pub mod toast_content;
pub mod toast_description;
pub mod toast_region;
pub mod toast_title;
pub mod toggle_button;
pub mod toggle_button_group;
pub mod toolbar;
pub mod tooltip;
pub mod tooltip_trigger;
pub mod virtual_list;
pub mod virtualizer;
pub mod visually_hidden;

use leptos::prelude::*;

use crate::pages::Fixture;

/// The fixture of every atom's performance test.
pub const FIXTURES: &[Fixture] = &[
    Fixture {
        group: "perf/atoms",
        name: "anchor_link",
        title: "AnchorLink performance",
        view: || view! { <anchor_link::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "breadcrumb",
        title: "Breadcrumb performance",
        view: || view! { <breadcrumb::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "breadcrumbs",
        title: "Breadcrumbs performance",
        view: || view! { <breadcrumbs::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "button",
        title: "Button performance",
        view: || view! { <button::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "calendar",
        title: "Calendar performance",
        view: || view! { <calendar::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "calendar_cell",
        title: "CalendarCell performance",
        view: || view! { <calendar_cell::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "calendar_cell_button",
        title: "CalendarCellButton performance",
        view: || view! { <calendar_cell_button::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "calendar_error_message",
        title: "CalendarErrorMessage performance",
        view: || view! { <calendar_error_message::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "calendar_grid",
        title: "CalendarGrid performance",
        view: || view! { <calendar_grid::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "calendar_grid_body",
        title: "CalendarGridBody performance",
        view: || view! { <calendar_grid_body::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "calendar_grid_header",
        title: "CalendarGridHeader performance",
        view: || view! { <calendar_grid_header::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "calendar_header_cell",
        title: "CalendarHeaderCell performance",
        view: || view! { <calendar_header_cell::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "calendar_header_row",
        title: "CalendarHeaderRow performance",
        view: || view! { <calendar_header_row::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "calendar_heading",
        title: "CalendarHeading performance",
        view: || view! { <calendar_heading::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "calendar_month_picker",
        title: "CalendarMonthPicker performance",
        view: || view! { <calendar_month_picker::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "calendar_next_button",
        title: "CalendarNextButton performance",
        view: || view! { <calendar_next_button::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "calendar_previous_button",
        title: "CalendarPreviousButton performance",
        view: || view! { <calendar_previous_button::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "calendar_week",
        title: "CalendarWeek performance",
        view: || view! { <calendar_week::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "calendar_year_picker",
        title: "CalendarYearPicker performance",
        view: || view! { <calendar_year_picker::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "checkbox_button",
        title: "CheckboxButton performance",
        view: || view! { <checkbox_button::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "checkbox_field",
        title: "CheckboxField performance",
        view: || view! { <checkbox_field::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "checkbox_group",
        title: "CheckboxGroup performance",
        view: || view! { <checkbox_group::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "clear_press_responder",
        title: "ClearPressResponder performance",
        view: || view! { <clear_press_responder::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "color_area",
        title: "ColorArea performance",
        view: || view! { <color_area::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "color_channel_field",
        title: "ColorChannelField performance",
        view: || view! { <color_channel_field::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "color_field",
        title: "ColorField performance",
        view: || view! { <color_field::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "color_picker",
        title: "ColorPicker performance",
        view: || view! { <color_picker::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "color_slider",
        title: "ColorSlider performance",
        view: || view! { <color_slider::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "color_slider_output",
        title: "ColorSliderOutput performance",
        view: || view! { <color_slider_output::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "color_slider_track",
        title: "ColorSliderTrack performance",
        view: || view! { <color_slider_track::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "color_swatch",
        title: "ColorSwatch performance",
        view: || view! { <color_swatch::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "color_swatch_picker",
        title: "ColorSwatchPicker performance",
        view: || view! { <color_swatch_picker::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "color_swatch_picker_item",
        title: "ColorSwatchPickerItem performance",
        view: || view! { <color_swatch_picker_item::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "color_swatch_picker_items",
        title: "ColorSwatchPickerItems performance",
        view: || view! { <color_swatch_picker_items::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "color_thumb",
        title: "ColorThumb performance",
        view: || view! { <color_thumb::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "color_wheel",
        title: "ColorWheel performance",
        view: || view! { <color_wheel::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "color_wheel_track",
        title: "ColorWheelTrack performance",
        view: || view! { <color_wheel_track::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "combo_box",
        title: "ComboBox performance",
        view: || view! { <combo_box::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "combo_box_button",
        title: "ComboBoxButton performance",
        view: || view! { <combo_box_button::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "combo_box_popover",
        title: "ComboBoxPopover performance",
        view: || view! { <combo_box_popover::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "combo_box_value",
        title: "ComboBoxValue performance",
        view: || view! { <combo_box_value::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "context_menu_trigger",
        title: "ContextMenuTrigger performance",
        view: || view! { <context_menu_trigger::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "date_field",
        title: "DateField performance",
        view: || view! { <date_field::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "date_input",
        title: "DateInput performance",
        view: || view! { <date_input::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "date_picker",
        title: "DatePicker performance",
        view: || view! { <date_picker::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "date_picker_button",
        title: "DatePickerButton performance",
        view: || view! { <date_picker_button::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "date_picker_group",
        title: "DatePickerGroup performance",
        view: || view! { <date_picker_group::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "date_range_picker",
        title: "DateRangePicker performance",
        view: || view! { <date_range_picker::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "date_segment",
        title: "DateSegment performance",
        view: || view! { <date_segment::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "description",
        title: "Description performance",
        view: || view! { <description::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "dialog",
        title: "Dialog performance",
        view: || view! { <dialog::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "dialog_description",
        title: "DialogDescription performance",
        view: || view! { <dialog_description::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "dialog_title",
        title: "DialogTitle performance",
        view: || view! { <dialog_title::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "dialog_trigger",
        title: "DialogTrigger performance",
        view: || view! { <dialog_trigger::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "disclosure",
        title: "Disclosure performance",
        view: || view! { <disclosure::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "disclosure_group",
        title: "DisclosureGroup performance",
        view: || view! { <disclosure_group::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "disclosure_panel",
        title: "DisclosurePanel performance",
        view: || view! { <disclosure_panel::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "disclosure_trigger",
        title: "DisclosureTrigger performance",
        view: || view! { <disclosure_trigger::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "dismiss_button",
        title: "DismissButton performance",
        view: || view! { <dismiss_button::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "field_error",
        title: "FieldError performance",
        view: || view! { <field_error::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "focus_manager_provider",
        title: "FocusManagerProvider performance",
        view: || view! { <focus_manager_provider::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "focus_scope",
        title: "FocusScope performance",
        view: || view! { <focus_scope::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "focusable",
        title: "Focusable performance",
        view: || view! { <focusable::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "form",
        title: "Form performance",
        view: || view! { <form::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "grid",
        title: "Grid performance",
        view: || view! { <grid::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "grid_cell",
        title: "GridCell performance",
        view: || view! { <grid_cell::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "grid_list",
        title: "GridList performance",
        view: || view! { <grid_list::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "grid_list_header",
        title: "GridListHeader performance",
        view: || view! { <grid_list_header::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "grid_list_item",
        title: "GridListItem performance",
        view: || view! { <grid_list_item::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "grid_list_item_description",
        title: "GridListItemDescription performance",
        view: || view! { <grid_list_item_description::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "grid_list_section",
        title: "GridListSection performance",
        view: || view! { <grid_list_section::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "grid_row",
        title: "GridRow performance",
        view: || view! { <grid_row::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "grid_row_group",
        title: "GridRowGroup performance",
        view: || view! { <grid_row_group::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "hidden_select",
        title: "HiddenSelect performance",
        view: || view! { <hidden_select::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "hoverable",
        title: "Hoverable performance",
        view: || view! { <hoverable::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "input",
        title: "Input performance",
        view: || view! { <input::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "keys",
        title: "Keys performance",
        view: || view! { <keys::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "label",
        title: "Label performance",
        view: || view! { <label::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "link",
        title: "Link performance",
        view: || view! { <link::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "list_box",
        title: "ListBox performance",
        view: || view! { <list_box::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "list_box_item",
        title: "ListBoxItem performance",
        view: || view! { <list_box_item::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "list_box_item_description",
        title: "ListBoxItemDescription performance",
        view: || view! { <list_box_item_description::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "list_box_item_label",
        title: "ListBoxItemLabel performance",
        view: || view! { <list_box_item_label::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "list_box_items",
        title: "ListBoxItems performance",
        view: || view! { <list_box_items::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "list_box_section",
        title: "ListBoxSection performance",
        view: || view! { <list_box_section::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "list_box_section_heading",
        title: "ListBoxSectionHeading performance",
        view: || view! { <list_box_section_heading::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "menu",
        title: "Menu performance",
        view: || view! { <menu::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "menu_item",
        title: "MenuItem performance",
        view: || view! { <menu_item::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "menu_item_description",
        title: "MenuItemDescription performance",
        view: || view! { <menu_item_description::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "menu_item_label",
        title: "MenuItemLabel performance",
        view: || view! { <menu_item_label::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "menu_item_shortcut",
        title: "MenuItemShortcut performance",
        view: || view! { <menu_item_shortcut::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "menu_items",
        title: "MenuItems performance",
        view: || view! { <menu_items::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "menu_section",
        title: "MenuSection performance",
        view: || view! { <menu_section::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "menu_trigger",
        title: "MenuTrigger performance",
        view: || view! { <menu_trigger::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "meter",
        title: "Meter performance",
        view: || view! { <meter::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "meter_fill",
        title: "MeterFill performance",
        view: || view! { <meter_fill::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "meter_value_text",
        title: "MeterValueText performance",
        view: || view! { <meter_value_text::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "modal_backdrop",
        title: "ModalBackdrop performance",
        view: || view! { <modal_backdrop::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "modal_content",
        title: "ModalContent performance",
        view: || view! { <modal_content::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "number_field",
        title: "NumberField performance",
        view: || view! { <number_field::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "number_field_decrement_button",
        title: "NumberFieldDecrementButton performance",
        view: || view! { <number_field_decrement_button::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "number_field_group",
        title: "NumberFieldGroup performance",
        view: || view! { <number_field_group::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "number_field_increment_button",
        title: "NumberFieldIncrementButton performance",
        view: || view! { <number_field_increment_button::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "overlay_arrow",
        title: "OverlayArrow performance",
        view: || view! { <overlay_arrow::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "popover",
        title: "Popover performance",
        view: || view! { <popover::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "press_responder",
        title: "PressResponder performance",
        view: || view! { <press_responder::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "pressable",
        title: "Pressable performance",
        view: || view! { <pressable::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "progress_bar",
        title: "ProgressBar performance",
        view: || view! { <progress_bar::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "progress_bar_fill",
        title: "ProgressBarFill performance",
        view: || view! { <progress_bar_fill::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "progress_bar_value_text",
        title: "ProgressBarValueText performance",
        view: || view! { <progress_bar_value_text::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "radio_button",
        title: "RadioButton performance",
        view: || view! { <radio_button::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "radio_field",
        title: "RadioField performance",
        view: || view! { <radio_field::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "radio_group",
        title: "RadioGroup performance",
        view: || view! { <radio_group::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "range_calendar",
        title: "RangeCalendar performance",
        view: || view! { <range_calendar::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "resizable_table_container",
        title: "ResizableTableContainer performance",
        view: || view! { <resizable_table_container::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "search_field",
        title: "SearchField performance",
        view: || view! { <search_field::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "search_field_clear_button",
        title: "SearchFieldClearButton performance",
        view: || view! { <search_field_clear_button::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "select",
        title: "Select performance",
        view: || view! { <select::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "select_popover",
        title: "SelectPopover performance",
        view: || view! { <select_popover::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "select_trigger",
        title: "SelectTrigger performance",
        view: || view! { <select_trigger::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "select_value",
        title: "SelectValue performance",
        view: || view! { <select_value::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "separator",
        title: "Separator performance",
        view: || view! { <separator::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "shortcut_keys",
        title: "ShortcutKeys performance",
        view: || view! { <shortcut_keys::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "slider",
        title: "Slider performance",
        view: || view! { <slider::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "slider_fill",
        title: "SliderFill performance",
        view: || view! { <slider_fill::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "slider_mark",
        title: "SliderMark performance",
        view: || view! { <slider_mark::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "slider_marks",
        title: "SliderMarks performance",
        view: || view! { <slider_marks::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "slider_output",
        title: "SliderOutput performance",
        view: || view! { <slider_output::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "slider_thumb",
        title: "SliderThumb performance",
        view: || view! { <slider_thumb::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "slider_thumb_tooltip",
        title: "SliderThumbTooltip performance",
        view: || view! { <slider_thumb_tooltip::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "slider_track",
        title: "SliderTrack performance",
        view: || view! { <slider_track::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "submenu_trigger",
        title: "SubmenuTrigger performance",
        view: || view! { <submenu_trigger::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "switch_button",
        title: "SwitchButton performance",
        view: || view! { <switch_button::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "switch_field",
        title: "SwitchField performance",
        view: || view! { <switch_field::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "tab",
        title: "Tab performance",
        view: || view! { <tab::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "tab_list",
        title: "TabList performance",
        view: || view! { <tab_list::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "tab_panel",
        title: "TabPanel performance",
        view: || view! { <tab_panel::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "tab_panels",
        title: "TabPanels performance",
        view: || view! { <tab_panels::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "table",
        title: "Table performance",
        view: || view! { <table::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "table_body",
        title: "TableBody performance",
        view: || view! { <table_body::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "table_cell",
        title: "TableCell performance",
        view: || view! { <table_cell::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "table_expand_button",
        title: "TableExpandButton performance",
        view: || view! { <table_expand_button::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "table_header",
        title: "TableHeader performance",
        view: || view! { <table_header::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "table_row",
        title: "TableRow performance",
        view: || view! { <table_row::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "tabs",
        title: "Tabs performance",
        view: || view! { <tabs::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "tag",
        title: "Tag performance",
        view: || view! { <tag::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "tag_group",
        title: "TagGroup performance",
        view: || view! { <tag_group::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "tag_items",
        title: "TagItems performance",
        view: || view! { <tag_items::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "tag_list",
        title: "TagList performance",
        view: || view! { <tag_list::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "tag_remove_button",
        title: "TagRemoveButton performance",
        view: || view! { <tag_remove_button::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "text_area",
        title: "TextArea performance",
        view: || view! { <text_area::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "text_field",
        title: "TextField performance",
        view: || view! { <text_field::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "theme_provider",
        title: "ThemeProvider performance",
        view: || view! { <theme_provider::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "time_field",
        title: "TimeField performance",
        view: || view! { <time_field::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "toast",
        title: "Toast performance",
        view: || view! { <toast::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "toast_close_button",
        title: "ToastCloseButton performance",
        view: || view! { <toast_close_button::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "toast_content",
        title: "ToastContent performance",
        view: || view! { <toast_content::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "toast_description",
        title: "ToastDescription performance",
        view: || view! { <toast_description::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "toast_region",
        title: "ToastRegion performance",
        view: || view! { <toast_region::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "toast_title",
        title: "ToastTitle performance",
        view: || view! { <toast_title::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "toggle_button",
        title: "ToggleButton performance",
        view: || view! { <toggle_button::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "toggle_button_group",
        title: "ToggleButtonGroup performance",
        view: || view! { <toggle_button_group::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "toolbar",
        title: "Toolbar performance",
        view: || view! { <toolbar::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "tooltip",
        title: "Tooltip performance",
        view: || view! { <tooltip::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "tooltip_trigger",
        title: "TooltipTrigger performance",
        view: || view! { <tooltip_trigger::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "virtual_list",
        title: "VirtualList performance",
        view: || view! { <virtual_list::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "virtualizer",
        title: "Virtualizer performance",
        view: || view! { <virtualizer::PagePerf /> }.into_any(),
    },
    Fixture {
        group: "perf/atoms",
        name: "visually_hidden",
        title: "VisuallyHidden performance",
        view: || view! { <visually_hidden::PagePerf /> }.into_any(),
    },
];
