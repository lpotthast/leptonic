use leptonic::hooks::*;
use leptos::prelude::*;

#[component]
pub fn ToolbarHorizontalDemo() -> impl IntoView {
    let (focused_idx, set_focused_idx) = signal(0usize);
    let (bold, set_bold) = signal(false);
    let (italic, set_italic) = signal(false);
    let (underline, set_underline) = signal(false);

    let toolbar = use_toolbar(UseToolbarInput {
        label: Some("Text Formatting".to_string()),
        orientation: ToolbarOrientation::Horizontal,
        on_focus_next: Some(Callback::new(move |()| {
            set_focused_idx.update(|i| *i = (*i + 1).min(2));
        })),
        on_focus_previous: Some(Callback::new(move |()| {
            set_focused_idx.update(|i| *i = i.saturating_sub(1));
        })),
        on_focus_first: Some(Callback::new(move |()| set_focused_idx.set(0))),
        on_focus_last: Some(Callback::new(move |()| set_focused_idx.set(2))),
        ..Default::default()
    });

    view! {
        <div
            {..toolbar.toolbar_props.into_attrs()}
            style="display: flex; gap: 0.5em; padding: 0.5em; background: #f5f5f5; border-radius: 4px;"
        >
            <button
                on:click=move |_| set_bold.update(|b| *b = !*b)
                style=move || format!(
                    "padding: 0.5em 1em; border: 1px solid #ccc; border-radius: 4px; cursor: pointer; font-weight: bold; {}",
                    if bold.get() { "background: #1976d2; color: white;" } else { "background: white;" }
                )
                style:outline=move || if focused_idx.get() == 0 { "2px solid var(--brand-color)" } else { "none" }
                tabindex=move || if focused_idx.get() == 0 { "0" } else { "-1" }
            >
                "B"
            </button>
            <button
                on:click=move |_| set_italic.update(|i| *i = !*i)
                style=move || format!(
                    "padding: 0.5em 1em; border: 1px solid #ccc; border-radius: 4px; cursor: pointer; font-style: italic; {}",
                    if italic.get() { "background: #1976d2; color: white;" } else { "background: white;" }
                )
                style:outline=move || if focused_idx.get() == 1 { "2px solid var(--brand-color)" } else { "none" }
                tabindex=move || if focused_idx.get() == 1 { "0" } else { "-1" }
            >
                "I"
            </button>
            <button
                on:click=move |_| set_underline.update(|u| *u = !*u)
                style=move || format!(
                    "padding: 0.5em 1em; border: 1px solid #ccc; border-radius: 4px; cursor: pointer; text-decoration: underline; {}",
                    if underline.get() { "background: #1976d2; color: white;" } else { "background: white;" }
                )
                style:outline=move || if focused_idx.get() == 2 { "2px solid var(--brand-color)" } else { "none" }
                tabindex=move || if focused_idx.get() == 2 { "0" } else { "-1" }
            >
                "U"
            </button>
        </div>

        <p>
            "Preview: "
            <span
                style:font-weight=move || if bold.get() { "bold" } else { "normal" }
                style:font-style=move || if italic.get() { "italic" } else { "normal" }
                style:text-decoration=move || if underline.get() { "underline" } else { "none" }
            >
                "Sample formatted text"
            </span>
        </p>
    }
}
