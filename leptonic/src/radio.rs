use leptos::*;

use crate::{
    form_control::{FormControlContext, FormInput},
    OptionalMaybeSignal, Out,
};

#[derive(Clone)]
struct RadioGroupContext {
    states: StoredValue<Vec<(uuid::Uuid, Signal<bool>, Out<bool>)>>,
}

impl RadioGroupContext {
    fn register(&mut self, id: uuid::Uuid, checked: Signal<bool>, set_checked: Out<bool>) {
        self.states.update_value(|states| {
            states.push((id, checked, set_checked));
        });
    }

    fn deregister(&mut self, id: uuid::Uuid) {
        if let Some(idx) = self
            .states
            .with_value(|states| states.iter().position(|it| it.0 == id))
        {
            self.states.update_value(|states| {
                states.remove(idx);
            });
        } else {
            tracing::warn!(
                "Could not deregister radio button {id}, as it is not currently registered."
            )
        }
    }

    fn toggle(&self, id: uuid::Uuid, new_state: bool) {
        let mut found = false;
        self.states.with_value(|states| {
            match new_state {
                // Uncheck everything, there is nothing we can check...
                false => {
                    for (stored_id, _get, set) in states.iter() {
                        if *stored_id == id {
                            found = true;
                            set.set(false);
                        } else {
                            set.set(false);
                        }
                    }
                }
                // Check targeted, uncheck all other.
                true => {
                    for (stored_id, _get, set) in states.iter() {
                        if *stored_id == id {
                            found = true;
                            set.set(true);
                        } else {
                            set.set(false);
                        }
                    }
                }
            }
        });
        if !found {
            tracing::warn!(
                "Could not toggle radio button with id '{id}' as it is not currently registered."
            );
        }
    }
}

#[component]
pub fn RadioGroup(children: Children) -> impl IntoView {
    let ctx = RadioGroupContext {
        states: store_value(Vec::new()),
    };
    view! {
        <leptonic-radio-group role="radiogroup">
            <Provider value=ctx>
                { children() }
            </Provider>
        </leptonic-radio-group>
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RadioContext {
    checked: Signal<bool>,
    set_checked: Out<bool>,
}

impl RadioContext {
    fn toggle(&self) {
        self.set_checked.set(!self.checked.get_untracked());
    }
}

impl FormInput for RadioContext {
    fn on_label_click(&self) {
        tracing::info!("toggke");
        self.toggle();
    }
}

#[component]
pub fn Radio(
    #[prop(into)] checked: Signal<bool>,
    #[prop(into)] set_checked: Out<bool>,
    #[prop(into, optional)] disabled: OptionalMaybeSignal<bool>,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
) -> impl IntoView {
    let ctx: RadioContext = RadioContext {
        checked,
        set_checked,
    };

    let form_ctrl_ctx = use_context::<FormControlContext>();

    if let Some(form_ctrl_ctx) = form_ctrl_ctx {
        form_ctrl_ctx.input.set(Some(Box::new(ctx)));
    }

    let group_ctx = use_context::<RadioGroupContext>();
    let mut opt_uuid = None;

    if let Some(mut ctx) = group_ctx.clone() {
        let uuid = uuid::Uuid::now_v7();
        ctx.register(uuid, checked, set_checked);
        opt_uuid = Some(uuid);

        on_cleanup(move || {
            ctx.deregister(uuid);
        })
    }

    let disabled = move || disabled.0.as_ref().map_or(false, SignalGet::get);

    view! {
        <leptonic-radio
            id=id
            class=class
            style=style
            role="radio"
            aria-disabled=move || match disabled() { true => "true", false => "false" }
            aria-checked=move || match checked.get() { true => "true", false => "false" }
            data-value=move || match checked.get() { true => "true", false => "false" }
            tabindex="0"
            on:click=move |_e| {
                if !disabled() {
                    match &group_ctx {
                        Some(group_ctx) => group_ctx.toggle(opt_uuid.expect("to be present"), !checked.get_untracked()),
                        None => ctx.toggle(),
                    }
                }
            }
        >
            <leptonic-radio-fill />
        </leptonic-radio>
    }
}
