use std::fmt::Display;

use leptos::prelude::*;
use web_sys::MouseEvent;

use crate::{
    Out,
    atoms::{
        listbox::{ListBox, ListBoxItem},
        select::{HiddenSelect, Select as SelectAtom, SelectPopover, SelectTrigger, SelectValue},
    },
    components::{
        chip::{Chip, ChipColor},
        icon::Icon,
        input::TextInput,
        prelude::Leptonic,
    },
    hooks::{PlacementX, PlacementY, Selection, SelectionKey, SelectionMode, SelectionSet},
    prelude::ViewCallback,
    utils::{classes::Classes, locale::WritingDirection, styles::Styles},
};

/// Trait for types that can be used as select options in the component-level
/// [`Select`], [`OptionalSelect`], and [`Multiselect`] components.
///
/// Extends [`SelectionKey`] and may gain select-specific methods in the future.
/// For custom item types that yield a different key type via [`Keyed`](crate::hooks::Keyed),
/// use the atom-level [`Select`](crate::atoms::select::Select) directly.
pub trait SelectOption: SelectionKey {}

impl<T: SelectionKey> SelectOption for T {}

/// Single-select component (required selection).
///
/// Displays a dropdown allowing the user to choose exactly one option.
/// Uses hook-based ARIA accessibility, keyboard navigation, and
/// overlay positioning internally via select atoms.
#[component]
#[allow(clippy::too_many_lines)]
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
    O: SelectOption + 'static,
{
    let autofocus_search =
        autofocus_search.unwrap_or(expect_context::<Leptonic>().is_desktop_device);

    // Search/filter state (component-level concern).
    let (search, set_search) = signal(String::new());

    let search_filter = resolve_search_filter(search_text_provider, search_filter_provider);
    let stored_options = StoredValue::new(options);
    let filtered_options =
        Memo::new(move |_| search_filter.run((search.get(), stored_options.get_value().get())));
    let has_options = Memo::new(move |_| !filtered_options.with(Vec::is_empty));

    // Map Signal<O> → Signal<Selection<O>>
    let selected_keys = Signal::derive(move || {
        let key = selected.get();
        Selection::Keys(std::iter::once(key).collect::<SelectionSet<O>>())
    });

    // Map Selection<O> → O
    let on_selection_change = Callback::new(move |sel: Selection<O>| {
        if let Selection::Keys(keys) = sel {
            if let Some(key) = keys.into_iter().next() {
                set_selected.set(key);
            }
        }
    });

    view! {
        <SelectAtom<O>
            items=filtered_options
            selection_mode=SelectionMode::Single
            selected_keys=selected_keys
            on_selection_change=on_selection_change
            get_text_value=search_text_provider
            classes=classes.add("leptonic-select")
            styles=styles
        >
            <SelectTrigger<O> classes="leptonic-select-selected">
                <SelectValue<O> />
                <SelectShowTriggerIcon />
            </SelectTrigger<O>>

            <SelectPopover<O>
                placement_x=Signal::derive(|| PlacementX::Left)
                placement_y=Signal::derive(|| PlacementY::Below)
                writing_direction=Signal::derive(|| WritingDirection::Ltr)
                classes="leptonic-select-options"
            >
                <SelectSearchInput
                    search=search
                    set_search=set_search
                    autofocus_search=autofocus_search
                />

                <ListBox<O> classes="leptonic-select-listbox">
                    {move || {
                        if has_options.get() {
                            filtered_options
                                .get()
                                .into_iter()
                                .map(|option| {
                                    let render_clone = option.clone();
                                    view! {
                                        <ListBoxItem<O> key=option classes="leptonic-select-option">
                                            {render_option.render(render_clone)}
                                        </ListBoxItem<O>>
                                    }
                                })
                                .collect_view()
                                .into_any()
                        } else {
                            view! {
                                <div class="leptonic-select-no-search-results">
                                    "No options..."
                                </div>
                            }
                            .into_any()
                        }
                    }}
                </ListBox<O>>
            </SelectPopover<O>>

            <HiddenSelect<O>
                get_text_value=search_text_provider
            />
        </SelectAtom<O>>
    }
}

/// Optional single-select component (nullable selection).
///
/// Displays a dropdown allowing the user to choose one option or deselect.
#[component]
#[allow(clippy::too_many_lines)]
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
    O: SelectOption + 'static,
{
    let autofocus_search =
        autofocus_search.unwrap_or(expect_context::<Leptonic>().is_desktop_device);

    let (search, set_search) = signal(String::new());

    let search_filter = resolve_search_filter(search_text_provider, search_filter_provider);
    let stored_options = StoredValue::new(options);
    let filtered_options =
        Memo::new(move |_| search_filter.run((search.get(), stored_options.get_value().get())));
    let has_options = Memo::new(move |_| !filtered_options.with(Vec::is_empty));

    // Map Signal<Option<O>> → Signal<Selection<O>>
    let selected_keys = Signal::derive(move || match selected.get() {
        Some(key) => Selection::Keys(std::iter::once(key).collect::<SelectionSet<O>>()),
        None => Selection::default(),
    });

    // Map Selection<O> → Option<O>
    let on_selection_change = Callback::new(move |sel: Selection<O>| {
        if let Selection::Keys(keys) = sel {
            set_selected.set(keys.into_iter().next());
        } else {
            set_selected.set(None);
        }
    });

    let deselect = move |e: MouseEvent| {
        e.prevent_default();
        e.stop_propagation();
        set_selected.set(None);
    };

    view! {
        <SelectAtom<O>
            items=filtered_options
            selection_mode=SelectionMode::Single
            selected_keys=selected_keys
            on_selection_change=on_selection_change
            get_text_value=search_text_provider
            classes=classes.add("leptonic-select")
            styles=styles
        >
            <SelectTrigger<O> classes="leptonic-select-selected">
                <SelectValue<O> />
                {move || {
                    (allow_deselect.get() && selected.get().is_some()).then(|| {
                        view! {
                            <div
                                class="leptonic-select-deselect-trigger"
                                on:click=deselect
                            >
                                <Icon icon=icondata::BsXCircleFill />
                            </div>
                        }
                    })
                }}
                <SelectShowTriggerIcon />
            </SelectTrigger<O>>

            <SelectPopover<O>
                placement_x=Signal::derive(|| PlacementX::Left)
                placement_y=Signal::derive(|| PlacementY::Below)
                writing_direction=Signal::derive(|| WritingDirection::Ltr)
                classes="leptonic-select-options"
            >
                <SelectSearchInput
                    search=search
                    set_search=set_search
                    autofocus_search=autofocus_search
                />

                <ListBox<O> classes="leptonic-select-listbox">
                    {move || {
                        if has_options.get() {
                            filtered_options
                                .get()
                                .into_iter()
                                .map(|option| {
                                    let render_clone = option.clone();
                                    view! {
                                        <ListBoxItem<O> key=option classes="leptonic-select-option">
                                            {render_option.render(render_clone)}
                                        </ListBoxItem<O>>
                                    }
                                })
                                .collect_view()
                                .into_any()
                        } else {
                            view! {
                                <div class="leptonic-select-no-search-results">
                                    "No options..."
                                </div>
                            }
                            .into_any()
                        }
                    }}
                </ListBox<O>>
            </SelectPopover<O>>

            <HiddenSelect<O>
                get_text_value=search_text_provider
            />
        </SelectAtom<O>>
    }
}

/// Multi-select component.
///
/// Displays a dropdown allowing the user to select multiple options,
/// shown as dismissible chips in the trigger area.
#[component]
#[allow(clippy::too_many_lines)]
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
    O: SelectOption + PartialOrd + Ord + 'static,
{
    let autofocus_search =
        autofocus_search.unwrap_or(expect_context::<Leptonic>().is_desktop_device);

    let (search, set_search) = signal(String::new());

    let search_filter = resolve_search_filter(search_text_provider, search_filter_provider);
    let stored_options = StoredValue::new(options);
    let filtered_options =
        Memo::new(move |_| search_filter.run((search.get(), stored_options.get_value().get())));
    let has_options = Memo::new(move |_| !filtered_options.with(Vec::is_empty));

    // Map Signal<Vec<O>> → Signal<Selection<O>>
    let selected_keys = Signal::derive(move || {
        Selection::Keys(selected.get().into_iter().collect::<SelectionSet<O>>())
    });

    // Map Selection<O> → Vec<O> (sorted, with max enforcement)
    let on_selection_change = Callback::new(move |sel: Selection<O>| {
        if let Selection::Keys(keys) = sel {
            let mut vec: Vec<O> = keys.into_iter().collect();
            vec.sort();
            vec.truncate(usize::try_from(max).unwrap_or(usize::MAX));
            set_selected.set(vec);
        }
    });

    let deselect = Callback::new(move |option: O| {
        let mut vec = selected.get_untracked();
        if let Some(pos) = vec.iter().position(|it| it == &option) {
            vec.remove(pos);
        }
        set_selected.set(vec);
    });

    view! {
        <SelectAtom<O>
            items=filtered_options
            classes=classes.add("leptonic-select").add("leptonic-multiselect")
            styles=styles
            selection_mode=SelectionMode::Multiple
            selected_keys=selected_keys
            on_selection_change=on_selection_change
            get_text_value=search_text_provider
        >
            <SelectTrigger<O> classes="leptonic-select-selected">
                {move || {
                    selected
                        .get()
                        .into_iter()
                        .map(|item| {
                            let deselect_clone = item.clone();
                            view! {
                                <div class="leptonic-select-option">
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
            </SelectTrigger<O>>

            <SelectPopover<O>
                placement_x=Signal::derive(|| PlacementX::Left)
                placement_y=Signal::derive(|| PlacementY::Below)
                writing_direction=Signal::derive(|| WritingDirection::Ltr)
                classes="leptonic-select-options"
            >
                <SelectSearchInput
                    search=search
                    set_search=set_search
                    autofocus_search=autofocus_search
                />

                <ListBox<O> classes="leptonic-select-listbox">
                    {move || {
                        if has_options.get() {
                            filtered_options
                                .get()
                                .into_iter()
                                .map(|option| {
                                    let render_clone = option.clone();
                                    view! {
                                        <ListBoxItem<O> key=option classes="leptonic-select-option">
                                            {render_option.render(render_clone)}
                                        </ListBoxItem<O>>
                                    }
                                })
                                .collect_view()
                                .into_any()
                        } else {
                            view! {
                                <div class="leptonic-select-no-search-results">
                                    "No options..."
                                </div>
                            }
                            .into_any()
                        }
                    }}
                </ListBox<O>>
            </SelectPopover<O>>

            <HiddenSelect<O>
                get_text_value=search_text_provider
            />
        </SelectAtom<O>>
    }
}

/// Resolves the search filter callback, using the default lowercase-contains
/// filter when no custom provider is given.
fn resolve_search_filter<O: Display + Clone + Send + Sync + 'static>(
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
    #[prop(into)] search: Signal<String>,
    #[prop(into)] set_search: WriteSignal<String>,
    #[prop(into)] autofocus_search: Signal<bool>,
) -> impl IntoView {
    view! {
        <TextInput
            get=search
            set=set_search
            should_be_focused=autofocus_search
            attr:class="search"
        />
    }
}
