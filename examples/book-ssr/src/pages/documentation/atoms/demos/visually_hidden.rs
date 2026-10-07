use leptonic::atoms::{
    button::Button, field::TextElement, link::AnchorLink, visually_hidden::VisuallyHidden,
};
use leptos::prelude::*;

const INVOICES: [(&str, &str); 3] = [
    ("1042", "Acme Corp"),
    ("1043", "Globex"),
    ("1044", "Initech"),
];

#[component]
pub fn VisuallyHiddenAtomDemo() -> impl IntoView {
    let last = RwSignal::new(String::from("nothing yet"));

    view! {
        // Shown while it has focus: lets keyboard users skip the list.
        <VisuallyHidden is_focusable=true>
            <AnchorLink href="#visually-hidden-atom-demo-status">"Skip the invoices"</AnchorLink>
        </VisuallyHidden>
        <ul>
            {INVOICES
                .into_iter()
                .map(|(number, customer)| view! {
                    <li class="demo-control-row">
                        <span>{format!("Invoice {number}, {customer}")}</span>
                        // Every button reads "Download", screen readers hear which invoice.
                        <Button on_press=move |_| last.set(format!("invoice {number}")) classes="demo-btn">
                            "Download"
                            <VisuallyHidden element=TextElement::Span>{format!(" invoice {number}")}</VisuallyHidden>
                        </Button>
                    </li>
                })
                .collect_view()}
        </ul>
        <p id="visually-hidden-atom-demo-status" class="demo-status">"Last download: "{last}</p>
    }
}
