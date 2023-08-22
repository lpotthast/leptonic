use std::ops::Deref;

use leptos::{html::ElementDescriptor, *};
use web_sys::HtmlInputElement;

use crate::{
    prelude::{Callable, Callback, create_callback},
    OptionalMaybeSignal, Out,
};

fn prepare_autofocus<
    T: ElementDescriptor + Clone + Deref<Target = web_sys::HtmlInputElement> + 'static,
>(
    cx: Scope,
    node_ref: NodeRef<T>,
) {
    node_ref.on_load(cx, move |elem| {
        let outcome = elem.focus();
        if let Err(err) = outcome {
            tracing::error!(?err, "Could not update autofocus.");
        }
    });
}

fn use_focus<T: ElementDescriptor + Clone + Deref<Target = web_sys::HtmlInputElement> + 'static>(
    cx: Scope,
    focus: Signal<bool>,
    node_ref: NodeRef<T>,
) {
    create_effect(cx, move |_prev| {
        let focus = focus.get();
        let elem = node_ref.get();
        if let Some(elem) = elem {
            let outcome = match focus {
                true => elem.focus(),
                false => elem.blur(),
            };
            if let Err(err) = outcome {
                tracing::error!(?err, "Could not update focus to {}.", focus);
            }
        }
    });
}

#[component]
pub fn TextInput(
    cx: Scope,
    #[prop(into)] get: MaybeSignal<String>,
    #[prop(into, optional)] set: Option<Out<String>>,
    #[prop(optional, into)] placeholder: OptionalMaybeSignal<String>,
    #[prop(optional, into)] prepend: OptionalMaybeSignal<View>,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] disabled: OptionalMaybeSignal<bool>,
    #[prop(into, optional)] should_be_focused: Option<Signal<bool>>,
    #[prop(into, optional)] on_focus_change: Option<Callback<bool>>,
    #[prop(into, optional)] autofocus: bool,
    #[prop(into, optional)] style: Option<AttributeValue>,
) -> impl IntoView {
    let node_ref: NodeRef<leptos::html::Input> = create_node_ref(cx);

    if autofocus {
        prepare_autofocus(cx, node_ref);
    }

    if let Some(focus) = should_be_focused {
        use_focus(cx, focus, node_ref);
    }

    view! { cx,
        <leptonic-input style=style>
            <input
                node_ref=node_ref
                id=id
                class=class
                placeholder=move || match &placeholder.0 {
                    Some(label) => std::borrow::Cow::Owned(label.get()),
                    None => std::borrow::Cow::Borrowed(""),
                }
                type="text"
                prop:disabled=move || disabled.0.as_ref().map(|it| it.get()).unwrap_or(false)
                prop:value=move || get.get()
                on:change=move |e| { if let Some(set) = set { set.set(event_target::<HtmlInputElement>(&e).value()) } }
                on:keyup=move |e| { if let Some(set) = set { set.set(event_target::<HtmlInputElement>(&e).value()) } }
                on:blur=move |_e| { if let Some(cb) = on_focus_change { cb.call(false) }; }
                on:focus=move |_e| { if let Some(cb) = on_focus_change { cb.call(true) }; }
            />
            {match prepend.0 {
                Some(view) => view! { cx,
                    <div>
                        { view.get() }
                    </div>
                }.into_view(cx),
                None => ().into_view(cx),
            }}
        </leptonic-input>
    }
}

#[component]
pub fn PasswordInput(
    cx: Scope,
    #[prop(into)] get: MaybeSignal<String>,
    #[prop(into, optional)] set: Option<Out<String>>,
    #[prop(optional, into)] placeholder: OptionalMaybeSignal<String>,
    #[prop(optional, into)] prepend: OptionalMaybeSignal<View>,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] disabled: OptionalMaybeSignal<bool>,
    #[prop(into, optional)] should_be_focused: Option<Signal<bool>>,
    #[prop(into, optional)] on_focus_change: Option<Callback<bool>>,
    #[prop(into, optional)] autofocus: bool,
    #[prop(into, optional)] style: Option<AttributeValue>,
) -> impl IntoView {
    let node_ref: NodeRef<leptos::html::Input> = create_node_ref(cx);

    if autofocus {
        prepare_autofocus(cx, node_ref);
    }

    if let Some(focus) = should_be_focused {
        use_focus(cx, focus, node_ref);
    }

    view! { cx,
        <leptonic-input style=style>
            <input
                node_ref=node_ref
                id=id
                class=class
                placeholder=move || match &placeholder.0 {
                    Some(label) => std::borrow::Cow::Owned(label.get()),
                    None => std::borrow::Cow::Borrowed(""),
                }
                type="password"
                prop:disabled=move || disabled.0.as_ref().map(|it| it.get()).unwrap_or(false)
                prop:value=move || get.get()
                on:change=move |e| { if let Some(set) = set { set.set(event_target::<HtmlInputElement>(&e).value()) } }
                on:keyup=move |e| { if let Some(set) = set { set.set(event_target::<HtmlInputElement>(&e).value()) } }
                on:blur=move |_e| { if let Some(cb) = on_focus_change { cb.call(false) }; }
                on:focus=move |_e| { if let Some(cb) = on_focus_change { cb.call(true) }; }
            />
            {match prepend.0 {
                Some(view) => view! { cx,
                    <div>
                        { view.get() }
                    </div>
                }.into_view(cx),
                None => ().into_view(cx),
            }}
        </leptonic-input>
    }
}

#[component]
pub fn NumberInput(
    cx: Scope,
    #[prop(into)] get: MaybeSignal<f64>,
    #[prop(into, optional)] set: Option<Out<f64>>,
    #[prop(optional)] min: Option<f64>,
    #[prop(optional)] max: Option<f64>,
    #[prop(optional)] step: Option<f64>,
    #[prop(optional, into)] placeholder: OptionalMaybeSignal<String>,
    #[prop(optional, into)] prepend: OptionalMaybeSignal<View>,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] disabled: OptionalMaybeSignal<bool>,
    #[prop(into, optional)] should_be_focused: Option<Signal<bool>>,
    #[prop(into, optional)] on_focus_change: Option<Callback<bool>>,
    #[prop(into, optional)] autofocus: bool,
    #[prop(into, optional)] style: Option<AttributeValue>,
) -> impl IntoView {
    let node_ref: NodeRef<leptos::html::Input> = create_node_ref(cx);

    if autofocus {
        prepare_autofocus(cx, node_ref);
    }

    if let Some(focus) = should_be_focused {
        use_focus(cx, focus, node_ref);
    }

    let set_value = set.map(|set| {
        create_callback(cx, move |v: String| {
            let parsed = str::parse::<f64>(&v).ok();
            if let Some(parsed) = parsed {
                set.set(parsed);
            }
        })
    });

    view! { cx,
        <leptonic-input style=style>
            <input
                node_ref=node_ref
                id=id
                class=class
                placeholder=move || match &placeholder.0 {
                    Some(label) => std::borrow::Cow::Owned(label.get()),
                    None => std::borrow::Cow::Borrowed(""),
                }
                type="number"
                min=min
                max=max
                step=step
                prop:disabled=move || disabled.0.as_ref().map(|it| it.get()).unwrap_or(false)
                prop:value=move || get.get()
                on:change=move |e| { if let Some(set_value) = set_value { set_value.call(event_target::<HtmlInputElement>(&e).value()) } }
                on:keyup=move |e| { if let Some(set_value) = set_value { set_value.call(event_target::<HtmlInputElement>(&e).value()) } }
                on:blur=move |_e| { if let Some(cb) = on_focus_change { cb.call(false) }; }
                on:focus=move |_e| { if let Some(cb) = on_focus_change { cb.call(true) }; }
            />
            {match prepend.0 {
                Some(view) => view! { cx,
                    <div>
                        { view.get() }
                    </div>
                }.into_view(cx),
                None => ().into_view(cx),
            }}
        </leptonic-input>
    }
}
