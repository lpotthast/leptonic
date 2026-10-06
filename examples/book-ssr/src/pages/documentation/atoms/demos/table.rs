use std::{collections::HashSet, sync::Arc};

use leptonic::{
    atoms::table::{Table, TableBody, TableCell, TableHeader, TableRow},
    components::prelude::{Button, ButtonColor},
    hooks::{
        DisabledBehavior, SelectionMode, SortDescriptor, SortDirection, TableCollection,
        TableOptions,
        collections::{Key, Selection},
    },
};
use leptos::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Member {
    id: &'static str,
    name: &'static str,
    role: &'static str,
    city: &'static str,
}

/// The team, in the order its members joined.
const MEMBERS: [Member; 5] = [
    Member {
        id: "grace",
        name: "Grace Hopper",
        role: "Admiral",
        city: "Arlington",
    },
    Member {
        id: "ada",
        name: "Ada Lovelace",
        role: "Engineer",
        city: "London",
    },
    Member {
        id: "linus",
        name: "Linus Torvalds",
        role: "Maintainer",
        city: "Portland",
    },
    Member {
        id: "alan",
        name: "Alan Turing",
        role: "Researcher",
        city: "Manchester",
    },
    Member {
        id: "katherine",
        name: "Katherine Johnson",
        role: "Mathematician",
        city: "Hampton",
    },
];

/// The members in the order of `sort`; without sorting, in the order they joined.
fn sorted(sort: Option<&SortDescriptor>) -> Vec<Member> {
    let mut members = MEMBERS.to_vec();
    let Some(sort) = sort else {
        return members;
    };
    match sort.column.as_str() {
        Some("role") => members.sort_by_key(|member| member.role),
        _ => members.sort_by_key(|member| member.name),
    }
    if sort.direction == SortDirection::Descending {
        members.reverse();
    }
    members
}

#[component]
pub fn TableAtomDemo() -> impl IntoView {
    // App state: the sorting and the selection. The table shows them, and the user's sorting and selecting
    // writes them.
    let sort = RwSignal::new(Some(SortDescriptor {
        column: Key::from("name"),
        direction: SortDirection::Ascending,
    }));
    let selection = RwSignal::new(Selection::default());
    let members = Memo::new(move |_| sort.with(|sort| sorted(sort.as_ref())));
    // Columns and rows, in the sorted order. The selection checkbox column comes first.
    let table = Memo::new(move |_| {
        Arc::new(TableCollection::build_with(
            TableOptions {
                show_selection_checkboxes: true,
            },
            |t| {
                t.column("name", "Name").row_header().allows_sorting();
                t.column("role", "Role").allows_sorting();
                t.column("city", "City");
                members.with(|members| {
                    for member in members {
                        t.row(member.id, member.name, |r| {
                            r.cell(member.name);
                            r.cell(member.role);
                            r.cell(member.city);
                        });
                    }
                });
            },
        ))
    });

    view! {
        <div class="demo-table-scroll">
            <Table
                table=table
                selection_mode=SelectionMode::Multiple
                // Alan is on leave: his row can neither be selected nor focused.
                disabled_keys=Signal::stored(HashSet::from([Key::from("alan")]))
                disabled_behavior=DisabledBehavior::All
                selection=selection
                set_selection=selection
                sort_descriptor=sort
                set_sort_descriptor=sort
                aria_label="Team"
                classes=["demo-table", "demo-atom-table"]
            >
                <TableHeader/>
                <TableBody>
                    <For each=move || members.get() key=|member| member.id let:member>
                        <TableRow key=member.id classes="demo-atom-table-row">
                            <TableCell column="name">{member.name}</TableCell>
                            <TableCell column="role">{member.role}</TableCell>
                            <TableCell column="city">{member.city}</TableCell>
                        </TableRow>
                    </For>
                </TableBody>
            </Table>
        </div>
        <p class="demo-status">
            {move || sort.with(|sort| match sort {
                Some(sort) => format!("Sorted by {}, {}. ", sort.column, direction(sort.direction)),
                None => "Not sorted. ".to_owned(),
            })}
            "Selected: "{move || selection.with(describe)}"."
        </p>
        <div class="demo-controls">
            // The app changes sorting and selection by writing its state. `None` clears the sorting.
            <Button on_press=move |_| sort.set(None) color=ButtonColor::Secondary>"Clear sorting"</Button>
            <Button on_press=move |_| selection.set(Selection::default()) color=ButtonColor::Secondary>
                "Clear selection"
            </Button>
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

fn direction(direction: SortDirection) -> &'static str {
    match direction {
        SortDirection::Ascending => "ascending",
        SortDirection::Descending => "descending",
    }
}
