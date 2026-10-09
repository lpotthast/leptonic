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
            "Leptonic was rebuilt in two layers: hooks, ported from react-aria\u{2019}s hooks, which carry the \
             accessibility and interaction logic, and atoms, unstyled Leptos components built on them. The styled \
             components of 0.5.0 and their themes are gone: apps style the atoms with their own CSS, or start from the \
             optional atom theme. Most of the API changed; the notes below list what to change when upgrading from 0.5.0.",
        ],
        groups: &[
            (
                "Requirements",
                &[
                    "Leptos 0.8 (was 0.6), Rust edition 2024 and Rust 1.89 or newer.",
                    "`icondata` is no longer re-exported. Depend on `icondata` and `leptos_icons` yourself where you show \
                 icons.",
                    "Every build needs the `--cfg=web_sys_unstable_apis` rustflag, not only builds with clipboard support. \
                 See Installation.",
                ],
            ),
            (
                "Imports and features",
                &[
                    "Import atoms from `leptonic::atoms::<family>` and hooks from `leptonic::hooks::<family>`. \
                 Shared types and utilities such as `Out`, `ValueBinding` and `signal_ls` are exported from the \
                 crate root. The preludes and public `utils` module were removed.",
                    "Hooks are always available. The `atoms` feature adds the unstyled atoms; `intl-strings`, enabled \
                 by default, provides localized messages. `full` enables atoms, clipboard, syntax highlighting and \
                 localized messages.",
                    "The `tiptap` and `csr` features were removed: use the `leptos-tiptap` crate directly, and enable \
                 neither `ssr` nor `hydrate` for client-side rendered apps.",
                ],
            ),
            (
                "State, events, classes and styles",
                &[
                    "Atoms take their state as a value and a setter: `x` (a value or any signal) and `set_x` (an `Out`: a \
                 signal, a closure or a `Callback`); the setter of an `is_x` prop is `set_x` (`is_selected` and \
                 `set_selected`, `is_open` and `set_open`). Without `x`, they keep the state themselves, starting at \
                 `default_x`. `on_change` (for named states `on_x_change`, e.g. `on_open_change`) reports every change. \
                 See Hooks & Atoms.",
                    "Hooks own their state: you pass an initial value, read signals and change the state through the \
                 methods a hook returns. To keep the state in your app, bind it with a `ValueBinding`, made from an \
                 `RwSignal` or a signal pair. See Callbacks.",
                    "Pressable elements report presses through an optional `on_press` with a `PressEvent` (mouse, touch, \
                 pen, keyboard and screen readers alike) instead of `on_click` with a `MouseEvent`.",
                    "Events handled by leptonic stop propagating unless the handler calls `continue_propagation()`. See \
                 Event Propagation.",
                    "Atoms take `classes` and `styles` (`Classes` and `Styles`, from `leptos-classes` and `leptos-styles`, \
                 with typed CSS declarations) instead of `id`, `class` and `style`. Set other attributes with \
                 Leptos\u{2019} `attr:` syntax, e.g. `attr:id=\"save\"`. See Classes & Styles.",
                    "Props take `Signal`, `MaybeProp` and `Option<Callback<..>>` instead of `MaybeSignal`, \
                 `OptionalMaybeSignal` and leptonic\u{2019}s `Consumer` and `Producer`.",
                ],
            ),
            (
                "Removed: the components layer",
                &[
                    "The styled components (`leptonic::components`) and their themes (`leptonic-themes`, with the CSS \
                 variables of each component) were removed. Every one of them has an atom or a recipe instead, listed \
                 below. Atoms bring no styles: select their default class (`leptonic-<AtomName>`, e.g. \
                 `.leptonic-Button`) and the data attributes of their state (`[data-pressed]`, `[data-selected]`, ...) \
                 in your CSS, or include the optional atom theme (`@use \"leptonic/leptonic-atoms\";`). See Themes and \
                 the \u{201c}Styling\u{201d} section of every atom page.",
                    "`Root` \u{2192} a `ThemeProvider` around your app (controlled with `signal_ls` to remember the \
                 user\u{2019}s theme) and, if your app shows toasts, a `ToastRegion` on a `ToastQueue`. Modals need no \
                 root. The `Leptonic` context is gone (use `leptonic::platform` for platform checks), and so is the \
                 `--leptonic-vh` variable: use the `dvh` unit (`min-height: 100dvh`). See Installation.",
                    "`ThemeToggle` and `ThemeIcon` \u{2192} a switch atom on the context of `use_theme`. See Themes.",
                    "Buttons and links: `Button` \u{2192} the `Button` atom; `Link` \u{2192} the `Link` atom (`AnchorLink` \
                 for in-page links); `LinkButton` \u{2192} a `Link` with your button styles. The `variations` and `active` \
                 props of buttons are gone (variations resulted in non-compliant HTML); a variant is a data attribute you \
                 style, e.g. `attr:data-variant=\"secondary\"`.",
                    "Fields: `TextInput`, `PasswordInput` and `NumberInput` (with `Field` and `FieldLabel`) \u{2192} the \
                 `TextField` (`input_type=InputType::Password` for passwords) and `NumberField` atoms, with `Label`, \
                 `Input`, `Description` and `FieldError` inside; `Checkbox`, `Radio` and `Toggle` \u{2192} a \
                 `CheckboxField` with a `CheckboxButton`, a `RadioGroup` with `RadioField`s and `RadioButton`s, and a \
                 `SwitchField` with a `SwitchButton`; `Slider` and `RangeSlider` \u{2192} the \
                 `Slider` atom (a range has two thumbs); `Select`, `OptionalSelect` and `Multiselect` \u{2192} the `Select` \
                 atom (`SelectMode` for multiple selection), or a `ComboBox` to search the options.",
                    "Overlays: `Modal` (with `ModalHeader`, `ModalTitle`, `ModalBody` and `ModalFooter`) and `ModalRoot` \
                 \u{2192} `ModalBackdrop`, `ModalContent` and `Dialog` (with `DialogTitle`), opened by a `DialogTrigger` \
                 or a signal; `Drawer` \u{2192} a modal whose panel slides in from the edge of the screen (see the Modal \
                 atom); `Popover` (with `PopoverTrigger`) \u{2192} the `Popover` atom in a `DialogTrigger`.",
                    "Navigation and disclosure: `Tabs` (with `Tab`) \u{2192} `Tabs`, `TabList`, `Tab` and `TabPanel`; \
                 `Collapsible` (with `Collapsibles`) \u{2192} `Disclosure` (with `DisclosureTrigger` and \
                 `DisclosurePanel`) and `DisclosureGroup`.",
                    "Collections and status: the `Table` component (with `TableHeaderCell`) \u{2192} the table atoms; \
                 `Chip` \u{2192} `TagGroup` with `TagList` and `Tag`; `ProgressBar` and `Meter` \u{2192} their atoms; the \
                 toasts (`ToastRoot`, `Toasts` and `Toast`) \u{2192} `ToastRegion` on a `ToastQueue`, with `Toast`, \
                 `ToastContent`, `ToastTitle`, `ToastDescription` and `ToastCloseButton`; `Alert` \u{2192} an element \
                 with `role=\"alert\"` (see Status).",
                    "Dates and colors: `DateSelector`, `DatePicker` and `DateTimeInput` (with its `GuideMode`) \u{2192} the \
                 `Calendar`, `DateField` and `DatePicker` atoms, which can be typed into and pick `jiff` dates (with \
                 times, if you like) instead of `time::OffsetDateTime`; `ColorPicker` (with `ColorPreview`, \
                 `ColorPalette` and `HueSlider`) \u{2192} the `ColorPicker` atom around `ColorArea`, `ColorSlider`, \
                 `ColorField`, `ColorSwatchPicker`, ...",
                    "Keys: `Kbd`, `KbdKey` and `KbdShortcut` \u{2192} `ShortcutKeys` (a `Shortcut` in the form of the \
                 user\u{2019}s platform) and `Keys` (keys as given).",
                    "Layout and decoration, which have no behavior: `AppBar`, `Card`, `Tile`, `Stack`, `Grid` (with `Row` \
                 and `Col`), `Skeleton`, `Box` and the typography components (`H1` to `H6`, `P`) \u{2192} HTML elements \
                 and your CSS (recipes in Content & Layout); `Separator` \u{2192} the `Separator` atom; `Icon` \u{2192} \
                 `leptos_icons` with `icondata`; the transitions (`Collapse`, `Fade`, `Grow`, `Slide` and `Zoom`) \u{2192} \
                 CSS animations on the atoms\u{2019} `data-entering` and `data-exiting` attributes.",
                    "Content: `Code` \u{2192} `leptonic` (feature `syntax-highlight`) for the \
                 highlighting and `leptonic::write_text` (feature `clipboard`) for a copy button; \
                 `TiptapEditor` \u{2192} the `leptos-tiptap` crate; `SanitizedHtml` \u{2192} the `ammonia` crate; \
                 `Quicksearch` (with `QuicksearchTrigger` and `QuicksearchOption`) \u{2192} a `SearchField`, a \
                 `ComboBox` or a modal.",
                ],
            ),
            (
                "Renamed",
                &[
                    "`Toggle` \u{2192} a `SwitchField` with a `SwitchButton`. Its `state` and `set_state` \u{2192} the \
                 field's `is_selected` and `set_selected`; its looks (`ToggleIcons`, `ToggleSize` and `ToggleVariant`) are your CSS.",
                    "`Anchor` \u{2192} `AnchorLink`.",
                    "`LinkExt` \u{2192} `Link`, which takes a `target`; `LinkExtTarget` \u{2192} `LinkTarget`.",
                    "`create_signal_ls` \u{2192} `signal_ls`.",
                    "The key caps\u{2019} `Key` \u{2192} `KeyboardKey` (`leptonic`), which keyboard handling \
                 uses too.",
                    "`HSV` and `RGB8` moved to `leptonic`, next to the new `HSL`, `Alpha<C>` (any color with \
                 an alpha channel; replaces `RGBA8`) and `Color`. `ColorSpace` was removed: a `Color` keeps the color \
                 space it was set in.",
                    "`Size` \u{2192} `CssDimension`, of which `Width` and `Height` are now aliases: write `em(1.0)`, \
                 `px(4)` or `pct(50.0)` (`leptonic::leptos_styles::css`).",
                ],
            ),
            (
                "Changed",
                &[
                    "`Button`: `on_click` \u{2192} `on_press`, `disabled` \u{2192} `is_disabled`. `button_type` makes a \
                 button submit or reset its form; without it, buttons have `type=\"button\"`.",
                    "`Checkbox` \u{2192} a `CheckboxField` with a `CheckboxButton`: `checked` and `set_checked` \u{2192} the \
                 field's `is_selected` and `set_selected`; the button's children are its label. New: `is_indeterminate`, `is_required`, `is_invalid`, `name` and `form_value`. Removed: \
                 `variant` and `size`.",
                    "`Slider` is generic over its number type and takes one value per thumb (`values` and `set_values`). \
                 `min` and `max` \u{2192} `min_value` and `max_value`, `value_display` \u{2192} `format_options`, \
                 `disabled` \u{2192} `is_disabled`; name it with a `Label` or `aria_label`.",
                ],
            ),
            (
                "Removed",
                &[
                    "`Consumer` and `Producer` (and `consumer` and `producer`). Use Leptos\u{2019} `Callback`.",
                    "The global event contexts `GlobalClickEvent` and `GlobalKeyboardEvent`. Listen to document events \
                 with leptos-use\u{2019}s `use_event_listener`, or detect outside interactions with \
                 `use_interact_outside`.",
                    "`OptionalSignal` and `OptionalMaybeSignal`.",
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
                 press before leptonic\u{2019}s atoms stop it.",
                    "Landmarks: `use_landmark` makes a region of the page a landmark, and F6 and Shift+F6 move the focus \
                 between landmarks (Alt+F6 to the main one).",
                    "Atoms, unstyled Leptos components rendering one element each: `Breadcrumbs`, `Button`, `Calendar` \
                 and `RangeCalendar` with `CalendarHeading`, `CalendarPreviousButton`, `CalendarNextButton`, \
                 `CalendarErrorMessage`, `CalendarGrid` and its parts, `CalendarCell` and `CalendarCellButton`, \
                 `CheckboxField` with `CheckboxButton`, `CheckboxGroup`, `ColorArea`, `ColorField` and `ColorChannelField`, `ColorPicker`, \
                 `ColorSlider` with `ColorSliderTrack` and `ColorSliderOutput`, `ColorSwatch`, `ColorSwatchPicker` with \
                 `ColorSwatchPickerItem`, `ColorThumb`, `ColorWheel` with `ColorWheelTrack`, `ComboBox`, `DateField` and \
                 `TimeField` with `DateInput` and `DateSegment`, `DatePicker` and `DateRangePicker` with `DatePickerGroup` \
                 and `DatePickerButton`, `Dialog` and `DialogTrigger`, `Disclosure` and `DisclosureGroup`, `DismissButton`, the field parts `Label`, `Description` and `FieldError`, \
                 `FocusManagerProvider`, `FocusRing`, `FocusScope`, `Focusable`, `Form`, `Grid`, `GridList`, \
                 `Hoverable`, `Input` and `TextArea`, `Link` and `AnchorLink`, `ListBox`, `Menu` with `MenuTrigger` and \
                 `SubmenuTrigger`, `Meter`, `ModalBackdrop` and `ModalContent`, `NumberField`, `OverlayArrow`, \
                 `Popover`, `Pressable` and `PressResponder`, `ProgressBar`, `RadioGroup` with `RadioField` and `RadioButton`, `SearchField`, \
                 `Select`, `Separator`, `ShortcutKeys` and `Keys`, `Slider`, `SwitchField` with `SwitchButton`, `Table`, `Tabs`, `TagGroup` with \
                 `TagList`, `Tag` and `TagRemoveButton`, `TextField`, `ToastRegion` and `Toast` with \
                 `ToastContent`, `ToastTitle`, `ToastDescription` and `ToastCloseButton`, `ToggleButton` and \
                 `ToggleButtonGroup`, `Toolbar`, `Tooltip` and `VisuallyHidden`.",
                    "Default classes: every atom rendering an element of its own carries the class \
                 `leptonic-<AtomName>` (e.g. `leptonic-Button`) in front of the `classes` you pass, and data attributes \
                 for its state, for your CSS to select.",
                    "The atom theme, an optional stylesheet for the atoms ported from react-aria-components\u{2019} starter \
                 styles: `@use \"leptonic/leptonic-atoms\";` after setting `style-dir` (see Installation). Its light and \
                 dark mode follow the nearest `ThemeProvider`; `--tint` recolors it.",
                    "`ThemeProvider` as an atom, with `theme` and `set_theme` (or `default_theme`) and `use_theme`, to \
                 read and change the theme of the closest provider.",
                    "Virtualization of long lists: the `Virtualizer` atom renders only the visible options of a \
                 `ListBox` (with `ListBoxItems`), positioned by a `ListLayout` of fixed or measured row sizes; \
                 `VirtualList` renders only the visible rows of a plain list such as a log, optionally following its \
                 end. `use_virtualizer_state`, `use_scroll_view`, `use_virtualizer_item` and the `Layout` trait \
                 (`leptonic::hooks::virtualizer`) virtualize markup and layouts of your own.",
                    "`Color` (`leptonic`): a color in any of the spaces HSV, HSL and RGB, parsed from \
                 CSS-like text (`#rgb`, `#rrggbb`, `rgb()`, `hsb()`, `hsl()`), with `color_name` and `hue_name` \
                 describing colors in words in the user\u{2019}s locale (\u{201c}dark vibrant blue\u{201d}).",
                    "`leptonic::jiff`, the re-exported date crate: calendars pick `jiff::civil::Date` values; date fields and \
                 pickers are generic over `civil::Date`, `civil::DateTime` and `Zoned`, time fields over `civil::Time`, \
                 `civil::DateTime` and `Zoned`, with segments in the order and format of the locale.",
                    "Internationalization (number, date and list formatting, collation, plural rules) based on ICU4X, \
                 which also works during server-side rendering, with `I18nProvider`, `use_locale` and `use_direction`.",
                    "Localized texts: the labels, descriptions and announcements of the hooks and atoms come in 34 \
                 languages and follow the locale (`intl-strings` feature, on by default; `use_localized_strings` gives \
                 your code the same messages). Grids, grid lists and tables announce selection changes.",
                    "Element ids that are stable between server-side rendering and hydration (`use_id`).",
                    "`leptonic::write_text`, to copy text to the clipboard (`clipboard` feature).",
                    "A live announcer for screen reader announcements, and keyboard shortcuts (`Shortcut`, \
                 `KeyboardShortcuts`); `Shortcut::keys` lists the keys to show for a shortcut on a platform.",
                    "`is_text_input` and `is_typing_target` (`leptonic`), telling whether keys pressed \
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
                    "For apps that followed the development branch: the `components` feature and `leptonic::components` \
                 were removed with the components layer (see \u{201c}Removed: the components layer\u{201d}), and with \
                 them the `sanitize` feature and the `TextField`, `SearchField`, `NumberField`, `CheckboxGroup` and \
                 `RadioGroup` components; use the atoms of the same names.",
                    "For apps that followed the development branch: the `FocusManager` atom \u{2192} \
                 `FocusManagerProvider`; `FieldLabelProps` \u{2192} `LabelContext`; `use_long_press` was merged into \
                 `use_press` (`on_long_press`); `PlacementX` and `PlacementY` \u{2192} `Placement`.",
                    "`use_press`: `force_propagation` \u{2192} `propagation: PressPropagation`, `force_prevent_default` \
                 was removed, `on_press` is an `Option<Callback<PressEvent>>`.",
                    "`use_move` reports movement deltas only: the arrow keys move by one pixel on both axes, and callers \
                 track the position and whether a move is in progress (`on_move_start`, `on_move_end`) themselves.",
                    "Hooks that took several arguments take one input struct, with the former arguments (`state`, \
                 `element`, \u{2026}) as fields: `use_calendar`, `use_range_calendar`, `use_calendar_heading`, \
                 `use_date_field`, `use_time_field`, `use_date_segment`, `use_date_picker`, `use_date_picker_group`, \
                 `use_date_range_picker`, `use_tooltip_trigger`, `use_landmark`, `use_toast`, `use_toast_region`, \
                 `use_formatted_text_field`, `use_list_keyboard_delegate`, `use_scroll_view`, `use_virtualizer_item`, \
                 `use_draggable_collection`, `use_table_header_placeholder`, `use_table_selection_checkbox` and \
                 `use_table_select_all_checkbox`.",
                    "Renamed: `UseFormValidationStateReturn` \u{2192} `FormValidationState`; the color methods lose \
                 their `get_` prefix (`channel_value`, `channel_range`, `display_color`, \u{2026}); `HSV::value` \
                 \u{2192} `brightness`; `RGB8::from_hex` \u{2192} `parse::<RGB8>()`; `DateTimeFormatOptions::hour12` \
                 \u{2192} `hour_cycle`; `tab_props` and `tab_panel_props` of the tab hooks \u{2192} `props`; the \
                 `on_change` of the color area\u{2019}s and color wheel\u{2019}s input props \u{2192} `on_input`.",
                    "Removed: the `LinkButton` atom (use a `Link` with your button styles), `use_filtered_list_state` and \
                 `use_slot_id`.",
                    "Value-like selections are typed: `RadioGroup<V>`, `CheckboxGroup<V>` and `ToggleButtonGroup<V>` hold \
                 values of any `V: SelectionValue` (`Key`, `String`, the integers, or your enum through \
                 `leptonic::selection_value!`) instead of `Key`s; their items still take a `Key`, into which a value \
                 converts. `Select<S>` and `ComboBox<S>` take the shape of their value as their type, which replaces the \
                 `selection_mode` prop: `Option<V>` selects one value, `Vec<V>` several (`<Select<Vec<Key>>>`); the \
                 `value` of `ComboBoxValue`, which their `validate` gets, is that typed value. `SelectionValue` replaces \
                 `ToKey`. A group without a typed prop names its type (`<RadioGroup<Key> name=\"plan\">`), and \
                 `default_value` takes the value itself, not a string. \
                 `ToggleButtonGroup`'s `default_selected_keys`, `selected_keys`, `set_selected_keys` and \
                 `on_selection_change` \u{2192} `default_value`, `value`, `set_value` and `on_change`. `TableBody`'s \
                 children are optional (`<TableBody/>` for an empty table).",
                    "The single `Checkbox`, `Radio` and `Switch` atoms were removed, as react-aria-components deprecates \
                 them. A checkbox is a `CheckboxField` (a `<div>` taking the state props) with a `CheckboxButton` inside \
                 (the clickable `<label>`, with the data attributes the single atom had); likewise a `RadioField` with a \
                 `RadioButton` in a `RadioGroup`, and a `SwitchField` with a `SwitchButton`. Move `classes` and the \
                 children to the button; style `.leptonic-CheckboxButton` instead of `.leptonic-Checkbox`.",
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
