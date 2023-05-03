use std::{collections::hash_map::DefaultHasher, fmt::Debug, hash::Hash, rc::Rc};

use leptos::*;

use crate::Margin;

pub trait SelectOption: Debug + Clone + PartialEq + Eq + Hash {}

impl<T: Debug + Clone + PartialEq + Eq + Hash> SelectOption for T {}

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

    #[derive(Debug, Clone)]
    struct Inner<T: Eq + Hash> {
        selection: Rc<Selection<T>>,
    }

    let (selected, set_selected) = create_signal(
        cx,
        Inner {
            selection: Rc::new(Selection::None),
        },
    );

    let (show_options, set_show_options) = create_signal(cx, false);

    let toggle_show = move || set_show_options.update(|val| *val = !*val);

    let select = move |option: O| {
        set_selected.update(|selected| {
            *selected = Inner {
                selection: Rc::new(Selection::Single(option)),
            }
        });
        set_show_options.set(false);
    };

    view! { cx,
        <leptonic-select aria-haspopup="listbox" style=style>
            <leptonic-select-selected on:click=move |_| toggle_show()>
                { move || match selected.get().selection.as_ref().clone() {
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
