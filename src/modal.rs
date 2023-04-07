use std::rc::Rc;

use indexmap::IndexMap;
use leptos::*;
use uuid::Uuid;

use crate::*;

#[derive(Clone)]
pub struct Modal {
    pub id: Uuid,
    pub children: Rc<ChildrenFn>,
    pub scope: Scope, // TODO: Is this required?
}

#[derive(Copy, Clone)]
pub struct ModalRootContext {
    pub modals: ReadSignal<IndexMap<Uuid, Modal>>,
    pub set_modals: WriteSignal<IndexMap<Uuid, Modal>>,
}

#[component]
pub fn ModalRoot(cx: Scope, children: Children) -> impl IntoView {
    let (modals, set_modals) = create_signal(cx, IndexMap::new());
    provide_context::<ModalRootContext>(cx, ModalRootContext { modals, set_modals });
    view! { cx,
        { children(cx) }

        <If sig=move || modals.get().len() != 0>
            <div id="modal-host">
                <div class={"leptonic-modal-backdrop"}></div>

                <div class="modals">
                    {move || modals.get().last().map(|(_, modal)| view! { cx,
                        <div class={"leptonic-modal"}>
                            { (modal.children)(modal.scope) }
                        </div>
                    })}
                </div>
            </div>
        </If>
    }
}

#[component]
pub fn Modal(cx: Scope, display_if: ReadSignal<bool>, children: ChildrenFn) -> impl IntoView {
    let modals = use_context::<ModalRootContext>(cx).unwrap();
    let children = Rc::new(children);

    let id = Uuid::new_v4();

    create_effect(cx, move |_| match display_if.get() {
        true => modals.set_modals.update(|modals| {
            modals.insert(
                id,
                Modal {
                    id,
                    children: children.clone(),
                    scope: cx,
                },
            );
        }),
        false => modals.set_modals.update(|modals| {
            modals.remove(&id);
        }),
    });

    // Intentionally empty, as children are rendered using the modal root.
    view! { cx,
    }
}

#[component]
pub fn ModalHeader(cx: Scope, children: Children) -> impl IntoView {
    view! { cx,
        <div class="leptonic-modal-header">
            { children(cx) }
        </div>
    }
}

#[component]
pub fn ModalTitle(cx: Scope, children: Children) -> impl IntoView {
    view! { cx,
        <div class="leptonic-modal-title">
            { children(cx) }
        </div>
    }
}

#[component]
pub fn ModalBody(cx: Scope, children: Children) -> impl IntoView {
    view! { cx,
        <div class="leptonic-modal-body">
            { children(cx) }
        </div>
    }
}

#[component]
pub fn ModalFooter(cx: Scope, children: Children) -> impl IntoView {
    view! { cx,
        <div class="leptonic-modal-footer">
            { children(cx) }
        </div>
    }
}
