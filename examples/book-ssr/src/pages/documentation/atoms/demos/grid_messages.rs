use std::{collections::HashSet, sync::Arc};

use leptonic::{
    atoms::grid::{Grid, GridCell, GridRow, GridRowGroup},
    hooks::{
        SelectionBehavior, SelectionMode,
        collections::{Collection, Key, Selection},
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
    let selected = RwSignal::new(String::from("none"));
    let opened = RwSignal::new(String::from("none"));

    // One row per message, one cell per column; the row text is used for type-ahead.
    let collection = Memo::new(|_| {
        Arc::new(Collection::build(|b| {
            for (sender, subject, date) in MESSAGES {
                b.row(sender, subject, |r| {
                    r.cell(sender);
                    r.cell(subject);
                    r.cell(date);
                });
            }
        }))
    });

    view! {
        <Grid
            collection
            selection_mode=SelectionMode::Multiple
            selection_behavior=SelectionBehavior::Replace
            disabled_keys=Signal::stored(HashSet::from([Key::from("Alan")]))
            on_selection_change=Callback::new(move |selection: Selection| selected.set(describe(&selection)))
            on_row_action=Callback::new(move |key: Key| opened.set(key.to_string()))
            aria_label="Messages"
            classes="demo-messages"
        >
            <GridRowGroup>
                {MESSAGES
                    .map(|(sender, subject, date)| {
                        let row = Key::from(sender);
                        view! {
                            <GridRow key=row.clone() classes="demo-messages-row">
                                <GridCell key=Key::cell(&row, 0)>{sender}</GridCell>
                                <GridCell key=Key::cell(&row, 1)>{subject}</GridCell>
                                <GridCell key=Key::cell(&row, 2)>{date}</GridCell>
                            </GridRow>
                        }
                    })
                    .collect_view()}
            </GridRowGroup>
        </Grid>
        <div class="demo-state-display">
            <div><strong>"Selected: "</strong>{selected}</div>
            <div><strong>"Opened (double click or Enter): "</strong>{opened}</div>
        </div>
    }
}

fn describe(selection: &Selection) -> String {
    match selection {
        Selection::All => "all".to_owned(),
        Selection::Keys(keys) if keys.is_empty() => "none".to_owned(),
        Selection::Keys(keys) => {
            let mut keys: Vec<String> = keys.iter().map(ToString::to_string).collect();
            keys.sort();
            keys.join(", ")
        }
    }
}
