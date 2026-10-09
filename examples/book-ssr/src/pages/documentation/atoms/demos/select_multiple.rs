use std::collections::HashSet;

use leptonic::{
    atoms::{
        field::Label,
        listbox::{ListBox, ListBoxItem},
        select::{Select, SelectPopover, SelectTrigger, SelectValue},
    },
    hooks::collections::use_collection,
    selection_value,
};
use leptos::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Topping {
    Basil,
    Mushrooms,
    Olives,
    Peppers,
}

selection_value!(Topping {
    Basil = "basil",
    Mushrooms = "mushrooms",
    Olives = "olives",
    Peppers = "peppers",
});

const TOPPINGS: [(Topping, &str); 4] = [
    (Topping::Basil, "Basil"),
    (Topping::Mushrooms, "Mushrooms"),
    (Topping::Olives, "Olives"),
    (Topping::Peppers, "Peppers"),
];

#[component]
pub fn SelectMultipleAtomDemo() -> impl IntoView {
    let toppings = use_collection(|b| {
        for (topping, text) in TOPPINGS {
            b.item(topping, text);
        }
    });
    // A `HashSet` makes it a multiple select: the popover stays open while you pick.
    let chosen = RwSignal::new(HashSet::from([Topping::Basil]));

    view! {
        <Select collection=toppings value=chosen set_value=chosen classes="demo-sel">
            <Label classes="demo-sel-label">"Toppings"</Label>
            <SelectTrigger classes="demo-sel-trigger">
                <SelectValue placeholder="No toppings" classes="demo-sel-value"/>
                <span class="demo-sel-caret" aria-hidden="true">"\u{25bc}"</span>
            </SelectTrigger>
            <SelectPopover classes="demo-sel-popover">
                <ListBox classes="demo-sel-listbox">
                    {TOPPINGS
                        .map(|(topping, text)| view! { <ListBoxItem key=topping classes="demo-sel-item">{text}</ListBoxItem> })
                        .collect_view()}
                </ListBox>
            </SelectPopover>
        </Select>

        <p class="demo-status">{move || format!("Selected: {:?}", chosen.get())}</p>
    }
}
