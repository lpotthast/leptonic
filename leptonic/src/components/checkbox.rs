use leptos::prelude::*;

use super::form_control::FormInput;
use crate::{
    components::{form_control::FormControlContext, icon::Icon},
    Out,
};

#[derive(Debug, Clone, Copy)]
pub struct CheckboxContext {
    checked: Signal<bool>,
    set_checked: Out<bool>,
}

impl CheckboxContext {
    fn toggle(&self) {
        self.set_checked.set(!self.checked.get_untracked());
    }
}

impl FormInput for CheckboxContext {
    fn on_label_press(&self) {
        self.toggle();
    }
}

#[component]
pub fn Checkbox(
    #[prop(into)] checked: Signal<bool>,
    #[prop(into)] set_checked: Out<bool>,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(default = icondata::BsCheck2)] checked_icon: icondata::Icon,
) -> impl IntoView {
    let ctx = CheckboxContext {
        checked,
        set_checked,
    };

    let form_ctrl_ctx = use_context::<FormControlContext>();

    if let Some(form_ctrl_ctx) = form_ctrl_ctx {
        form_ctrl_ctx.input.set(Some(Box::new(ctx)));
    }

    view! {
        <leptonic-checkbox
            role="checkbox"
            aria-checked=move || if checked.get() { "true" } else { "false" }
            aria-disabled=move || if disabled.get() { "true" } else { "false" }
            tabindex="0"
            on:click=move |_e| {
                if !disabled.get_untracked() {
                    set_checked.set(!checked.get_untracked());
                }
            }
        >
            <Icon
                icon=checked_icon
                attr:style=move || if checked.get() { "display: inherit" } else { "display: none" }
            />
        </leptonic-checkbox>
    }
}
