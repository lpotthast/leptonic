use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn NumberFieldConceptDemo() -> impl IntoView {
    // The field's number type is the signal's: `u8`. The value is `None` while the field is empty.
    let tickets = RwSignal::new(Some(2_u8));
    let disabled = RwSignal::new(false);

    view! {
        <NumberField
            label="Tickets"
            description="1 to 10 tickets per order."
            value=tickets
            set_value=tickets
            min_value=1_u8
            max_value=10_u8
            is_disabled=disabled
        />

        <p class="demo-status">
            {move || match tickets.get() {
                None => "No tickets selected.".to_owned(),
                Some(1) => "1 ticket selected.".to_owned(),
                Some(tickets) => format!("{tickets} tickets selected."),
            }}
        </p>

        <div class="demo-controls">
            <Checkbox is_selected=disabled set_selected=disabled>"Disabled"</Checkbox>
        </div>
    }
}
