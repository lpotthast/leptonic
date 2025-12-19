use leptos::prelude::*;
use uuid::Uuid;

use crate::{
    hooks::{use_press, use_prevent_scroll, UsePressInput, UsePressReturn, UsePreventScrollInput},
    prelude::GlobalKeyboardEvent,
};

#[derive(Clone)]
struct ShownModal {
    modal_id: Uuid,
    modal_renderer: ViewFn,
    on_backdrop_interaction: Option<Callback<(), ()>>,
}

#[derive(Debug, Clone, Copy)]
struct ModalRootContext {
    /// List of modals currently shown to the user.
    shown_modals: RwSignal<Vec<ShownModal>>,
}

impl ModalRootContext {
    fn is_shown(&self, modal_id: Uuid) -> bool {
        self.shown_modals
            .read_untracked()
            .iter()
            .any(|it| it.modal_id == modal_id)
    }

    /// Shows the modal with `modal_id`.
    ///
    /// # Panics
    /// - If the modal is already shown.
    fn show(&self, data: ShownModal) {
        assert!(!self.is_shown(data.modal_id));

        self.shown_modals.update(move |m| m.push(data));
    }

    /// Removes the modal with `modal_id` from the list of shown modals.
    /// Should the modal not be shown right now, nothing happens.
    fn hide(&self, modal_id: Uuid) {
        self.shown_modals.update(move |m| {
            if let Some(idx) = m.iter().position(|it| it.modal_id == modal_id) {
                m.remove(idx);
            }
        });
        assert!(!self.is_shown(modal_id));
    }
}

#[component]
pub fn ModalRoot(children: Children) -> impl IntoView {
    let shown_modals = RwSignal::new(Vec::new());
    let ctx = ModalRootContext { shown_modals };
    provide_context::<ModalRootContext>(ctx.clone());

    let has_modals = Memo::new(move |_| shown_modals.with(|modals| !modals.is_empty()));

    let disable_prevent_scroll = Signal::derive(move || !has_modals.get());

    let _ = use_prevent_scroll(UsePreventScrollInput {
        disabled: disable_prevent_scroll.into(),
    });

    let UsePressReturn {
        attrs,
        is_pressed: _,
    } = use_press(UsePressInput {
        disabled: false.into(),
        force_prevent_default: true,
        allow_propagation: false,
        on_press: Callback::new(move |_| {
            if let Some(modal_on_top) = shown_modals.get_untracked().into_iter().rev().next() {
                if let Some(on_backdrop_interaction) = modal_on_top.on_backdrop_interaction {
                    on_backdrop_interaction.run(());
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
            <leptonic-modal-backdrop {..attrs}/>

            <leptonic-modals>
                <For
                    each=move || ctx.shown_modals.get()
                    key=|it| it.modal_id
                    children=|it| it.modal_renderer.run()
                />
            </leptonic-modals>
        </leptonic-modal-host>
    }
}

#[component]
pub fn Modal(
    #[prop(into)] show_when: Signal<bool>,
    #[prop(into, optional)] id: Option<String>,
    #[prop(into, optional)] class: Option<String>,
    #[prop(into, optional)] on_escape: Option<Callback<(), ()>>,
    #[prop(into, optional)] on_backdrop_interaction: Option<Callback<(), ()>>,
    children: ChildrenFn,
) -> impl IntoView {
    let ctx = expect_context::<ModalRootContext>();

    if let Some(on_escape) = on_escape {
        let g_keyboard_event = expect_context::<GlobalKeyboardEvent>();
        Effect::new(move |_| {
            if let Some(e) = g_keyboard_event.read_signal.get() {
                if show_when.get_untracked() && e.key().as_str() == "Escape" {
                    on_escape.run(());
                }
            }
        });
    }

    let key = Uuid::now_v7();
    let should_be_shown = Memo::new(move |_| show_when.get());

    let id = StoredValue::new(id.unwrap_or_else(|| key.to_string()));
    let class = StoredValue::new(class);

    let modal_renderer = ViewFn::from(move || {
        view! {
            <leptonic-modal id=id.get_value() class=class.get_value()>
                { children() }
            </leptonic-modal>
        }
    });

    Effect::new_isomorphic(move |_| {
        if should_be_shown.get() {
            ctx.show(ShownModal {
                modal_id: key,
                modal_renderer: modal_renderer.clone(),
                on_backdrop_interaction,
            });
        } else {
            ctx.hide(key);
        }
    });

    on_cleanup(move || {
        ctx.hide(key);
    });

    ()
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
pub fn ModalBody(children: Children) -> impl IntoView {
    view! {
        <leptonic-modal-body>
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
