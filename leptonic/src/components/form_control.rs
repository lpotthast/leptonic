use std::fmt::Debug;

use leptos::*;

pub trait FormInput: Debug {
    fn on_label_press(&self);
}

#[derive(Debug, Clone, Copy)]
pub struct FormControlContext {
    pub input: RwSignal<Option<Box<dyn FormInput>>>,
}

#[component]
pub fn FormControl(
    children: Children,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
) -> impl IntoView {
    let input = create_rw_signal(None);

    let ctx = FormControlContext { input };

    view! {
        <leptonic-form-control id=id class=class style=style>
            <Provider value=ctx>
                { children() }
            </Provider>
        </leptonic-form-control>
    }
}
