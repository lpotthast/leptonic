use leptonic::atoms::focus_scope::FocusScope;
use leptos::prelude::*;

#[component]
pub fn PageAtomFocusScope() -> impl IntoView {
    // Signal to toggle the restore-focus scope visibility.
    let (show_restore, set_show_restore) = signal(false);
    let (show_nested_restore, set_show_nested_restore) = signal(false);
    // Restore fallback: the inner scope and its node to restore go away together.
    let fallback = RwSignal::new(false);
    let fallback_empty = RwSignal::new(false);
    // A dialog opened from a menu, rendered outside the menu's scope.
    let show_menu = RwSignal::new(false);
    let show_dialog = RwSignal::new(false);

    view! {
        <div id="test-page-atom-focus-scope">
            <h1>"FocusScope Atom Test Page"</h1>

            // ---- Section 1: Basic containment ----
            <section>
                <h2>"Containment"</h2>
                <FocusScope contain=true>
                    <div id="test-fs-contain-scope">
                        <button id="test-fs-contain-btn-1">"Button 1"</button>
                        <button id="test-fs-contain-btn-2">"Button 2"</button>
                        <button id="test-fs-contain-btn-3">"Button 3"</button>
                    </div>
                </FocusScope>
            </section>

            // ---- Section 2: Auto-focus ----
            <section>
                <h2>"Auto Focus"</h2>
                <FocusScope auto_focus=true>
                    <div id="test-fs-autofocus-scope">
                        <button id="test-fs-autofocus-btn-1">"AF Button 1"</button>
                        <button id="test-fs-autofocus-btn-2">"AF Button 2"</button>
                    </div>
                </FocusScope>
            </section>

            // ---- Section 3: Restore focus ----
            <section>
                <h2>"Restore Focus"</h2>
                <button
                    id="test-fs-restore-toggle"
                    on:click=move |_| set_show_restore.update(|v| *v = !*v)
                >
                    "Toggle Scope"
                </button>
                <Show when=move || show_restore.get()>
                    <FocusScope auto_focus=true restore_focus=true>
                        <div id="test-fs-restore-scope">
                            <button id="test-fs-restore-btn">"Restore Button"</button>
                        </div>
                    </FocusScope>
                </Show>
            </section>

            // ---- Section 4: Nested scopes ----
            <section>
                <h2>"Nested Scopes"</h2>
                <FocusScope contain=true>
                    <div id="test-fs-nested-outer">
                        <button id="test-fs-nested-outer-btn">"Outer Button"</button>
                        <FocusScope contain=true>
                            <div id="test-fs-nested-inner">
                                <button id="test-fs-nested-inner-btn-1">"Inner 1"</button>
                                <button id="test-fs-nested-inner-btn-2">"Inner 2"</button>
                            </div>
                        </FocusScope>
                    </div>
                </FocusScope>
            </section>

            // ---- Section 5: Nested restore ----
            <section>
                <h2>"Nested Restore"</h2>
                <button
                    id="test-fs-nested-restore-trigger"
                    on:click=move |_| set_show_nested_restore.update(|v| *v = !*v)
                >
                    "Toggle Nested Restore"
                </button>
                <Show when=move || show_nested_restore.get()>
                    <FocusScope restore_focus=true auto_focus=true>
                        <div id="test-fs-nested-restore-outer">
                            <button id="test-fs-nested-restore-outer-btn">"Outer Btn"</button>
                            <FocusScope restore_focus=true auto_focus=true>
                                <div id="test-fs-nested-restore-inner">
                                    <button id="test-fs-nested-restore-inner-btn">
                                        "Inner Btn"
                                    </button>
                                </div>
                            </FocusScope>
                        </div>
                    </FocusScope>
                </Show>
            </section>

            // ---- Restore fallback (FocusScope.test.js "does not throw when there is no focusable
            // element to restore focus to", with and without another element) ----
            <section>
                <h2>"Restore fallback"</h2>
                <FallbackScopes open=fallback with_other=true prefix="test-fs-fallback" />
                <FallbackScopes open=fallback_empty with_other=false prefix="test-fs-fallback-empty" />
            </section>

            // ---- FocusScope.test.js "tracks node to restore if the node to restore was removed in
            // another part of the tree" (keydown handlers, as upstream: keyboard modality) ----
            <section>
                <h2>"Dialog from a menu"</h2>
                <FocusScope>
                    <button id="test-fs-open-menu" on:keydown=move |_| show_menu.set(true)>
                        "Open Menu"
                    </button>
                    <Show when=move || show_menu.get()>
                        <FocusScope contain=true restore_focus=true auto_focus=true>
                            <button
                                id="test-fs-open-dialog"
                                on:keydown=move |_| {
                                    show_menu.set(false);
                                    show_dialog.set(true);
                                }
                            >
                                "Open Dialog"
                            </button>
                        </FocusScope>
                    </Show>
                </FocusScope>
                <Show when=move || show_dialog.get()>
                    <FocusScope contain=true restore_focus=true auto_focus=true>
                        <button id="test-fs-close-dialog" on:keydown=move |_| show_dialog.set(false)>
                            "Close"
                        </button>
                    </FocusScope>
                </Show>
            </section>

            // ---- Button outside all scopes (for containment tests) ----
            <button id="test-fs-outside">"Outside"</button>
        </div>
    }
}

/// An outer scope with a restore target (and optionally another button); its button opens an
/// inner scope (restoring focus, auto-focused), whose button closes it and removes the target.
#[component]
fn FallbackScopes(open: RwSignal<bool>, with_other: bool, prefix: &'static str) -> impl IntoView {
    let target_shown = RwSignal::new(true);
    view! {
        <FocusScope>
            <div>
                <Show when=move || target_shown.get()>
                    <button id=format!("{prefix}-target") on:click=move |_| open.set(true)>
                        "Restore target"
                    </button>
                </Show>
            </div>
            {with_other.then(|| view! { <button id=format!("{prefix}-other")>"Other"</button> })}
            <Show when=move || open.get()>
                <FocusScope restore_focus=true auto_focus=true>
                    <button
                        id=format!("{prefix}-inside")
                        on:click=move |_| {
                            open.set(false);
                            target_shown.set(false);
                        }
                    >
                        "Close and remove the target"
                    </button>
                </FocusScope>
            </Show>
        </FocusScope>
    }
}
