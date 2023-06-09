use std::{collections::hash_map::DefaultHasher, fmt::Debug, hash::Hash};

use leptos::*;

use web_sys::{HtmlElement, KeyboardEvent};

use crate::prelude::*;

pub trait SelectOption: Debug + Clone + PartialEq + Eq + Hash {}

impl<T: Debug + Clone + PartialEq + Eq + Hash> SelectOption for T {}

#[component]
pub fn Select<O, S, V>(
    cx: Scope,
    #[prop(into)] options: MaybeSignal<Vec<O>>,
    #[prop(into)] selected: Signal<O>,
    #[prop(into)] set_selected: Callback<O>,
    render_option: S,
    #[prop(optional)] margin: Option<Margin>,
) -> impl IntoView
where
    O: SelectOption + 'static,
    S: Fn(Scope, &O) -> V + Copy + 'static,
    V: IntoView + 'static,
{
    let style = margin.map(|it| format!("--margin: {it}"));

    let id: uuid::Uuid = uuid::Uuid::new_v4();
    let id_string = format!("s-{id}");
    let id_selector_string = format!("#{id_string}");

    let (show_options, set_show_options) = create_signal(cx, false);

    // We need to check for global mouse events.
    // If our option list is shown and such an event occurs and does not target our option list, the options list should be closed.
    create_click_away_listener(
        cx,
        id_selector_string.clone(),
        move || show_options.get_untracked(),
        move || set_show_options.set(false),
    );

    create_key_down_listener(
        cx,
        move |e| show_options.get_untracked() && e.key().as_str() == "Escape",
        move || set_show_options.set(false),
    );

    let toggle_show = move || set_show_options.update(|val| *val = !*val);

    let select = Callback::new(cx, move |option: O| {
        set_selected.call(option);
        set_show_options.set(false);
    });

    view! { cx,
        <leptonic-select id=id_string variant="select" aria-haspopup="listbox" style=style>
            <leptonic-select-selected on:click=move |_| toggle_show()>
                { move || render_option(cx, &selected.get()) }
            </leptonic-select-selected>
            <SelectOptions options=options show_options=show_options render_option=render_option select=select/>
        </leptonic-select>
    }
}

#[component]
pub fn OptionalSelect<O, S, V>(
    cx: Scope,
    #[prop(into)] options: MaybeSignal<Vec<O>>,
    #[prop(into)] selected: Signal<Option<O>>,
    #[prop(into)] set_selected: Callback<Option<O>>,
    render_option: S,
    #[prop(into)] allow_deselect: MaybeSignal<bool>,
    #[prop(optional)] margin: Option<Margin>,
) -> impl IntoView
where
    O: SelectOption + 'static,
    S: Fn(Scope, &O) -> V + Copy + 'static,
    V: IntoView + 'static,
{
    let style = margin.map(|it| format!("--margin: {it}"));

    let id: uuid::Uuid = uuid::Uuid::new_v4();
    let id_string = format!("s-{id}");
    let id_selector_string = format!("#{id_string}");

    let (show_options, set_show_options) = create_signal(cx, false);

    // We need to check for global mouse events.
    // If our option list is shown and such an event occurs and does not target our option list, the options list should be closed.
    create_click_away_listener(
        cx,
        id_selector_string.clone(),
        move || show_options.get_untracked(),
        move || set_show_options.set(false),
    );

    create_key_down_listener(
        cx,
        move |e| show_options.get_untracked() && e.key().as_str() == "Escape",
        move || set_show_options.set(false),
    );

    let toggle_show = move || set_show_options.update(|val| *val = !*val);

    let select = Callback::new(cx, move |option: O| {
        set_selected.call(Some(option));
        set_show_options.set(false);
    });

    let deselect = move || {
        set_selected.call(None);
    };

    view! { cx,
        <leptonic-select id=id_string variant="optional-select" aria-haspopup="listbox" style=style>
            <leptonic-select-selected on:click=move |_| toggle_show()>
                { move || match selected.get().clone() {
                    None => ().into_view(cx),
                    Some(selected) => view! {cx,
                        <leptonic-select-option>
                            {render_option(cx, &selected)}
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
                            "x"
                        </leptonic-select-deselect-trigger>
                    }.into_view(cx),
                }}
            </leptonic-select-selected>
            <SelectOptions options=options show_options=show_options render_option=render_option select=select/>
        </leptonic-select>
    }
}

#[component]
pub fn Multiselect<O, S, V>(
    cx: Scope,
    #[prop(optional, default=u64::MAX)] max: u64,
    #[prop(into)] options: MaybeSignal<Vec<O>>,
    #[prop(into)] selected: Signal<Vec<O>>,
    #[prop(into)] set_selected: Callback<Vec<O>>,
    render_option: S,
    #[prop(optional)] margin: Option<Margin>,
) -> impl IntoView
where
    O: SelectOption + 'static,
    S: Fn(Scope, &O) -> V + Copy + 'static,
    V: IntoView + 'static,
{
    let style = margin.map(|it| format!("--margin: {it}"));

    let id: uuid::Uuid = uuid::Uuid::new_v4();
    let id_string = format!("s-{id}");
    let id_selector_string = format!("#{id_string}");

    let (show_options, set_show_options) = create_signal(cx, false);

    // We need to check for global mouse events.
    // If our option list is shown and such an event occurs and does not target our option list, the options list should be closed.
    create_click_away_listener(
        cx,
        id_selector_string.clone(),
        move || show_options.get_untracked(),
        move || set_show_options.set(false),
    );

    create_key_down_listener(
        cx,
        move |e| show_options.get_untracked() && e.key().as_str() == "Escape",
        move || set_show_options.set(false),
    );

    let toggle_show = move || set_show_options.update(|val| *val = !*val);

    let select = Callback::new(cx, move |option: O| {
        let mut vec = selected.get_untracked();
        if !vec.contains(&option) {
            vec.push(option); // TODO
        }
        tracing::info!(?vec, "selected");
        set_selected.call(vec);
        set_show_options.set(false);
    });

    view! { cx,
        <leptonic-select id=id_string variant="multiselect" aria-haspopup="listbox" style=style>
            <leptonic-select-selected on:click=move |_| toggle_show()>
                { move || selected.get().into_iter().map(|selected| view! { cx,
                    <leptonic-select-option>
                        { render_option(cx, &selected) }
                    </leptonic-select-option>
                }).collect_view(cx) }
            </leptonic-select-selected>

            <SelectOptions options=options show_options=show_options render_option=render_option select=select/>
        </leptonic-select>
    }
}

#[component]
pub fn SelectOptions<O, S, V>(
    cx: Scope,
    #[prop(into)] options: MaybeSignal<Vec<O>>,
    #[prop(into)] show_options: Signal<bool>,
    render_option: S,
    select: Callback<O>,
) -> impl IntoView
where
    O: SelectOption + 'static,
    S: Fn(Scope, &O) -> V + Copy + 'static,
    V: IntoView + 'static,
{
    view! {cx,
        <leptonic-select-options class:shown=move || show_options.get()>
            <For
                each=move || options.get()
                key=|option| option.hash(&mut DefaultHasher::new())
                view=move |cx, option| {
                    let clone = option.clone();
                    view! { cx,
                        <div class="option" on:click=move |_| select.call(clone.clone())>
                            { render_option(cx, &option) }
                        </div>
                    }
                }
            />
        </leptonic-select-options>
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
                let target = e.target().unwrap();
                let target_elem = target.dyn_ref::<HtmlElement>().unwrap().clone();
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
    });
}

fn create_key_down_listener<W: Fn(KeyboardEvent) -> bool + 'static, T: Fn() + 'static>(
    cx: Scope,
    when: W,
    then: T,
) {
    let g_keyboard_event =
        use_context::<GlobalKeyboardEvent>(cx).expect("Must be a child of the Root component.");

    create_effect(cx, move |_old| {
        let g_keyboard_event = g_keyboard_event.read_signal.get();
        if let Some(e) = g_keyboard_event {
            if when(e) {
                then();
            }
        }
    });
}

// TODO: Prop: close_options_menu_on_selection: bool
// TODO: Prop: selection_changed: Callback<Selection<T>>
