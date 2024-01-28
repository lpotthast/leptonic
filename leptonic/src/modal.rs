use leptos::*;
use std::rc::Rc;
use uuid::Uuid;

use crate::{prelude::{GlobalKeyboardEvent, Producer}, OptionalMaybeSignal};

#[derive(Copy, Clone)]
pub(crate) struct ModalRootContext {
    pub(crate) host: NodeRef<html::Custom>,
    pub(crate) modals: RwSignal<Vec<Uuid>>,
}

#[component]
pub fn ModalRoot(children: Children) -> impl IntoView {
    let host: NodeRef<html::Custom> = create_node_ref();
    let modals = create_rw_signal(Vec::new());
    provide_context::<ModalRootContext>(ModalRootContext { host, modals });

    let has_modals = create_memo(move |_| match modals.with(|it| it.is_empty()) {
        true => "false",
        false => "true",
    });

    let children = children();
    view! {
        { children }

        <leptonic-modal-host data-has-modals=has_modals>
            <leptonic-modal-backdrop />

            <leptonic-modals ref=host />
        </leptonic-modal-host>
    }
}

#[component]
pub fn Modal(
    #[prop(into)] show_when: MaybeSignal<bool>,
    #[prop(into, optional)] id: Option<String>,
    #[prop(into, optional)] class: Option<String>,
    #[prop(into, optional)] on_escape: Option<Producer<()>>,
    children: ChildrenFn,
) -> impl IntoView {
    let internal_id = Uuid::new_v4();

    let ctx = expect_context::<ModalRootContext>();

    if let Some(on_escape) = on_escape {
        let g_keyboard_event = expect_context::<GlobalKeyboardEvent>();
        create_effect(move |_| {
            if let Some(e) = g_keyboard_event.read_signal.get() {
                tracing::info!(e = e.key().as_str());
                if show_when.get_untracked() && e.key().as_str() == "Escape" {
                    on_escape.call(());
                }
            }
        });
    }

    create_isomorphic_effect(move |_| match show_when.get() {
        true => ctx.modals.update(|m| m.push(internal_id)),
        false => ctx.modals.update(|m| {
            if let Some(ctx) = m.iter().position(|id| *id == internal_id) {
                m.remove(ctx);
            }
        }),
    });

    let this = Rc::new(move || {
        let id = id.clone();
        let class = class.clone();
        let children = children.clone();
        view! {
            <leptonic-modal id=id.unwrap_or_else(|| internal_id.to_string()) class=class>
                { children() }
            </leptonic-modal>
        }
        .into_view()
        .into()
    });

    let portaled_modal = Callback::new(move |()| {
        use std::ops::Deref;
        let mount = ctx.host.get_untracked().unwrap().into_any();
        let mount: &web_sys::Element = mount.deref();
        let mount: web_sys::Element = mount.clone();

        let this = this.clone();
        view! {
            <Portal mount children=this />
        }
        .into_view()
    });

    view! {
        <Show when=move || show_when.get()>
            { move || portaled_modal.call(()) }
        </Show>
    }
}

#[component]
pub fn ModalHeader(children: Children) -> impl IntoView {
    view! {
        <leptonic-modal-header>
            { children() }
        </leptonic-modal-header>
    }
}

#[component]
pub fn ModalTitle(children: Children) -> impl IntoView {
    view! {
        <leptonic-modal-title>
            { children() }
        </leptonic-modal-title>
    }
}

#[component]
pub fn ModalBody(
    children: Children,
    #[prop(into, optional)] style: OptionalMaybeSignal<String>,
) -> impl IntoView {
    view! {
        <leptonic-modal-body style=move || style.0.as_ref().map(SignalGet::get)>
            { children() }
        </leptonic-modal-body>
    }
}

#[component]
pub fn ModalFooter(children: Children) -> impl IntoView {
    view! {
        <leptonic-modal-footer>
            { children() }
        </leptonic-modal-footer>
    }
}
