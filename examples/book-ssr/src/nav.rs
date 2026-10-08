//! The documentation's navigation: the single definition of which pages exist, how they are grouped, and what kind of
//! page each one is.
//!
//! Drives the sidebar ([`DocLayout`](crate::pages::documentation::doc_layout::DocLayout)), the tabs of concept pages
//! ([`ConceptLayout`](crate::pages::documentation::concept_layout::ConceptLayout)), the member tables of group
//! overviews and the page classification of the Markdown export. Adding a page means adding a route in `routes.rs` and
//! an entry here. The rules (which part a page belongs to, names, markers, tabs) are in "Navigation" of
//! `documentation/documentation-strategy.md`; the tests below enforce them.

use std::sync::LazyLock;

use icondata::{self, Icon};

use crate::routes::doc;

/// What a documentation page is about. See "Page Types" in `documentation/documentation-strategy.md`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PageKind {
    /// Getting-started pages and other guides.
    Guide,
    /// Overview of a concept group (Fields, Overlays, ...) or a building-block area (Interactions, Focus, ...).
    Overview,
    /// Overview of a concept implemented at both layers (Button, Slider, ...).
    Concept,
    Hook,
    Atom,
    /// A function or type from `leptonic::utils`, outside the two layers (e.g. the live announcer, `I18nProvider`).
    Utility,
}

impl PageKind {
    /// Name of the kind, e.g. in the member tables of group overviews.
    pub fn label(self) -> &'static str {
        match self {
            Self::Guide => "Guide",
            Self::Overview => "Overview",
            Self::Concept => "Concept",
            Self::Hook => "Hook",
            Self::Atom => "Atom",
            Self::Utility => "Utility",
        }
    }

    /// Short label shown as a badge next to building blocks in the sidebar.
    pub fn badge(self) -> Option<&'static str> {
        match self {
            Self::Hook => Some("hook"),
            Self::Atom => Some("atom"),
            Self::Utility => Some("util"),
            Self::Guide | Self::Overview | Self::Concept => None,
        }
    }

    /// The layer a page of this kind documents.
    pub fn layer(self) -> Option<Layer> {
        match self {
            Self::Hook => Some(Layer::Hook),
            Self::Atom => Some(Layer::Atom),
            Self::Guide | Self::Overview | Self::Concept | Self::Utility => None,
        }
    }
}

/// How much of a concept leptonic implements for you. Ordered from the lowest layer to the highest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Layer {
    Hook,
    Atom,
}

impl Layer {
    pub const ALL: [Layer; 2] = [Layer::Hook, Layer::Atom];

    pub fn kind(self) -> PageKind {
        match self {
            Self::Hook => PageKind::Hook,
            Self::Atom => PageKind::Atom,
        }
    }

    /// Lowercase name, e.g. for `data-layer` attributes.
    pub fn name(self) -> &'static str {
        match self {
            Self::Hook => "hook",
            Self::Atom => "atom",
        }
    }

    /// The letter of the layer marker in the sidebar.
    pub fn letter(self) -> &'static str {
        match self {
            Self::Hook => "H",
            Self::Atom => "A",
        }
    }
}

/// The parts of the sidebar, each answering one question of the reader.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PartKind {
    /// Getting started and guides.
    Guides,
    /// "Which UI element do I need?": every concept, grouped by purpose.
    Concepts,
    /// "How do I give my own element a behavior?": hooks, atoms and utilities shared by many concepts, grouped by
    /// behavior.
    BuildingBlocks,
}

/// A part of the sidebar.
pub struct NavPart {
    pub kind: PartKind,
    /// The heading above the part's groups. The guides have none.
    pub title: Option<&'static str>,
    pub groups: Vec<NavGroup>,
}

/// A group of pages in the sidebar: a concept group, a building-block area, or a group of guides.
pub struct NavGroup {
    pub title: &'static str,
    pub icon: Icon,
    /// The group's overview page. Groups of guides and small building-block areas have none.
    pub overview: Option<String>,
    pub entries: Vec<NavEntry>,
}

/// A page listed in the sidebar.
pub struct NavEntry {
    pub title: &'static str,
    /// One line on what the page covers, without a trailing period. Shown in the member tables of group overviews.
    pub summary: &'static str,
    pub href: String,
    pub kind: PageKind,
    /// The layer pages of a concept, shown as tabs next to its overview (which is not listed here).
    pub tabs: Vec<NavTab>,
}

/// A layer page of a concept, shown as a tab.
pub struct NavTab {
    pub href: String,
    pub layer: Layer,
    /// Whether the page documents several items of its layer (several hooks, several atoms), making its label plural.
    pub several: bool,
}

impl NavTab {
    pub fn label(&self) -> &'static str {
        match (self.layer, self.several) {
            (Layer::Hook, false) => "Hook",
            (Layer::Hook, true) => "Hooks",
            (Layer::Atom, false) => "Atom",
            (Layer::Atom, true) => "Atoms",
        }
    }
}

pub struct Nav {
    pub parts: Vec<NavPart>,
}

/// The navigation of the documentation.
pub fn nav() -> &'static Nav {
    static NAV: LazyLock<Nav> = LazyLock::new(build);
    &NAV
}

impl Nav {
    /// The kind of the page at `path`, if it is part of the navigation.
    pub fn page_kind(&self, path: &str) -> Option<PageKind> {
        self.groups().find_map(|group| {
            if group.overview.as_deref() == Some(path) {
                return Some(PageKind::Overview);
            }
            group.entries.iter().find_map(|entry| entry.page_kind(path))
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
        self.groups().flat_map(|group| {
            group
                .overview
                .iter()
                .map(String::as_str)
                .chain(group.entries.iter().flat_map(NavEntry::pages))
        })
    }

    pub fn groups(&self) -> impl Iterator<Item = &NavGroup> {
        self.parts.iter().flat_map(|part| &part.groups)
    }

    fn entries(&self) -> impl Iterator<Item = &NavEntry> {
        self.groups().flat_map(|group| &group.entries)
    }
}

impl NavGroup {
    /// Whether the page at `path` belongs to this group.
    pub fn contains(&self, path: &str) -> bool {
        self.overview.as_deref() == Some(path)
            || self
                .entries
                .iter()
                .any(|entry| entry.page_kind(path).is_some())
    }
}

impl NavEntry {
    /// The layers this entry documents: the tabs of a concept, or the layer of a single-layer page.
    pub fn layers(&self) -> Vec<Layer> {
        if self.tabs.is_empty() {
            self.kind.layer().into_iter().collect()
        } else {
            self.tabs.iter().map(|tab| tab.layer).collect()
        }
    }

    fn page_kind(&self, path: &str) -> Option<PageKind> {
        if self.href == path {
            return Some(self.kind);
        }
        self.tabs
            .iter()
            .find(|tab| tab.href == path)
            .map(|tab| tab.layer.kind())
    }

    fn pages(&self) -> impl Iterator<Item = &str> {
        std::iter::once(self.href.as_str()).chain(self.tabs.iter().map(|tab| tab.href.as_str()))
    }
}

fn part(kind: PartKind, title: Option<&'static str>, groups: Vec<NavGroup>) -> NavPart {
    NavPart {
        kind,
        title,
        groups,
    }
}

fn group(
    title: &'static str,
    icon: Icon,
    overview: Option<String>,
    entries: Vec<NavEntry>,
) -> NavGroup {
    NavGroup {
        title,
        icon,
        overview,
        entries,
    }
}

/// A page without tabs: a guide, a building block or a single-layer concept.
fn page(title: &'static str, summary: &'static str, href: String, kind: PageKind) -> NavEntry {
    NavEntry {
        title,
        summary,
        href,
        kind,
        tabs: Vec::new(),
    }
}

/// A concept with an overview page and a tab for each of its layers.
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
    }
}

fn tab(layer: Layer, several: bool, href: String) -> NavTab {
    NavTab {
        href,
        layer,
        several,
    }
}

fn hook(href: String) -> NavTab {
    tab(Layer::Hook, false, href)
}

fn hooks(href: String) -> NavTab {
    tab(Layer::Hook, true, href)
}

fn atom(href: String) -> NavTab {
    tab(Layer::Atom, false, href)
}

fn atoms(href: String) -> NavTab {
    tab(Layer::Atom, true, href)
}

fn build() -> Nav {
    Nav {
        parts: vec![
            part(PartKind::Guides, None, vec![getting_started(), guides()]),
            part(
                PartKind::Concepts,
                Some("Concepts"),
                vec![
                    buttons(),
                    fields(),
                    pickers(),
                    collections(),
                    date_time(),
                    color(),
                    overlays(),
                    navigation(),
                    status(),
                    layout(),
                ],
            ),
            part(
                PartKind::BuildingBlocks,
                Some("Building blocks"),
                vec![
                    interactions(),
                    focus(),
                    overlay_behavior(),
                    collection_state(),
                    drag_and_drop(),
                    animation(),
                    screen_readers(),
                    utilities(),
                ],
            ),
        ],
    }
}

// ── Guides ───────────────────────────────────────────────────────────────────

fn getting_started() -> NavGroup {
    use PageKind::Guide;
    group(
        "Getting started",
        icondata::BsBook,
        None,
        vec![
            page(
                "Overview",
                "What leptonic is and how the documentation is organized",
                doc::Overview.materialize(),
                Guide,
            ),
            page(
                "Installation",
                "Adds leptonic to a Leptos app: features, build configuration, styles and the theme provider",
                doc::Installation.materialize(),
                Guide,
            ),
            page(
                "Changelog",
                "Changes of every release, with migration notes",
                doc::Changelog.materialize(),
                Guide,
            ),
        ],
    )
}

fn guides() -> NavGroup {
    use PageKind::Guide;
    group(
        "Guides",
        icondata::BsLightbulb,
        None,
        vec![
            page(
                "Hooks & Atoms",
                "The two layers of leptonic, when to use which, and how to style atoms",
                doc::Architecture.materialize(),
                Guide,
            ),
            page(
                "Event Propagation",
                "Why leptonic events stop propagating and how to let them bubble",
                doc::EventPropagation.materialize(),
                Guide,
            ),
            page(
                "Classes & Styles",
                "Passing classes and typed styles to atoms and through your own Leptos components",
                doc::ClassesAndStyles.materialize(),
                Guide,
            ),
            page(
                "Callbacks",
                "The prop types for events, state setters and views, and binding hook state to your app",
                doc::Callbacks.materialize(),
                Guide,
            ),
            page(
                "Themes",
                "Themes with ThemeProvider, switching them, and the optional atom theme",
                doc::Themes.materialize(),
                Guide,
            ),
            page(
                "Forms & Validation",
                "Form fields, validation behavior and error messages",
                doc::Forms.materialize(),
                Guide,
            ),
            page(
                "Server-Side Rendering",
                "How leptonic renders on the server and hydrates, and what your code has to watch out for",
                doc::Ssr.materialize(),
                Guide,
            ),
            page(
                "Accessibility",
                "What leptonic does for accessibility at each layer and what your app still has to do",
                doc::Accessibility.materialize(),
                Guide,
            ),
            page(
                "Build Times & Bundle Size",
                "Settings that keep rebuilds fast and the browser bundle small",
                doc::BuildTimes.materialize(),
                Guide,
            ),
        ],
    )
}

// ── Concepts ─────────────────────────────────────────────────────────────────

fn buttons() -> NavGroup {
    group(
        "Buttons",
        icondata::BsHandIndex,
        Some(doc::Buttons.materialize()),
        vec![
            concept(
                "Button",
                "Triggers an action, such as submit, delete or open",
                doc::Button.materialize(),
                vec![
                    hook(doc::button::Hook.materialize()),
                    atom(doc::button::Atom.materialize()),
                ],
            ),
            concept(
                "Toggle Button",
                "A button that stays pressed, alone or in a group of options",
                doc::ToggleButton.materialize(),
                vec![
                    hooks(doc::toggle_button::Hook.materialize()),
                    atoms(doc::toggle_button::Atom.materialize()),
                ],
            ),
        ],
    )
}

fn fields() -> NavGroup {
    group(
        "Fields",
        icondata::BsInputCursorText,
        Some(doc::Fields.materialize()),
        vec![
            concept(
                "Checkbox",
                "Turns an independent option on or off, alone or in a group",
                doc::Checkbox.materialize(),
                vec![
                    hooks(doc::checkbox::Hook.materialize()),
                    atoms(doc::checkbox::Atom.materialize()),
                ],
            ),
            concept(
                "Field",
                "Label, description and error message of any form field",
                doc::Field.materialize(),
                vec![
                    hooks(doc::field::Hook.materialize()),
                    atoms(doc::field::Atom.materialize()),
                ],
            ),
            concept(
                "Form",
                "Submits fields together, deciding when they show errors and passing on server errors",
                doc::Form.materialize(),
                vec![
                    hooks(doc::form::Hook.materialize()),
                    atom(doc::form::Atom.materialize()),
                ],
            ),
            concept(
                "Number Field",
                "Enters a number by typing or stepping, with locale-aware formatting",
                doc::NumberField.materialize(),
                vec![
                    hooks(doc::number_field::Hook.materialize()),
                    atoms(doc::number_field::Atom.materialize()),
                ],
            ),
            concept(
                "Radio",
                "Picks exactly one option of a small, visible set",
                doc::Radio.materialize(),
                vec![
                    hooks(doc::radio::Hook.materialize()),
                    atoms(doc::radio::Atom.materialize()),
                ],
            ),
            concept(
                "Search Field",
                "A text field for search queries, submitted with Enter and cleared with Escape",
                doc::SearchField.materialize(),
                vec![
                    hook(doc::search_field::Hook.materialize()),
                    atoms(doc::search_field::Atom.materialize()),
                ],
            ),
            concept(
                "Slider",
                "Picks a number, or a range, by dragging along a track",
                doc::Slider.materialize(),
                vec![
                    hooks(doc::slider::Hook.materialize()),
                    atoms(doc::slider::Atom.materialize()),
                ],
            ),
            concept(
                "Switch",
                "Switches a setting on or off, with immediate effect",
                doc::Switch.materialize(),
                vec![
                    hooks(doc::switch::Hook.materialize()),
                    atoms(doc::switch::Atom.materialize()),
                ],
            ),
            concept(
                "Text Field",
                "Single-line and multi-line text input, such as names, emails and passwords",
                doc::TextField.materialize(),
                vec![
                    hooks(doc::text_field::Hook.materialize()),
                    atoms(doc::text_field::Atom.materialize()),
                ],
            ),
        ],
    )
}

fn pickers() -> NavGroup {
    group(
        "Pickers",
        icondata::BsMenuDown,
        Some(doc::Pickers.materialize()),
        vec![
            concept(
                "Combobox",
                "A text field with a filtered list of options, for large option sets",
                doc::Combobox.materialize(),
                vec![
                    hooks(doc::combobox::Hook.materialize()),
                    atoms(doc::combobox::Atom.materialize()),
                ],
            ),
            concept(
                "Select",
                "Picks an option from a dropdown, where space is limited",
                doc::Select.materialize(),
                vec![
                    hooks(doc::select::Hook.materialize()),
                    atoms(doc::select::Atom.materialize()),
                ],
            ),
        ],
    )
}

fn collections() -> NavGroup {
    group(
        "Collections",
        icondata::BsListUl,
        Some(doc::Collections.materialize()),
        vec![
            concept(
                "Grid",
                "Rows and cells navigated in two dimensions, with row selection",
                doc::Grid.materialize(),
                vec![
                    hooks(doc::grid::Hook.materialize()),
                    atoms(doc::grid::Atom.materialize()),
                ],
            ),
            concept(
                "Grid List",
                "A list of rows that may contain buttons, checkboxes or links",
                doc::GridList.materialize(),
                vec![
                    hooks(doc::grid_list::Hook.materialize()),
                    atoms(doc::grid_list::Atom.materialize()),
                ],
            ),
            concept(
                "Listbox",
                "A list of options that are all visible at once, with single or multiple selection",
                doc::Listbox.materialize(),
                vec![
                    hooks(doc::listbox::Hook.materialize()),
                    atoms(doc::listbox::Atom.materialize()),
                ],
            ),
            concept(
                "Menu",
                "A list of actions opened from a trigger",
                doc::Menu.materialize(),
                vec![
                    hooks(doc::menu::Hook.materialize()),
                    atoms(doc::menu::Atom.materialize()),
                ],
            ),
            concept(
                "Table",
                "Columns of data with sortable headers and selectable rows",
                doc::Table.materialize(),
                vec![
                    hooks(doc::table::Hook.materialize()),
                    atoms(doc::table::Atom.materialize()),
                ],
            ),
            concept(
                "Tag Group",
                "A focusable list of tags that can be selected and removed",
                doc::TagGroup.materialize(),
                vec![
                    hooks(doc::tag_group::Hook.materialize()),
                    atoms(doc::tag_group::Atom.materialize()),
                ],
            ),
            page(
                "Tree",
                "Hierarchical items with expandable and collapsible children",
                doc::Tree.materialize(),
                PageKind::Hook,
            ),
        ],
    )
}

fn date_time() -> NavGroup {
    group(
        "Date & Time",
        icondata::BsCalendar3,
        Some(doc::DateTime.materialize()),
        vec![
            concept(
                "Calendar",
                "A month grid for picking a date or a range of dates",
                doc::Calendar.materialize(),
                vec![
                    hooks(doc::calendar::Hook.materialize()),
                    atoms(doc::calendar::Atom.materialize()),
                ],
            ),
            concept(
                "Date Field",
                "Enters a date, or a date and time, in editable segments",
                doc::DateField.materialize(),
                vec![
                    hooks(doc::date_field::Hook.materialize()),
                    atoms(doc::date_field::Atom.materialize()),
                ],
            ),
            concept(
                "Date Picker",
                "A date field with a calendar in a popover, for a date or a range",
                doc::DatePicker.materialize(),
                vec![
                    hooks(doc::date_picker::Hook.materialize()),
                    atoms(doc::date_picker::Atom.materialize()),
                ],
            ),
            concept(
                "Time Field",
                "Enters a time of day in editable segments",
                doc::TimeField.materialize(),
                vec![
                    hooks(doc::time_field::Hook.materialize()),
                    atom(doc::time_field::Atom.materialize()),
                ],
            ),
        ],
    )
}

fn color() -> NavGroup {
    group(
        "Color",
        icondata::BsPalette,
        Some(doc::Color.materialize()),
        vec![
            concept(
                "Color Area",
                "Picks two channels of a color, e.g. saturation and brightness, in a 2D area",
                doc::ColorArea.materialize(),
                vec![
                    hooks(doc::color_area::Hook.materialize()),
                    atoms(doc::color_area::Atom.materialize()),
                ],
            ),
            concept(
                "Color Field",
                "Enters a color, or one of its channels, as text",
                doc::ColorField.materialize(),
                vec![
                    hooks(doc::color_field::Hook.materialize()),
                    atoms(doc::color_field::Atom.materialize()),
                ],
            ),
            concept(
                "Color Picker",
                "Combines color areas, sliders and fields into one picker",
                doc::ColorPicker.materialize(),
                vec![
                    hook(doc::color_picker::Hook.materialize()),
                    atom(doc::color_picker::Atom.materialize()),
                ],
            ),
            concept(
                "Color Slider",
                "Picks one channel of a color along a track",
                doc::ColorSlider.materialize(),
                vec![
                    hooks(doc::color_slider::Hook.materialize()),
                    atoms(doc::color_slider::Atom.materialize()),
                ],
            ),
            concept(
                "Color Swatch",
                "Shows a color as a small preview",
                doc::ColorSwatch.materialize(),
                vec![
                    hook(doc::color_swatch::Hook.materialize()),
                    atom(doc::color_swatch::Atom.materialize()),
                ],
            ),
            page(
                "Color Swatch Picker",
                "Picks one of a few predefined colors, shown as swatches",
                doc::ColorSwatchPicker.materialize(),
                PageKind::Atom,
            ),
            concept(
                "Color Wheel",
                "Picks a hue on a circular track",
                doc::ColorWheel.materialize(),
                vec![
                    hooks(doc::color_wheel::Hook.materialize()),
                    atoms(doc::color_wheel::Atom.materialize()),
                ],
            ),
        ],
    )
}

fn overlays() -> NavGroup {
    group(
        "Overlays",
        icondata::BsWindowStack,
        Some(doc::Overlays.materialize()),
        vec![
            concept(
                "Dialog",
                "The named content of a modal or popover, such as a confirmation or a form",
                doc::Dialog.materialize(),
                vec![
                    hook(doc::dialog::Hook.materialize()),
                    atoms(doc::dialog::Atom.materialize()),
                ],
            ),
            concept(
                "Modal",
                "A dialog that blocks the page until the user responds",
                doc::Modal.materialize(),
                vec![
                    hooks(doc::modal::Hook.materialize()),
                    atoms(doc::modal::Atom.materialize()),
                ],
            ),
            concept(
                "Popover",
                "Content anchored to a trigger element",
                doc::Popover.materialize(),
                vec![
                    hook(doc::popover::Hook.materialize()),
                    atoms(doc::popover::Atom.materialize()),
                ],
            ),
            concept(
                "Tooltip",
                "A short hint shown when a button or icon is hovered or focused",
                doc::Tooltip.materialize(),
                vec![
                    hooks(doc::tooltip::Hook.materialize()),
                    atoms(doc::tooltip::Atom.materialize()),
                ],
            ),
        ],
    )
}

fn navigation() -> NavGroup {
    group(
        "Navigation",
        icondata::BsSignpost,
        Some(doc::Navigation.materialize()),
        vec![
            concept(
                "Breadcrumbs",
                "A trail of links to the current page's ancestors",
                doc::Breadcrumbs.materialize(),
                vec![
                    hooks(doc::breadcrumbs::Hook.materialize()),
                    atoms(doc::breadcrumbs::Atom.materialize()),
                ],
            ),
            concept(
                "Disclosure",
                "Content that expands and collapses under a trigger, alone or as an accordion",
                doc::Disclosure.materialize(),
                vec![
                    hooks(doc::disclosure::Hook.materialize()),
                    atoms(doc::disclosure::Atom.materialize()),
                ],
            ),
            concept(
                "Link",
                "Navigates to a URL or an anchor on the page",
                doc::Link.materialize(),
                vec![
                    hooks(doc::link::Hook.materialize()),
                    atoms(doc::link::Atom.materialize()),
                ],
            ),
            concept(
                "Tabs",
                "Switches between panels that share the same space",
                doc::Tabs.materialize(),
                vec![
                    hooks(doc::tabs::Hook.materialize()),
                    atoms(doc::tabs::Atom.materialize()),
                ],
            ),
        ],
    )
}

fn status() -> NavGroup {
    group(
        "Status",
        icondata::BsInfoCircle,
        Some(doc::Status.materialize()),
        vec![
            concept(
                "Meter",
                "Shows a value within a known range, such as disk usage",
                doc::Meter.materialize(),
                vec![
                    hook(doc::meter::Hook.materialize()),
                    atoms(doc::meter::Atom.materialize()),
                ],
            ),
            concept(
                "Progress Bar",
                "Shows the progress of an ongoing task, determinate or not",
                doc::ProgressBar.materialize(),
                vec![
                    hook(doc::progress_bar::Hook.materialize()),
                    atoms(doc::progress_bar::Atom.materialize()),
                ],
            ),
            concept(
                "Toast",
                "A short notification above the app that usually closes by itself",
                doc::Toast.materialize(),
                vec![
                    hooks(doc::toast::Hook.materialize()),
                    atoms(doc::toast::Atom.materialize()),
                ],
            ),
        ],
    )
}

fn layout() -> NavGroup {
    group(
        "Content & Layout",
        icondata::BsColumnsGap,
        Some(doc::Layout.materialize()),
        vec![
            page(
                "Kbd",
                "Displays keys and keyboard shortcuts",
                doc::Kbd.materialize(),
                PageKind::Atom,
            ),
            concept(
                "Separator",
                "A visual divider between groups of content",
                doc::Separator.materialize(),
                vec![
                    hook(doc::separator::Hook.materialize()),
                    atom(doc::separator::Atom.materialize()),
                ],
            ),
            concept(
                "Toolbar",
                "A group of controls navigated with the arrow keys",
                doc::Toolbar.materialize(),
                vec![
                    hook(doc::toolbar::Hook.materialize()),
                    atom(doc::toolbar::Atom.materialize()),
                ],
            ),
        ],
    )
}

// ── Building blocks ──────────────────────────────────────────────────────────

fn interactions() -> NavGroup {
    use PageKind::{Atom, Hook};
    group(
        "Interactions",
        icondata::BsCursor,
        Some(doc::Interactions.materialize()),
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
                "use_hover",
                "Tracks whether a mouse or pen hovers an element, for tooltips and highlights",
                doc::interactions::UseHover.materialize(),
                Hook,
            ),
            page(
                "Hoverable",
                "Adds hover tracking to its child without rendering an element",
                doc::interactions::Hoverable.materialize(),
                Atom,
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
                "use_global_shortcuts",
                "Keyboard shortcuts for the whole page, e.g. one opening a search from anywhere",
                doc::interactions::UseGlobalShortcuts.materialize(),
                Hook,
            ),
            page(
                "use_context_menu",
                "Requests for a context menu by right click, keyboard or long press, with their position",
                doc::interactions::UseContextMenu.materialize(),
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
        ],
    )
}

fn focus() -> NavGroup {
    use PageKind::{Atom, Hook};
    group(
        "Focus",
        icondata::BsEye,
        Some(doc::Focus.materialize()),
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
                "Focusable",
                "Makes its child focusable, e.g. an icon that shows a tooltip",
                doc::focus::Focusable.materialize(),
                Atom,
            ),
            page(
                "use_focus_visible",
                "Tracks whether the user navigates with the keyboard",
                doc::focus::UseFocusVisible.materialize(),
                Hook,
            ),
            page(
                "use_focus_ring",
                "Tells whether an element should show a keyboard focus ring",
                doc::focus::UseFocusRing.materialize(),
                Hook,
            ),
            page(
                "FocusRing",
                "Marks its child with data-focus-visible while it has keyboard focus",
                doc::focus::FocusRing.materialize(),
                Atom,
            ),
            page(
                "FocusScope",
                "Contains, restores and auto-focuses focus within a subtree, e.g. a dialog",
                doc::focus::FocusScope.materialize(),
                Atom,
            ),
            page(
                "use_focus_manager",
                "Moves focus to the next, previous, first or last element of a container",
                doc::focus::UseFocusManager.materialize(),
                Hook,
            ),
            page(
                "FocusManagerProvider",
                "A container that hands a focus manager to its children",
                doc::focus::FocusManagerProvider.materialize(),
                Atom,
            ),
            page(
                "use_landmark",
                "Makes a region of the page a landmark that F6 moves to",
                doc::focus::UseLandmark.materialize(),
                Hook,
            ),
            page(
                "use_has_tabbable_child",
                "Tells whether a container has tabbable descendants",
                doc::focus::UseHasTabbableChild.materialize(),
                Hook,
            ),
            page(
                "focusability",
                "Checks whether an element can take focus or is reached by Tab",
                doc::focus::Focusability.materialize(),
                PageKind::Utility,
            ),
            page(
                "virtual_focus",
                "Focuses an option for assistive technology while DOM focus stays in an input",
                doc::focus::VirtualFocus.materialize(),
                PageKind::Utility,
            ),
        ],
    )
}

fn overlay_behavior() -> NavGroup {
    use PageKind::{Atom, Hook, Utility};
    group(
        "Overlay Behavior",
        icondata::BsLayers,
        Some(doc::OverlayBehavior.materialize()),
        vec![
            page(
                "use_overlay_trigger_state",
                "Whether an overlay is open, shared by its trigger and the overlay",
                doc::overlay_behavior::UseOverlayTriggerState.materialize(),
                Hook,
            ),
            page(
                "use_overlay",
                "Dismisses an overlay on Escape, a press outside or blur",
                doc::overlay_behavior::UseOverlay.materialize(),
                Hook,
            ),
            page(
                "use_overlay_trigger",
                "Connects a trigger to the overlay it opens with ARIA attributes",
                doc::overlay_behavior::UseOverlayTrigger.materialize(),
                Hook,
            ),
            page(
                "use_overlay_position",
                "Places an overlay next to its target and flips it when space runs out",
                doc::overlay_behavior::UseOverlayPosition.materialize(),
                Hook,
            ),
            page(
                "use_close_on_scroll",
                "Closes an overlay when its trigger scrolls away",
                doc::overlay_behavior::UseCloseOnScroll.materialize(),
                Hook,
            ),
            page(
                "use_prevent_scroll",
                "Locks page scrolling while a modal or overlay is open",
                doc::overlay_behavior::UsePreventScroll.materialize(),
                Hook,
            ),
            page(
                "aria_hide_outside",
                "Hides the page outside a modal overlay from assistive technology",
                doc::overlay_behavior::AriaHideOutside.materialize(),
                Utility,
            ),
            page(
                "use_overlay_focus_contain",
                "Lets overlay content, such as a dialog, keep focus inside the overlay",
                doc::overlay_behavior::UseOverlayFocusContain.materialize(),
                Hook,
            ),
            page(
                "DismissButton",
                "A visually hidden button that lets screen reader users dismiss an overlay",
                doc::overlay_behavior::DismissButton.materialize(),
                Atom,
            ),
        ],
    )
}

fn collection_state() -> NavGroup {
    group(
        "Collection State",
        icondata::BsCollection,
        Some(doc::CollectionState.materialize()),
        vec![
            page(
                "Virtualizer",
                "Renders only the visible options of a long listbox, and VirtualList only the visible rows of a plain list",
                doc::collection_state::Virtualizer.materialize(),
                PageKind::Atom,
            ),
            page(
                "use_virtualizer_state",
                "Lays out a collection and renders only what is visible, with use_scroll_view and use_virtualizer_item",
                doc::collection_state::UseVirtualizerState.materialize(),
                PageKind::Hook,
            ),
        ],
    )
}

fn drag_and_drop() -> NavGroup {
    group(
        "Drag & Drop",
        icondata::BsArrowsMove,
        Some(doc::DragAndDrop.materialize()),
        Vec::new(),
    )
}

fn animation() -> NavGroup {
    group(
        "Animation",
        icondata::BsPlayCircle,
        Some(doc::Animation.materialize()),
        Vec::new(),
    )
}

fn screen_readers() -> NavGroup {
    group(
        "Screen Readers",
        icondata::BsMegaphone,
        None,
        vec![
            page(
                "live_announcer",
                "Announces messages to screen reader users",
                doc::screen_readers::LiveAnnouncer.materialize(),
                PageKind::Utility,
            ),
            page(
                "use_visually_hidden",
                "Hides an element visually but not from screen readers, optionally showing it on focus",
                doc::screen_readers::UseVisuallyHidden.materialize(),
                PageKind::Hook,
            ),
            page(
                "VisuallyHidden",
                "Content only screen readers read, such as the context of a link or a skip link",
                doc::screen_readers::VisuallyHidden.materialize(),
                PageKind::Atom,
            ),
            page(
                "use_description",
                "Describes an element to screen readers through a hidden text and aria-describedby",
                doc::screen_readers::UseDescription.materialize(),
                PageKind::Hook,
            ),
        ],
    )
}

fn utilities() -> NavGroup {
    group(
        "Utilities",
        icondata::BsTools,
        None,
        vec![
            page(
                "I18nProvider",
                "Sets the locale and writing direction that hooks format and lay out with",
                doc::utilities::I18nProvider.materialize(),
                PageKind::Utility,
            ),
            page(
                "NumberFormatter",
                "Formats and parses numbers for a locale, and picks plural forms",
                doc::utilities::NumberFormatter.materialize(),
                PageKind::Utility,
            ),
            page(
                "DateTimeFormatter",
                "Formats dates and times for a locale",
                doc::utilities::DateTimeFormatter.materialize(),
                PageKind::Utility,
            ),
            page(
                "ListFormatter",
                "Joins items into a list such as \u{201c}A, B, and C\u{201d} for a locale",
                doc::utilities::ListFormatter.materialize(),
                PageKind::Utility,
            ),
            page(
                "use_localized_strings",
                "The hooks' labels, descriptions and announcements in 34 languages, for your own code too",
                doc::utilities::UseLocalizedStrings.materialize(),
                PageKind::Utility,
            ),
            page(
                "Collator",
                "Sorts and filters text by the rules of a locale",
                doc::utilities::Collator.materialize(),
                PageKind::Utility,
            ),
            page(
                "scroll",
                "Finds scrolling containers and scrolls an element into view inside them",
                doc::utilities::Scroll.materialize(),
                PageKind::Utility,
            ),
            page(
                "use_spin_button",
                "Steps a number up and down with the keyboard and hold-to-spin buttons",
                doc::utilities::UseSpinButton.materialize(),
                PageKind::Hook,
            ),
            page(
                "use_clipboard",
                "Cut, copy and paste of your app's data on a focused element",
                doc::utilities::UseClipboard.materialize(),
                PageKind::Hook,
            ),
        ],
    )
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use assertr::prelude::*;

    use super::{NavEntry, PageKind, PartKind, nav};

    fn entries_of(kind: PartKind) -> impl Iterator<Item = &'static NavEntry> {
        nav()
            .parts
            .iter()
            .filter(move |part| part.kind == kind)
            .flat_map(|part| &part.groups)
            .flat_map(|group| &group.entries)
    }

    #[test]
    fn every_entry_has_a_one_line_summary() {
        for entry in nav().groups().flat_map(|group| &group.entries) {
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

    #[test]
    fn every_page_is_listed_once() {
        let mut seen = HashSet::new();
        for page in nav().pages() {
            assert_that!(seen.insert(page))
                .with_detail_message(format!("{page} is listed twice"))
                .is_true();
        }
    }

    #[test]
    fn guides_are_guides() {
        for entry in entries_of(PartKind::Guides) {
            assert_that!(entry.kind)
                .with_detail_message(entry.title)
                .is_equal_to(PageKind::Guide);
        }
    }

    /// A concept is either an overview with a tab per layer, or a single page of one layer.
    #[test]
    fn concepts_have_layers() {
        for entry in entries_of(PartKind::Concepts) {
            if entry.kind == PageKind::Concept {
                assert_that!(entry.tabs.len() >= 2)
                    .with_detail_message(format!(
                        "{}: a concept with one layer is a single page, without tabs",
                        entry.title
                    ))
                    .is_true();
                // Strictly ascending: one tab per layer, hooks before atoms.
                assert_that!(entry.tabs.windows(2).all(|w| w[0].layer < w[1].layer))
                    .with_detail_message(format!(
                        "{}: tabs must be one per layer, in the order hook, atom",
                        entry.title
                    ))
                    .is_true();
            } else {
                assert_that!(entry.kind.layer().is_some() && entry.tabs.is_empty())
                    .with_detail_message(format!(
                        "{}: a single-layer concept is a hook or atom page",
                        entry.title
                    ))
                    .is_true();
            }
        }
    }

    #[test]
    fn building_blocks_are_single_pages_with_a_badge() {
        for entry in entries_of(PartKind::BuildingBlocks) {
            assert_that!(entry.tabs.is_empty() && entry.kind.badge().is_some())
                .with_detail_message(entry.title)
                .is_true();
        }
    }

    #[test]
    fn concept_groups_are_sorted() {
        for group in nav()
            .parts
            .iter()
            .filter(|part| part.kind == PartKind::Concepts)
            .flat_map(|part| &part.groups)
        {
            let titles: Vec<_> = group.entries.iter().map(|entry| entry.title).collect();
            let mut sorted = titles.clone();
            sorted.sort_unstable();
            assert_that!(titles)
                .with_detail_message(group.title)
                .is_equal_to(sorted);
        }
    }

    /// A hook and an atom of the same name are one concept with tabs, not two entries.
    #[test]
    fn concepts_are_not_split_into_layer_entries() {
        let mut seen = HashSet::new();
        for entry in entries_of(PartKind::Concepts) {
            let name = entry
                .title
                .trim_start_matches("use_")
                .replace(['_', ' '], "")
                .to_lowercase();
            assert_that!(seen.insert(name))
                .with_detail_message(entry.title)
                .is_true();
        }
    }
}
