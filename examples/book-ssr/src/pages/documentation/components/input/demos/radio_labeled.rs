use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn RadioLabeledDemo() -> impl IntoView {
    let (checked, set_checked) = signal(false);

    view! {
        <FormControl>
            <Radio checked=checked set_checked=set_checked />
            <Label>
                "Label"
            </Label>
        </FormControl>
    }
}
