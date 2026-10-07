use std::fmt::Display;

use leptos::prelude::*;
use wasm_bindgen::JsCast;

use crate::{
    Out,
    atoms::{
        button::Button,
        field::Label,
        input::Input,
        listbox::{ListBox, ListBoxItem},
        search_field::SearchField,
        select::{HiddenSelect, Select as SelectAtom, SelectPopover, SelectTrigger, SelectValue},
    },
    components::{
        chip::{Chip, ChipColor},
        icon::Icon,
        prelude::Leptonic,
    },
    hooks::{
        SelectMode,
        collections::{CollectionMemo, Key, use_list_collection},
    },
    prelude::ViewCallback,
    utils::{classes::Classes, styles::Styles, visually_hidden::visually_hidden_styles},
};

/// Types usable as options of [`Select`], [`OptionalSelect`] and [`Multiselect`]. Options are
/// identified by their `Display` text, which must be unique among the options.
pub trait SelectOption: Clone + PartialEq + Display + Send + Sync + 'static {}

impl<T: Clone + PartialEq + Display + Send + Sync + 'static> SelectOption for T {}

fn option_key<O: Display>(option: &O) -> Key {
    Key::from(option.to_string())
}

/// What the select components share: the searchable options and their collection.
struct Options<O: SelectOption> {
    all: Signal<Vec<O>>,
    filtered: Memo<Vec<O>>,
    collection: CollectionMemo,
    search: RwSignal<String>,
}

// Not derived: that would require `O: Copy`.
#[allow(clippy::expl_impl_clone_on_copy)]
impl<O: SelectOption> Clone for Options<O> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<O: SelectOption> Copy for Options<O> {}

impl<O: SelectOption> Options<O> {
    fn new(
        all: Signal<Vec<O>>,
        search_text_provider: Callback<O, String>,
        search_filter_provider: Option<Callback<(String, Vec<O>), Vec<O>>>,
    ) -> Self {
        let search = RwSignal::new(String::new());
        let filter = resolve_search_filter(search_text_provider, search_filter_provider);
        let filtered = Memo::new(move |_| filter.run((search.get(), all.get())));
        let collection = use_list_collection(filtered.into(), option_key, move |option: &O| {
            search_text_provider.run(option.clone())
        });
        Self {
            all,
            filtered,
            collection,
            search,
        }
    }

    /// The options with the given keys.
    fn lookup(&self, keys: &[Key]) -> Vec<O> {
        self.all.with_untracked(|all| {
            keys.iter()
                .filter_map(|key| all.iter().find(|o| option_key(*o) == *key).cloned())
                .collect()
        })
    }
}

/// The field's visible label, while `label` is set.
fn label_view(label: MaybeProp<String>) -> impl IntoView {
    move || {
        label
            .get()
            .map(|label| view! { <Label classes="leptonic-select-label">{label}</Label> })
    }
}

/// The popover with search input and options, shared by the select components.
#[component]
fn SelectOptionsPopover<O: SelectOption>(
    options: Options<O>,
    render_option: ViewCallback<O>,
    autofocus_search: Signal<bool>,
) -> impl IntoView {
    view! {
        <SelectPopover classes="leptonic-select-options">
            <SelectSearchInput search=options.search autofocus_search=autofocus_search />
            <ListBox classes="leptonic-select-listbox">
                // One memo source: a `has_options` memo read before `filtered` (derived from the
                // same) skipped updates that changed the options but not their emptiness ("Effect
                // Read Order" in documentation/hooks-implementation.md).
                {move || {
                    let filtered = options.filtered.get();
                    if filtered.is_empty() {
                        view! {
                            <div class="leptonic-select-no-search-results">"No options..."</div>
                        }
                            .into_any()
                    } else {
                        filtered
                            .into_iter()
                            .map(|option| {
                                view! {
                                    <ListBoxItem key=option_key(&option) classes="leptonic-select-option">
                                        {render_option.render(option)}
                                    </ListBoxItem>
                                }
                            })
                            .collect_view()
                            .into_any()
                    }
                }}
            </ListBox>
        </SelectPopover>
    }
}

/// Single-select component (required selection).
///
/// Displays a dropdown allowing the user to choose exactly one option.
#[component]
pub fn Select<O>(
    #[prop(into)] options: Signal<Vec<O>>,
    #[prop(into)] selected: Signal<O>,
    #[prop(into)] set_selected: Out<O>,
    #[prop(into)] search_text_provider: Callback<O, String>,
    #[prop(into)] render_option: ViewCallback<O>,
    #[prop(into, optional)] search_filter_provider: Option<Callback<(String, Vec<O>), Vec<O>>>,
    #[prop(into, optional)] autofocus_search: Option<Signal<bool>>,
    /// A visible label above the select.
    #[prop(into, optional)]
    label: MaybeProp<String>,
    /// Names the select when there is no visible `label`.
    #[prop(into, optional)]
    aria_label: MaybeProp<String>,
    #[prop(into, optional)] is_disabled: Signal<bool>,
    /// The name of the hidden form element holding the selection.
    #[prop(into, optional)]
    name: Option<String>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
) -> impl IntoView
where
    O: SelectOption,
{
    let autofocus_search =
        autofocus_search.unwrap_or(expect_context::<Leptonic>().is_desktop_device);
    let options = Options::new(options, search_text_provider, search_filter_provider);
    let keys = Signal::derive(move || vec![option_key(&selected.get())]);

    view! {
        <SelectAtom
            collection=options.collection
            value=keys
            set_value=Callback::new(move |keys: Vec<Key>| {
                if let Some(option) = options.lookup(&keys).into_iter().next() {
                    set_selected.set(option);
                }
            })
            aria_label=aria_label
            is_disabled=is_disabled
            nostrip:name=name
            classes=classes.add("leptonic-select")
            styles=styles
        >
            {label_view(label)}
            <SelectTrigger classes="leptonic-select-selected">
                <SelectValue />
                <SelectShowTriggerIcon />
            </SelectTrigger>
            <SelectOptionsPopover options=options render_option=render_option autofocus_search=autofocus_search />
            <HiddenSelect />
        </SelectAtom>
    }
}

/// Optional single-select component (nullable selection).
///
/// Displays a dropdown allowing the user to choose one option or deselect.
#[component]
pub fn OptionalSelect<O>(
    #[prop(into)] options: Signal<Vec<O>>,
    #[prop(into)] selected: Signal<Option<O>>,
    #[prop(into)] set_selected: Out<Option<O>>,
    #[prop(into)] search_text_provider: Callback<O, String>,
    #[prop(into)] render_option: ViewCallback<O>,
    #[prop(into)] allow_deselect: Signal<bool>,
    #[prop(into, optional)] search_filter_provider: Option<Callback<(String, Vec<O>), Vec<O>>>,
    #[prop(into, optional)] autofocus_search: Option<Signal<bool>>,
    /// A visible label above the select.
    #[prop(into, optional)]
    label: MaybeProp<String>,
    /// Names the select when there is no visible `label`.
    #[prop(into, optional)]
    aria_label: MaybeProp<String>,
    #[prop(into, optional)] is_disabled: Signal<bool>,
    /// The name of the hidden form element holding the selection.
    #[prop(into, optional)]
    name: Option<String>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
) -> impl IntoView
where
    O: SelectOption,
{
    let autofocus_search =
        autofocus_search.unwrap_or(expect_context::<Leptonic>().is_desktop_device);
    let options = Options::new(options, search_text_provider, search_filter_provider);
    let keys = Signal::derive(move || selected.get().iter().map(option_key).collect::<Vec<_>>());

    let control = NodeRef::<leptos::html::Div>::new();

    view! {
        <SelectAtom
            collection=options.collection
            value=keys
            set_value=Callback::new(move |keys: Vec<Key>| {
                set_selected.set(options.lookup(&keys).into_iter().next());
            })
            aria_label=aria_label
            is_disabled=is_disabled
            nostrip:name=name
            classes=classes.add("leptonic-select")
            styles=styles
        >
            {label_view(label)}
            // The clear button is the trigger's sibling: buttons can't contain buttons.
            <div class="leptonic-select-control" node_ref=control>
                <SelectTrigger classes="leptonic-select-selected">
                    <SelectValue />
                    <SelectShowTriggerIcon />
                </SelectTrigger>
                {move || {
                    (allow_deselect.get() && selected.get().is_some())
                        .then(|| {
                            view! {
                                <Button
                                    classes="leptonic-select-deselect-trigger"
                                    aria_label="Clear selection"
                                    is_disabled=is_disabled
                                    on_press=move |_| {
                                        set_selected.set(None);
                                        // The pressed button disappears: keep the focus in the select.
                                        focus_after_render(control, &["button.leptonic-select-selected"], 0);
                                    }
                                >
                                    <Icon icon=icondata::BsXCircleFill />
                                </Button>
                            }
                        })
                }}
            </div>
            <SelectOptionsPopover options=options render_option=render_option autofocus_search=autofocus_search />
            <HiddenSelect />
        </SelectAtom>
    }
}

/// Multi-select component.
///
/// Displays a dropdown allowing the user to select multiple options,
/// shown as dismissible chips in the trigger area.
#[component]
pub fn Multiselect<O>(
    #[prop(optional, default = u64::MAX)] max: u64,
    #[prop(into)] options: Signal<Vec<O>>,
    #[prop(into)] selected: Signal<Vec<O>>,
    #[prop(into)] set_selected: Out<Vec<O>>,
    #[prop(into)] search_text_provider: Callback<O, String>,
    #[prop(into)] render_option: ViewCallback<O>,
    #[prop(into, optional)] search_filter_provider: Option<Callback<(String, Vec<O>), Vec<O>>>,
    #[prop(into, optional)] autofocus_search: Option<Signal<bool>>,
    /// A visible label above the select.
    #[prop(into, optional)]
    label: MaybeProp<String>,
    /// Names the select when there is no visible `label`.
    #[prop(into, optional)]
    aria_label: MaybeProp<String>,
    #[prop(into, optional)] is_disabled: Signal<bool>,
    /// The name of the hidden form element holding the selection.
    #[prop(into, optional)]
    name: Option<String>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
) -> impl IntoView
where
    O: SelectOption + Ord,
{
    let autofocus_search =
        autofocus_search.unwrap_or(expect_context::<Leptonic>().is_desktop_device);
    let options = Options::new(options, search_text_provider, search_filter_provider);
    let keys = Signal::derive(move || selected.get().iter().map(option_key).collect::<Vec<_>>());

    let control = NodeRef::<leptos::html::Div>::new();
    let deselect = Callback::new(move |option: O| {
        let mut vec = selected.get_untracked();
        let index = vec.iter().position(|it| it == &option).unwrap_or_default();
        vec.retain(|it| it != &option);
        set_selected.set(vec);
        // The pressed chip disappears: focus the next chip's dismiss button, else the trigger.
        focus_after_render(
            control,
            &[
                "button.leptonic-chip-dismiss",
                "button.leptonic-multiselect-trigger",
            ],
            index,
        );
    });

    view! {
        <SelectAtom
            collection=options.collection
            selection_mode=SelectMode::Multiple
            value=keys
            set_value=Callback::new(move |keys: Vec<Key>| {
                // At `max`, further options are refused (instead of dropping chosen ones).
                if keys.len() > usize::try_from(max).unwrap_or(usize::MAX) {
                    return;
                }
                let mut vec = options.lookup(&keys);
                vec.sort();
                set_selected.set(vec);
            })
            aria_label=aria_label
            is_disabled=is_disabled
            nostrip:name=name
            classes=classes.add("leptonic-select").add("leptonic-multiselect")
            styles=styles
        >
            {label_view(label)}
            // The box holds the chips and the trigger side by side: the chips' dismiss buttons
            // can't be inside the trigger button.
            <div class="leptonic-select-control leptonic-select-selected" node_ref=control>
                <For
                    each=move || selected.get()
                    key=option_key
                    children=move |item| {
                        let dismiss_label = format!("Remove {}", search_text_provider.run(item.clone()));
                        let removed = item.clone();
                        view! {
                            <Chip
                                color=ChipColor::Secondary
                                classes="leptonic-select-option"
                                dismiss_label=dismiss_label
                                is_disabled=is_disabled
                                on_dismiss=move |()| deselect.run(removed.clone())
                            >
                                {render_option.render(item)}
                            </Chip>
                        }
                    }
                />
                <SelectTrigger classes="leptonic-multiselect-trigger">
                    // The trigger's name includes the selection, as react-aria's value.
                    <SelectValue styles=visually_hidden_styles() />
                    <SelectShowTriggerIcon />
                </SelectTrigger>
            </div>
            <SelectOptionsPopover options=options render_option=render_option autofocus_search=autofocus_search />
            <HiddenSelect />
        </SelectAtom>
    }
}

/// Resolves the search filter callback, using the default lowercase-contains
/// filter when no custom provider is given.
fn resolve_search_filter<O: SelectOption>(
    search_text_provider: Callback<O, String>,
    custom: Option<Callback<(String, Vec<O>), Vec<O>>>,
) -> Callback<(String, Vec<O>), Vec<O>> {
    custom.unwrap_or(Callback::new(move |(s, opts): (String, Vec<O>)| {
        let lowered = s.to_lowercase();
        opts.into_iter()
            .filter(|it| {
                search_text_provider
                    .run(it.clone())
                    .to_lowercase()
                    .contains(lowered.as_str())
            })
            .collect()
    }))
}

/// Internal: the caret up/down icon shown in the trigger area.
#[component]
fn SelectShowTriggerIcon() -> impl IntoView {
    // A static caret: styles can turn it with the trigger's `data-pressed`/`aria-expanded`.
    view! {
        <div class="leptonic-select-show-trigger">
            <Icon icon=icondata::BsCaretDownFill />
        </div>
    }
}

/// Internal: the search text input inside the dropdown popover.
#[component]
fn SelectSearchInput(
    search: RwSignal<String>,
    #[prop(into)] autofocus_search: Signal<bool>,
) -> impl IntoView {
    // Mounted with the popover, so focusing on mount focuses on every opening.
    view! {
        <SearchField
            value=search
            set_value=search
            aria_label="Search"
            auto_focus=autofocus_search.get_untracked()
            classes=["leptonic-input", "search"]
        >
            <Input />
        </SearchField>
    }
}

/// After the next render, focuses the `index`th element of `control` matching the first selector
/// that matches any (clamped to the last match): keeps the focus in a select whose focused button
/// a press removed.
fn focus_after_render(
    control: NodeRef<leptos::html::Div>,
    selectors: &'static [&'static str],
    index: usize,
) {
    request_animation_frame(move || {
        let Some(control) = control.get_untracked() else {
            return;
        };
        for selector in selectors {
            let Ok(matches) = control.query_selector_all(selector) else {
                continue;
            };
            let count = matches.length();
            if count == 0 {
                continue;
            }
            let index = u32::try_from(index).unwrap_or(u32::MAX).min(count - 1);
            if let Some(element) = matches
                .item(index)
                .and_then(|node| node.dyn_into::<web_sys::HtmlElement>().ok())
            {
                let _ = element.focus();
            }
            return;
        }
    });
}
