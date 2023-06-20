use std::{
    fmt::{Debug, Display},
    hash::Hash,
};

use leptos::*;

use leptos_icons::BsIcon;
use web_sys::{HtmlElement, KeyboardEvent};

use crate::prelude::*;

pub trait SelectOption: Debug + Clone + PartialEq + Eq + Hash {
    fn matches_lowercase(&self, search: &str) -> bool;
}

impl<T: Debug + Display + Clone + PartialEq + Eq + Hash> SelectOption for T {
    fn matches_lowercase(&self, search: &str) -> bool {
        self.to_string().to_lowercase().contains(search)
    }
}

// TODO: select_previous and select_next could be made more efficient.
// If we would know that the initial vec from which the current preselect'ed option was taken didn't change
// and if we also keep track of the index of this option in the vec, we can just read the previous / next option
// be decrementing or incrementing the old index!

// TODO: Prop: close_options_menu_on_selection: bool
// TODO: Prop: selection_changed: Callback<Selection<T>>
// TODO: Sort multiselect options
// TODO: multiselect deselect performance

fn select_previous<O: SelectOption + 'static>(
    available: &Vec<O>,
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
    tracing::info!(?previous, "previous");
    set_preselected.set(previous);
}

fn select_next<O: SelectOption + 'static>(
    available: &Vec<O>,
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
    tracing::info!(?next, "next");
    set_preselected.set(next);
}

fn handle_key<O: SelectOption + 'static>(
    e: KeyboardEvent,
    show_options: ReadSignal<bool>,
    set_show_options: WriteSignal<bool>,
    focus: ReadSignal<bool>,
    options_available_for_preselect: Signal<Vec<O>>,
    preselected: ReadSignal<Option<O>>,
    set_preselected: WriteSignal<Option<O>>,
    select: Callback<O>,
) {
    match (show_options.get_untracked(), focus.get_untracked()) {
        (true, _) => match e.key().as_str() {
            "Escape" | "Backspace" => set_show_options.set(false),
            "ArrowUp" => {
                e.prevent_default();
                e.stop_propagation();
                // TODO: Use options_available_for_preselect.with_untracked when https://github.com/leptos-rs/leptos/issues/1212 is resolved and released.
                select_previous(
                    &options_available_for_preselect.get_untracked(),
                    preselected,
                    set_preselected,
                );
            }
            "ArrowDown" => {
                e.prevent_default();
                e.stop_propagation();
                // TODO: Use options_available_for_preselect.with_untracked when https://github.com/leptos-rs/leptos/issues/1212 is resolved and released.
                select_next(
                    &options_available_for_preselect.get_untracked(),
                    preselected,
                    set_preselected,
                );
            }
            "Enter" => {
                e.prevent_default();
                e.stop_propagation();
                if let Some(preselected) = preselected.get_untracked() {
                    select.call(preselected);
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
}

#[component]
pub fn Select<O>(
    cx: Scope,
    #[prop(into)] options: MaybeSignal<Vec<O>>,
    #[prop(into)] selected: Signal<O>,
    #[prop(into)] set_selected: Callback<O>,
    #[prop(into)] render_option: Callback<(Scope, O), View>,
    #[prop(into, optional)] style: Option<AttributeValue>,
) -> impl IntoView
where
    O: SelectOption + 'static,
{
    let id: uuid::Uuid = uuid::Uuid::new_v4();
    let id_string = format!("s-{id}");
    let id_selector_string = format!("#{id_string}");

    let (focus, set_focus) = create_signal(cx, false);

    let (show_options, set_show_options) = create_signal(cx, false);

    let stored_options = store_value(cx, options);
    let (preselected, set_preselected) = create_signal(cx, Option::<O>::None);

    let (search, set_search) = create_signal(cx, "".to_owned());

    let filtered_options = create_memo(cx, move |_| {
        let search = search.get().to_lowercase();
        stored_options
            .get_value()
            .get()
            .into_iter()
            .filter(|it| it.matches_lowercase(search.as_str()))
            .collect::<Vec<O>>()
    });

    let has_options = create_memo(cx, move |_| {
        !filtered_options.with(|options| options.is_empty())
    });

    let select = Callback::new(cx, move |option: O| {
        set_selected.call(option);
        set_show_options.set(false);
    });

    // We need to check for global mouse events.
    // If our option list is shown and such an event occurs and does not target our option list, the options list should be closed.
    create_click_away_listener(
        cx,
        id_selector_string.clone(),
        move || show_options.get_untracked(),
        move || set_show_options.set(false),
    );

    create_key_down_listener(cx, move |e| {
        handle_key(
            e,
            show_options,
            set_show_options,
            focus,
            filtered_options.into(),
            preselected,
            set_preselected,
            select,
        );
    });

    let toggle_show = move || set_show_options.update(|val| *val = !*val);

    view! { cx,
        <leptonic-select
            on:blur=move |_| set_focus.set(false)
            on:focus=move |_| set_focus.set(true)
            tabindex=0
            id=id_string
            variant="select"
            aria-haspopup="listbox"
            style=style
        >
            <leptonic-select-selected on:click=move |_| toggle_show()>
                { move || render_option.call((cx, selected.get())) }

                <leptonic-select-show-trigger>
                    {move || match show_options.get() {
                        true => view! {cx, <Icon icon=BsIcon::BsCaretUpFill/>},
                        false => view! {cx, <Icon icon=BsIcon::BsCaretDownFill/>}
                    }}
                </leptonic-select-show-trigger>
            </leptonic-select-selected>

            <leptonic-select-options class:shown=move || show_options.get()>
                <Input get=search set=move |s| set_search.set(s)/>

                { move || {
                    filtered_options.get().into_iter().map(|option| move || {
                        let clone = option.clone();
                        let clone2 = option.clone();
                        let is_preselected = preselected.with(|preselected| preselected.as_ref() == Some(&option));
                        let is_selected = selected.with(|selected| selected == &option);
                        view! { cx,
                            <div
                                class="option"
                                class:preselected=is_preselected
                                class:selected=is_selected
                                on:click=move |_| select.call(clone.clone())
                            >
                                { render_option.call((cx, clone2)) }
                            </div>
                        }
                    }).collect_view(cx)
                } }

                { move || match has_options.get() {
                    true => ().into_view(cx),
                    false => view! {cx,
                        <div class="option">
                            "No options..."
                        </div>
                    }.into_view(cx),
                } }
            </leptonic-select-options>
        </leptonic-select>
    }
}

#[component]
pub fn OptionalSelect<O>(
    cx: Scope,
    #[prop(into)] options: MaybeSignal<Vec<O>>,
    #[prop(into)] selected: Signal<Option<O>>,
    #[prop(into)] set_selected: Callback<Option<O>>,
    #[prop(into)] render_option: Callback<(Scope, O), View>,
    #[prop(into)] allow_deselect: MaybeSignal<bool>,
    #[prop(into, optional)] style: Option<AttributeValue>,
) -> impl IntoView
where
    O: SelectOption + 'static,
{
    let id: uuid::Uuid = uuid::Uuid::new_v4();
    let id_string = format!("s-{id}");
    let id_selector_string = format!("#{id_string}");

    let (focus, set_focus) = create_signal(cx, false);

    let (show_options, set_show_options) = create_signal(cx, false);

    let stored_options = store_value(cx, options);
    let (preselected, set_preselected) = create_signal(cx, Option::<O>::None);

    let (search, set_search) = create_signal(cx, "".to_owned());

    let filtered_options = create_memo(cx, move |_| {
        let search = search.get().to_lowercase();
        stored_options
            .get_value()
            .get()
            .into_iter()
            .filter(|it| it.matches_lowercase(search.as_str()))
            .collect::<Vec<_>>()
    });

    let has_options = create_memo(cx, move |_| {
        !filtered_options.with(|options| options.is_empty())
    });

    let select = Callback::new(cx, move |option: O| {
        set_selected.call(Some(option));
        set_show_options.set(false);
    });

    let deselect = move || {
        set_selected.call(None);
    };

    // We need to check for global mouse events.
    // If our option list is shown and such an event occurs and does not target our option list, the options list should be closed.
    create_click_away_listener(
        cx,
        id_selector_string.clone(),
        move || show_options.get_untracked(),
        move || set_show_options.set(false),
    );

    create_key_down_listener(cx, move |e| {
        handle_key(
            e,
            show_options,
            set_show_options,
            focus,
            filtered_options.into(),
            preselected,
            set_preselected,
            select,
        );
    });

    let toggle_show = move || set_show_options.update(|val| *val = !*val);

    view! { cx,
        <leptonic-select
            on:blur=move |_| set_focus.set(false)
            on:focus=move |_| set_focus.set(true)
            tabindex=0
            id=id_string
            variant="optional-select"
            aria-haspopup="listbox"
            style=style
        >
            <leptonic-select-selected on:click=move |_| toggle_show()>
                { move || match selected.get().clone() {
                    None => ().into_view(cx),
                    Some(selected) => view! {cx,
                        <leptonic-select-option>
                            { render_option.call((cx, selected)) }
                        </leptonic-select-option>
                    }.into_view(cx),
                }}

                { match allow_deselect.get() {
                    false => ().into_view(cx),
                    true => view! {cx,
                        <leptonic-select-deselect-trigger on:click=move |e| {
                            e.prevent_default();
                            e.stop_propagation();
                            deselect();
                        }>
                            <Icon icon=BsIcon::BsXCircleFill/>
                        </leptonic-select-deselect-trigger>
                    }.into_view(cx),
                }}

                <leptonic-select-show-trigger>
                    {move || match show_options.get() {
                        true => view! {cx, <Icon icon=BsIcon::BsCaretUpFill/>},
                        false => view! {cx, <Icon icon=BsIcon::BsCaretDownFill/>}
                    }}
                </leptonic-select-show-trigger>
            </leptonic-select-selected>

            <leptonic-select-options class:shown=move || show_options.get()>
                <Input get=search set=move |s| set_search.set(s)/>

                { move || {
                    filtered_options.get().into_iter().map(|option| move || {
                        let clone = option.clone();
                        let clone2 = option.clone();
                        let is_preselected = preselected.with(|preselected| preselected.as_ref() == Some(&option));
                        let is_selected = selected.with(|selected| selected.as_ref() == Some(&option));
                        view! { cx,
                            <div
                                class="option"
                                class:preselected=is_preselected
                                class:selected=is_selected
                                on:click=move |_| select.call(clone.clone())
                            >
                                { render_option.call((cx, clone2)) }
                            </div>
                        }
                    }).collect_view(cx)
                } }

                { move || match has_options.get() {
                    true => ().into_view(cx),
                    false => view! {cx,
                        <div class="option">
                            "No options..."
                        </div>
                    }.into_view(cx),
                } }
            </leptonic-select-options>
        </leptonic-select>
    }
}

#[component]
pub fn Multiselect<O>(
    cx: Scope,
    #[prop(optional, default=u64::MAX)] max: u64,
    #[prop(into)] options: MaybeSignal<Vec<O>>,
    #[prop(into)] selected: Signal<Vec<O>>,
    #[prop(into)] set_selected: Callback<Vec<O>>,
    #[prop(into)] render_option: Callback<(Scope, O), View>,
    #[prop(into, optional)] style: Option<AttributeValue>,
) -> impl IntoView
where
    O: SelectOption + PartialOrd + Ord + 'static,
{
    let id: uuid::Uuid = uuid::Uuid::new_v4();
    let id_string = format!("s-{id}");
    let id_selector_string = format!("#{id_string}");

    let (focus, set_focus) = create_signal(cx, false);

    let (show_options, set_show_options) = create_signal(cx, false);

    let stored_options = store_value(cx, options);
    let (preselected, set_preselected) = create_signal(cx, Option::<O>::None);

    let (search, set_search) = create_signal(cx, "".to_owned());

    let filtered_options = create_memo(cx, move |_| {
        let search = search.get().to_lowercase();
        stored_options
            .get_value()
            .get()
            .into_iter()
            .filter(|it| it.matches_lowercase(search.as_str()))
            .collect::<Vec<_>>()
    });

    let has_options = create_memo(cx, move |_| {
        !filtered_options.with(|options| options.is_empty())
    });

    let select = Callback::new(cx, move |option: O| {
        let mut vec = selected.get_untracked();
        if !vec.contains(&option) {
            vec.push(option); // TODO
        }
        vec.sort();
        tracing::info!(?vec, "selected");
        set_selected.call(vec);
        set_show_options.set(false);
    });

    let deselect = Callback::new(cx, move |option: O| {
        let mut vec = selected.get_untracked();
        if let Some(pos) = vec.iter().position(|it| it == &option) {
            vec.remove(pos);
        }
        tracing::info!(?vec, "deselected");
        set_selected.call(vec);
        set_show_options.set(false);
    });

    // We need to check for global mouse events.
    // If our option list is shown and such an event occurs and does not target our option list, the options list should be closed.
    create_click_away_listener(
        cx,
        id_selector_string.clone(),
        move || show_options.get_untracked(),
        move || set_show_options.set(false),
    );

    create_key_down_listener(cx, move |e| {
        handle_key(
            e,
            show_options,
            set_show_options,
            focus,
            filtered_options.into(),
            preselected,
            set_preselected,
            select,
        );
    });

    let toggle_show = move || set_show_options.update(|val| *val = !*val);

    view! { cx,
        <leptonic-select
            on:blur=move |_| set_focus.set(false)
            on:focus=move |_| set_focus.set(true)
            tabindex=0
            id=id_string
            variant="multiselect"
            aria-haspopup="listbox"
            style=style
        >
            <leptonic-select-selected on:click=move |_| toggle_show()>
                { move || selected.get().into_iter().map(|selected| {
                    let clone = selected.clone();
                    view! { cx,
                        <leptonic-select-option>
                            <Chip
                                color=ChipColor::Secondary
                                on:click=move |e| { e.stop_propagation(); }
                                dismissible=Callback::new(cx, move |_| deselect.call(clone.clone()))>
                                { render_option.call((cx, selected)) }
                            </Chip>
                        </leptonic-select-option>
                    }}).collect_view(cx)
                }

                <leptonic-select-show-trigger>
                    {move || match show_options.get() {
                        true => view! {cx, <Icon icon=BsIcon::BsCaretUpFill/>},
                        false => view! {cx, <Icon icon=BsIcon::BsCaretDownFill/>}
                    }}
                </leptonic-select-show-trigger>
            </leptonic-select-selected>

            <leptonic-select-options class:shown=move || show_options.get()>
                <Input get=search set=move |s| set_search.set(s)/>

                { move || {
                    filtered_options.get().into_iter().map(|option| move || {
                        let clone = option.clone();
                        let clone2 = option.clone();
                        let is_preselected = preselected.with(|preselected| preselected.as_ref() == Some(&option));
                        let is_selected = selected.with(|selected| selected.contains(&option));
                        view! { cx,
                            <div
                                class="option"
                                class:preselected=is_preselected
                                class:selected=is_selected
                                on:click=move |_| select.call(clone.clone())
                            >
                                { render_option.call((cx, clone2)) }
                            </div>
                        }
                    }).collect_view(cx)
                } }

                { move || match has_options.get() {
                    true => ().into_view(cx),
                    false => view! {cx,
                        <div class="option">
                            "No options..."
                        </div>
                    }.into_view(cx),
                } }
            </leptonic-select-options>
        </leptonic-select>
    }
}

fn create_click_away_listener<W: Fn() -> bool + 'static, O: Fn() + 'static>(
    cx: Scope,
    id_selector_string: String,
    when: W,
    on_click_outside: O,
) {
    let g_mouse_event =
        use_context::<GlobalMouseEvent>(cx).expect("Must be a child of the Root component.");

    create_effect(cx, move |_old| {
        use wasm_bindgen::JsCast;
        let last_mouse_event = g_mouse_event.read_signal.get();

        if when() {
            if let Some(e) = last_mouse_event {
                if let Some(target) = e.target() {
                    if let Some(target_elem) = target.dyn_ref::<HtmlElement>() {
                        match target_elem.closest(id_selector_string.as_ref()) {
                            Ok(closest) => {
                                if let Some(_found) = closest {
                                    // User clicked on the options list. Ignoring this global mouse event.
                                } else {
                                    // User clicked outside.
                                    on_click_outside();
                                }
                            }
                            Err(err) => {
                                error!("Error processing latest mouse event: {err:?}");
                            }
                        }
                    }
                }
            }
        }
    });
}

fn create_key_down_listener<T: Fn(KeyboardEvent) + 'static>(cx: Scope, then: T) {
    let g_keyboard_event =
        use_context::<GlobalKeyboardEvent>(cx).expect("Must be a child of the Root component.");

    create_effect(cx, move |_old| {
        let g_keyboard_event = g_keyboard_event.read_signal.get();
        if let Some(e) = g_keyboard_event {
            then(e);
        }
    });
}
