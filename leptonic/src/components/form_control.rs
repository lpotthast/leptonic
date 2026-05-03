use std::fmt::Debug;

use leptos::{context::Provider, prelude::*};
use leptos_classes::Classes;
use leptos_styles::Styles;

pub trait FormInput: Debug + Send + Sync {
    fn on_label_press(&self);
}

#[derive(Debug, Clone, Copy)]
pub struct FormControlContext {
    pub input: RwSignal<Option<Box<dyn FormInput>>>,
}

#[component]
pub fn FormControl(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let input = RwSignal::new(None);

    let ctx = FormControlContext { input };

    view! {
        <div class=classes.add("leptonic-form-control") style=styles>
            <Provider value=ctx>{children()}</Provider>
        </div>
    }
}
