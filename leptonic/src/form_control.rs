use std::fmt::Debug;

use leptos::*;

pub trait FormInput: Debug {
    fn on_label_click(&self);
}

#[derive(Debug, Clone, Copy)]
pub struct FormControlContext {
    pub input: RwSignal<Option<Box<dyn FormInput>>>,
}

#[component]
pub fn FormControl(children: Children) -> impl IntoView {
    let input = create_rw_signal(None);

    let ctx = FormControlContext { input };

    view! {
        <Provider value=ctx>
            { children() }
        </Provider>
    }
}
