use leptonic::{
    atoms::grid::{Grid, GridCell, GridRow, GridRowGroup},
    hooks::collections::{Key, Selection, SelectionMode, use_collection},
};
use leptos::prelude::*;

const MESSAGES: [(&str, &str); 3] = [
    ("Ada", "Analytical engine notes"),
    ("Grace", "Compiler draft"),
    ("Alan", "On computable numbers"),
];

#[component]
pub fn GridConceptDemo() -> impl IntoView {
    // One row per message, one cell per column. The row text is used for type-ahead.
    let collection = use_collection(|b| {
        for (sender, subject) in MESSAGES {
            b.row(sender, subject, |r| {
                r.cell(sender);
                r.cell(subject);
            });
        }
    });
    let selection = RwSignal::new(Selection::default());

    view! {
        <Grid
            collection
            selection_mode=SelectionMode::Single
            selection=selection
            set_selection=selection
            aria_label="Messages"
            classes="demo-messages"
        >
            <GridRowGroup>
                {MESSAGES
                    .map(|(sender, subject)| {
                        let row = Key::from(sender);
                        view! {
                            <GridRow key=row.clone() classes="demo-messages-row">
                                <GridCell key=Key::cell(&row, 0) classes="demo-messages-cell">{sender}</GridCell>
                                <GridCell key=Key::cell(&row, 1) classes="demo-messages-cell">{subject}</GridCell>
                            </GridRow>
                        }
                    })
                    .collect_view()}
            </GridRowGroup>
        </Grid>
        <p class="demo-status">
            {move || selection.with(|selection| match selection {
                Selection::Keys(keys) => keys.iter().next().map_or_else(|| "No message selected.".to_owned(), |key| format!("Selected: {key}.")),
                Selection::All => "All messages selected.".to_owned(),
            })}
        </p>
    }
}
