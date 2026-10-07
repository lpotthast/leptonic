//! Test fixtures driven by the browser tests in `leptonic/tests/`.
//!
//! Every fixture is reachable at `/{group}/{name}`, where `group` is `atoms` or `hooks`. To add one, create a module with a page component and register it in
//! [`FIXTURES`].

pub mod atoms;
pub mod hooks;

use leptos::{prelude::*, web_sys};

/// A test page, reachable at `/{group}/{name}`.
pub struct Fixture {
    pub group: &'static str,
    pub name: &'static str,
    pub title: &'static str,
    pub view: fn() -> AnyView,
}

impl Fixture {
    pub fn path(&self) -> String {
        format!("/{}/{}", self.group, self.name)
    }
}

pub const FIXTURES: &[Fixture] = &[
    Fixture {
        group: "atoms",
        name: "overlay-position",
        title: "Overlay position",
        view: || view! { <atoms::overlay_position::PageAtomOverlayPosition /> }.into_any(),
    },
    Fixture {
        group: "atoms",
        name: "label-slots",
        title: "Label slots",
        view: || view! { <atoms::label_slots::PageAtomLabelSlots /> }.into_any(),
    },
    Fixture {
        group: "atoms",
        name: "link",
        title: "Link atoms",
        view: || view! { <atoms::link::PageAtomLink /> }.into_any(),
    },
    Fixture {
        group: "atoms",
        name: "breadcrumbs",
        title: "Breadcrumbs atoms",
        view: || view! { <atoms::breadcrumbs::PageAtomBreadcrumbs /> }.into_any(),
    },
    Fixture {
        group: "atoms",
        name: "focusable",
        title: "Focusable atom",
        view: || view! { <atoms::focusable::PageAtomFocusable /> }.into_any(),
    },
    Fixture {
        group: "atoms",
        name: "pressable",
        title: "Pressable atom",
        view: || view! { <atoms::pressable::PageAtomPressable /> }.into_any(),
    },
    Fixture {
        group: "atoms",
        name: "progress-bar",
        title: "Progress bar and meter atoms",
        view: || view! { <atoms::progress_bar::PageAtomProgressBar /> }.into_any(),
    },
    Fixture {
        group: "atoms",
        name: "submenu",
        title: "Submenu atoms",
        view: || view! { <atoms::submenu::PageAtomSubmenu /> }.into_any(),
    },
    Fixture {
        group: "atoms",
        name: "toolbar",
        title: "Toolbar atom",
        view: || view! { <atoms::toolbar::PageAtomToolbar /> }.into_any(),
    },
    Fixture {
        group: "atoms",
        name: "disclosure",
        title: "Disclosure atoms",
        view: || view! { <atoms::disclosure::PageAtomDisclosure /> }.into_any(),
    },
    Fixture {
        group: "atoms",
        name: "context-menu",
        title: "Context menus",
        view: || view! { <atoms::context_menu::PageAtomContextMenu /> }.into_any(),
    },
    Fixture {
        group: "atoms",
        name: "menu",
        title: "Menu atoms",
        view: || view! { <atoms::menu::PageAtomMenu /> }.into_any(),
    },
    Fixture {
        group: "atoms",
        name: "toast",
        title: "Toast",
        view: || view! { <atoms::toast::PageAtomToast /> }.into_any(),
    },
    Fixture {
        group: "atoms",
        name: "toast-single",
        title: "One toast at a time",
        view: || view! { <atoms::toast_single::PageAtomToastSingle /> }.into_any(),
    },
    Fixture {
        group: "atoms",
        name: "virtual_list",
        title: "VirtualList",
        view: || view! { <atoms::virtual_list::PageAtomVirtualList /> }.into_any(),
    },
    Fixture {
        group: "atoms",
        name: "virtualizer",
        title: "Virtualizer",
        view: || view! { <atoms::virtualizer::PageAtomVirtualizer /> }.into_any(),
    },
    Fixture {
        group: "atoms",
        name: "tooltip",
        title: "Tooltip",
        view: || view! { <atoms::tooltip::PageAtomTooltip /> }.into_any(),
    },
    Fixture {
        group: "atoms",
        name: "popover",
        title: "Popover",
        view: || view! { <atoms::popover::PageAtomPopover /> }.into_any(),
    },
    Fixture {
        group: "atoms",
        name: "dialog",
        title: "Dialog",
        view: || view! { <atoms::dialog::PageAtomDialog /> }.into_any(),
    },
    Fixture {
        group: "atoms",
        name: "checkbox",
        title: "Checkbox",
        view: || view! { <atoms::checkbox::PageAtomCheckbox /> }.into_any(),
    },
    Fixture {
        group: "atoms",
        name: "radio-group",
        title: "Radio Group",
        view: || view! { <atoms::radio_group::PageAtomRadioGroup /> }.into_any(),
    },
    Fixture {
        group: "atoms",
        name: "text-field",
        title: "TextField",
        view: || view! { <atoms::text_field::PageAtomTextField /> }.into_any(),
    },
    Fixture {
        group: "atoms",
        name: "number-field",
        title: "NumberField",
        view: || view! { <atoms::number_field::PageAtomNumberField /> }.into_any(),
    },
    Fixture {
        group: "atoms",
        name: "search-field",
        title: "SearchField",
        view: || view! { <atoms::search_field::PageAtomSearchField /> }.into_any(),
    },
    Fixture {
        group: "atoms",
        name: "switch",
        title: "Switch",
        view: || view! { <atoms::switch::PageAtomSwitch /> }.into_any(),
    },
    Fixture {
        group: "atoms",
        name: "toggle-button",
        title: "Toggle Button",
        view: || view! { <atoms::toggle_button::PageAtomToggleButton /> }.into_any(),
    },
    Fixture {
        group: "hooks",
        name: "global-shortcuts",
        title: "use_global_shortcuts",
        view: || view! { <hooks::global_shortcuts::PageHookGlobalShortcuts /> }.into_any(),
    },
    Fixture {
        group: "hooks",
        name: "landmark",
        title: "use_landmark",
        view: || view! { <hooks::landmark::PageHookLandmark /> }.into_any(),
    },
    Fixture {
        group: "hooks",
        name: "spin-button",
        title: "use_spin_button",
        view: || view! { <hooks::spin_button::PageHookSpinButton /> }.into_any(),
    },
    Fixture {
        group: "hooks",
        name: "aria-hide-outside",
        title: "aria_hide_outside",
        view: || view! { <hooks::aria_hide_outside::PageHookAriaHideOutside /> }.into_any(),
    },
    Fixture {
        group: "hooks",
        name: "tree",
        title: "Tree",
        view: || view! { <hooks::tree::PageHookTree /> }.into_any(),
    },
    Fixture {
        group: "hooks",
        name: "tag-group",
        title: "Tag Group",
        view: || view! { <hooks::tag_group::PageHookTagGroup /> }.into_any(),
    },
    Fixture {
        group: "atoms",
        name: "color-area",
        title: "Color area atoms",
        view: || view! { <atoms::color_area::PageAtomColorArea /> }.into_any(),
    },
    Fixture {
        group: "atoms",
        name: "color-field",
        title: "Color field atoms",
        view: || view! { <atoms::color_field::PageAtomColorField /> }.into_any(),
    },
    Fixture {
        group: "atoms",
        name: "color-picker",
        title: "Color picker atom",
        view: || view! { <atoms::color_picker::PageAtomColorPicker /> }.into_any(),
    },
    Fixture {
        group: "atoms",
        name: "color-slider",
        title: "Color slider atoms",
        view: || view! { <atoms::color_slider::PageAtomColorSlider /> }.into_any(),
    },
    Fixture {
        group: "atoms",
        name: "color-swatch",
        title: "Color swatch atoms",
        view: || view! { <atoms::color_swatch::PageAtomColorSwatch /> }.into_any(),
    },
    Fixture {
        group: "atoms",
        name: "color-wheel",
        title: "Color wheel atoms",
        view: || view! { <atoms::color_wheel::PageAtomColorWheel /> }.into_any(),
    },
    Fixture {
        group: "atoms",
        name: "combobox",
        title: "ComboBox",
        view: || view! { <atoms::combobox::PageAtomComboBox /> }.into_any(),
    },
    Fixture {
        group: "hooks",
        name: "text-field",
        title: "Text Field",
        view: || view! { <hooks::text_field::PageHookTextField /> }.into_any(),
    },
    Fixture {
        group: "atoms",
        name: "button",
        title: "Button",
        view: || view! { <atoms::button::PageAtomButton /> }.into_any(),
    },
    Fixture {
        group: "atoms",
        name: "focus-scope",
        title: "Focus Scope",
        view: || view! { <atoms::focus_scope::PageAtomFocusScope /> }.into_any(),
    },
    Fixture {
        group: "hooks",
        name: "focus",
        title: "Focus",
        view: || view! { <hooks::focus::PageHookFocus /> }.into_any(),
    },
    Fixture {
        group: "hooks",
        name: "focus-within",
        title: "Focus Within",
        view: || view! { <hooks::focus_within::PageHookFocusWithin /> }.into_any(),
    },
    Fixture {
        group: "hooks",
        name: "focus-ring",
        title: "Focus Ring",
        view: || view! { <hooks::focus_ring::PageHookFocusRing /> }.into_any(),
    },
    Fixture {
        group: "hooks",
        name: "focusable",
        title: "Focusable",
        view: || view! { <hooks::focusable::PageHookFocusable /> }.into_any(),
    },
    Fixture {
        group: "hooks",
        name: "focus-manager",
        title: "Focus Manager",
        view: || view! { <hooks::focus_manager::PageHookFocusManager /> }.into_any(),
    },
    Fixture {
        group: "hooks",
        name: "focus-visible",
        title: "Focus Visible",
        view: || view! { <hooks::focus_visible::PageHookFocusVisible /> }.into_any(),
    },
    Fixture {
        group: "hooks",
        name: "has-tabbable-child",
        title: "Has Tabbable Child",
        view: || view! { <hooks::has_tabbable_child::PageHookHasTabbableChild /> }.into_any(),
    },
    Fixture {
        group: "hooks",
        name: "context-menu",
        title: "Context menu",
        view: || view! { <hooks::context_menu::PageHookContextMenu /> }.into_any(),
    },
    Fixture {
        group: "hooks",
        name: "focus-safely",
        title: "Focus safely",
        view: || view! { <hooks::focus_safely::PageHookFocusSafely /> }.into_any(),
    },
    Fixture {
        group: "hooks",
        name: "hover",
        title: "Hover",
        view: || view! { <hooks::hover::PageHookHover /> }.into_any(),
    },
    Fixture {
        group: "hooks",
        name: "interact-outside",
        title: "Interact outside",
        view: || view! { <hooks::interact_outside::PageHookInteractOutside /> }.into_any(),
    },
    Fixture {
        group: "hooks",
        name: "keyboard",
        title: "Keyboard",
        view: || view! { <hooks::keyboard::PageHookKeyboard /> }.into_any(),
    },
    Fixture {
        group: "hooks",
        name: "long-press",
        title: "Long press",
        view: || view! { <hooks::long_press::PageHookLongPress /> }.into_any(),
    },
    Fixture {
        group: "hooks",
        name: "move",
        title: "Move",
        view: || view! { <hooks::move_hook::PageHookMove /> }.into_any(),
    },
    Fixture {
        group: "hooks",
        name: "press",
        title: "Press",
        view: || view! { <hooks::press::PageHookPress /> }.into_any(),
    },
    Fixture {
        group: "hooks",
        name: "button",
        title: "Button",
        view: || view! { <hooks::button::PageHookButton /> }.into_any(),
    },
    Fixture {
        group: "hooks",
        name: "menu-trigger",
        title: "Menu Trigger",
        view: || view! { <hooks::menu_trigger::PageHookMenuTrigger /> }.into_any(),
    },
    Fixture {
        group: "hooks",
        name: "number-field",
        title: "Number Field",
        view: || view! { <hooks::number_field::PageHookNumberField /> }.into_any(),
    },
    Fixture {
        group: "hooks",
        name: "live-announcer",
        title: "Live Announcer",
        view: || view! { <hooks::live_announcer::PageLiveAnnouncer /> }.into_any(),
    },
    Fixture {
        group: "atoms",
        name: "listbox",
        title: "ListBox",
        view: || view! { <atoms::listbox::PageAtomListBox /> }.into_any(),
    },
    Fixture {
        group: "atoms",
        name: "select",
        title: "Select",
        view: || view! { <atoms::select::PageAtomSelect /> }.into_any(),
    },
    Fixture {
        group: "atoms",
        name: "grid-list",
        title: "GridList",
        view: || view! { <atoms::grid_list::PageAtomGridList /> }.into_any(),
    },
    Fixture {
        group: "atoms",
        name: "grid",
        title: "Grid",
        view: || view! { <atoms::grid::PageAtomGrid /> }.into_any(),
    },
    Fixture {
        group: "atoms",
        name: "table",
        title: "Table",
        view: || view! { <atoms::table::PageAtomTable /> }.into_any(),
    },
    Fixture {
        group: "atoms",
        name: "table-resizing",
        title: "Table column resizing",
        view: || view! { <atoms::table_resizing::PageAtomTableResizing /> }.into_any(),
    },
    Fixture {
        group: "atoms",
        name: "date-field",
        title: "Date and time fields",
        view: || view! { <atoms::date_field::PageAtomDateField /> }.into_any(),
    },
    Fixture {
        group: "atoms",
        name: "calendar",
        title: "Calendar",
        view: || view! { <atoms::calendar::PageAtomCalendar /> }.into_any(),
    },
    Fixture {
        group: "atoms",
        name: "tag-group",
        title: "TagGroup",
        view: || view! { <atoms::tag_group::PageAtomTagGroup /> }.into_any(),
    },
    Fixture {
        group: "atoms",
        name: "tabs",
        title: "Tabs",
        view: || view! { <atoms::tabs::PageAtomTabs /> }.into_any(),
    },
    Fixture {
        group: "atoms",
        name: "slider",
        title: "Slider",
        view: || view! { <atoms::slider::PageAtomSlider /> }.into_any(),
    },
    Fixture {
        group: "hooks",
        name: "dnd",
        title: "Drag and drop",
        view: || view! { <hooks::dnd::PageHookDnd /> }.into_any(),
    },
    Fixture {
        group: "hooks",
        name: "menu",
        title: "Menu",
        view: || view! { <hooks::menu::PageHookMenu /> }.into_any(),
    },
];

pub fn find_fixture(group: &str, name: &str) -> Option<&'static Fixture> {
    FIXTURES
        .iter()
        .find(|fixture| fixture.group == group && fixture.name == name)
}

/// Prevent mousedown from stealing focus (used on control buttons in focus-manager tests).
pub fn prevent_focus_steal(e: web_sys::MouseEvent) {
    e.prevent_default();
}
