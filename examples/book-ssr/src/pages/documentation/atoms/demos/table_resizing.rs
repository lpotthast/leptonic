use std::{collections::HashMap, sync::Arc};

use leptonic::{
    atoms::table::{ResizableTableContainer, Table, TableBody, TableCell, TableHeader, TableRow},
    hooks::{
        collections::Key,
        table::{ColumnBound, ColumnSize, TableCollection},
    },
};
use leptos::prelude::*;

#[derive(Debug, Clone, Copy)]
struct Task {
    id: &'static str,
    title: &'static str,
    assignee: &'static str,
    status: &'static str,
    due: &'static str,
}

const TASKS: [Task; 4] = [
    Task {
        id: "1",
        title: "Migrate the build to the new CI runners",
        assignee: "Ada Lovelace",
        status: "In progress",
        due: "Oct 9",
    },
    Task {
        id: "2",
        title: "Review the accessibility audit",
        assignee: "Grace Hopper",
        status: "Open",
        due: "Oct 14",
    },
    Task {
        id: "3",
        title: "Update dependencies",
        assignee: "Linus Torvalds",
        status: "Done",
        due: "Oct 2",
    },
    Task {
        id: "4",
        title: "Write the release notes",
        assignee: "Katherine Johnson",
        status: "Blocked",
        due: "Oct 20",
    },
];

const COLUMNS: [(&str, &str); 4] = [
    ("title", "Task"),
    ("assignee", "Assignee"),
    ("status", "Status"),
    ("due", "Due"),
];

/// The reported column sizes, in column order.
fn describe(sizes: &HashMap<Key, ColumnSize>) -> String {
    COLUMNS
        .map(|(key, text)| match sizes.get(&Key::from(key)) {
            Some(ColumnSize::Px(px)) => format!("{text} {px}px"),
            Some(ColumnSize::Percent(percent)) => format!("{text} {percent}%"),
            Some(ColumnSize::Fr(fr)) => format!("{text} {fr}fr"),
            None => format!("{text} \u{2014}"),
        })
        .join(" \u{b7} ")
}

#[component]
pub fn TableResizingAtomDemo() -> impl IntoView {
    let table = Memo::new(|_| {
        Arc::new(TableCollection::build(|t| {
            // Task and Assignee share the space the other columns leave, 2:1. Resizing Task makes Assignee take what
            // is left.
            t.column("title", "Task")
                .row_header()
                .allows_resizing()
                .default_width(ColumnSize::Fr(2.0))
                .min_width(ColumnBound::Px(160.0));
            t.column("assignee", "Assignee")
                .allows_resizing()
                .min_width(ColumnBound::Px(120.0));
            // Between 90 and 200 pixels.
            t.column("status", "Status")
                .allows_resizing()
                .default_width(ColumnSize::Px(120.0))
                .min_width(ColumnBound::Px(90.0))
                .max_width(ColumnBound::Px(200.0));
            // Not resizable.
            t.column("due", "Due").default_width(ColumnSize::Px(90.0));
            for task in TASKS {
                t.row(task.id, task.title, |r| {
                    r.cell(task.title);
                    r.cell(task.assignee);
                    r.cell(task.status);
                    r.cell(task.due);
                });
            }
        }))
    });
    let resizing = RwSignal::new(false);
    let sizes = RwSignal::new(String::from("\u{2014}"));

    view! {
        <ResizableTableContainer
            classes="demo-table-scroll"
            on_resize_start=Callback::new(move |_| resizing.set(true))
            on_resize=Callback::new(move |new_sizes: HashMap<Key, ColumnSize>| sizes.set(describe(&new_sizes)))
            on_resize_end=Callback::new(move |new_sizes: HashMap<Key, ColumnSize>| {
                resizing.set(false);
                sizes.set(describe(&new_sizes));
            })
        >
            <Table table=table aria_label="Tasks" classes=["demo-table", "demo-resizable-table"]>
                <TableHeader/>
                <TableBody>
                    {TASKS
                        .map(|task| {
                            view! {
                                <TableRow key=task.id>
                                    <TableCell column="title">{task.title}</TableCell>
                                    <TableCell column="assignee">{task.assignee}</TableCell>
                                    <TableCell column="status">{task.status}</TableCell>
                                    <TableCell column="due">{task.due}</TableCell>
                                </TableRow>
                            }
                        })
                        .collect_view()}
                </TableBody>
            </Table>
        </ResizableTableContainer>
        <p class="demo-status">
            "Resizing: "{move || if resizing.get() { "yes" } else { "no" }}". Reported sizes: "{sizes}"."
        </p>
    }
}
