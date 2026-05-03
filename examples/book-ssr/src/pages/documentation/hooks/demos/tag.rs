use leptonic::hooks::*;
use leptos::prelude::*;

#[component]
pub fn TagDemo() -> impl IntoView {
    let (tags, set_tags) = signal(vec![
        "Rust".to_string(),
        "Leptos".to_string(),
        "WASM".to_string(),
        "Frontend".to_string(),
    ]);
    let (focused_idx, set_focused_idx) = signal(0usize);

    let UseTagGroupReturn {
        group_props,
        label_props,
        ..
    } = use_tag_group(UseTagGroupInput {
        label: Some("Technologies".to_string()),
        selection_mode: TagGroupSelectionMode::None,
        allow_removal: true,
        on_remove: Some(Callback::new(move |key: String| {
            set_tags.update(|t| t.retain(|k| k != &key));
        })),
        ..Default::default()
    });

    view! {
        <label id={label_props.id} style="display: block; margin-bottom: 0.5em; font-weight: 500;">
            "Technologies"
        </label>
        <div {..group_props.into_attrs()} style="display: flex; flex-wrap: wrap; gap: 0.5em;">
            <For
                each=move || tags.get().into_iter().enumerate()
                key=|(_, tag)| tag.clone()
                children=move |(idx, tag)| {
                    let tag_key = tag.clone();
                    let is_focused = Signal::derive(move || focused_idx.get() == idx);
                    let tag_for_remove = tag.clone();

                    let tag_hook = use_tag(UseTagInput {
                        tag_key: tag.clone(),
                        is_selected: Signal::derive(|| false),
                        is_focused,
                        is_disabled: Signal::derive(|| false),
                        allow_removal: true,
                        on_select: None,
                        on_remove: Some(Callback::new(move |()| {
                            set_tags.update(|t| t.retain(|k| k != &tag_for_remove));
                        })),
                        on_focus_next: Some(Callback::new(move |()| {
                            set_focused_idx.update(|i| {
                                let len = tags.get_untracked().len();
                                *i = (*i + 1).min(len.saturating_sub(1));
                            });
                        })),
                        on_focus_previous: Some(Callback::new(move |()| {
                            set_focused_idx.update(|i| *i = i.saturating_sub(1));
                        })),
                    });

                    view! {
                        <div
                            {..tag_hook.row_props.into_attrs()}
                            style="display: inline-flex; align-items: center; gap: 0.25em; padding: 0.25em 0.5em; background: #e3f2fd; border-radius: 16px; cursor: pointer; outline: none;"
                            style:box-shadow=move || if is_focused.get() { "0 0 0 2px var(--brand-color)" } else { "none" }
                        >
                            <span {..tag_hook.cell_props.into_attrs()}>{ tag_key }</span>
                            <button
                                {..tag_hook.remove_button_props.into_attrs()}
                                style="border: none; background: none; cursor: pointer; padding: 0; width: 16px; height: 16px; border-radius: 50%; display: flex; align-items: center; justify-content: center;"
                            >
                                {"\u{00d7}"}
                            </button>
                        </div>
                    }
                }
            />
        </div>

        <p style="margin-top: 1em; font-size: 0.875em; opacity: 0.7;">
            "Use arrow keys to navigate, Delete/Backspace to remove"
        </p>
    }
}
