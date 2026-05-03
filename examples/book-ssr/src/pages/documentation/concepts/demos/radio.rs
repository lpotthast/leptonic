use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn RadioConceptDemo() -> impl IntoView {
    let (checked_a, set_checked_a) = signal(true);
    let (checked_b, set_checked_b) = signal(false);

    view! {
        <RadioGroup>
            <FormControl>
                <Radio checked=checked_a set_checked=set_checked_a />
                <Label>"Option A"</Label>
            </FormControl>
            <FormControl>
                <Radio checked=checked_b set_checked=set_checked_b />
                <Label>"Option B"</Label>
            </FormControl>
        </RadioGroup>
    }
}
