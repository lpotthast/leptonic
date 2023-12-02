use std::rc::Rc;

use indexmap::IndexMap;
use leptos::*;
use uuid::Uuid;

use crate::OptionalMaybeSignal;

// TODO: Add mount prop.
// TODO: Add dialog component, making modal the underlying technology?

#[derive(Debug, Clone)]
pub struct ModalData {
    pub internal_id: Uuid,
    pub id: Option<String>,
    pub class: Option<String>,
    pub children: ModalChildren,
}

#[derive(Clone)]
pub enum ModalChildren {
    Once(View),
    Dynamic(Rc<ChildrenFn>),
}

impl std::fmt::Debug for ModalChildren {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Once(_view) => f.debug_tuple("Once").finish(),
            Self::Dynamic(_children) => f.debug_tuple("Dynamic").finish(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ModalRootContext {
    pub modals: ReadSignal<IndexMap<Uuid, ModalData>>,
    pub set_modals: WriteSignal<IndexMap<Uuid, ModalData>>,
}

#[component]
pub fn ModalRoot(children: Children) -> impl IntoView {
    let (modals, set_modals) = create_signal(IndexMap::new());
    provide_context::<ModalRootContext>(ModalRootContext { modals, set_modals });
    view! {
        { children() }

        <leptonic-modal-host>
            <Show fallback=|| () when=move || !modals.get().is_empty()>
                <leptonic-modal-backdrop></leptonic-modal-backdrop>

                <leptonic-modals>
                    {move || modals.get().last().map(|(_, modal)| view! {
                        <leptonic-modal id=modal.id.clone() class=modal.class.clone()>
                            { match &modal.children {
                                ModalChildren::Once(view) => view.clone(),
                                ModalChildren::Dynamic(children) => children().into_view()
                            } }
                        </leptonic-modal>
                    })}
                </leptonic-modals>
            </Show>
        </leptonic-modal-host>
    }
}

#[component]
pub fn Modal(
    #[prop(into)] show_when: MaybeSignal<bool>,
    #[prop(into, optional)] id: Option<String>,
    #[prop(into, optional)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    let modals = expect_context::<ModalRootContext>();
    let children = children().into_view(); // TODO: Is it ok to build this view once?

    let internal_id = Uuid::new_v4();

    create_effect(move |_| match show_when.get() {
        true => modals.set_modals.update(|modals| {
            modals.insert(
                internal_id,
                ModalData {
                    internal_id,
                    id: id.clone(),
                    class: class.clone(),
                    children: ModalChildren::Once(children.clone()),
                },
            );
        }),
        false => modals.set_modals.update(|modals| {
            modals.remove(&internal_id);
        }),
    });

    // Intentionally empty, as children are rendered using the modal root.
    view! {}
}

// TODO: Show a modal in a different scope. This way, not including a shown modal anymore would remove it.
#[component]
pub fn ModalFn(
    #[prop(into)] show_when: MaybeSignal<bool>, // TODO: When https://github.com/leptos-rs/leptos/pull/918 is merged, this should receive a rework!
    #[prop(into, optional)] id: Option<String>,
    #[prop(into, optional)] class: Option<String>,
    children: ChildrenFn,
) -> impl IntoView {
    let modals = expect_context::<ModalRootContext>();
    let children = Rc::new(children);

    let internal_id = Uuid::new_v4();

    create_effect(move |_| match show_when.get() {
        true => modals.set_modals.update(|modals| {
            modals.insert(
                internal_id,
                ModalData {
                    internal_id,
                    id: id.clone(),
                    class: class.clone(),
                    children: ModalChildren::Dynamic(children.clone()),
                },
            );
        }),
        false => modals.set_modals.update(|modals| {
            modals.remove(&internal_id);
        }),
    });

    // Intentionally empty, as children are rendered using the modal root.
    view! {}
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
