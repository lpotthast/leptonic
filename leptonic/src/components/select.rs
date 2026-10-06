use std::fmt::Display;

use leptos::prelude::*;
use web_sys::{MouseEvent, PointerEvent};

use crate::{
    Out,
    atoms::{
        input::Input,
        listbox::{ListBox, ListBoxItem},
        search_field::SearchField,
        select::{
            HiddenSelect, Select as SelectAtom, SelectCtx, SelectPopover, SelectTrigger,
            SelectValue,
        },
    },
    components::{
        chip::{Chip, ChipColor},
        icon::Icon,
        prelude::Leptonic,
    },
    hooks::{
        SelectMode, TextFieldState,
        collections::{CollectionMemo, Key, use_list_collection},
    },
    prelude::ViewCallback,
    utils::{classes::Classes, styles::Styles},
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

/// Keeps the select's value in sync with a value owned by the caller.
#[component]
fn SyncValue(#[prop(into)] keys: Signal<Vec<Key>>) -> impl IntoView {
    let state = expect_context::<SelectCtx>().state;
    Effect::new(move |_| {
        let keys = keys.get();
        if untrack(|| state.value()) != keys {
            state.set_value(keys);
        }
    });
}

/// The popover with search input and options, shared by the select components.
#[component]
fn SelectOptionsPopover<O: SelectOption>(
    options: Options<O>,
    render_option: ViewCallback<O>,
    autofocus_search: Signal<bool>,
) -> impl IntoView {
    let has_options = Memo::new(move |_| !options.filtered.with(Vec::is_empty));
    view! {
        <SelectPopover classes="leptonic-select-options">
            <SelectSearchInput search=options.search autofocus_search=autofocus_search />
            <ListBox classes="leptonic-select-listbox">
                {move || {
                    if has_options.get() {
                        options
                            .filtered
                            .get()
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
                    } else {
                        view! {
                            <div class="leptonic-select-no-search-results">"No options..."</div>
                        }
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
            default_value=keys.get_untracked()
            on_change=Callback::new(move |keys: Vec<Key>| {
                if let Some(option) = options.lookup(&keys).into_iter().next() {
                    set_selected.set(option);
                }
            })
            classes=classes.add("leptonic-select")
            styles=styles
        >
            <SyncValue keys=keys />
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

    let deselect = move |e: MouseEvent| {
        e.prevent_default();
        e.stop_propagation();
        set_selected.set(None);
    };

    view! {
        <SelectAtom
            collection=options.collection
            default_value=keys.get_untracked()
            on_change=Callback::new(move |keys: Vec<Key>| {
                set_selected.set(options.lookup(&keys).into_iter().next());
            })
            classes=classes.add("leptonic-select")
            styles=styles
        >
            <SyncValue keys=keys />
            <SelectTrigger classes="leptonic-select-selected">
                <SelectValue />
                {move || {
                    (allow_deselect.get() && selected.get().is_some())
                        .then(|| {
                            view! {
                                // Inside the trigger: keep the press from opening the popover.
                                <div
                                    class="leptonic-select-deselect-trigger"
                                    on:pointerdown=|e: PointerEvent| e.stop_propagation()
                                    on:click=deselect
                                >
                                    <Icon icon=icondata::BsXCircleFill />
                                </div>
                            }
                        })
                }}
                <SelectShowTriggerIcon />
            </SelectTrigger>
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

    let deselect = Callback::new(move |option: O| {
        let mut vec = selected.get_untracked();
        vec.retain(|it| it != &option);
        set_selected.set(vec);
    });

    view! {
        <SelectAtom
            collection=options.collection
            selection_mode=SelectMode::Multiple
            default_value=keys.get_untracked()
            on_change=Callback::new(move |keys: Vec<Key>| {
                let mut vec = options.lookup(&keys);
                vec.sort();
                vec.truncate(usize::try_from(max).unwrap_or(usize::MAX));
                set_selected.set(vec);
            })
            classes=classes.add("leptonic-select").add("leptonic-multiselect")
            styles=styles
        >
            <SyncValue keys=keys />
            <SelectTrigger classes="leptonic-select-selected">
                {move || {
                    selected
                        .get()
                        .into_iter()
                        .map(|item| {
                            let deselect_clone = item.clone();
                            view! {
                                // Inside the trigger: keep presses on chips from opening the popover.
                                <div
                                    class="leptonic-select-option"
                                    on:pointerdown=|e: PointerEvent| e.stop_propagation()
                                >
                                    <Chip
                                        color=ChipColor::Secondary
                                        on:click=move |e: MouseEvent| {
                                            e.stop_propagation();
                                        }
                                        dismissible=move |e: MouseEvent| {
                                            e.stop_propagation();
                                            deselect.run(deselect_clone.clone());
                                        }
                                    >
                                        {render_option.render(item)}
                                    </Chip>
                                </div>
                            }
                        })
                        .collect_view()
                }}
                <SelectShowTriggerIcon />
            </SelectTrigger>
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
    // Read is_open from any SelectCtx available. Since this is always
    // inside a SelectTrigger, we can reach the context dynamically.
    // However, SelectCtx is generic over K, so we pass the open state
    // via a simple signal instead.
    //
    // For now, render a static down-caret. The open/close animation
    // should be handled via CSS data-open attribute on the trigger.
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
            state=TextFieldState::from(search)
            aria_label="Search"
            auto_focus=autofocus_search.get_untracked()
            classes="leptonic-input search"
        >
            <Input />
        </SearchField>
    }
}
