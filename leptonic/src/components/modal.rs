use leptos::*;
use std::rc::Rc;
use uuid::Uuid;

use crate::{
    hooks::*,
    prelude::{GlobalKeyboardEvent, Producer},
    OptMaybeSignal,
};

#[derive(Clone)]
struct ShownModalData {
    key: Uuid,
    children: ChildrenFn,
    on_backdrop_interaction: Option<Producer<()>>,
}

#[derive(Copy, Clone)]
struct ModalRootContext {
    modals: RwSignal<Vec<Uuid>>,

    shown_modals: RwSignal<Vec<ShownModalData>>,
}

impl ModalRootContext {
    fn push(&self, data: Uuid) {
        self.modals.update(move |m| m.push(data));
    }

    fn remove(&self, key: Uuid) {
        self.modals.update(move |m| {
            if let Some(idx) = m.iter().position(|it| *it == key) {
                m.remove(idx);
            }
        });
    }

    fn push_shown(&self, data: ShownModalData) {
        self.shown_modals.update(move |m| m.push(data));
    }

    fn remove_shown(&self, key: Uuid) {
        self.shown_modals.update(move |m| {
            if let Some(idx) = m.iter().position(|it| it.key == key) {
                m.remove(idx);
            }
        });
    }
}

#[component]
pub fn ModalRoot(children: Children) -> impl IntoView {
    let modals = create_rw_signal(Vec::new());
    let shown_modals = create_rw_signal(Vec::new());
    let ctx = ModalRootContext {
        modals,
        shown_modals,
    };
    provide_context::<ModalRootContext>(ctx.clone());

    let has_modals = create_memo(move |_| shown_modals.with(|modals| !modals.is_empty()));

    let disable_prevent_scroll = Signal::derive(move || !has_modals.get());

    let _ = use_prevent_scroll(UsePreventScrollInput {
        disabled: disable_prevent_scroll.into(),
    });

    let UsePressReturn {
        props,
        is_pressed: _,
    } = use_press(UsePressInput {
        disabled: false.into(),
        force_prevent_default: true,
        on_press: Callback::new(move |_| {
            if let Some(modal_on_top) = shown_modals.get_untracked().into_iter().rev().next() {
                if let Some(on_backdrop_interaction) = modal_on_top.on_backdrop_interaction {
                    on_backdrop_interaction.call(());
                }
            }
        }),
        on_press_up: None,
        on_press_start: None,
        on_press_end: None,
    });

    view! {
        { children() }

        <leptonic-modal-host data-has-modals=move || match has_modals.get() { true => "true", false => "false" }>
            <leptonic-modal-backdrop
                //use:attrs=props.attrs
                //use:handlers=props.handlers
            />

            <leptonic-modals>
                <For
                    each=move || ctx.shown_modals.get()
                    key=|it| it.key
                    children=|it| view! { {(it.children)()} }
                />
            </leptonic-modals>
        </leptonic-modal-host>
    }
}

#[component]
pub fn Modal(
    #[prop(into)] show_when: MaybeSignal<bool>,
    #[prop(into, optional)] id: Option<String>,
    #[prop(into, optional)] class: Option<String>,
    #[prop(into, optional)] on_escape: Option<Producer<()>>,
    #[prop(into, optional)] on_backdrop_interaction: Option<Producer<()>>,
    children: ChildrenFn,
) -> impl IntoView {
    let ctx = expect_context::<ModalRootContext>();

    if let Some(on_escape) = on_escape {
        let g_keyboard_event = expect_context::<GlobalKeyboardEvent>();
        create_effect(move |_| {
            if let Some(e) = g_keyboard_event.read_signal.get() {
                if show_when.get_untracked() && e.key().as_str() == "Escape" {
                    on_escape.produce();
                }
            }
        });
    }

    let key = Uuid::now_v7();
    let shown = create_memo(move |_| show_when.get());

    let id = store_value(id.unwrap_or_else(|| key.to_string()));
    let class = store_value(class);

    let modal = Rc::new(move || {
        view! {
            <leptonic-modal id=id.get_value() class=class.get_value()>
                { children() }
            </leptonic-modal>
        }
        .into_view()
        .into()
    });

    create_isomorphic_effect(move |_| match shown.get() {
        true => ctx.push_shown(ShownModalData {
            key,
            children: modal.clone(),
            on_backdrop_interaction,
        }),
        false => {
            ctx.remove_shown(key);
        }
    });

    ctx.push(key);

    on_cleanup(move || {
        ctx.remove(key);
    });

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
    #[prop(into, optional)] style: OptMaybeSignal<String>,
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
