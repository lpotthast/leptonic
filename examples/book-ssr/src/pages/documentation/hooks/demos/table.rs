use std::collections::HashSet;

use leptos::prelude::*;

#[component]
pub fn TableDemo() -> impl IntoView {
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
        <div style="overflow-x: auto;">
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
                                                if sort_direction.get() { "\u{25b2}" } else { "\u{25bc}" }
                                            } else {
                                                "\u{2b0d}"
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
    }
}
