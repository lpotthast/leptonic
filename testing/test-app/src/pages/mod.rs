//! Test fixtures driven by the browser tests in `leptonic/tests/`.
//!
//! Every fixture is reachable at `/{group}/{name}`, where `group` is `atoms` or `hooks`. To add one, create a module with a page component and register it in
//! [`FIXTURES`].
//!
//! A fixture with several independent parts wraps each in a [`Section`]: loaded with
//! `?only=<name>,...`, the page renders only the sections named, so a test of one part renders
//! and hydrates only that part.

pub mod atoms;
pub mod hooks;

use leptos::{prelude::*, web_sys};
use leptos_router::hooks::use_query_map;

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
        name: "overlay-position-options",
        title: "Overlay position options",
        view: || {
            view! { <atoms::overlay_position_options::PageAtomOverlayPositionOptions /> }.into_any()
        },
    },
    Fixture {
        group: "atoms",
        name: "overlay-state",
        title: "Overlay state",
        view: || view! { <atoms::overlay_state::PageAtomOverlayState /> }.into_any(),
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
        name: "localized",
        title: "Localized atom texts (de-DE)",
        view: || view! { <atoms::localized::PageAtomLocalized /> }.into_any(),
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
        name: "visually-hidden",
        title: "Visually hidden atom",
        view: || view! { <atoms::visually_hidden::PageAtomVisuallyHidden /> }.into_any(),
    },
    Fixture {
        group: "atoms",
        name: "separator",
        title: "Separator atom",
        view: || view! { <atoms::separator::PageAtomSeparator /> }.into_any(),
    },
    Fixture {
        group: "atoms",
        name: "theme",
        title: "Theme provider",
        view: || view! { <atoms::theme::PageAtomTheme /> }.into_any(),
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
        name: "menu-states",
        title: "Menus without a trigger",
        view: || view! { <atoms::menu_states::PageAtomMenuStates /> }.into_any(),
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
        name: "virtual-list",
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
        group: "hooks",
        name: "landmark-nested",
        title: "Nested landmarks",
        view: || view! { <hooks::landmark_nested::PageHookLandmarkNested /> }.into_any(),
    },
    Fixture {
        group: "hooks",
        name: "overlay",
        title: "Overlay",
        view: || view! { <hooks::overlay::PageHookOverlay /> }.into_any(),
    },
    Fixture {
        group: "atoms",
        name: "dismiss-button",
        title: "Dismiss button",
        view: || view! { <atoms::dismiss_button::PageAtomDismissButton /> }.into_any(),
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
        name: "forms",
        title: "Forms",
        view: || view! { <atoms::forms::PageAtomForms /> }.into_any(),
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
        name: "clipboard-write",
        title: "Clipboard writes",
        view: || view! { <hooks::clipboard_write::PageHookClipboardWrite /> }.into_any(),
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
        name: "scroll",
        title: "Scroll utilities",
        view: || view! { <hooks::scroll::PageHookScroll /> }.into_any(),
    },
    Fixture {
        group: "hooks",
        name: "scroll-wheel",
        title: "Scroll wheel",
        view: || view! { <hooks::scroll_wheel::PageHookScrollWheel /> }.into_any(),
    },
    Fixture {
        group: "hooks",
        name: "tree",
        title: "Tree",
        view: || view! { <hooks::tree::PageHookTree /> }.into_any(),
    },
    Fixture {
        group: "hooks",
        name: "tree-cases",
        title: "Tree cases",
        view: || view! { <hooks::tree_cases::PageHookTreeCases /> }.into_any(),
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
        group: "atoms",
        name: "combobox-forms",
        title: "ComboBox forms",
        view: || view! { <atoms::combobox_forms::PageAtomComboBoxForms /> }.into_any(),
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
        name: "listbox-features",
        title: "ListBox features",
        view: || view! { <atoms::listbox_features::PageAtomListBoxFeatures /> }.into_any(),
    },
    Fixture {
        group: "atoms",
        name: "listbox-selection",
        title: "ListBox selection",
        view: || view! { <atoms::listbox_selection::PageAtomListBoxSelection /> }.into_any(),
    },
    Fixture {
        group: "atoms",
        name: "select",
        title: "Select",
        view: || view! { <atoms::select::PageAtomSelect /> }.into_any(),
    },
    Fixture {
        group: "atoms",
        name: "select-behavior",
        title: "Select behavior",
        view: || view! { <atoms::select_behavior::PageAtomSelectBehavior /> }.into_any(),
    },
    Fixture {
        group: "atoms",
        name: "select-forms",
        title: "Select forms",
        view: || view! { <atoms::select_forms::PageAtomSelectForms /> }.into_any(),
    },
    Fixture {
        group: "atoms",
        name: "grid-list",
        title: "GridList",
        view: || view! { <atoms::grid_list::PageAtomGridList /> }.into_any(),
    },
    Fixture {
        group: "atoms",
        name: "grid-list-cases",
        title: "GridList cases",
        view: || view! { <atoms::grid_list_cases::PageAtomGridListCases /> }.into_any(),
    },
    Fixture {
        group: "atoms",
        name: "grid-list-features",
        title: "GridList features",
        view: || view! { <atoms::grid_list_features::PageAtomGridListFeatures /> }.into_any(),
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
        name: "table-navigation",
        title: "Table navigation",
        view: || view! { <atoms::table_navigation::PageAtomTableNavigation /> }.into_any(),
    },
    Fixture {
        group: "atoms",
        name: "table-tree",
        title: "Tree table atoms",
        view: || view! { <atoms::table_tree::PageAtomTableTree /> }.into_any(),
    },
    Fixture {
        group: "atoms",
        name: "table-selection",
        title: "Table selection",
        view: || view! { <atoms::table_selection::PageAtomTableSelection /> }.into_any(),
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
        name: "date-picker",
        title: "Date pickers",
        view: || view! { <atoms::date_picker::PageAtomDatePicker /> }.into_any(),
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
        group: "atoms",
        name: "slider-interactions",
        title: "Slider interactions",
        view: || view! { <atoms::slider_interactions::PageAtomSliderInteractions /> }.into_any(),
    },
    Fixture {
        group: "hooks",
        name: "dnd",
        title: "Drag and drop",
        view: || view! { <hooks::dnd::PageHookDnd /> }.into_any(),
    },
    Fixture {
        group: "hooks",
        name: "dnd-targets",
        title: "Drag and drop targets",
        view: || view! { <hooks::dnd_targets::PageHookDndTargets /> }.into_any(),
    },
    Fixture {
        group: "hooks",
        name: "dnd-draggable-collection",
        title: "Draggable collections",
        view: || {
            view! { <hooks::dnd_draggable_collection::PageHookDndDraggableCollection /> }.into_any()
        },
    },
    Fixture {
        group: "hooks",
        name: "dnd-native",
        title: "Native drag and drop",
        view: || view! { <hooks::dnd_native::PageHookDndNative /> }.into_any(),
    },
    Fixture {
        group: "hooks",
        name: "dnd-collection",
        title: "Droppable collection",
        view: || view! { <hooks::dnd_collection::PageHookDndCollection /> }.into_any(),
    },
    Fixture {
        group: "hooks",
        name: "clipboard",
        title: "Clipboard",
        view: || view! { <hooks::clipboard::PageHookClipboard /> }.into_any(),
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

/// One part of a fixture page with several, named `name`. Rendered unless the page was loaded
/// with `?only=<name>,...` naming other sections only (`PageActions::goto_sections` in the tests).
/// Without `only` every section renders: the hydration test and manual inspection see the whole
/// page.
#[component]
pub fn Section(name: &'static str, children: Children) -> impl IntoView {
    // The query is read once: a section doesn't come and go while the page is open.
    let shown = use_query_map().with_untracked(|query| {
        query
            .get_str("only")
            .is_none_or(|only| only.split(',').any(|only| only == name))
    });
    shown.then(children)
}

/// Prevent mousedown from stealing focus (used on control buttons in focus-manager tests).
pub fn prevent_focus_steal(e: web_sys::MouseEvent) {
    e.prevent_default();
}
