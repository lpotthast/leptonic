use leptos::html;
use leptos::html::ElementType;
use leptos::prelude::*;
use std::ops::Deref;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, HtmlInputElement};

use crate::{
    components::form_control::{FormControlContext, FormInput},
    Out,
};

fn prepare_autofocus<T>(
    node_ref: NodeRef<T>,
) where
    T: ElementType + Clone + 'static,
    <T as ElementType>::Output: JsCast + Clone + Deref<Target=HtmlElement>,
{
    node_ref.on_load(move |elem| {
        let input_elem = elem.deref();
        let outcome = input_elem.focus();
        if let Err(err) = outcome {
            tracing::error!(?err, "Could not update autofocus.");
        }
    });
}

// TODO: make this a hook!
fn use_focus<T>(
    focus: Signal<bool>,
    node_ref: NodeRef<T>,
) where
    T: ElementType + Clone + 'static,
    <T as ElementType>::Output: JsCast + Clone + Deref<Target=HtmlElement>,
{
    Effect::new(move |_prev| {
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

// TODO: id and class were previously placed on inner input. This is no longer possible with a spread id class.
#[component]
pub fn TextInput(
    #[prop(into)] get: Signal<String>,
    #[prop(into, optional)] set: Option<Out<String>>,
    #[prop(into, optional, default = Signal::from(Oco::Borrowed("")))] placeholder: Signal<Oco<'static, str>>,
    #[prop(into, optional)] append: ViewFn,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(into, optional)] should_be_focused: Option<Signal<bool>>,
    #[prop(into, optional)] on_focus_change: Out<bool>,
    #[prop(into, optional)] autofocus: bool,
) -> impl IntoView {
    let node_ref: NodeRef<html::Input> = NodeRef::new();

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
        <leptonic-input>
            <input
                node_ref=node_ref
                placeholder=placeholder
                type="text"
                prop:disabled=move || disabled.get()
                prop:value=move || get.get()
                on:change=move |e| { if let Some(set) = &set { set.set(event_target::<HtmlInputElement>(&e).value()) } }
                on:keyup=move |e| { if let Some(set) = &set { set.set(event_target::<HtmlInputElement>(&e).value()) } }
                on:blur=move |_e| { on_focus_change.set(false); }
                on:focus=move |_e| { on_focus_change.set(true); }
            />
            { append.run() }
        </leptonic-input>
    }
}

// TODO: id and class were previously placed on inner input. This is no longer possible with a spread id class.
#[component]
pub fn PasswordInput(
    #[prop(into)] get: Signal<String>,
    #[prop(into, optional)] set: Option<Out<String>>,
    #[prop(into, optional, default = Signal::from(Oco::Borrowed("")))] placeholder: Signal<Oco<'static, str>>,
    #[prop(optional, into)] append: ViewFn,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(into, optional)] should_be_focused: Option<Signal<bool>>,
    #[prop(into, optional)] on_focus_change: Out<bool>,
    #[prop(into, optional)] autofocus: bool,
) -> impl IntoView {
    let node_ref: NodeRef<html::Input> = NodeRef::new();

    if autofocus {
        prepare_autofocus(node_ref);
    }

    if let Some(focus) = should_be_focused {
        use_focus(focus, node_ref);
    }

    view! {
        <leptonic-input>
            <input
                node_ref=node_ref
                placeholder=placeholder
                type="password"
                prop:disabled=move || disabled.get()
                prop:value=move || get.get()
                on:change=move |e| { if let Some(set) = &set { set.set(event_target::<HtmlInputElement>(&e).value()) } }
                on:keyup=move |e| { if let Some(set) = &set { set.set(event_target::<HtmlInputElement>(&e).value()) } }
                on:blur=move |_e| { on_focus_change.set(false); }
                on:focus=move |_e| { on_focus_change.set(true); }
            />
            { append.run() }
        </leptonic-input>
    }
}

#[component]
pub fn NumberInput(
    #[prop(into)] get: Signal<f64>,
    #[prop(into, optional)] set: Option<Out<f64>>,
    #[prop(into, optional)] min: Option<Signal<f64>>,
    #[prop(into, optional)] max: Option<Signal<f64>>,
    #[prop(into, optional, default = Signal::from(0.0))] step: Signal<f64>,
    #[prop(into, optional, default = Signal::from(Oco::Borrowed("")))] placeholder: Signal<Oco<'static, str>>,
    #[prop(into, optional)] append: ViewFn,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(into, optional)] should_be_focused: Option<Signal<bool>>,
    #[prop(into, optional)] on_focus_change: Out<bool>,
    #[prop(into, optional)] autofocus: bool,
) -> impl IntoView {
    let node_ref: NodeRef<html::Input> = NodeRef::new();

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

    view! {
        <leptonic-input>
            <input
                node_ref=node_ref
                placeholder=placeholder
                type="number"
                min=min
                max=max
                step=step
                prop:disabled=move || disabled.get()
                prop:value=move || get.get()
                on:change=move |e| { if let Some(set_value) = &set_value { set_value(event_target::<HtmlInputElement>(&e).value()) } }
                on:keyup=move |e| { if let Some(set_value) = &set_value { set_value(event_target::<HtmlInputElement>(&e).value()) } }
                on:blur=move |_e| { on_focus_change.set(false); }
                on:focus=move |_e| { on_focus_change.set(true); }
            />
            { append.run() }
        </leptonic-input>
    }
}
