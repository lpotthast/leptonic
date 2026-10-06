//! The documentation's navigation: the single definition of which pages exist, how they are grouped, and what kind of
//! page each one is.
//!
//! Drives the sidebar ([`DocLayout`](crate::pages::documentation::doc_layout::DocLayout)), the tabs of concept pages
//! ([`ConceptLayout`](crate::pages::documentation::concept_layout::ConceptLayout)) and the page classification of the
//! Markdown export. Adding a page means adding a route in `routes.rs` and an entry here.

use std::sync::LazyLock;

use leptonic::prelude::icondata::{self, Icon};

use crate::routes::doc;

/// What a documentation page is about. See "Page Types" in `documentation/documentation-strategy.md`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PageKind {
    /// Getting-started pages and other guides.
    Guide,
    /// Overview of a behavioral domain (Interactions, Focus, ...) or a category (Input, Layout, ...).
    Domain,
    /// Overview of a concept implemented at several layers (Button, Slider, ...).
    Concept,
    Hook,
    Atom,
    Component,
}

impl PageKind {
    /// Name of the kind, e.g. in the member tables of section overviews.
    pub fn label(self) -> &'static str {
        match self {
            Self::Guide => "Guide",
            Self::Domain => "Overview",
            Self::Concept => "Concept",
            Self::Hook => "Hook",
            Self::Atom => "Atom",
            Self::Component => "Component",
        }
    }

    /// Short label of a layer, shown as a badge next to standalone pages in the sidebar.
    pub fn badge(self) -> Option<&'static str> {
        match self {
            Self::Hook => Some("hook"),
            Self::Atom => Some("atom"),
            Self::Component => Some("comp"),
            Self::Guide | Self::Domain | Self::Concept => None,
        }
    }
}

/// A group of pages in the sidebar.
pub struct NavSection {
    pub title: &'static str,
    pub icon: Icon,
    /// The section's overview page. Getting started has none.
    pub overview: Option<String>,
    /// Whether entries are standalone pages of different layers, which the sidebar marks with a badge. Members of a
    /// behavioral domain are shown without badges.
    pub badges: bool,
    pub entries: Vec<NavEntry>,
}

/// A page listed in the sidebar.
pub struct NavEntry {
    pub title: &'static str,
    /// One line on what the page covers, without a trailing period. Shown in the member tables of section overviews.
    pub summary: &'static str,
    pub href: String,
    pub kind: PageKind,
    /// Sub-pages of a concept, shown as tabs next to its overview (which is not listed here).
    pub tabs: Vec<NavTab>,
    /// Pages belonging to this entry that are reached through its content instead of the sidebar.
    pub hidden: Vec<NavTab>,
}

/// A page that is not listed in the sidebar itself: a concept tab, or a page reached through another page.
pub struct NavTab {
    pub label: &'static str,
    pub href: String,
    pub kind: PageKind,
}

pub struct Nav {
    pub sections: Vec<NavSection>,
}

/// The navigation of the documentation.
pub fn nav() -> &'static Nav {
    static NAV: LazyLock<Nav> = LazyLock::new(build);
    &NAV
}

impl Nav {
    /// The kind of the page at `path`, if it is part of the navigation.
    pub fn page_kind(&self, path: &str) -> Option<PageKind> {
        self.sections.iter().find_map(|section| {
            if section.overview.as_deref() == Some(path) {
                return Some(PageKind::Domain);
            }
            section.entries.iter().find_map(|entry| {
                if entry.href == path {
                    return Some(entry.kind);
                }
                entry
                    .tabs
                    .iter()
                    .chain(&entry.hidden)
                    .find(|tab| tab.href == path)
                    .map(|tab| tab.kind)
            })
        })
    }

    /// The concept whose overview or tab is shown at `path`.
    pub fn concept_at(&self, path: &str) -> Option<&NavEntry> {
        self.entries().find(|entry| {
            entry.kind == PageKind::Concept
                && (entry.href == path || entry.tabs.iter().any(|tab| tab.href == path))
        })
    }

    /// Every page of the documentation.
    pub fn pages(&self) -> impl Iterator<Item = &str> {
        self.sections.iter().flat_map(|section| {
            section
                .overview
                .iter()
                .map(String::as_str)
                .chain(section.entries.iter().flat_map(|entry| {
                    std::iter::once(entry.href.as_str()).chain(
                        entry
                            .tabs
                            .iter()
                            .chain(&entry.hidden)
                            .map(|tab| tab.href.as_str()),
                    )
                }))
        })
    }

    fn entries(&self) -> impl Iterator<Item = &NavEntry> {
        self.sections.iter().flat_map(|section| &section.entries)
    }
}

fn section(
    title: &'static str,
    icon: Icon,
    overview: Option<String>,
    badges: bool,
    entries: Vec<NavEntry>,
) -> NavSection {
    NavSection {
        title,
        icon,
        overview,
        badges,
        entries,
    }
}

fn page(title: &'static str, summary: &'static str, href: String, kind: PageKind) -> NavEntry {
    NavEntry {
        title,
        summary,
        href,
        kind,
        tabs: Vec::new(),
        hidden: Vec::new(),
    }
}

fn concept(
    title: &'static str,
    summary: &'static str,
    href: String,
    tabs: Vec<NavTab>,
) -> NavEntry {
    NavEntry {
        title,
        summary,
        href,
        kind: PageKind::Concept,
        tabs,
        hidden: Vec::new(),
    }
}

fn tab(label: &'static str, href: String, kind: PageKind) -> NavTab {
    NavTab { label, href, kind }
}

impl NavEntry {
    fn with_hidden(mut self, hidden: Vec<NavTab>) -> Self {
        self.hidden = hidden;
        self
    }
}

#[allow(clippy::too_many_lines)]
fn build() -> Nav {
    use PageKind::{Atom, Component, Guide, Hook};

    let hook = |href: String| tab("Hook", href, Hook);
    let hooks = |href: String| tab("Hooks", href, Hook);
    let atom = |href: String| tab("Atom", href, Atom);
    let component = |href: String| tab("Component", href, Component);

    Nav {
        sections: vec![
            section(
                "Getting started",
                icondata::BsBook,
                None,
                false,
                vec![
                    page(
                        "Overview",
                        "What leptonic is and how the documentation is organized",
                        doc::Overview.materialize(),
                        Guide,
                    ),
                    page(
                        "Installation",
                        "Adds leptonic to a Leptos app, with styles and the theme",
                        doc::Installation.materialize(),
                        Guide,
                    ),
                    page(
                        "Themes",
                        "Light and dark themes, switching between them and customizing them",
                        doc::Themes.materialize(),
                        Guide,
                    ),
                    page(
                        "Changelog",
                        "Changes of every release, with migration notes",
                        doc::Changelog.materialize(),
                        Guide,
                    ),
                    page(
                        "Event Propagation",
                        "Why leptonic events stop propagating and how to let them bubble",
                        doc::EventPropagation.materialize(),
                        Guide,
                    ),
                    page(
                        "Hooks, Atoms & Components",
                        "The three layers of leptonic and when to use which",
                        doc::Architecture.materialize(),
                        Guide,
                    ),
                    page(
                        "Classes & Styles",
                        "Passing classes and typed styles to atoms and components",
                        doc::ClassesAndStyles.materialize(),
                        Guide,
                    ),
                    page(
                        "Forms & Validation",
                        "Form fields, validation behavior and error messages",
                        doc::Forms.materialize(),
                        Guide,
                    ),
                ],
            ),
            section(
                "Interactions",
                icondata::BsCursor,
                Some(doc::Interactions.materialize()),
                false,
                vec![
                    page(
                        "use_press",
                        "Press interactions across mouse, touch, keyboard and screen readers, including long press",
                        doc::interactions::UsePress.materialize(),
                        Hook,
                    ),
                    page(
                        "PressResponder",
                        "Passes press handling to a pressable descendant, e.g. a menu or dialog trigger",
                        doc::interactions::PressResponder.materialize(),
                        Atom,
                    ),
                    page(
                        "Hoverable",
                        "Adds hover tracking to its child without rendering an element",
                        doc::interactions::Hoverable.materialize(),
                        Atom,
                    ),
                    page(
                        "use_hover",
                        "Tracks whether a mouse or pen hovers an element, for tooltips and highlights",
                        doc::interactions::UseHover.materialize(),
                        Hook,
                    ),
                    page(
                        "use_move",
                        "Reports pointer drags and arrow keys as movement, for sliders and color areas",
                        doc::interactions::UseMove.materialize(),
                        Hook,
                    ),
                    page(
                        "use_keyboard",
                        "Key events and keyboard shortcuts on an element",
                        doc::interactions::UseKeyboard.materialize(),
                        Hook,
                    ),
                    page(
                        "use_interact_outside",
                        "Detects interactions outside an element, to dismiss popovers and menus",
                        doc::interactions::UseInteractOutside.materialize(),
                        Hook,
                    ),
                    page(
                        "use_scroll_wheel",
                        "Scroll wheel events without scrolling the page",
                        doc::interactions::UseScrollWheel.materialize(),
                        Hook,
                    ),
                    page(
                        "use_prevent_scroll",
                        "Locks page scrolling while a modal or overlay is open",
                        doc::interactions::UsePreventScroll.materialize(),
                        Hook,
                    ),
                    page(
                        "Drag & Drop",
                        "Draggable elements, drop targets and reorderable collections, with keyboard support",
                        doc::interactions::Dnd.materialize(),
                        Hook,
                    ),
                ],
            ),
            section(
                "Focus",
                icondata::BsEye,
                Some(doc::Focus.materialize()),
                false,
                vec![
                    page(
                        "use_focus",
                        "Tracks focus and blur of an element",
                        doc::focus::UseFocus.materialize(),
                        Hook,
                    ),
                    page(
                        "use_focus_within",
                        "Tracks whether focus is inside a container",
                        doc::focus::UseFocusWithin.materialize(),
                        Hook,
                    ),
                    page(
                        "use_focusable",
                        "Makes a custom element focusable, with keyboard events and programmatic focus",
                        doc::focus::UseFocusable.materialize(),
                        Hook,
                    ),
                    page(
                        "use_focus_manager",
                        "Moves focus to the next, previous, first or last element of a container",
                        doc::focus::UseFocusManager.materialize(),
                        Hook,
                    ),
                    page(
                        "use_has_tabbable_child",
                        "Tells whether a container has tabbable descendants",
                        doc::focus::UseHasTabbableChild.materialize(),
                        Hook,
                    ),
                    page(
                        "use_focus_ring",
                        "Tells whether an element should show a keyboard focus ring",
                        doc::focus::UseFocusRing.materialize(),
                        Hook,
                    ),
                    page(
                        "use_focus_visible",
                        "Tracks whether the user navigates with the keyboard",
                        doc::focus::UseFocusVisible.materialize(),
                        Hook,
                    ),
                    page(
                        "FocusScope",
                        "Contains, restores and auto-focuses focus within a subtree, e.g. a dialog",
                        doc::focus::FocusScope.materialize(),
                        Atom,
                    ),
                    page(
                        "FocusRing",
                        "Marks its child with data-focus-visible while it has keyboard focus",
                        doc::focus::FocusRing.materialize(),
                        Atom,
                    ),
                    page(
                        "FocusManager",
                        "A container that hands a focus manager to its children",
                        doc::focus::FocusManager.materialize(),
                        Atom,
                    ),
                ],
            ),
            section(
                "Overlays",
                icondata::BsWindowStack,
                Some(doc::Overlays.materialize()),
                false,
                vec![
                    page(
                        "Overlay Hooks",
                        "Dismissing, positioning and trigger attributes of floating content",
                        doc::overlays::UseOverlay.materialize(),
                        Hook,
                    ),
                    page(
                        "DismissButton",
                        "A visually hidden button that lets screen reader users dismiss an overlay",
                        doc::overlays::DismissButton.materialize(),
                        Atom,
                    ),
                    page(
                        "Animation Hooks",
                        "Enter and exit animation states for elements that mount and unmount",
                        doc::hooks::Animation.materialize(),
                        Hook,
                    ),
                    page(
                        "Transitions",
                        "Ready-made collapse, fade, grow, slide and zoom transitions",
                        doc::components::Transitions.materialize(),
                        Component,
                    ),
                ],
            ),
            section(
                "Collections",
                icondata::BsListUl,
                Some(doc::Collections.materialize()),
                false,
                vec![],
            ),
            section(
                "Input",
                icondata::BsToggles,
                Some(doc::InputCategory.materialize()),
                true,
                vec![
                    concept(
                        "Button",
                        "Triggers an action, such as submit, delete or open",
                        doc::Button.materialize(),
                        vec![
                            hook(doc::button::Hook.materialize()),
                            atom(doc::button::Atom.materialize()),
                            component(doc::button::Component.materialize()),
                        ],
                    ),
                    concept(
                        "Checkbox",
                        "Turns an independent option on or off",
                        doc::Checkbox.materialize(),
                        vec![
                            hook(doc::checkbox::Hook.materialize()),
                            atom(doc::checkbox::Atom.materialize()),
                            component(doc::checkbox::Component.materialize()),
                        ],
                    ),
                    concept(
                        "Color",
                        "Picks colors with areas, sliders, wheels, swatches and fields",
                        doc::Color.materialize(),
                        vec![
                            hooks(doc::color::Hooks.materialize()),
                            atom(doc::color::Atom.materialize()),
                            component(doc::color::Component.materialize()),
                        ],
                    )
                    .with_hidden(vec![
                        tab(
                            "use_color_area",
                            doc::hooks::UseColorArea.materialize(),
                            Hook,
                        ),
                        tab(
                            "use_color_slider",
                            doc::hooks::UseColorSlider.materialize(),
                            Hook,
                        ),
                        tab(
                            "use_color_wheel",
                            doc::hooks::UseColorWheel.materialize(),
                            Hook,
                        ),
                        tab(
                            "use_color_field",
                            doc::hooks::UseColorField.materialize(),
                            Hook,
                        ),
                        tab(
                            "use_color_swatch",
                            doc::hooks::UseColorSwatch.materialize(),
                            Hook,
                        ),
                        tab(
                            "use_color_channel_field",
                            doc::hooks::UseColorChannelField.materialize(),
                            Hook,
                        ),
                    ]),
                    concept(
                        "Combobox",
                        "A text field with a filtered list of options, for large option sets",
                        doc::Combobox.materialize(),
                        vec![
                            hook(doc::combobox::Hook.materialize()),
                            atom(doc::combobox::Atom.materialize()),
                        ],
                    ),
                    concept(
                        "Date & Time",
                        "Calendars, date and time fields, and date pickers",
                        doc::DateTime.materialize(),
                        vec![
                            tab(
                                "Calendar Hooks",
                                doc::date_time::CalendarHooks.materialize(),
                                Hook,
                            ),
                            tab(
                                "Date Field Hooks",
                                doc::date_time::DateFieldHooks.materialize(),
                                Hook,
                            ),
                            tab(
                                "Date Picker Hooks",
                                doc::date_time::DatePickerHooks.materialize(),
                                Hook,
                            ),
                            component(doc::date_time::Component.materialize()),
                        ],
                    ),
                    concept(
                        "Listbox",
                        "A list of options that are all visible at once, with single or multiple selection",
                        doc::Listbox.materialize(),
                        vec![
                            hook(doc::listbox::Hook.materialize()),
                            atom(doc::listbox::Atom.materialize()),
                        ],
                    ),
                    concept(
                        "Radio",
                        "Picks exactly one option of a small, visible set",
                        doc::Radio.materialize(),
                        vec![
                            hook(doc::radio::Hook.materialize()),
                            atom(doc::radio::Atom.materialize()),
                            component(doc::radio::Component.materialize()),
                        ],
                    ),
                    concept(
                        "Select",
                        "Picks an option from a dropdown, where space is limited",
                        doc::Select.materialize(),
                        vec![
                            hook(doc::select::Hook.materialize()),
                            atom(doc::select::Atom.materialize()),
                            component(doc::select::Component.materialize()),
                        ],
                    ),
                    concept(
                        "Slider",
                        "Picks a number, or a range, by dragging along a track",
                        doc::Slider.materialize(),
                        vec![
                            hook(doc::slider::Hook.materialize()),
                            atom(doc::slider::Atom.materialize()),
                            component(doc::slider::Component.materialize()),
                        ],
                    ),
                    concept(
                        "Text Field",
                        "Single-line and multi-line text input",
                        doc::TextField.materialize(),
                        vec![
                            hook(doc::text_field::Hook.materialize()),
                            tab(
                                "Number Field Hook",
                                doc::text_field::NumberFieldHook.materialize(),
                                Hook,
                            ),
                            atom(doc::text_field::Atom.materialize()),
                            tab(
                                "Number Field Atoms",
                                doc::text_field::NumberFieldAtom.materialize(),
                                Atom,
                            ),
                            component(doc::text_field::Component.materialize()),
                        ],
                    ),
                    concept(
                        "Switch",
                        "Switches a setting on or off, with immediate effect",
                        doc::Switch.materialize(),
                        vec![
                            hook(doc::switch::Hook.materialize()),
                            atom(doc::switch::Atom.materialize()),
                            component(doc::switch::Component.materialize()),
                        ],
                    ),
                    concept(
                        "Toggle Button",
                        "A button that stays pressed, alone or in a group of options",
                        doc::ToggleButton.materialize(),
                        vec![
                            hook(doc::toggle_button::Hook.materialize()),
                            atom(doc::toggle_button::Atom.materialize()),
                        ],
                    ),
                    page(
                        "Field Atoms",
                        "Label, description and error message of any field atom",
                        doc::atoms::Field.materialize(),
                        Atom,
                    ),
                    page(
                        "Form Atom",
                        "A form whose fields share a validation behavior and show server errors",
                        doc::atoms::Form.materialize(),
                        Atom,
                    ),
                    page(
                        "Tiptap Editor",
                        "A rich text editor based on Tiptap",
                        doc::components::TiptapEditor.materialize(),
                        Component,
                    ),
                    page(
                        "use_label",
                        "Associates a label, description and error message with a field",
                        doc::hooks::UseLabel.materialize(),
                        Hook,
                    ),
                    page(
                        "use_spin_button",
                        "Steps a number up and down with the keyboard and hold-to-spin buttons",
                        doc::hooks::UseSpinButton.materialize(),
                        Hook,
                    ),
                ],
            ),
            section(
                "Data Display",
                icondata::BsGrid,
                Some(doc::DataDisplay.materialize()),
                true,
                vec![
                    concept(
                        "Grid",
                        "Rows and cells navigated in two dimensions, with row selection",
                        doc::Grid.materialize(),
                        vec![
                            hook(doc::grid::Hook.materialize()),
                            atom(doc::grid::Atom.materialize()),
                            component(doc::grid::Component.materialize()),
                        ],
                    ),
                    concept(
                        "Table",
                        "Columns of data with sortable headers and selectable rows",
                        doc::Table.materialize(),
                        vec![
                            hook(doc::table::Hook.materialize()),
                            atom(doc::table::Atom.materialize()),
                            component(doc::table::Component.materialize()),
                        ],
                    ),
                    page(
                        "use_tree",
                        "Hierarchical items with expandable and collapsible children",
                        doc::hooks::UseTree.materialize(),
                        Hook,
                    ),
                ],
            ),
            section(
                "Layout",
                icondata::BsColumnsGap,
                Some(doc::LayoutCategory.materialize()),
                true,
                vec![
                    concept(
                        "Collapsible",
                        "Content that expands and collapses under a header",
                        doc::Collapsible.materialize(),
                        vec![
                            hook(doc::collapsible::Hook.materialize()),
                            component(doc::collapsible::Component.materialize()),
                        ],
                    ),
                    concept(
                        "Separator",
                        "A visual divider between groups of content",
                        doc::Separator.materialize(),
                        vec![
                            hook(doc::separator::Hook.materialize()),
                            component(doc::separator::Component.materialize()),
                        ],
                    ),
                    concept(
                        "Tabs",
                        "Switches between panels that share the same space",
                        doc::Tabs.materialize(),
                        vec![
                            hook(doc::tabs::Hook.materialize()),
                            atom(doc::tabs::Atom.materialize()),
                            component(doc::tabs::Component.materialize()),
                        ],
                    ),
                    page(
                        "App Bar",
                        "The application's top bar",
                        doc::components::AppBar.materialize(),
                        Component,
                    ),
                    page(
                        "Drawer",
                        "A side panel that slides in",
                        doc::components::Drawer.materialize(),
                        Component,
                    ),
                    page(
                        "Skeleton",
                        "Placeholder shapes while content loads",
                        doc::components::Skeleton.materialize(),
                        Component,
                    ),
                    page(
                        "Stack",
                        "Stacks elements vertically or horizontally with even spacing",
                        doc::components::Stack.materialize(),
                        Component,
                    ),
                    page(
                        "use_toolbar",
                        "A group of controls navigated with the arrow keys",
                        doc::hooks::UseToolbar.materialize(),
                        Hook,
                    ),
                ],
            ),
            section(
                "Feedback",
                icondata::BsChatSquare,
                Some(doc::Feedback.materialize()),
                true,
                vec![
                    concept(
                        "Chip",
                        "Compact tags, filters and status labels that can be removed",
                        doc::Chip.materialize(),
                        vec![
                            hook(doc::chip::Hook.materialize()),
                            component(doc::chip::Component.materialize()),
                        ],
                    ),
                    concept(
                        "Modal",
                        "A dialog that blocks the page until the user responds",
                        doc::Modal.materialize(),
                        vec![
                            hook(doc::modal::Hook.materialize()),
                            atom(doc::modal::Atom.materialize()),
                            component(doc::modal::Component.materialize()),
                        ],
                    ),
                    concept(
                        "Popover",
                        "Content anchored to a trigger element",
                        doc::Popover.materialize(),
                        vec![
                            hook(doc::popover::Hook.materialize()),
                            atom(doc::popover::Atom.materialize()),
                            component(doc::popover::Component.materialize()),
                        ],
                    ),
                    concept(
                        "Progress",
                        "Shows the progress of an ongoing task, determinate or not",
                        doc::Progress.materialize(),
                        vec![
                            hook(doc::progress::Hook.materialize()),
                            component(doc::progress::Component.materialize()),
                        ],
                    ),
                    concept(
                        "Tooltip",
                        "A short hint shown when a button or icon is hovered or focused",
                        doc::Tooltip.materialize(),
                        vec![
                            hook(doc::tooltip::Hook.materialize()),
                            atom(doc::tooltip::Atom.materialize()),
                        ],
                    ),
                    page(
                        "Alert",
                        "A prominent status message",
                        doc::components::Alert.materialize(),
                        Component,
                    ),
                    page(
                        "Kbd",
                        "Displays keys and keyboard shortcuts",
                        doc::components::Kbd.materialize(),
                        Component,
                    ),
                    page(
                        "Toast",
                        "Temporary notifications",
                        doc::components::Toast.materialize(),
                        Component,
                    ),
                    page(
                        "use_meter",
                        "Shows a value within a known range, such as disk usage",
                        doc::hooks::UseMeter.materialize(),
                        Hook,
                    ),
                ],
            ),
            section(
                "Navigation",
                icondata::BsSignpost,
                Some(doc::Navigation.materialize()),
                true,
                vec![
                    concept(
                        "Link",
                        "Navigates to a URL or an anchor on the page",
                        doc::Link.materialize(),
                        vec![
                            tab("use_link", doc::link::UseLink.materialize(), Hook),
                            tab(
                                "use_anchor_link",
                                doc::link::UseAnchorLink.materialize(),
                                Hook,
                            ),
                            tab("Link Atom", doc::link::LinkAtom.materialize(), Atom),
                            tab(
                                "AnchorLink Atom",
                                doc::link::AnchorLinkAtom.materialize(),
                                Atom,
                            ),
                        ],
                    ),
                    concept(
                        "Menu",
                        "A list of actions opened from a trigger",
                        doc::Menu.materialize(),
                        vec![
                            hook(doc::menu::Hook.materialize()),
                            atom(doc::menu::Atom.materialize()),
                        ],
                    ),
                    page(
                        "use_breadcrumbs",
                        "A trail of links to the current page's ancestors",
                        doc::hooks::UseBreadcrumbs.materialize(),
                        Hook,
                    ),
                ],
            ),
            section(
                "General",
                icondata::BsCircleSquare,
                Some(doc::General.materialize()),
                true,
                vec![
                    page(
                        "Typography",
                        "Headings, paragraphs and code with the theme's text styles",
                        doc::components::Typography.materialize(),
                        Component,
                    ),
                    page(
                        "Icon",
                        "Renders icons from the icondata sets",
                        doc::components::Icon.materialize(),
                        Component,
                    ),
                    page(
                        "Callback",
                        "The prop types components use for callbacks, outputs and views",
                        doc::components::Callback.materialize(),
                        Component,
                    ),
                    page(
                        "Live Announcer",
                        "Announces messages to screen reader users",
                        doc::utils::LiveAnnouncer.materialize(),
                        Guide,
                    ),
                ],
            ),
        ],
    }
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::nav;

    #[test]
    fn every_entry_has_a_one_line_summary() {
        for entry in nav().sections.iter().flat_map(|section| &section.entries) {
            assert_that!(entry.summary)
                .with_detail_message(entry.title)
                .is_not_empty();
            assert_that!(entry.summary.ends_with('.'))
                .with_detail_message(entry.title)
                .is_false();
            assert_that!(entry.summary.contains('\n'))
                .with_detail_message(entry.title)
                .is_false();
        }
    }
}
