use leptos::*;
use std::fmt::Debug;
use std::hash::Hash;
use web_sys::{HtmlElement, KeyboardEvent, MouseEvent};

use crate::{
    components::{
        chip::{Chip, ChipColor},
        icon::Icon,
        input::TextInput,
        prelude::Leptonic,
    },
    prelude::{Consumer, GlobalClickEvent, GlobalKeyboardEvent, ViewCallback},
    Out,
};

pub trait SearchTextProvider {
    fn get_searchable_content() -> String;
}

pub trait SelectSearchable {
    fn get_searchable_content() -> String;

    fn matches(&self, lowercase_searchable_content: &str, lowercase_search: &str) -> bool {
        lowercase_searchable_content.contains(lowercase_search)
    }
}

pub trait SelectOption: Debug + Clone + PartialEq {}

impl<T: Debug + Clone + PartialEq> SelectOption for T {}

// TODO: select_previous and select_next could be made more efficient.
// If we would know that the initial vec from which the current preselect'ed option was taken didn't change
// and if we also keep track of the index of this option in the vec, we can just read the previous / next option
// be decrementing or incrementing the old index!

// TODO: Prop: close_options_menu_on_selection: bool
// TODO: Prop: selection_changed: Consumer<Selection<T>>
// TODO: multiselect deselect performance
// TODO: remove code duplication between select variants

// TODO: Replace select_previous and select_next with a function that stores the current index and does not need to traverse on each call!

fn select_previous<O: SelectOption + 'static>(
    available: &[O],
    preselected: ReadSignal<Option<O>>,
    set_preselected: WriteSignal<Option<O>>,
) {
    let previous = preselected.with_untracked(|current| match current {
        Some(current) => match available.iter().position(|it| it == current) {
            Some(current_pos) => match current_pos >= 1 {
                true => Some(available[current_pos - 1].clone()),
                false => available.last().cloned(),
            },
            None => available.last().cloned(),
        },
        None => available.last().cloned(),
    });
    set_preselected.set(previous);
}

fn select_next<O: SelectOption + 'static>(
    available: &[O],
    preselected: ReadSignal<Option<O>>,
    set_preselected: WriteSignal<Option<O>>,
) {
    let next = preselected.with_untracked(|current| match current {
        Some(current) => match available.iter().position(|it| it == current) {
            Some(current_pos) => match (current_pos + 1) < available.len() {
                true => Some(available[current_pos + 1].clone()),
                false => available.first().cloned(),
            },
            None => available.first().cloned(),
        },
        None => available.first().cloned(),
    });
    set_preselected.set(next);
}

#[component]
#[allow(clippy::too_many_lines)]
pub fn Select<O>(
    #[prop(into)] options: MaybeSignal<Vec<O>>,
    #[prop(into)] selected: Signal<O>,
    #[prop(into)] set_selected: Out<O>,
    #[prop(into)] search_text_provider: Consumer<O, String>,
    #[prop(into)] render_option: ViewCallback<O>,
    #[prop(into, optional)] render_selected_option: Option<ViewCallback<O>>,
    #[prop(into, optional)] search_filter_provider: Option<Consumer<(String, Vec<O>), Vec<O>>>,
    #[prop(into, optional)] autofocus_search: Option<Signal<bool>>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    #[prop(into, optional)] no_options: Option<ViewCallback<String>>,
    #[prop(into, optional)] new_option_when_no_options: Option<Consumer<String, O>>,
    #[prop(into, optional)] hide_disabled: bool,
) -> impl IntoView
where
    O: SelectOption + 'static,
{
    let id: uuid::Uuid = uuid::Uuid::new_v4();
    let id_string = format!("s-{id}");
    let id_selector_string = format!("#{id_string}");

    let (focused, set_focused) = create_signal(false);
    let (show_options, set_show_options) = create_signal(false);

    let autofocus_search =
        autofocus_search.unwrap_or(expect_context::<Leptonic>().is_desktop_device);

    let render_selected_option = render_selected_option.unwrap_or(render_option);

    let search_should_be_focused =
        Signal::derive(move || show_options.get() && autofocus_search.get());
    let (search_is_focused, set_search_is_focused) = create_signal(false);

    let stored_options = store_value(options);
    let (preselected, set_preselected) = create_signal(Option::<O>::None);
    let memoized_preselected = create_memo(move |_| preselected.get());

    let (search, set_search) = create_signal(String::new());

    let search_filter_provider =
        search_filter_provider.unwrap_or(Consumer::new(move |(s, o): (String, Vec<O>)| {
            let lowercased_search = s.to_lowercase();
            o.into_iter()
                .filter(|it| {
                    search_text_provider
                        .consume(it.clone())
                        .to_lowercase()
                        .contains(lowercased_search.as_str())
                })
                .collect::<Vec<O>>()
        }));

    let filtered_options = create_memo(move |_| {
        search_filter_provider.consume((search.get(), stored_options.get_value().get()))
    });

    let has_options = create_memo(move |_| !filtered_options.with(Vec::is_empty));

    let select = Consumer::new(move |option: O| {
        set_selected.set(option);
        set_show_options.set(false);
    });

    let is_selected = move |option: &O| selected.with(|selected| selected == option);

    let is_disabled = move |option: &O| selected.with(|selected| selected == option);

    let is_disabled_untracked =
        move |option: &O| selected.with_untracked(|selected| selected == option);

    // We need to check for global mouse events.
    // If our option list is shown and such an event occurs and does not target our option list, the options list should be closed.
    create_click_away_listener(id_selector_string, show_options, set_show_options.into());

    create_key_down_listener(move |e| {
        match (show_options.get_untracked(), focused.get_untracked()) {
            (true, _) => match e.key().as_str() {
                "Escape" => set_show_options.set(false),
                "Backspace" => {
                    if !search_is_focused.get_untracked() {
                        set_show_options.set(false);
                    }
                }
                "ArrowUp" => {
                    e.prevent_default();
                    e.stop_propagation();
                    // TODO: Use options_available_for_preselect.with_untracked when https://github.com/leptos-rs/leptos/issues/1212 is resolved and released.
                    select_previous(
                        &filtered_options.get_untracked(),
                        preselected,
                        set_preselected,
                    );
                }
                "ArrowDown" => {
                    e.prevent_default();
                    e.stop_propagation();
                    // TODO: Use options_available_for_preselect.with_untracked when https://github.com/leptos-rs/leptos/issues/1212 is resolved and released.
                    select_next(
                        &filtered_options.get_untracked(),
                        preselected,
                        set_preselected,
                    );
                }
                "Enter" => {
                    e.prevent_default();
                    e.stop_propagation();
                    if let Some(preselected) = preselected.get_untracked() {
                        if !is_disabled_untracked(&preselected) {
                            select.consume(preselected);
                        }
                    }
                }
                _ => {}
            },
            (false, true) => match e.key().as_str() {
                "Enter" | "ArrowDown" => {
                    e.prevent_default();
                    e.stop_propagation();
                    set_show_options.set(true);
                }
                _ => {}
            },
            _ => {}
        }
    });

    let toggle_show = move || set_show_options.update(|val| *val = !*val);

    let wrapper: NodeRef<html::Div> = create_node_ref();

    // Put focus back on our wrapper when the dropdown was closed while the search input had focus.
    create_effect(move |_| {
        if !show_options.get() && search_is_focused.get_untracked() {
            // TODO: Use with() when available.
            if let Some(wrapper) = wrapper.get() {
                wrapper.focus().expect("wrapper to be focusable");
            } else {
                tracing::warn!("missing node_ref");
            }
        }
    });

    view! {
        <div
            node_ref=wrapper
            class="leptonic-select-wrapper"
            tabindex=0
            on:blur=move |_| set_focused.set(false)
            on:focus=move |_| set_focused.set(true)
        >
            <leptonic-select
                id=id_string
                data-variant="select"
                aria-haspopup="listbox"
                class=class
                class:active=move || show_options.get()
                style=style
            >
                <leptonic-select-selected on:click=move |_| toggle_show()>
                    {move || render_selected_option.render(selected.get())}
                    <leptonic-select-show-trigger>
                        {move || match show_options.get() {
                            true => view! { <Icon icon=icondata::BsCaretUpFill/> },
                            false => view! { <Icon icon=icondata::BsCaretDownFill/> },
                        }}

                    </leptonic-select-show-trigger>
                </leptonic-select-selected>

                <leptonic-select-options class:shown=move || {
                    show_options.get()
                }>
                    <TextInput
                        get=search
                        set=set_search
                        should_be_focused=search_should_be_focused
                        on_focus_change=move |focused| {
                            if show_options.get_untracked() {
                                set_search_is_focused.set(focused);
                            }
                        }

                        class="search"
                    />

                    <Show when=move || show_options.get() fallback=move || ()>
                        // TOD: Use <For> once leptos 0.4 is out. Use full option for hash.
                        {filtered_options
                            .get()
                            .into_iter()
                            .map(|option| {
                                let clone1 = option.clone();
                                let clone2 = option.clone();
                                let clone3 = option.clone();
                                let clone4 = option.clone();
                                let clone5 = option.clone();
                                view! {
                                    <leptonic-select-option
                                        class:preselected=move || {
                                            memoized_preselected
                                                .with(|preselected| preselected.as_ref() == Some(&option))
                                        }

                                        class:selected=move || is_selected(&clone4)
                                        class:disabled=move || is_disabled(&clone5)
                                        on:mouseenter=move |_e| {
                                            set_preselected.set(Some(clone3.clone()));
                                        }

                                        on:click=move |_e| {
                                            if !is_disabled_untracked(&clone2) {
                                                select.consume(clone2.clone());
                                            }
                                        }
                                    >

                                        {render_option.render(clone1)}
                                    </leptonic-select-option>
                                }
                            })
                            .collect_view()}

                        {move || match has_options.get() {
                            true => ().into_view(),
                            false => {
                                new_option_when_no_options
                                    .map_or_else(
                                        move || {
                                            view! {
                                                <leptonic-select-no-search-results>
                                                    {no_options
                                                        .map_or_else(
                                                            || { "No options...".into_view() },
                                                            |x| x.render(search.get()).into_view(),
                                                        )}

                                                </leptonic-select-no-search-results>
                                            }
                                                .into_view()
                                        },
                                        move |x| {
                                            let option = x.consume(search.get());
                                            set_preselected.set(Some(option.clone()));
                                            let clone1 = option.clone();
                                            let clone2 = option.clone();
                                            let clone3 = option.clone();
                                            let clone4 = option.clone();
                                            let clone5 = option.clone();
                                            let clone6 = option.clone();
                                            view! {
                                                <leptonic-select-option
                                                    class:preselected=move || {
                                                        memoized_preselected
                                                            .with(|preselected| preselected.as_ref() == Some(&option))
                                                    }

                                                    class:selected=move || is_selected(&clone4)
                                                    class:disabled=move || is_disabled(&clone5)
                                                    class:hidden=move || is_disabled(&clone6) && hide_disabled
                                                    on:mouseenter=move |_e| {
                                                        set_preselected.set(Some(clone3.clone()));
                                                    }

                                                    on:click=move |_e| {
                                                        if !is_disabled_untracked(&clone2) {
                                                            select.consume(clone2.clone());
                                                        }
                                                    }
                                                >

                                                    {render_option.render(clone1)}
                                                </leptonic-select-option>
                                            }
                                                .into_view()
                                        },
                                    )
                            }
                        }}

                    </Show>
                </leptonic-select-options>
            </leptonic-select>
        </div>
    }
}

#[component]
#[allow(clippy::too_many_lines)]
pub fn OptionalSelect<O>(
    #[prop(into)] options: MaybeSignal<Vec<O>>,
    #[prop(into)] selected: Signal<Option<O>>,
    #[prop(into)] set_selected: Out<Option<O>>,
    #[prop(into)] search_text_provider: Consumer<O, String>,
    #[prop(into)] render_option: ViewCallback<O>,
    #[prop(into)] allow_deselect: MaybeSignal<bool>,
    #[prop(into, optional)] search_filter_provider: Option<Consumer<(String, Vec<O>), Vec<O>>>,
    #[prop(into, optional)] autofocus_search: Option<Signal<bool>>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    #[prop(into, optional)] no_options: Option<ViewCallback<String>>,
    #[prop(into, optional)] new_option_when_no_options: Option<Consumer<String, O>>,
    #[prop(into, optional)] hide_disabled: bool,
) -> impl IntoView
where
    O: SelectOption + 'static,
{
    let id: uuid::Uuid = uuid::Uuid::new_v4();
    let id_string = format!("s-{id}");
    let id_selector_string = format!("#{id_string}");

    let (focused, set_focused) = create_signal(false);
    let (show_options, set_show_options) = create_signal(false);

    let autofocus_search =
        autofocus_search.unwrap_or(expect_context::<Leptonic>().is_desktop_device);

    let search_should_be_focused =
        Signal::derive(move || show_options.get() && autofocus_search.get());
    let (search_is_focused, set_search_is_focused) = create_signal(false);

    let stored_options = store_value(options);
    let (preselected, set_preselected) = create_signal(Option::<O>::None);
    let memoized_preselected = create_memo(move |_| preselected.get());

    let (search, set_search) = create_signal(String::new());

    let search_filter_provider =
        search_filter_provider.unwrap_or(Consumer::new(move |(s, o): (String, Vec<O>)| {
            let lowercased_search = s.to_lowercase();
            o.into_iter()
                .filter(|it| {
                    search_text_provider
                        .consume(it.clone())
                        .to_lowercase()
                        .contains(lowercased_search.as_str())
                })
                .collect::<Vec<O>>()
        }));

    let filtered_options = create_memo(move |_| {
        search_filter_provider.consume((search.get(), stored_options.get_value().get()))
    });

    let has_options = create_memo(move |_| !filtered_options.with(Vec::is_empty));

    let select = Consumer::new(move |option: O| {
        set_selected.set(Some(option));
        set_show_options.set(false);
    });

    let deselect = move || {
        set_selected.set(None);
    };

    let is_selected = move |option: &O| selected.with(|selected| selected.as_ref() == Some(option));

    let is_disabled = move |option: &O| selected.with(|selected| selected.as_ref() == Some(option));

    let is_disabled_untracked =
        move |option: &O| selected.with_untracked(|selected| selected.as_ref() == Some(option));

    // We need to check for global mouse events.
    // If our option list is shown and such an event occurs and does not target our option list, the options list should be closed.
    create_click_away_listener(id_selector_string, show_options, set_show_options.into());

    create_key_down_listener(move |e| {
        match (show_options.get_untracked(), focused.get_untracked()) {
            (true, _) => match e.key().as_str() {
                "Escape" => set_show_options.set(false),
                "Backspace" => {
                    if !search_is_focused.get_untracked() {
                        set_show_options.set(false);
                    }
                }
                "ArrowUp" => {
                    e.prevent_default();
                    e.stop_propagation();
                    // TODO: Use options_available_for_preselect.with_untracked when https://github.com/leptos-rs/leptos/issues/1212 is resolved and released.
                    select_previous(
                        &filtered_options.get_untracked(),
                        preselected,
                        set_preselected,
                    );
                }
                "ArrowDown" => {
                    e.prevent_default();
                    e.stop_propagation();
                    // TODO: Use options_available_for_preselect.with_untracked when https://github.com/leptos-rs/leptos/issues/1212 is resolved and released.
                    select_next(
                        &filtered_options.get_untracked(),
                        preselected,
                        set_preselected,
                    );
                }
                "Enter" => {
                    e.prevent_default();
                    e.stop_propagation();
                    if let Some(preselected) = preselected.get_untracked() {
                        if !is_disabled_untracked(&preselected) {
                            select.consume(preselected);
                        }
                    }
                }
                _ => {}
            },
            (false, true) => match e.key().as_str() {
                "Enter" | "ArrowDown" => {
                    e.prevent_default();
                    e.stop_propagation();
                    set_show_options.set(true);
                }
                _ => {}
            },
            _ => {}
        }
    });

    let toggle_show = move || set_show_options.update(|val| *val = !*val);

    let wrapper: NodeRef<html::Div> = create_node_ref();

    // Put focus back on our wrapper when the dropdown was closed while the search input had focus.
    create_effect(move |_| {
        if !show_options.get() && search_is_focused.get_untracked() {
            // TODO: Use with() when available.
            if let Some(wrapper) = wrapper.get() {
                wrapper.focus().expect("wrapper to be focusable");
            } else {
                tracing::warn!("missing node_ref");
            }
        }
    });

    view! {
        <div
            node_ref=wrapper
            class="leptonic-select-wrapper"
            tabindex=0
            on:blur=move |_| set_focused.set(false)
            on:focus=move |_| set_focused.set(true)
        >
            <leptonic-select
                id=id_string
                data-variant="optional-select"
                aria-haspopup="listbox"
                class=class
                style=style
            >
                <leptonic-select-selected on:click=move |_| toggle_show()>
                    {move || match selected.get() {
                        Some(selected) => {
                            view! {
                                <leptonic-select-option>
                                    {render_option.render(selected)}
                                </leptonic-select-option>
                            }
                                .into_view()
                        }
                        None => ().into_view(),
                    }}
                    {match allow_deselect.get() {
                        false => ().into_view(),
                        true => {
                            view! {
                                <leptonic-select-deselect-trigger on:click=move |e| {
                                    e.prevent_default();
                                    e.stop_propagation();
                                    deselect();
                                }>
                                    <Icon icon=icondata::BsXCircleFill/>
                                </leptonic-select-deselect-trigger>
                            }
                                .into_view()
                        }
                    }}
                    <leptonic-select-show-trigger>
                        {move || match show_options.get() {
                            true => view! { <Icon icon=icondata::BsCaretUpFill/> },
                            false => view! { <Icon icon=icondata::BsCaretDownFill/> },
                        }}

                    </leptonic-select-show-trigger>
                </leptonic-select-selected>

                <leptonic-select-options class:shown=move || {
                    show_options.get()
                }>
                    <TextInput
                        get=search
                        set=set_search
                        should_be_focused=search_should_be_focused
                        on_focus_change=move |focused| {
                            if show_options.get_untracked() {
                                set_search_is_focused.set(focused);
                            }
                        }

                        class="search"
                    />

                    <Show when=move || show_options.get() fallback=move || ()>
                        // TOD: Use <For> once leptos 0.4 is out. Use full option for hash.
                        {filtered_options
                            .get()
                            .into_iter()
                            .map(|option| {
                                let clone1 = option.clone();
                                let clone2 = option.clone();
                                let clone3 = option.clone();
                                let clone4 = option.clone();
                                let clone5 = option.clone();
                                view! {
                                    <leptonic-select-option
                                        class:preselected=move || {
                                            memoized_preselected
                                                .with(|preselected| preselected.as_ref() == Some(&option))
                                        }

                                        class:selected=move || is_selected(&clone4)
                                        class:disabled=move || is_disabled(&clone5)
                                        on:mouseenter=move |_e| {
                                            set_preselected.set(Some(clone3.clone()));
                                        }

                                        on:click=move |_e| {
                                            if !is_disabled_untracked(&clone2) {
                                                select.consume(clone2.clone());
                                            }
                                        }
                                    >

                                        {render_option.render(clone1)}
                                    </leptonic-select-option>
                                }
                            })
                            .collect_view()}

                        {move || match has_options.get() {
                            true => ().into_view(),
                            false => {
                                new_option_when_no_options
                                    .map_or_else(
                                        move || {
                                            view! {
                                                <leptonic-select-no-search-results>
                                                    {no_options
                                                        .map_or_else(
                                                            || { "No options...".into_view() },
                                                            |x| x.render(search.get()).into_view(),
                                                        )}

                                                </leptonic-select-no-search-results>
                                            }
                                                .into_view()
                                        },
                                        move |x| {
                                            let option = x.consume(search.get());
                                            set_preselected.set(Some(option.clone()));
                                            let clone1 = option.clone();
                                            let clone2 = option.clone();
                                            let clone3 = option.clone();
                                            let clone4 = option.clone();
                                            let clone5 = option.clone();
                                            let clone6 = option.clone();
                                            view! {
                                                <leptonic-select-option
                                                    class:preselected=move || {
                                                        memoized_preselected
                                                            .with(|preselected| preselected.as_ref() == Some(&option))
                                                    }

                                                    class:selected=move || is_selected(&clone4)
                                                    class:disabled=move || is_disabled(&clone5)
                                                    class:hidden=move || is_disabled(&clone6) && hide_disabled
                                                    on:mouseenter=move |_e| {
                                                        set_preselected.set(Some(clone3.clone()));
                                                    }

                                                    on:click=move |_e| {
                                                        if !is_disabled_untracked(&clone2) {
                                                            select.consume(clone2.clone());
                                                        }
                                                    }
                                                >

                                                    {render_option.render(clone1)}
                                                </leptonic-select-option>
                                            }
                                                .into_view()
                                        },
                                    )
                            }
                        }}

                    </Show>
                </leptonic-select-options>
            </leptonic-select>
        </div>
    }
}
#[component]
#[allow(clippy::too_many_lines)]
pub fn Multiselect<O>(
    #[prop(optional, default=u64::MAX)] max: u64,
    #[prop(into)] options: MaybeSignal<Vec<O>>,
    #[prop(into)] selected: Signal<Vec<O>>,
    #[prop(into)] set_selected: Out<Vec<O>>,
    #[prop(into)] search_text_provider: Consumer<O, String>,
    #[prop(into)] render_option: ViewCallback<O>,
    #[prop(into, optional)] search_filter_provider: Option<Consumer<(String, Vec<O>), Vec<O>>>,
    #[prop(into, optional)] autofocus_search: Option<Signal<bool>>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    #[prop(into, optional)] no_options: Option<ViewCallback<String>>,
    #[prop(into, optional)] new_option_when_no_options: Option<Consumer<String, O>>,
    #[prop(into, optional)] hide_disabled: bool,
) -> impl IntoView
where
    O: SelectOption + PartialOrd + Ord + 'static + Hash,
{
    let id: uuid::Uuid = uuid::Uuid::new_v4();
    let id_string = format!("s-{id}");
    let id_selector_string = format!("#{id_string}");

    let (focused, set_focused) = create_signal(false);
    let (show_options, set_show_options) = create_signal(false);

    let autofocus_search =
        autofocus_search.unwrap_or(expect_context::<Leptonic>().is_desktop_device);

    let search_should_be_focused =
        Signal::derive(move || show_options.get() && autofocus_search.get());
    let (search_is_focused, set_search_is_focused) = create_signal(false);

    let stored_options = store_value(options);
    let (preselected, set_preselected) = create_signal(Option::<O>::None);
    let memoized_preselected = create_memo(move |_| preselected.get());

    let (search, set_search) = create_signal(String::new());

    let search_filter_provider =
        search_filter_provider.unwrap_or(Consumer::new(move |(s, o): (String, Vec<O>)| {
            let lowercased_search = s.to_lowercase();
            o.into_iter()
                .filter(|it| {
                    search_text_provider
                        .consume(it.clone())
                        .to_lowercase()
                        .contains(lowercased_search.as_str())
                })
                .collect::<Vec<O>>()
        }));

    let filtered_options = create_memo(move |_| {
        search_filter_provider.consume((search.get(), stored_options.get_value().get()))
    });

    let has_options = create_memo(move |_| !filtered_options.with(Vec::is_empty));

    let select = Consumer::new(move |option: O| {
        let mut vec = selected.get_untracked();
        if !vec.contains(&option) {
            vec.push(option); // TODO
        }
        vec.sort();
        tracing::info!(?vec, "selected");
        set_selected.set(vec);
        set_show_options.set(false); // TODO: Make this optional.
    });

    let deselect = Consumer::new(move |option: O| {
        let mut vec = selected.get_untracked();
        if let Some(pos) = vec.iter().position(|it| it == &option) {
            vec.remove(pos);
        }
        tracing::info!(?vec, "deselected");
        set_selected.set(vec);
        // set_show_options.set(false); // TODO: Make this optional.
    });

    let is_selected = move |option: &O| selected.with(|selected| selected.contains(option));

    let is_disabled = move |option: &O| {
        selected.with(|selected| selected.contains(option) || selected.len() as u64 == max)
    };

    let is_disabled_untracked = move |option: &O| {
        selected
            .with_untracked(|selected| selected.contains(option) || selected.len() as u64 == max)
    };

    // We need to check for global mouse events.
    // If our option list is shown and such an event occurs and does not target our option list, the options list should be closed.
    create_click_away_listener(id_selector_string, show_options, set_show_options.into());

    create_key_down_listener(move |e| {
        match (show_options.get_untracked(), focused.get_untracked()) {
            (true, _) => match e.key().as_str() {
                "Escape" => set_show_options.set(false),
                "Backspace" => {
                    if !search_is_focused.get_untracked() {
                        set_show_options.set(false);
                    }
                }
                "ArrowUp" => {
                    e.prevent_default();
                    e.stop_propagation();
                    // TODO: Use options_available_for_preselect.with_untracked when https://github.com/leptos-rs/leptos/issues/1212 is resolved and released.
                    select_previous(
                        &filtered_options.get_untracked(),
                        preselected,
                        set_preselected,
                    );
                }
                "ArrowDown" => {
                    e.prevent_default();
                    e.stop_propagation();
                    // TODO: Use options_available_for_preselect.with_untracked when https://github.com/leptos-rs/leptos/issues/1212 is resolved and released.
                    select_next(
                        &filtered_options.get_untracked(),
                        preselected,
                        set_preselected,
                    );
                }
                "Enter" => {
                    e.prevent_default();
                    e.stop_propagation();
                    if let Some(preselected) = preselected.get_untracked() {
                        if !is_disabled_untracked(&preselected) {
                            select.consume(preselected);
                        }
                    }
                }
                _ => {}
            },
            (false, true) => match e.key().as_str() {
                "Enter" | "ArrowDown" => {
                    e.prevent_default();
                    e.stop_propagation();
                    set_show_options.set(true);
                }
                _ => {}
            },
            _ => {}
        }
    });

    let toggle_show = move || set_show_options.update(|val| *val = !*val);

    let wrapper: NodeRef<html::Div> = create_node_ref();

    // Put focus back on our wrapper when the dropdown was closed while the search input had focus.
    create_effect(move |_| {
        if !show_options.get() && search_is_focused.get_untracked() {
            // TODO: Use with() when available.
            if let Some(wrapper) = wrapper.get() {
                wrapper.focus().expect("wrapper to be focusable");
            } else {
                tracing::warn!("missing node_ref");
            }
        }
    });

    view! {
        <div
            node_ref=wrapper
            class="leptonic-select-wrapper"
            tabindex=0
            on:blur=move |_| set_focused.set(false)
            on:focus=move |_| set_focused.set(true)
        >
            <leptonic-select
                id=id_string
                data-variant="multiselect"
                aria-haspopup="listbox"
                class=class
                style=style
            >
                <leptonic-select-selected on:click=move |_| toggle_show()>
                    // TOD: Use <For> once leptos 0.4 is out. Use full option for hash.
                    {move || {
                        selected
                            .get()
                            .into_iter()
                            .map(|selected| {
                                let clone = selected.clone();
                                view! {
                                    <leptonic-select-option>
                                        <Chip
                                            color=ChipColor::Secondary
                                            on:click=move |e| {
                                                e.stop_propagation();
                                            }

                                            dismissible=move |e: MouseEvent| {
                                                e.stop_propagation();
                                                deselect.consume(clone.clone());
                                            }
                                        >

                                            {render_option.render(selected)}
                                        </Chip>
                                    </leptonic-select-option>
                                }
                            })
                            .collect_view()
                    }}
                    <leptonic-select-show-trigger>
                        {move || match show_options.get() {
                            true => view! { <Icon icon=icondata::BsCaretUpFill/> },
                            false => view! { <Icon icon=icondata::BsCaretDownFill/> },
                        }}

                    </leptonic-select-show-trigger>
                </leptonic-select-selected>

                <leptonic-select-options class:shown=show_options>

                    <TextInput
                        get=search
                        set=set_search
                        should_be_focused=search_should_be_focused
                        on_focus_change=move |focused| {
                            if show_options.get_untracked() {
                                set_search_is_focused.set(focused);
                            }
                        }

                        class="search"
                    />

                    <Show when=move || show_options.get() fallback=move || ()>
                        <For
                            each=filtered_options
                            key=|option| option.clone()
                            let:option
                        >

                            {
                                let clone1 = option.clone();
                                let clone2 = option.clone();
                                let clone3 = option.clone();
                                let clone4 = option.clone();
                                let clone5 = option.clone();
                                let clone6 = option.clone();
                                view! {
                                    <leptonic-select-option
                                        class:preselected=move || {
                                            memoized_preselected
                                                .with(|preselected| preselected.as_ref() == Some(&option))
                                        }

                                        class:selected=move || is_selected(&clone4)
                                        class:disabled=move || is_disabled(&clone5)
                                        class:hidden=move || is_disabled(&clone6) && hide_disabled
                                        on:mouseenter=move |_e| {
                                            set_preselected.set(Some(clone3.clone()));
                                        }

                                        on:click=move |_e| {
                                            if !is_disabled_untracked(&clone2) {
                                                select.consume(clone2.clone());
                                            }
                                        }
                                    >

                                        {render_option.render(clone1)}
                                    </leptonic-select-option>
                                }
                            }

                        </For>

                        {move || match has_options.get() {
                            true => ().into_view(),
                            false => {
                                new_option_when_no_options
                                    .map_or_else(
                                        move || {
                                            view! {
                                                <leptonic-select-no-search-results>
                                                    {no_options
                                                        .map_or_else(
                                                            || { "No options...".into_view() },
                                                            |x| x.render(search.get()).into_view(),
                                                        )}

                                                </leptonic-select-no-search-results>
                                            }
                                                .into_view()
                                        },
                                        move |x| {
                                            let option = x.consume(search.get());
                                            set_preselected.set(Some(option.clone()));
                                            let clone1 = option.clone();
                                            let clone2 = option.clone();
                                            let clone3 = option.clone();
                                            let clone4 = option.clone();
                                            let clone5 = option.clone();
                                            let clone6 = option.clone();
                                            view! {
                                                <leptonic-select-option
                                                    class:preselected=move || {
                                                        memoized_preselected
                                                            .with(|preselected| preselected.as_ref() == Some(&option))
                                                    }

                                                    class:selected=move || is_selected(&clone4)
                                                    class:disabled=move || is_disabled(&clone5)
                                                    class:hidden=move || is_disabled(&clone6) && hide_disabled
                                                    on:mouseenter=move |_e| {
                                                        set_preselected.set(Some(clone3.clone()));
                                                    }

                                                    on:click=move |_e| {
                                                        if !is_disabled_untracked(&clone2) {
                                                            select.consume(clone2.clone());
                                                        }
                                                    }
                                                >

                                                    {render_option.render(clone1)}
                                                </leptonic-select-option>
                                            }
                                                .into_view()
                                        },
                                    )
                            }
                        }}

                    </Show>
                </leptonic-select-options>
            </leptonic-select>
        </div>
    }
}

fn create_click_away_listener(
    id_selector_string: String,
    when: ReadSignal<bool>,
    on_click_outside: Out<bool>,
) {
    let g_mouse_event =
        use_context::<GlobalClickEvent>().expect("Must be a child of the Root component.");

    create_effect(move |_old| {
        use wasm_bindgen::JsCast;
        let last_mouse_event = g_mouse_event.read_signal.get();

        if when.get_untracked() {
            if let Some(e) = last_mouse_event {
                if let Some(target) = e.target() {
                    if let Some(target_elem) = target.dyn_ref::<HtmlElement>() {
                        match target_elem.closest(id_selector_string.as_ref()) {
                            Ok(closest) => {
                                if let Some(_found) = closest {
                                    // User clicked on the options list. Ignoring this global mouse event.
                                } else {
                                    // User clicked outside.
                                    on_click_outside.set(false);
                                }
                            }
                            Err(err) => {
                                tracing::error!("Error processing latest mouse event: {err:?}");
                            }
                        }
                    }
                }
            }
        }
    });
}

fn create_key_down_listener<T: Fn(KeyboardEvent) + 'static>(then: T) {
    let g_keyboard_event =
        use_context::<GlobalKeyboardEvent>().expect("Must be a child of the Root component.");

    create_effect(move |_old| {
        let g_keyboard_event = g_keyboard_event.read_signal.get();
        if let Some(e) = g_keyboard_event {
            then(e);
        }
    });
}
