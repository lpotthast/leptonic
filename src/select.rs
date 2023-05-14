use std::{collections::hash_map::DefaultHasher, fmt::Debug, hash::Hash};

use leptos::*;

use web_sys::HtmlElement;

use crate::{
    root::{GlobalKeyboardEvent, GlobalMouseEvent},
    Margin,
};

pub trait SelectOption: Debug + Clone + PartialEq + Eq + Hash {}

impl<T: Debug + Clone + PartialEq + Eq + Hash> SelectOption for T {}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SelectMode {
    #[default]
    Single,
    Multiple,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Selection<T: Eq + std::hash::Hash> {
    None,
    Single(T),
    Multiple(Vec<T>),
}

pub trait HasKey<K: Eq + std::hash::Hash + 'static> {
    fn key(&self) -> K;
}

#[component]
pub fn Select<O, S, V>(
    cx: Scope,
    #[prop(optional)] mode: SelectMode,
    #[prop(into)] options: MaybeSignal<Vec<O>>,
    render_option: S,
    #[prop(optional)] margin: Option<Margin>,
) -> impl IntoView
where
    O: SelectOption + 'static,
    S: Fn(Scope, &O) -> V + Copy + 'static,
    V: IntoView + 'static,
{
    let style = margin.map(|it| format!("--margin: {it}"));

    let (selected, set_selected) = create_signal(cx, Selection::None);

    let id: uuid::Uuid = uuid::Uuid::new_v4();
    let id_string = format!("s-{id}");
    let id_selector_string = format!("#{id_string}");

    let (show_options, set_show_options) = create_signal(cx, false);

    // We need to check for global mouse events.
    // If our option list is shown and such an event occurs and does not target our option list, the options list should be closed.
    let g_mouse_event =
        use_context::<GlobalMouseEvent>(cx).expect("Must be a child of the Root component.");
    create_effect(cx, move |_old| {
        use wasm_bindgen::JsCast;
        let last_mouse_event = g_mouse_event.read_signal.get();
        let is_shown = show_options.get_untracked();

        if is_shown {
            if let Some(e) = last_mouse_event {
                let target = e.target().unwrap();
                let target_elem = target.dyn_ref::<HtmlElement>().unwrap().clone();
                match target_elem.closest(id_selector_string.as_ref()) {
                    Ok(closest) => {
                        if let Some(_found) = closest {
                            // User clicked on the options list. Ignoring this global mouse event.
                        } else {
                            // User clicked outside.
                            set_show_options.set(false);
                        }
                    }
                    Err(err) => {
                        error!("Error processing latest mouse event: {err:?}");
                    }
                }
            }
        }
    });

    let g_keyboard_event =
        use_context::<GlobalKeyboardEvent>(cx).expect("Must be a child of the Root component.");
    create_effect(cx, move |_old| {
        let is_shown = show_options.get_untracked();
        if let Some(e) = g_keyboard_event.read_signal.get() {
            if is_shown && e.key().as_str() == "Escape" {
                set_show_options.set(false);
            }
        }
    });

    let toggle_show = move || set_show_options.update(|val| *val = !*val);

    let select = move |option: O| {
        set_selected.update(|selected| match mode {
            SelectMode::Single => *selected = Selection::Single(option),
            SelectMode::Multiple => {
                let s = match selected {
                    Selection::None => vec![option],
                    Selection::Single(s) => vec![s.clone(), option], // todo
                    Selection::Multiple(vec) => {
                        let mut vec = vec.clone();
                        vec.push(option);
                        vec // todo
                    }
                };
                *selected = Selection::Multiple(s);
            }
        });
        set_show_options.set(false);
    };

    view! { cx,
        <leptonic-select id=id_string aria-haspopup="listbox" style=style>
            <leptonic-select-selected on:click=move |_| toggle_show()>
                { move || match selected.get().clone() {
                    Selection::None => view! { cx, }.into_view(cx),
                    Selection::Single(selected) => render_option(cx, &selected).into_view(cx),
                    Selection::Multiple(selected) => view! { cx,
                        <For
                            each=move || selected.clone()
                            key=|selected| selected.hash(&mut DefaultHasher::new())
                            view=move |cx, selected| {
                                view! { cx,
                                    { render_option(cx, &selected) }
                                    ", "
                                }
                            }
                        />
                    }.into_view(cx),
                }}
            </leptonic-select-selected>
            <leptonic-select-options class:shown=show_options>
                <For
                    each=options
                    key=|option| option.hash(&mut DefaultHasher::new())
                    view=move |cx, option| {
                        let clone = option.clone();
                        view! { cx,
                            <div class="option" on:click=move |_| select(clone.clone())>
                                { render_option(cx, &option) }
                            </div>
                        }
                    }
                />
            </leptonic-select-options>
        </leptonic-select>
    }
}

// TODO: Prop: close_options_menu_on_selection: bool
// TODO: Prop: selection_changed: Callback<Selection<T>>
