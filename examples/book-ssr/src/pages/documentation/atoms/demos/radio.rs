use leptonic::{
    atoms::{
        field::{Description, Label},
        radio::{Radio, RadioGroup},
    },
    components::prelude::Checkbox,
    hooks::{Key, Orientation},
};
use leptos::prelude::*;

const PLANS: [(&str, &str); 3] = [("free", "Free"), ("pro", "Pro"), ("team", "Team")];

#[component]
pub fn RadioAtomDemo() -> impl IntoView {
    let plan = RwSignal::new(Some(Key::from("free")));
    let disabled = RwSignal::new(false);
    let read_only = RwSignal::new(false);
    let team_disabled = RwSignal::new(true);

    view! {
        <RadioGroup
            value=plan
            set_value=plan
            orientation=Orientation::Horizontal
            is_disabled=disabled
            is_read_only=read_only
            classes="demo-choice-group"
        >
            <Label classes="demo-choice-group-label">"Plan"</Label>
            <div class="demo-choice-group-items">
                {PLANS
                    .into_iter()
                    .map(|(value, label)| view! {
                        // The atom renders a `<label>` around a visually hidden input; the children draw the circle.
                        <Radio
                            value
                            is_disabled=Signal::derive(move || value == "team" && team_disabled.get())
                            classes="demo-radio"
                        >
                            <span class="demo-radio-circle" aria-hidden="true"></span>
                            {label}
                        </Radio>
                    })
                    .collect_view()}
            </div>
            <Description classes="demo-choice-group-description">"You can change your plan at any time."</Description>
        </RadioGroup>

        <p class="demo-status">
            {move || {
                let selected = plan.get();
                PLANS
                    .into_iter()
                    .find(|(value, _)| selected.as_ref() == Some(&Key::from(*value)))
                    .map_or_else(|| "No plan selected.".to_owned(), |(_, label)| format!("Plan: {label}."))
            }}
        </p>

        <div class="demo-controls">
            <Checkbox is_selected=disabled set_selected=disabled>"Disabled"</Checkbox>
            <Checkbox is_selected=read_only set_selected=read_only>"Read-only"</Checkbox>
            <Checkbox is_selected=team_disabled set_selected=team_disabled>"Team plan disabled"</Checkbox>
        </div>
    }
}
