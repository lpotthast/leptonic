use leptos::context::Provider;
use leptos::prelude::*;
use std::fmt::Debug;

pub trait FormInput: Debug + Send + Sync {
    fn on_label_press(&self);
}

#[derive(Debug, Clone, Copy)]
pub struct FormControlContext {
    pub input: RwSignal<Option<Box<dyn FormInput>>>,
}

#[component]
pub fn FormControl(children: Children) -> impl IntoView {
    let input = RwSignal::new(None);

    let ctx = FormControlContext { input };

    view! {
        <leptonic-form-control>
            <Provider value=ctx>{children()}</Provider>
        </leptonic-form-control>
    }
}
