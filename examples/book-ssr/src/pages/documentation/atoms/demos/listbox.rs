use leptonic::{
    atoms::{
        button::Button,
        listbox::{
            ListBox, ListBoxItem, ListBoxItemDescription, ListBoxItemLabel, ListBoxSection,
            ListBoxSectionHeading,
        },
    },
    hooks::{
        SelectionMode,
        collections::{Key, Selection, use_collection},
    },
};
use leptos::prelude::*;

/// A section of the menu.
struct Group {
    key: &'static str,
    heading: &'static str,
    toppings: &'static [Topping],
}

struct Topping {
    key: &'static str,
    name: &'static str,
    price: &'static str,
}

const MENU: [Group; 3] = [
    Group {
        key: "cheese",
        heading: "Cheese",
        toppings: &[
            Topping {
                key: "mozzarella",
                name: "Mozzarella",
                price: "Included",
            },
            Topping {
                key: "gorgonzola",
                name: "Gorgonzola",
                price: "+ \u{20ac}1.50",
            },
            Topping {
                key: "parmesan",
                name: "Parmesan",
                price: "+ \u{20ac}1.00",
            },
        ],
    },
    Group {
        key: "vegetables",
        heading: "Vegetables",
        toppings: &[
            Topping {
                key: "mushrooms",
                name: "Mushrooms",
                price: "+ \u{20ac}1.00",
            },
            Topping {
                key: "olives",
                name: "Olives",
                price: "+ \u{20ac}1.00",
            },
            Topping {
                key: "peppers",
                name: "Peppers",
                price: "+ \u{20ac}0.80",
            },
        ],
    },
    Group {
        key: "meat",
        heading: "Meat & fish",
        toppings: &[
            Topping {
                key: "ham",
                name: "Ham",
                price: "+ \u{20ac}1.50",
            },
            Topping {
                key: "salami",
                name: "Salami",
                price: "+ \u{20ac}1.50",
            },
            Topping {
                key: "anchovies",
                name: "Anchovies",
                price: "Sold out",
            },
        ],
    },
];

/// Toppings that can't be ordered today.
const SOLD_OUT: &str = "anchovies";

#[component]
pub fn ListBoxAtomDemo() -> impl IntoView {
    // The options, grouped into sections with a header each. Sold-out toppings are disabled.
    let toppings = use_collection(|b| {
        for group in &MENU {
            b.section(group.key, |s| {
                s.header(format!("{}-heading", group.key), group.heading);
                for topping in group.toppings {
                    s.item(topping.key, topping.name)
                        .disabled(topping.key == SOLD_OUT);
                }
            });
        }
    });
    // App state: the selected toppings. The list box shows them, and selecting writes them.
    let selection = RwSignal::new(Selection::keys([Key::from("mozzarella")]));

    view! {
        <div class="demo-lb">
            <span id="demo-lb-label" class="demo-lb-label">"Toppings"</span>
            <ListBox
                collection=toppings
                selection_mode=SelectionMode::Multiple
                selection=selection
                set_selection=selection
                aria_labelledby="demo-lb-label"
                classes="demo-lb-listbox"
            >
                {MENU
                    .iter()
                    .map(|group| {
                        view! {
                            <ListBoxSection key=group.key classes="demo-lb-group">
                                <ListBoxSectionHeading classes="demo-lb-heading" />
                                {group
                                    .toppings
                                    .iter()
                                    .map(|topping| {
                                        view! {
                                            <ListBoxItem key=topping.key classes="demo-lb-item">
                                                <span class="demo-lb-check" aria-hidden="true"></span>
                                                <ListBoxItemLabel>{topping.name}</ListBoxItemLabel>
                                                <ListBoxItemDescription classes="demo-lb-price">
                                                    {topping.price}
                                                </ListBoxItemDescription>
                                            </ListBoxItem>
                                        }
                                    })
                                    .collect_view()}
                            </ListBoxSection>
                        }
                    })
                    .collect_view()}
            </ListBox>
        </div>

        <p class="demo-status">"Selected: "{move || selected_names(&selection.get())}"."</p>
        <div class="demo-controls">
            // The app changes the selection by writing its state.
            <Button on_press=move |_| selection.set(Selection::default()) classes="demo-btn">
                "Clear"
            </Button>
        </div>
    }
}

/// The names of the selected toppings, in menu order.
fn selected_names(selection: &Selection) -> String {
    let names: Vec<&str> = MENU
        .iter()
        .flat_map(|group| group.toppings)
        .filter(|topping| match selection {
            // Select all (Ctrl + A, Cmd + A on macOS) selects every topping that isn't disabled.
            Selection::All => topping.key != SOLD_OUT,
            Selection::Keys(keys) => keys.contains(&Key::from(topping.key)),
        })
        .map(|topping| topping.name)
        .collect();
    if names.is_empty() {
        "none".to_owned()
    } else {
        names.join(", ")
    }
}
