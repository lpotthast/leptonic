use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn InputLabeledDemo() -> impl IntoView {
    let (text, set_text) = signal("text".to_owned());

    view! {
        <FormControl>
            <Label>
                "Label"
            </Label>
            <TextInput get=text set=set_text/>
        </FormControl>
    }
}
