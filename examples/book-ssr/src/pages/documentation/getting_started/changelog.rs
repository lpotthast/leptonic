use leptos::prelude::*;

use crate::kit::*;

/// A released (or upcoming) version.
struct Release {
    title: &'static str,
    /// Optional introduction, before the change groups.
    intro: &'static [&'static str],
    groups: &'static [(&'static str, &'static [&'static str])],
}

/// Entries are plain text; identifiers are written in backticks and rendered as inline code.
const RELEASES: &[Release] = &[
    Release {
        title: "Unreleased",
        intro: &[
            "Leptonic was rebuilt in three layers: hooks, atoms and components. The hooks are ports of react-aria\u{2019}s \
             hooks and carry the accessibility and interaction logic of everything built on top of them. Most of the API \
             changed; the notes below list what to change when upgrading from 0.5.0.",
        ],
        groups: &[
            (
                "Requirements",
                &[
                    "Leptos 0.8 (was 0.6), Rust edition 2024 and Rust 1.89 or newer.",
                    "`icondata` 0.7 (was 0.3), still re-exported as `leptonic::prelude::icondata`.",
                    "Every build needs the `--cfg=web_sys_unstable_apis` rustflag, not only builds with clipboard support. \
                 See Installation.",
                ],
            ),
            (
                "Imports and features",
                &[
                    "The flat `leptonic::prelude` was split by layer: `leptonic::components::prelude` (the themed \
                 components, `Root` and the themes), `leptonic::atoms::prelude` (the atoms; import it `as atoms`, as \
                 their names overlap with the components\u{2019}), `leptonic::hooks` (the hooks) and `leptonic::prelude` \
                 (shared types such as `Out`, `ValueBinding`, `Mount`, `Width`, `Margin`, `signal_ls` and `icondata`). \
                 Utilities live in `leptonic::utils`.",
                    "Features choose the layers: `hooks` (the only default), `atoms` and `components`, each including the \
                 layers below. Apps upgrading from 0.5.0 need `components`. `syntax-highlight` and `sanitize` are new \
                 extras; `tiptap` includes `components`, `clipboard` needs only `hooks`; `full` enables all layers and extras.",
                    "The `csr` feature was removed: client-side rendered apps enable neither `ssr` nor `hydrate`.",
                ],
            ),
            (
                "State, events, classes and styles",
                &[
                    "Atoms and components take their state as a value and a setter: `x` (a value or any signal) and \
                 `set_x` (an `Out`: a signal, a closure or a `Callback`); the setter of an `is_x` prop is `set_x` \
                 (`is_selected` and `set_selected`, `is_open` and `set_open`). Without `x`, they keep the state \
                 themselves, starting at `default_x`. `on_change` (for named states `on_x_change`, e.g. \
                 `on_open_change`) reports every change. See Hooks, Atoms & Components.",
                    "Hooks own their state: you pass an initial value, read signals and change the state through the \
                 methods a hook returns. To keep the state in your app, bind it with a `ValueBinding`, made from an \
                 `RwSignal` or a signal pair. See Callbacks.",
                    "Pressable elements report presses through an optional `on_press` with a `PressEvent` (mouse, touch, \
                 pen, keyboard and screen readers alike) instead of `on_click` with a `MouseEvent`.",
                    "Events handled by leptonic stop propagating unless the handler calls `continue_propagation()`. See \
                 Event Propagation.",
                    "Atoms and components take `classes` and `styles` (`Classes` and `Styles`, from `leptos-classes` and \
                 `leptos-styles`, with typed CSS declarations) instead of `id`, `class` and `style`. Set other \
                 attributes with Leptos\u{2019} `attr:` syntax, e.g. `attr:id=\"save\"`. See Classes & Styles.",
                    "Props take `Signal`, `MaybeProp` and `Option<Callback<..>>` instead of `MaybeSignal`, \
                 `OptionalMaybeSignal` and leptonic\u{2019}s `Consumer` and `Producer`.",
                ],
            ),
            (
                "Renamed",
                &[
                    "`Toggle` \u{2192} `Switch`, and `ToggleIcons`, `ToggleSize` and `ToggleVariant` \u{2192} \
                 `SwitchIcons`, `SwitchSize` and `SwitchVariant`. Its `state` and `set_state` \u{2192} `is_selected` and \
                 `set_selected`.",
                    "`Anchor` \u{2192} `AnchorLink`.",
                    "`LinkExt` \u{2192} `Link`, which takes a `target`; `LinkExtTarget` \u{2192} `LinkTarget`.",
                    "`create_signal_ls` \u{2192} `signal_ls`.",
                    "The key caps\u{2019} `Key` \u{2192} `KeyboardKey` (`leptonic::utils::key`), which keyboard handling \
                 uses too.",
                    "`HSV` and `RGB8` moved to `leptonic::utils::color`, next to the new `HSL`, `Alpha<C>` (any color with \
                 an alpha channel; replaces `RGBA8`) and `Color`. `ColorSpace` was removed: a `Color` keeps the color \
                 space it was set in.",
                    "`Size` \u{2192} `CssDimension`, of which `Width` and `Height` are now aliases: write `em(1.0)`, \
                 `px(4)` or `pct(50.0)` (`leptonic::utils::css`).",
                ],
            ),
            (
                "Changed",
                &[
                    "`Button` and `LinkButton`: `on_click` \u{2192} `on_press`, `disabled` \u{2192} `is_disabled`. \
                 `button_type` makes a button submit or reset its form; without it, buttons have `type=\"button\"`.",
                    "`Checkbox`: `checked` and `set_checked` \u{2192} `is_selected` and `set_selected`. Its children are its \
                 label. New: `is_indeterminate`, `is_required`, `is_invalid`, `name` and `form_value`. Removed: \
                 `variant`, `size` and `id`.",
                    "`Slider` and `RangeSlider` are generic over their number type. `min` and `max` \u{2192} `min_value` \
                 and `max_value`, `value_display` \u{2192} `format_options`, `disabled` \u{2192} `is_disabled`; name them \
                 with `aria_label`.",
                    "`Select`, `OptionalSelect` and `Multiselect` take `label` (or `aria_label`), `is_disabled` and `name`.",
                ],
            ),
            (
                "Removed",
                &[
                    "`TextInput`, `PasswordInput` and `NumberInput`, with their `Field` and `FieldLabel` helpers. Use \
                 `TextField` (`input_type=InputType::Password` for passwords) and `NumberField`, which bring their label, \
                 description and validation.",
                    "`Quicksearch`, `QuicksearchTrigger` and `QuicksearchOption`. Build a search from a `SearchField`, a \
                 `ComboBox` or a `Modal`.",
                    "`Box`. Use a `<div>` with your own classes.",
                    "The typography components `H1` to `H6` and `P`. Write the HTML elements; the theme styles them.",
                    "`ModalRoot`. Modals need no root anymore.",
                    "`DateTimeInput` and its `GuideMode`. Use the `DatePicker` atoms, which can be typed into and pick \
                 `jiff` dates (with times, if you like) instead of `time::OffsetDateTime`.",
                    "The styled `Table` (with `TableHeaderCell`), `DateSelector`, `DatePicker` and `ColorPicker` (with \
                 `ColorPreview`, `ColorPalette` and `HueSlider`) components. Use the table, calendar, date picker and \
                 color atoms (`ColorPicker` around `ColorArea`, `ColorSlider`, `ColorField`, ...) with your own CSS.",
                    "The `Modal` (with `ModalHeader`, `ModalTitle`, `ModalBody` and `ModalFooter`), `Popover` (with \
                 `PopoverTrigger`) and `Drawer` components. Use the modal, dialog and popover atoms with your own CSS; a \
                 drawer is a modal whose panel slides in from the edge of the screen.",
                    "The styled `Link`, `LinkButton`, `Tabs` (with `Tab`) and `Collapsible` (with `Collapsibles`) \
                 components. Use the link, tabs and disclosure atoms with your own CSS; a link that looks like a button is a \
                 `Link` with your button styles.",
                    "The `Alert`, `Meter`, `ProgressBar` and toast (`ToastRoot`, `Toasts`, `Toast`) components. Use the \
                 meter and progress bar atoms and the toast atoms (`ToastRegion` on a `ToastQueue`); an alert is an \
                 element with `role=\"alert\"` (see Status).",
                    "The `AppBar`, `Card`, `Tile`, `Chip`, `Grid` (with `Row` and `Col`), `Icon`, `KbdKey`, `KbdShortcut`, \
                 `SanitizedHtml`, `Separator`, `Skeleton`, `Stack`, `Code`, `TiptapEditor` and transition (`Collapse`, \
                 `Fade`, `Grow`, `Slide` and `Zoom`) components. Use the `TagGroup`, `Keys` and `Separator` atoms, \
                 `leptos_icons`, `ammonia`, `leptos-tiptap` and CSS (see Content & Layout).",
                    "`Consumer` and `Producer` (and `consumer` and `producer`). Use Leptos\u{2019} `Callback`.",
                    "The global event contexts `GlobalClickEvent` and `GlobalKeyboardEvent`. Listen to document events \
                 with leptos-use\u{2019}s `use_event_listener`, or detect outside interactions with \
                 `use_interact_outside`.",
                    "`OptionalSignal` and `OptionalMaybeSignal`.",
                    "The `variations` and `active` props of buttons. Variations resulted in non-compliant HTML.",
                ],
            ),
            (
                "Added",
                &[
                    "Hooks ported from react-aria: interactions (`use_press`, `use_hover`, `use_move`, `use_keyboard`, \
                 `use_context_menu`), focus management, overlays, collections and selection, drag and drop, animations, \
                 and the ARIA patterns of buttons, toggle buttons, links, breadcrumbs, text, search and number fields, \
                 checkboxes, radio groups, switches, sliders, listboxes, selects, comboboxes, menus, tooltips, dialogs, \
                 disclosures, tabs, toolbars, separators, meters, progress bars, grids, grid lists, tables, trees, tag \
                 groups, calendars, date fields, date pickers, color controls and toasts.",
                    "Document-wide keyboard shortcuts: `use_global_shortcuts` binds shortcuts that work anywhere (e.g. \
                 Ctrl+K, also while typing) and shortcuts that work only outside text fields (e.g. /), seeing every key \
                 press before leptonic\u{2019}s components stop it.",
                    "Landmarks: `use_landmark` makes a region of the page a landmark, and F6 and Shift+F6 move the focus \
                 between landmarks (Alt+F6 to the main one).",
                    "Atoms, unstyled Leptos components rendering one element each: `Breadcrumbs`, `Button`, `Calendar` \
                 and `RangeCalendar` with `CalendarHeading`, `CalendarPreviousButton`, `CalendarNextButton`, \
                 `CalendarErrorMessage`, `CalendarGrid` and its parts, `CalendarCell` and `CalendarCellButton`, `Checkbox` \
                 and `CheckboxGroup`, `ColorArea`, `ColorField` and `ColorChannelField`, `ColorPicker`, \
                 `ColorSlider` with `ColorSliderTrack` and `ColorSliderOutput`, `ColorSwatch`, `ColorSwatchPicker` with \
                 `ColorSwatchPickerItem`, `ColorThumb`, `ColorWheel` with `ColorWheelTrack`, `ComboBox`, `DateField` and \
                 `TimeField` with `DateInput` and `DateSegment`, `DatePicker` and `DateRangePicker` with `DatePickerGroup` \
                 and `DatePickerButton`, `Dialog` and `DialogTrigger`, `Disclosure` and `DisclosureGroup`, `DismissButton`, the field parts `Label`, `Description` and `FieldError`, \
                 `FocusManagerProvider`, `FocusRing`, `FocusScope`, `Focusable`, `Form`, `Grid`, `GridList`, \
                 `Hoverable`, `Input` and `TextArea`, `Link` and `AnchorLink`, `ListBox`, `Menu` with `MenuTrigger` and \
                 `SubmenuTrigger`, `Meter`, `ModalBackdrop` and `ModalContent`, `NumberField`, `OverlayArrow`, \
                 `Popover`, `Pressable` and `PressResponder`, `ProgressBar`, `Radio` and `RadioGroup`, `SearchField`, \
                 `Select`, `Separator`, `ShortcutKeys` and `Keys`, `Slider`, `Switch`, `Table`, `Tabs`, `TagGroup` with \
                 `TagList`, `Tag` and `TagRemoveButton`, `TextField`, `ToastRegion` and `Toast` with \
                 `ToastContent`, `ToastTitle`, `ToastDescription` and `ToastCloseButton`, `ToggleButton` and \
                 `ToggleButtonGroup`, `Toolbar`, `Tooltip` and `VisuallyHidden`.",
                    "Components: `TextField` and `SearchField` with label, description and validation, `NumberField` \
                 (generic over its number type), `CheckboxGroup`, `Radio` and `RadioGroup`.",
                    "Virtualization of long lists: the `Virtualizer` atom renders only the visible options of a \
                 `ListBox` (with `ListBoxItems`), positioned by a `ListLayout` of fixed or measured row sizes; \
                 `VirtualList` renders only the visible rows of a plain list such as a log, optionally following its \
                 end. `use_virtualizer_state`, `use_scroll_view`, `use_virtualizer_item` and the `Layout` trait \
                 (`leptonic::hooks::virtualizer`) virtualize markup and layouts of your own.",
                    "`Color` (`leptonic::utils::color`): a color in any of the spaces HSV, HSL and RGB, parsed from \
                 CSS-like text (`#rgb`, `#rrggbb`, `rgb()`, `hsb()`, `hsl()`), with `color_name()` and `hue_name()` \
                 describing colors in words (\u{201c}dark vibrant blue\u{201d}).",
                    "`use_theme`, to read and change the theme of the closest `ThemeProvider`.",
                    "`leptonic::jiff`, the re-exported date crate: calendars pick `jiff::civil::Date` values; date fields and \
                 pickers are generic over `civil::Date`, `civil::DateTime` and `Zoned`, time fields over `civil::Time`, \
                 `civil::DateTime` and `Zoned`, with segments in the order and format of the locale.",
                    "Internationalization (number, date and list formatting, collation, plural rules) based on ICU4X, \
                 which also works during server-side rendering, with `I18nProvider`, `use_locale` and `use_direction`.",
                    "Element ids that are stable between server-side rendering and hydration (`use_id`).",
                    "`leptonic::utils::clipboard::write_text`, to copy text to the clipboard (`clipboard` feature).",
                    "A live announcer for screen reader announcements, and keyboard shortcuts (`Shortcut`, \
                 `KeyboardShortcuts`); `Shortcut::keys` lists the keys to show for a shortcut on a platform.",
                    "`is_text_input` and `is_typing_target` (`leptonic::utils::focusability`), telling whether keys pressed \
                 at an element are the user typing.",
                ],
            ),
            (
                "Fixed",
                &[
                    "Converting an RGB color to HSV or HSL no longer gives negative hues (e.g. \u{2212}60\u{00b0} instead of \
                 300\u{00b0} for magenta).",
                    "Converting an HSV or HSL color to RGB rounds each channel to the nearest integer instead of \
                 truncating it.",
                ],
            ),
            (
                "Changed during development",
                &[
                    "For apps that followed the development branch: the `FocusManager` atom \u{2192} \
                 `FocusManagerProvider`; `FieldLabelProps` \u{2192} `LabelContext`; `use_long_press` was merged into \
                 `use_press` (`on_long_press`); `PlacementX` and `PlacementY` \u{2192} `Placement`.",
                    "`use_press`: `force_propagation` \u{2192} `propagation: PressPropagation`, `force_prevent_default` \
                 was removed, `on_press` is an `Option<Callback<PressEvent>>`.",
                    "`use_move` reports movement deltas only: the arrow keys move by one pixel on both axes, and callers \
                 track the position and whether a move is in progress (`on_move_start`, `on_move_end`) themselves.",
                ],
            ),
        ],
    },
    Release {
        title: "0.5.0",
        intro: &["Leptonic now supports Leptos 0.6!"],
        groups: &[
            (
                "Changed",
                &[
                    "Leptos dependencies were updated to v0.6.",
                    "The `Icon` component no longer wraps an icon from `leptos-icons`. It now expects v0.3 `icondata` icons.",
                ],
            ),
            (
                "Added",
                &[
                    "Dependency on the `icondata` crate. Leptonic re-exports it through its prelude, so you don\u{2019}t have to \
                 depend on it yourself.",
                ],
            ),
            (
                "Removed",
                &[
                    "Dependency on the `leptos-icons` crate. You can remove it from your `Cargo.toml` as well if you don\u{2019}t \
                 use it outside of leptonic.",
                ],
            ),
        ],
    },
    Release {
        title: "0.4.0",
        intro: &[
            "Leptonic now supports server-side rendering (SSR). This book is now deployed with SSR enabled.",
            "The `leptonic-template-ssr` and `leptonic-template-csr` templates were created. Use them to get started quickly.",
        ],
        groups: &[(
            "Changed",
            &[
                "Getting started instructions were consolidated.",
                "Installation instructions are now much more straightforward.",
                "Modals are now SSR compatible. The `ModalFn` component was dropped. If you used it, rename all occurrences \
                 to `Modal`. If you previously used `Modal`, you might see some \u{201c}requires Fn but is FnOnce\u{201d} errors. Storing \
                 values moved into a `Modal`\u{2019}s children with `StoredValue::new` should be a quick fix.",
                "Tabs are now SSR compatible. Rendering order changed to make this possible. This should not affect anyone.",
                "Toggles are now SSR compatible. Rendering of the (optional) icons changed. This should only affect you if \
                 custom styling is in play.",
                "Tables are now SSR compatible. Components were renamed to `TableHeader`, `TableBody`, `TableRow`, \
                 `TableHeaderCell` and `TableCell`.",
                "The `build.rs` script, which previously had to be created by the consumer, is no longer required. It moved \
                 into leptonic itself and copies the files required for a build.",
                "The `Code` block component now has a \u{201c}Copy to clipboard\u{201d} button (thanks to https://github.com/wt).",
                "A `SelectOption` no longer has to be `Eq`.",
                "The `uuid` dependency was bumped to version 1.6.",
                "A `RwSignal` can act as an `Out` type.",
                "Modals now support the `on_escape` prop, letting you handle escape key presses.",
                "The Tiptap editor is now gated behind the new `tiptap` feature. Enabling it includes the required JS files \
                 in the build.",
                "Fixed a bug which led to buttons not getting disabled properly.",
            ],
        )],
    },
    Release {
        title: "0.3.0",
        intro: &[],
        groups: &[
            (
                "Added",
                &[
                    "The `Consumer` type. Use `Consumer<In>` when you would otherwise write `Callback<In, ()>`.",
                    "The `Producer` type. Use `Producer<Out>` when you would otherwise write `Callback<(), Out>`.",
                    "The `ViewProducer` type. Use `ViewProducer` when you would otherwise write `Callback<(), leptos::View>`.",
                    "The `ViewCallback` type. Use `ViewCallback<In>` when you would otherwise write `Callback<In, leptos::View>`.",
                ],
            ),
            (
                "Changed",
                &[
                    "Updated to Leptos 0.5.1. No more `cx`!",
                    "The `render_option` prop of select inputs no longer requires you to call `.into_view()` on whatever your \
                 closure returns.",
                    "Collapsibles now use the slot approach.",
                ],
            ),
            ("Fixed", &["Indeterminate progress bars animate again."]),
            (
                "Removed",
                &[
                    "The `Callback` and `Callable` types moved into Leptos itself. They are imported with `use leptos::*`, which \
                 should already be present wherever the leptonic `Callback` was used.",
                ],
            ),
        ],
    },
    Release {
        title: "0.2.0",
        intro: &[],
        groups: &[
            (
                "Added",
                &[
                    "The `Out` type, abstracting over `Callback`s and `WriteSignal`s, for components where users are equally \
                 likely to want a new value stored or to handle it themselves. The input component is the first one using it.",
                    "The `Select`, `OptionalSelect` and `Multiselect` components accept a `class` prop.",
                    "The `Kbd` component together with `KbdShortcut`, displaying keyboard keys and shortcuts.",
                    "The `Chip` component accepts custom `id`, `class` and `style` props.",
                    "Slider styling variables: `--slider-bar-background-image`, `--slider-range-background-color`, \
                 `--slider-range-background-image`, `--slider-knob-border-width`, `--slider-knob-border-color`, \
                 `--slider-knob-border-style`, `--slider-knob-background-color` and `--slider-knob-halo-background-color`.",
                    "Initial version of a `ColorPicker` component.",
                ],
            ),
            (
                "Changed",
                &[
                    "The `on_change` prop of the `DateSelector` takes a `Callback` instead of a generic function.",
                    "Outlined and filled buttons use the `--button-outlined-[color]-*` and `--button-filled-[color]-*` \
                 variables. Outlined primary buttons use a dark text color.",
                    "Number inputs take optional `min`, `max` and `step` values, propagated to the input element.",
                    "The `set` prop of inputs is optional and no longer generic; it expects an `Out<String>`.",
                    "The `set_value` props of `Slider` and `RangeSlider` expect an `Out<f64>`.",
                    "The `on_toggle` prop of `Toggle` is now called `set_value` and expects an `Out<bool>`.",
                    "The `set_value` prop of `TiptapEditor` expects an `Option<Out<TiptapContent>>`.",
                    "Custom attributes are rendered with a `data-` prefix, making them standard-compliant.",
                    "The `max` and `progress` props of `ProgressBar` accept signals.",
                    "The `title` prop of `Alert` is a `Callback` instead of a generic closure.",
                    "The `step` prop of `Slider` is optional, making continuous sliders easier to set up.",
                    "The `Input` component was split into `TextInput`, `PasswordInput` and `NumberInput`. Their `label` prop was \
                 renamed to `placeholder`. The `InputType` enum was removed.",
                    "All `Select` components require a `search_text_provider` prop. `SelectOption` no longer requires `Display`.",
                ],
            ),
            (
                "Fixed",
                &[
                    "Buttons with variants respect their disabled state and trigger only one action per interaction.",
                    "Flat info buttons are styled correctly.",
                    "The installation instructions describe the required `web_sys_unstable_apis` opt-in.",
                ],
            ),
        ],
    },
    Release {
        title: "0.1.0",
        intro: &["Initial release."],
        groups: &[
            (
                "Added utilities",
                &[
                    "Callback types",
                    "`OptionalMaybeSignal` type",
                    "Global event listener contexts",
                ],
            ),
            (
                "Added components (with styles)",
                &[
                    "Root, Skeleton, Stack, Grid, Separator, Tabs, Collapsible, AppBar, Drawer, Button, Input, DateSelector, \
                 Slider, Select, Toggle, Alert, Toast, Modal, Progress, Popover, Chip, Icon, Link, Anchor, Typography and \
                 Transition components.",
                ],
            ),
        ],
    },
];

#[component]
pub fn PageChangelog() -> impl IntoView {
    view! {
        <DocPage title="Changelog">
            <p>"Notable changes of each leptonic version, newest first."</p>
            {RELEASES.iter().map(|release| view! { <ReleaseNotes release/> }).collect_view()}
        </DocPage>
    }
}

#[component]
fn ReleaseNotes(release: &'static Release) -> impl IntoView {
    view! {
        <Section title=release.title>
            {release.intro.iter().map(|paragraph| view! { <p>{with_inline_code(paragraph)}</p> }).collect_view()}
            {release
                .groups
                .iter()
                .map(|(title, entries)| view! {
                    <Section title=*title>
                        <ul>
                            {entries.iter().map(|entry| view! { <li>{with_inline_code(entry)}</li> }).collect_view()}
                        </ul>
                    </Section>
                })
                .collect_view()}
        </Section>
    }
}

/// Renders the backtick-quoted parts of `text` as inline code.
fn with_inline_code(text: &'static str) -> impl IntoView {
    text.split('`')
        .enumerate()
        .map(|(i, part)| {
            if i % 2 == 1 {
                view! { <Code inline=true>{part}</Code> }.into_any()
            } else {
                part.into_any()
            }
        })
        .collect_view()
}
