use leptonic::{
    atoms::{button::Button, checkbox::{CheckboxButton, CheckboxField}, virtualizer::VirtualList},
    hooks::{collections::Key, virtualizer::ListLayoutOptions},
};
use leptos::prelude::*;

/// A line of the build log.
#[derive(Clone)]
struct LogLine {
    number: usize,
    text: String,
}

/// The made-up line `number` of a build log; every seventh one is a long warning that wraps.
fn log_line(number: usize) -> LogLine {
    let text = if number.is_multiple_of(7) {
        format!(
            "warning: unused variable `step_{number}` in build script; if this is intentional, prefix it \
             with an underscore: `_step_{number}`"
        )
    } else {
        format!("Compiling module {number} of the release build")
    };
    LogLine { number, text }
}

#[component]
pub fn VirtualListDemo() -> impl IntoView {
    let lines = RwSignal::new((1..=1_000).map(log_line).collect::<Vec<_>>());
    // Whether the end of the log stays in view while lines are added.
    let follow = RwSignal::new(true);

    let add_lines = move |_| {
        lines.update(|lines| {
            let next = lines.len() + 1;
            lines.extend((next..next + 20).map(log_line));
        });
    };

    view! {
        // Rows are about 24px high, long ones wrap: each row is measured once it is rendered.
        <VirtualList
            items=lines
            key=|line: &LogLine| Key::from(line.number)
            layout_options=ListLayoutOptions {
                estimated_row_size: Some(24.0),
                scroll_end_threshold: 8.0,
                ..ListLayoutOptions::default()
            }
            is_anchored_to_end=follow
            set_anchored_to_end=follow
            is_focusable=true
            classes="demo-virt-log"
            attr:role="region"
            attr:aria-label="Build log"
            let:line
        >
            <div class="demo-virt-log-line">{line.text}</div>
        </VirtualList>

        <div class="demo-controls">
            <Button on_press=add_lines classes="demo-btn">"Add 20 lines"</Button>
            <CheckboxField is_selected=follow set_selected=follow>
                <CheckboxButton classes="demo-check">
                    <span class="demo-check-box" aria-hidden="true"></span>
                    "Follow the end"
                </CheckboxButton>
            </CheckboxField>
        </div>
        <p class="demo-status">
            {move || {
                let count = lines.with(Vec::len);
                let state = if follow.get() { "following the end" } else { "not following" };
                format!("{count} lines, {state}.")
            }}
        </p>
    }
}
