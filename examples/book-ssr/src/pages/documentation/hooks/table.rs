use std::collections::HashSet;

use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::pages::documentation::{article::Article, toc::Toc};

#[component]
pub fn PageUseTableHook() -> impl IntoView {
    let (selected_rows, set_selected_rows) = signal::<HashSet<String>>(HashSet::new());
    let (sort_column, set_sort_column) = signal::<Option<String>>(None);
    let (sort_direction, set_sort_direction) = signal(true); // true = ascending

    let data: &[(&str, &str, &str, &str)] = &[
        ("1", "Alice", "alice@example.com", "Admin"),
        ("2", "Bob", "bob@example.com", "User"),
        ("3", "Charlie", "charlie@example.com", "User"),
        ("4", "Diana", "diana@example.com", "Moderator"),
    ];

    view! {
        <Article>
            <h1 id="table" class="anchor">
                "Table Hooks"
                <AnchorLink href="#table" description="Direct link to article header"/>
            </h1>

            <p>"Hooks for creating accessible data tables with selection, sorting, and keyboard navigation."</p>

            <h2 id="demo" class="anchor">
                "Interactive Demo"
                <AnchorLink href="#demo" description="Direct link to demo"/>
            </h2>

            <div style="overflow-x: auto; margin: 1em 0;">
                <table
                    role="grid"
                    aria-label="Users"
                    style="width: 100%; border-collapse: collapse; border: 1px solid #ddd;"
                >
                    <thead>
                        <tr>
                            <th style="padding: 0.75em; text-align: left; border-bottom: 2px solid var(--brand-color); width: 40px;">
                                <input
                                    type="checkbox"
                                    checked=move || selected_rows.get().len() == data.len()
                                    on:change=move |_| {
                                        set_selected_rows.update(|s| {
                                            if s.len() == data.len() {
                                                s.clear();
                                            } else {
                                                for (id, _, _, _) in data {
                                                    s.insert(id.to_string());
                                                }
                                            }
                                        });
                                    }
                                />
                            </th>
                            {["Name", "Email", "Role"].into_iter().map(|col| {
                                let col_key = col.to_lowercase();
                                let col_for_aria = col_key.clone();
                                let col_for_click = col_key.clone();
                                let col_for_check_click = col_key.clone();
                                let col_for_icon = col_key.clone();
                                view! {
                                    <th
                                        role="columnheader"
                                        aria-sort=move || {
                                            if sort_column.get().as_ref() == Some(&col_for_aria) {
                                                if sort_direction.get() { "ascending" } else { "descending" }
                                            } else {
                                                "none"
                                            }
                                        }
                                        on:click=move |_| {
                                            if sort_column.get().as_ref() == Some(&col_for_check_click) {
                                                set_sort_direction.update(|d| *d = !*d);
                                            } else {
                                                set_sort_column.set(Some(col_for_click.clone()));
                                                set_sort_direction.set(true);
                                            }
                                        }
                                        style="padding: 0.75em; text-align: left; border-bottom: 2px solid var(--brand-color); cursor: pointer; user-select: none;"
                                    >
                                        { col }
                                        <span style="margin-left: 0.5em;">
                                            {move || {
                                                if sort_column.get().as_ref() == Some(&col_for_icon) {
                                                    if sort_direction.get() { "▲" } else { "▼" }
                                                } else {
                                                    "⬍"
                                                }
                                            }}
                                        </span>
                                    </th>
                                }
                            }).collect::<Vec<_>>()}
                        </tr>
                    </thead>
                    <tbody>
                        {data.iter().map(|(id, name, email, role)| {
                            let id_owned = id.to_string();
                            let id_for_check = id_owned.clone();
                            let id_for_style = id_owned.clone();
                            let id_for_checkbox = id_owned.clone();
                            let id_for_toggle = id_owned.clone();
                            view! {
                                <tr
                                    role="row"
                                    aria-selected=move || selected_rows.get().contains(&id_for_check)
                                    style=move || format!(
                                        "transition: background 0.15s; {}",
                                        if selected_rows.get().contains(&id_for_style) {
                                            "background: var(--brand-color-light, rgba(230, 105, 86, 0.15));"
                                        } else {
                                            ""
                                        }
                                    )
                                >
                                    <td style="padding: 0.75em; border-bottom: 1px solid #ddd;">
                                        <input
                                            type="checkbox"
                                            checked=move || selected_rows.get().contains(&id_for_checkbox)
                                            on:change=move |_| {
                                                let id_clone = id_for_toggle.clone();
                                                set_selected_rows.update(|s| {
                                                    if s.contains(&id_clone) {
                                                        s.remove(&id_clone);
                                                    } else {
                                                        s.insert(id_clone);
                                                    }
                                                });
                                            }
                                        />
                                    </td>
                                    <td style="padding: 0.75em; border-bottom: 1px solid #ddd;">{ *name }</td>
                                    <td style="padding: 0.75em; border-bottom: 1px solid #ddd;">{ *email }</td>
                                    <td style="padding: 0.75em; border-bottom: 1px solid #ddd;">{ *role }</td>
                                </tr>
                            }
                        }).collect::<Vec<_>>()}
                    </tbody>
                </table>
            </div>

            <p>"Selected: " { move || format!("{:?}", selected_rows.get()) }</p>

            <h2 id="use_table" class="anchor">
                "use_table"
                <AnchorLink href="#use_table" description="Direct link to use_table"/>
            </h2>

            <Code>
                {indoc!(r#"
                    let UseTableReturn { table_attrs, state } = use_table(UseTableInput {
                        label: Some("Users".into()),
                        selection_mode: TableSelectionMode::Multiple,
                        selected_keys: selected_rows.into(),
                        sort_descriptor: None,
                        on_selection_change: Some(Callback::new(move |keys| set_selected_rows.set(keys))),
                        on_sort_change: Some(Callback::new(move |sort| { /* handle sort */ })),
                    });
                "#)}
            </Code>

            <h2 id="related-hooks" class="anchor">
                "Related Hooks"
                <AnchorLink href="#related-hooks" description="Direct link to related hooks"/>
            </h2>

            <h3>"use_table_header"</h3>
            <p>"For the thead element with role=\"rowgroup\"."</p>

            <h3>"use_table_column_header"</h3>
            <p>"For sortable column headers with aria-sort."</p>

            <h3>"use_table_row"</h3>
            <p>"For table rows with selection and navigation."</p>

            <h3>"use_table_cell"</h3>
            <p>"For individual cells with focus management."</p>

            <h3>"use_table_checkbox_cell"</h3>
            <p>"Special cell type for row selection checkboxes."</p>

            <h2 id="keyboard" class="anchor">
                "Keyboard Navigation"
                <AnchorLink href="#keyboard" description="Direct link to keyboard"/>
            </h2>

            <ul>
                <li><strong>"Arrow keys"</strong> " - Navigate between cells"</li>
                <li><strong>"Home/End"</strong> " - Jump to first/last cell in row"</li>
                <li><strong>"Ctrl+Home/End"</strong> " - Jump to first/last cell in table"</li>
                <li><strong>"Space"</strong> " - Toggle row selection"</li>
                <li><strong>"Ctrl+A"</strong> " - Select all rows"</li>
            </ul>

            <h2 id="features" class="anchor">
                "Features"
                <AnchorLink href="#features" description="Direct link to features"/>
            </h2>

            <ul>
                <li>"None/single/multiple selection modes"</li>
                <li>"Sortable columns with aria-sort"</li>
                <li>"Keyboard navigation (grid pattern)"</li>
                <li>"Row selection checkboxes"</li>
                <li>"Header rowgroup semantics"</li>
                <li>"Proper ARIA grid attributes"</li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Table Hooks", link: "#table" },
                Toc::Leaf { title: "Demo", link: "#demo" },
                Toc::Leaf { title: "use_table", link: "#use_table" },
                Toc::Leaf { title: "Related Hooks", link: "#related-hooks" },
                Toc::Leaf { title: "Keyboard Navigation", link: "#keyboard" },
                Toc::Leaf { title: "Features", link: "#features" },
            ]
        }/>
    }
}
