use leptonic::{
    components::prelude::*,
    utils::number_formatter::{NumberFormatOptions, NumberStyle, use_number_formatter},
};
use leptos::prelude::*;

#[component]
pub fn NumberFieldDemo() -> impl IntoView {
    // Each field's number type is the one of its signal: `u32` and `f64` here.
    let quantity = RwSignal::new(Some(2_u32));
    let price = RwSignal::new(Some(4.5_f64));
    let discount = RwSignal::new(Some(0.1_f64));
    let disabled = RwSignal::new(false);

    let euros = NumberFormatOptions {
        style: NumberStyle::Currency,
        currency: Some("EUR".to_owned()),
        ..NumberFormatOptions::default()
    };
    let percent = NumberFormatOptions {
        style: NumberStyle::Percent,
        ..NumberFormatOptions::default()
    };

    // The total is formatted like the price, for the current locale.
    let currency = use_number_formatter(Signal::stored(euros.clone()));
    let total = move || match (quantity.get(), price.get()) {
        (Some(quantity), Some(price)) => {
            let total = f64::from(quantity) * price * (1.0 - discount.get().unwrap_or_default());
            format!("Total: {}", currency.with(|currency| currency.format(total)))
        }
        _ => "Enter a quantity and a price.".to_owned(),
    };

    view! {
        <div class="demo-form">
            <NumberField
                label="Quantity"
                description="1 to 99 pieces."
                value=quantity set_value=quantity
                min_value=1_u32
                max_value=99_u32
                is_disabled=disabled
            />
            <NumberField label="Unit price" value=price set_value=price min_value=0.0 step=0.5 format_options=euros is_disabled=disabled/>
            <NumberField
                label="Discount"
                description="Up to 50%, in steps of 5%."
                value=discount set_value=discount
                min_value=0.0
                max_value=0.5
                step=0.05
                format_options=percent
                is_disabled=disabled
            />
        </div>
        <p class="demo-status">{total}</p>

        <div class="demo-controls">
            <Checkbox is_selected=disabled set_selected=disabled>"Disabled"</Checkbox>
        </div>
    }
}
