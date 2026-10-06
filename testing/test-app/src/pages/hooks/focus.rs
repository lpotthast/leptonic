use leptonic::hooks::{IntoAttrs, UseFocusInput, use_focus};
use leptos::prelude::*;

#[component]
pub fn PageHookFocus() -> impl IntoView {
    let (focus_count, set_focus_count) = signal(0u32);
    let (blur_count, set_blur_count) = signal(0u32);
    let (is_focused, set_is_focused) = signal(false);
    let (focus_change_count, set_focus_change_count) = signal(0u32);

    let focus = use_focus(UseFocusInput {
        is_disabled: Signal::derive(|| false),
        on_focus: Some(Callback::new(move |_| {
            set_focus_count.update(|c| *c += 1);
        })),
        on_blur: Some(Callback::new(move |_| {
            set_blur_count.update(|c| *c += 1);
        })),
        on_focus_change: Some(Callback::new(move |focused| {
            set_is_focused.set(focused);
            set_focus_change_count.update(|c| *c += 1);
        })),
    });

    let (disabled_focus_count, set_disabled_focus_count) = signal(0u32);

    let disabled_focus = use_focus(UseFocusInput {
        is_disabled: Signal::derive(|| true),
        on_focus: Some(Callback::new(move |_| {
            set_disabled_focus_count.update(|c| *c += 1);
        })),
        on_blur: None,
        on_focus_change: None,
    });

    view! {
        <div id="test-page-hook-focus">
            <h1>"Focus Hook Test Page"</h1>

            <section>
                <h2>"Basic Focus"</h2>
                <button id="test-focus-before">"Before"</button>

                <div id="test-focus-target" tabindex="0" {..focus.props.into_attrs()}>
                    "Focus Target"
                </div>

                <div>"Focus count: " <span id="test-focus-count">{focus_count}</span></div>
                <div>"Blur count: " <span id="test-blur-count">{blur_count}</span></div>
                <div>
                    "Is focused: "
                    <span id="test-is-focused">
                        {move || if is_focused.get() { "true" } else { "false" }}
                    </span>
                </div>
                <div>
                    "Focus change count: "
                    <span id="test-focus-change-count">{focus_change_count}</span>
                </div>
            </section>

            <section>
                <h2>"Disabled Focus"</h2>
                <div id="test-focus-disabled" tabindex="0" {..disabled_focus.props.into_attrs()}>
                    "Disabled Focus Target"
                </div>
                <div>
                    "Disabled focus count: "
                    <span id="test-disabled-focus-count">{disabled_focus_count}</span>
                </div>
            </section>

            <button id="test-focus-elsewhere">"Elsewhere"</button>

            <section>
                <h2>"Child Focus Filtering"</h2>
                {
                    let (parent_focus_count, set_parent_focus_count) = signal(0u32);
                    let (parent_blur_count, set_parent_blur_count) = signal(0u32);
                    let parent_focus = use_focus(UseFocusInput {
                        is_disabled: Signal::derive(|| false),
                        on_focus: Some(
                            Callback::new(move |_| {
                                set_parent_focus_count.update(|c| *c += 1);
                            }),
                        ),
                        on_blur: Some(
                            Callback::new(move |_| {
                                set_parent_blur_count.update(|c| *c += 1);
                            }),
                        ),
                        on_focus_change: None,
                    });

                    view! {
                        <div
                            id="test-focus-parent"
                            tabindex="0"
                            {..parent_focus.props.into_attrs()}
                        >
                            "Parent"
                            <button id="test-focus-child">"Child"</button>
                        </div>
                        <div>
                            "Parent focus count: "
                            <span id="test-focus-parent-focus-count">{parent_focus_count}</span>
                        </div>
                        <div>
                            "Parent blur count: "
                            <span id="test-focus-parent-blur-count">{parent_blur_count}</span>
                        </div>
                    }
                }
            </section>
        </div>
    }
}
