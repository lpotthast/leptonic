use std::collections::HashSet;

use leptonic::{
    atoms::grid::{Grid, GridCell, GridRow, GridRowGroup},
    hooks::{
        Key, SelectionBehavior, SelectionMode, use_collection,
        collections::Selection,
    },
};
use leptos::prelude::*;

/// Messages: sender, subject, date.
const MESSAGES: [(&str, &str, &str); 4] = [
    ("Ada", "Analytical engine notes", "Mon"),
    ("Grace", "Compiler draft", "Tue"),
    ("Alan", "On computable numbers", "Wed"),
    ("Edsger", "Goto considered harmful", "Thu"),
];

#[component]
pub fn GridMessagesDemo() -> impl IntoView {
    // One row per message, one cell per column; the row text is used for type-ahead. Cell keys derive from the row.
    let collection = use_collection(|b| {
        for (sender, subject, date) in MESSAGES {
            b.row(sender, subject, |r| {
                r.cell(sender);
                r.cell(subject);
                r.cell(date);
            });
        }
    });
    // App state: the selected messages, and the last one opened.
    let selection = RwSignal::new(Selection::default());
    let opened = RwSignal::new(None::<Key>);

    let status = move || {
        let selected = selection.with(|selection| match selection {
            Selection::All => "all".to_owned(),
            Selection::Keys(keys) if keys.is_empty() => "none".to_owned(),
            Selection::Keys(keys) => {
                let mut keys: Vec<String> = keys.iter().map(ToString::to_string).collect();
                keys.sort();
                keys.join(", ")
            }
        });
        let opened = opened.get().map_or_else(|| "none".to_owned(), |key| key.to_string());
        format!("Selected: {selected}. Opened: {opened}.")
    };

    view! {
        <Grid
            collection
            selection_mode=SelectionMode::Multiple
            selection_behavior=SelectionBehavior::Replace
            selection=selection
            set_selection=selection
            disabled_keys=Signal::stored(HashSet::from([Key::from("Alan")]))
            on_row_action=move |key: Key| opened.set(Some(key))
            aria_label="Messages"
            classes="demo-messages"
        >
            <GridRowGroup>
                {MESSAGES
                    .map(|(sender, subject, date)| {
                        let row = Key::from(sender);
                        view! {
                            <GridRow key=row.clone() classes="demo-messages-row">
                                <GridCell key=Key::cell(&row, 0) classes="demo-messages-cell">{sender}</GridCell>
                                <GridCell key=Key::cell(&row, 1) classes="demo-messages-cell">{subject}</GridCell>
                                <GridCell key=Key::cell(&row, 2) classes="demo-messages-cell">{date}</GridCell>
                            </GridRow>
                        }
                    })
                    .collect_view()}
            </GridRowGroup>
        </Grid>
        <p class="demo-status">{status}</p>
    }
}
