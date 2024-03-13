use std::ops::Deref;

use leptos::{html::ElementDescriptor, *};
use web_sys::HtmlInputElement;

use crate::{
    components::form_control::{FormControlContext, FormInput},
    OptMaybeSignal, Out,
};

fn prepare_autofocus<T: ElementDescriptor + Clone + Deref<Target = HtmlInputElement> + 'static>(
    node_ref: NodeRef<T>,
) {
    node_ref.on_load(move |elem| {
        let outcome = elem.focus();
        if let Err(err) = outcome {
            tracing::error!(?err, "Could not update autofocus.");
        }
    });
}

fn use_focus<T: ElementDescriptor + Clone + Deref<Target = HtmlInputElement> + 'static>(
    focus: Signal<bool>,
    node_ref: NodeRef<T>,
) {
    create_effect(move |_prev| {
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

#[derive(Clone, Copy)]
pub struct TextInputContext {
    el: NodeRef<html::Input>,
}

impl TextInputContext {
    fn focus(&self) {
        if let Some(el) = self.el.get_untracked() {
            match el.focus() {
                Ok(()) => {}
                Err(err) => {
                    tracing::warn!(?err, "Could not focus TextInput element");
                }
            }
        }
    }
}

impl std::fmt::Debug for TextInputContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TextInputContext")
            .field("el", &"NodeRef<html::Input>")
            .finish()
    }
}

impl FormInput for TextInputContext {
    fn on_label_press(&self) {
        self.focus();
    }
}

#[component]
pub fn TextInput(
    #[prop(into)] get: MaybeSignal<String>,
    #[prop(into, optional)] set: Option<Out<String>>,
    #[prop(optional, into)] placeholder: OptMaybeSignal<String>,
    #[prop(optional, into)] prepend: OptMaybeSignal<View>,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] disabled: OptMaybeSignal<bool>,
    #[prop(into, optional)] should_be_focused: Option<Signal<bool>>,
    #[prop(into, optional)] on_focus_change: Option<Out<bool>>,
    #[prop(into, optional)] autofocus: bool,
    #[prop(into, optional)] style: Option<AttributeValue>,
) -> impl IntoView {
    let node_ref: NodeRef<html::Input> = create_node_ref();

    let ctx = TextInputContext { el: node_ref };

    let form_ctrl_ctx = use_context::<FormControlContext>();

    if let Some(fc_ctx) = form_ctrl_ctx {
        fc_ctx.input.set(Some(Box::new(ctx)));
    }

    if autofocus {
        prepare_autofocus(node_ref);
    }

    if let Some(focus) = should_be_focused {
        use_focus(focus, node_ref);
    }

    view! {
        <leptonic-input style=style>
            <input
                node_ref=node_ref
                id=id
                class=class
                placeholder=move || match &placeholder.0 {
                    Some(label) => Oco::from(label.get()),
                    None => Oco::from(""),
                }
                type="text"
                prop:disabled=move || disabled.0.as_ref().map_or(false, SignalGet::get)
                prop:value=move || get.get()
                on:change=move |e| { if let Some(set) = &set { set.set(event_target::<HtmlInputElement>(&e).value()) } }
                on:keyup=move |e| { if let Some(set) = &set { set.set(event_target::<HtmlInputElement>(&e).value()) } }
                on:blur=move |_e| { if let Some(cb) = &on_focus_change { cb.set(false) }; }
                on:focus=move |_e| { if let Some(cb) = &on_focus_change { cb.set(true) }; }
            />
            {match prepend.0 {
                Some(view) => view! {
                    <div>
                        { view.get() }
                    </div>
                }.into_view(),
                None => ().into_view(),
            }}
        </leptonic-input>
    }
}

#[component]
pub fn PasswordInput(
    #[prop(into)] get: MaybeSignal<String>,
    #[prop(into, optional)] set: Option<Out<String>>,
    #[prop(optional, into)] placeholder: OptMaybeSignal<String>,
    #[prop(optional, into)] prepend: OptMaybeSignal<View>,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] disabled: OptMaybeSignal<bool>,
    #[prop(into, optional)] should_be_focused: Option<Signal<bool>>,
    #[prop(into, optional)] on_focus_change: Option<Out<bool>>,
    #[prop(into, optional)] autofocus: bool,
    #[prop(into, optional)] style: Option<AttributeValue>,
) -> impl IntoView {
    let node_ref: NodeRef<html::Input> = create_node_ref();

    if autofocus {
        prepare_autofocus(node_ref);
    }

    if let Some(focus) = should_be_focused {
        use_focus(focus, node_ref);
    }

    view! {
        <leptonic-input style=style>
            <input
                node_ref=node_ref
                id=id
                class=class
                placeholder=move || match &placeholder.0 {
                    Some(label) => Oco::from(label.get()),
                    None => Oco::from(""),
                }
                type="password"
                prop:disabled=move || disabled.0.as_ref().map_or(false, SignalGet::get)
                prop:value=move || get.get()
                on:change=move |e| { if let Some(set) = &set { set.set(event_target::<HtmlInputElement>(&e).value()) } }
                on:keyup=move |e| { if let Some(set) = &set { set.set(event_target::<HtmlInputElement>(&e).value()) } }
                on:blur=move |_e| { if let Some(cb) = &on_focus_change { cb.set(false) }; }
                on:focus=move |_e| { if let Some(cb) = &on_focus_change { cb.set(true) }; }
            />
            {match prepend.0 {
                Some(view) => view! {
                    <div>
                        { view.get() }
                    </div>
                }.into_view(),
                None => ().into_view(),
            }}
        </leptonic-input>
    }
}

#[component]
pub fn NumberInput(
    #[prop(into)] get: MaybeSignal<f64>,
    #[prop(into, optional)] set: Option<Out<f64>>,
    #[prop(into, optional)] min: OptMaybeSignal<f64>,
    #[prop(into, optional)] max: OptMaybeSignal<f64>,
    #[prop(into, optional)] step: OptMaybeSignal<f64>,
    #[prop(into, optional)] placeholder: OptMaybeSignal<String>,
    #[prop(into, optional)] prepend: OptMaybeSignal<View>,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] disabled: OptMaybeSignal<bool>,
    #[prop(into, optional)] should_be_focused: Option<Signal<bool>>,
    #[prop(into, optional)] on_focus_change: Option<Out<bool>>,
    #[prop(into, optional)] autofocus: bool,
    #[prop(into, optional)] style: Option<AttributeValue>,
) -> impl IntoView {
    let node_ref: NodeRef<html::Input> = create_node_ref();

    if autofocus {
        prepare_autofocus(node_ref);
    }

    if let Some(focus) = should_be_focused {
        use_focus(focus, node_ref);
    }

    let set_value = set.map(|set| {
        move |v: String| {
            let parsed = str::parse::<f64>(&v).ok();
            if let Some(parsed) = parsed {
                set.set(parsed);
            }
        }
    });

    let min_value = move || {
        min.0
            .as_ref()
            .map(SignalGet::get)
            .map(|v| v.to_string())
            .unwrap_or_default()
    };
    let max_value = move || {
        max.0
            .as_ref()
            .map(SignalGet::get)
            .map(|v| v.to_string())
            .unwrap_or_default()
    };

    view! {
        <leptonic-input style=style>
            <input
                node_ref=node_ref
                id=id
                class=class
                placeholder=move || match &placeholder.0 {
                    Some(label) => Oco::from(label.get()),
                    None => Oco::from(""),
                }
                type="number"
                min=min_value
                max=max_value
                step=move || step.0.as_ref().map(SignalGet::get).unwrap_or(0.0)
                prop:disabled=move || disabled.0.as_ref().map(SignalGet::get).unwrap_or(false)
                prop:value=move || get.get()
                on:change=move |e| { if let Some(set_value) = &set_value { set_value(event_target::<HtmlInputElement>(&e).value()) } }
                on:keyup=move |e| { if let Some(set_value) = &set_value { set_value(event_target::<HtmlInputElement>(&e).value()) } }
                on:blur=move |_e| { if let Some(cb) = &on_focus_change { cb.set(false) }; }
                on:focus=move |_e| { if let Some(cb) = &on_focus_change { cb.set(true) }; }
            />
            {match prepend.0 {
                Some(view) => view! {
                    <div>
                        { view.get() }
                    </div>
                }.into_view(),
                None => ().into_view(),
            }}
        </leptonic-input>
    }
}
