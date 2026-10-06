use leptonic::components::prelude::*;
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
        title: "Unreleased (main)",
        intro: &[
            "Leptonic was rebuilt in three layers: hooks, atoms and components. The hooks are ports of react-aria's \
             hooks and carry the accessibility and interaction logic of everything built on top of them.",
        ],
        groups: &[
            (
                "Added",
                &[
                    "Hooks ported from react-aria for interactions (`use_press`, `use_hover`, `use_move`, `use_keyboard`, \
                 drag and drop, ...), focus management, overlays, selection and collections, and for buttons, links, \
                 text and number fields, checkboxes, radios, switches, sliders, listboxes, selects, menus, comboboxes, \
                 tooltips, tabs, grids, tables, trees, color controls, dates and more.",
                    "Atoms: unstyled single-element components built on the hooks.",
                    "`Checkbox`, `Radio` and `RadioGroup` components.",
                    "`TextField`, `SearchField` and `NumberField` components: inputs with label, description and validation. \
                 `NumberField` is generic over its number type.",
                    "Internationalization (number, date and list formatting, collation, plural rules) based on ICU4X, which \
                 also works during server-side rendering.",
                    "Element ids that are stable between server-side rendering and hydration.",
                ],
            ),
            (
                "Changed",
                &[
                    "Requires Leptos 0.8 and Rust edition 2024.",
                    "Events handled by leptonic stop propagating unless the handler calls `continue_propagation()`. See \
                 Event Propagation.",
                    "Hooks own their state: you set an initial value, read signals and change state through the callbacks \
                 a hook returns. There are no controlled/uncontrolled prop pairs.",
                    "Components take `classes` and `styles` props (based on `leptos-classes` and `leptos-styles`).",
                    "Reworked the `Popover` component for much greater flexibility.",
                ],
            ),
            (
                "Removed",
                &[
                    "Buttons no longer support `variations`. They resulted in non-compliant HTML.",
                    "The `OptionalSignal` prop type.",
                    "The `TextInput`, `PasswordInput` and `NumberInput` components and their `Label`, `Field`, `FieldLabel` \
                 and `FormControl` helpers. Use `TextField` (with `input_type=InputType::Password` for passwords) and \
                 `NumberField`.",
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
                    "Dependency on the `icondata` crate. Leptonic re-exports it through its prelude, so you don't have to \
                 depend on it yourself.",
                ],
            ),
            (
                "Removed",
                &[
                    "Dependency on the `leptos-icons` crate. You can remove it from your `Cargo.toml` as well if you don't \
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
                 to `Modal`. If you previously used `Modal`, you might see some 'requires Fn but is FnOnce' errors. Storing \
                 values moved into a `Modal`'s children with `StoredValue::new` should be a quick fix.",
                "Tabs are now SSR compatible. Rendering order changed to make this possible. This should not affect anyone.",
                "Toggles are now SSR compatible. Rendering of the (optional) icons changed. This should only affect you if \
                 custom styling is in play.",
                "Tables are now SSR compatible. Components were renamed to `TableHeader`, `TableBody`, `TableRow`, \
                 `TableHeaderCell` and `TableCell`.",
                "The `build.rs` script, which previously had to be created by the consumer, is no longer required. It moved \
                 into leptonic itself and copies the files required for a build.",
                "The `Code` block component now has a \"Copy to clipboard\" button (thanks to https://github.com/wt).",
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
                    "Outlined and filled buttons use the `--button-outlined-[color]-...` and `--button-filled-[color]-...` \
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
