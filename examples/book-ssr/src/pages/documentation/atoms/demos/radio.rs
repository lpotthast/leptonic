use leptonic::{
    atoms::{
        checkbox::{CheckboxButton, CheckboxField},
        field::{Description, Label},
        radio::{RadioButton, RadioField, RadioGroup},
    },
    hooks::Orientation,
    selection_value,
};
use leptos::prelude::*;

/// The plans. The group's value is a `Plan`: `selection_value!` names each plan's key, which forms submit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Plan {
    Free,
    Pro,
    Team,
}

selection_value!(Plan { Free = "free", Pro = "pro", Team = "team" });

const PLANS: [(Plan, &str); 3] = [(Plan::Free, "Free"), (Plan::Pro, "Pro"), (Plan::Team, "Team")];

#[component]
pub fn RadioAtomDemo() -> impl IntoView {
    let plan = RwSignal::new(Some(Plan::Free));
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
                        // The button is a `<label>` around a hidden input; its children draw the circle.
                        <RadioField
                            value
                            is_disabled=Signal::derive(move || value == Plan::Team && team_disabled.get())
                        >
                            <RadioButton classes="demo-radio">
                                <span class="demo-radio-circle" aria-hidden="true"></span>
                                {label}
                            </RadioButton>
                        </RadioField>
                    })
                    .collect_view()}
            </div>
            <Description classes="demo-choice-group-description">"You can change your plan at any time."</Description>
        </RadioGroup>

        <p class="demo-status">
            {move || match plan.get() {
                Some(plan) => format!("Plan: {plan:?}."),
                None => "No plan selected.".to_owned(),
            }}
        </p>

        <div class="demo-controls">
            <CheckboxField is_selected=disabled set_selected=disabled>
                <CheckboxButton classes="demo-check">
                    <span class="demo-check-box" aria-hidden="true"></span>
                    "Disabled"
                </CheckboxButton>
            </CheckboxField>
            <CheckboxField is_selected=read_only set_selected=read_only>
                <CheckboxButton classes="demo-check">
                    <span class="demo-check-box" aria-hidden="true"></span>
                    "Read-only"
                </CheckboxButton>
            </CheckboxField>
            <CheckboxField is_selected=team_disabled set_selected=team_disabled>
                <CheckboxButton classes="demo-check">
                    <span class="demo-check-box" aria-hidden="true"></span>
                    "Team plan disabled"
                </CheckboxButton>
            </CheckboxField>
        </div>
    }
}
