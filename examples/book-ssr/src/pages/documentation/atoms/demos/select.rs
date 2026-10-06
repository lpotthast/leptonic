use leptonic::{
    atoms::{
        field::{Description, Label},
        listbox::{ListBox, ListBoxItem, ListBoxItemDescription, ListBoxItemLabel, ListBoxSection},
        select::{Select, SelectPopover, SelectTrigger, SelectValue},
    },
    components::prelude::{Button, ButtonColor, Checkbox},
    hooks::collections::{Key, use_collection},
};
use leptos::prelude::*;

/// A section of the office list: its key, heading and offices (key, city).
struct Region {
    key: &'static str,
    heading: &'static str,
    offices: &'static [(&'static str, &'static str)],
}

const REGIONS: [Region; 3] = [
    Region {
        key: "europe",
        heading: "Europe",
        offices: &[
            ("berlin", "Berlin"),
            ("lisbon", "Lisbon"),
            ("london", "London"),
        ],
    },
    Region {
        key: "americas",
        heading: "Americas",
        offices: &[("new-york", "New York"), ("toronto", "Toronto")],
    },
    Region {
        key: "asia-pacific",
        heading: "Asia-Pacific",
        offices: &[
            ("singapore", "Singapore"),
            ("sydney", "Sydney"),
            ("tokyo", "Tokyo"),
        ],
    },
];

/// Offices without a free desk.
const FULLY_BOOKED: &str = "lisbon";

#[component]
pub fn SelectAtomDemo() -> impl IntoView {
    // The options, grouped into sections with a header each. Fully booked offices are disabled.
    let offices = use_collection(|b| {
        for region in &REGIONS {
            b.section(region.key, |s| {
                s.header(format!("{}-heading", region.key), region.heading);
                for (key, city) in region.offices {
                    s.item(*key, *city).disabled(*key == FULLY_BOOKED);
                }
            });
        }
    });
    // App state: the selected office. The select shows it, and choosing an option writes it.
    let office = RwSignal::new(Vec::<Key>::new());
    let disabled = RwSignal::new(false);

    view! {
        <Select
            collection=offices
            value=office
            set_value=office
            is_disabled=disabled
            classes="demo-sel"
        >
            <Label classes="demo-sel-label">"Office"</Label>
            <SelectTrigger classes="demo-sel-trigger">
                <SelectValue placeholder="Choose an office" classes="demo-sel-value"/>
                <span class="demo-sel-caret" aria-hidden="true">"\u{25bc}"</span>
            </SelectTrigger>
            <Description classes="demo-sel-description">"Where you\u{2019}ll book a desk."</Description>
            <SelectPopover classes="demo-sel-popover">
                <ListBox classes="demo-sel-listbox">
                    {REGIONS
                        .iter()
                        .map(|region| {
                            view! {
                                <ListBoxSection key=region.key heading_classes="demo-sel-heading">
                                    {region
                                        .offices
                                        .iter()
                                        .map(|(key, city)| {
                                            view! {
                                                <ListBoxItem key=*key classes="demo-sel-item">
                                                    <ListBoxItemLabel>{*city}</ListBoxItemLabel>
                                                    {(*key == FULLY_BOOKED).then(|| view! {
                                                        <ListBoxItemDescription classes="demo-sel-note">
                                                            "Fully booked"
                                                        </ListBoxItemDescription>
                                                    })}
                                                </ListBoxItem>
                                            }
                                        })
                                        .collect_view()}
                                </ListBoxSection>
                            }
                        })
                        .collect_view()}
                </ListBox>
            </SelectPopover>
        </Select>

        <p class="demo-status">
            "Selected key: "
            {move || office.with(|keys| keys.first().map_or_else(|| "none".to_owned(), ToString::to_string))}
        </p>
        <div class="demo-controls">
            <Checkbox is_selected=disabled set_selected=disabled>"Disabled"</Checkbox>
            // The app changes the selection by writing its state.
            <Button on_press=move |_| office.set(Vec::new()) color=ButtonColor::Secondary>"Clear"</Button>
        </div>

    }
}
