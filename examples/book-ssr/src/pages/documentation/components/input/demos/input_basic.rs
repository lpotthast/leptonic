use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn InputBasicDemo() -> impl IntoView {
    let (text, set_text) = signal("text".to_owned());

    view! {
        <TextInput get=text set=set_text/>
        <p style="color: gray; margin-top: 0; font-style: italic;">"Text is: " {move || text.get()}</p>
    }
}
