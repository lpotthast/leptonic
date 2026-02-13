use indoc::indoc;
use leptonic::{components::prelude::*, hooks::*};
use leptos::prelude::*;

use crate::pages::documentation::{article::Article, doc_styles::*, toc::Toc};

#[component]
pub fn PageUseFocusVisible() -> impl IntoView {
    let UseFocusVisibleReturn {
        focus_should_be_visible,
        modality,
    } = use_focus_visible(UseFocusVisibleInput::default());

    let modality_display = Memo::new(move |_| match modality.get() {
        Modality::Unknown => "Unknown",
        Modality::Pointer => "Pointer",
        Modality::Keyboard => "Keyboard",
        Modality::Virtual => "Virtual",
    });

    view! {
        <Article>
            <h1 id="use_focus_visible" class="anchor">
                "use_focus_visible"
                <AnchorLink href="#use_focus_visible" description="Direct link to article header"/>
            </h1>

            <p>"Tracks whether focus should be made visible (e.g., with a focus ring). When the user navigates via keyboard or assistive technology, focus is visible. When they use a pointer (mouse, touch), focus is hidden."</p>

            <h2 id="basic-usage" class="anchor">
                "Basic Usage"
                <AnchorLink href="#basic-usage" description="Direct link to basic usage"/>
            </h2>

            <Code>
                {indoc!(r"
                    let UseFocusVisibleReturn { focus_should_be_visible, modality } =
                        use_focus_visible(UseFocusVisibleInput::default());

                    // focus_should_be_visible: Signal<bool> — true during keyboard/virtual modality.
                    // modality: Signal<Modality> — tracks the current input modality.
                ")}
            </Code>

            <p>"Try tabbing to the button below (keyboard), then clicking it (mouse). The button shows a focus outline only during keyboard navigation:"</p>

            <style>
                {format!(
                    ".focus-visible-demo:focus {{ {} }}",
                    FOCUS_OUTLINE_CSS
                )}
            </style>

            <button
                tabindex=0
                class="focus-visible-demo"
                style=move || format!(
                    "padding: 1em 2em; font-size: 1em; border-radius: 8px; cursor: pointer; transition: all 0.2s; {}",
                    if focus_should_be_visible.get() {
                        "border: 2px solid var(--brand-color); background: var(--brand-color-light, rgba(230, 105, 86, 0.1));"
                    } else {
                        "border: 2px solid #ccc; background: white;"
                    }
                )
            >
                "Interact with me"
            </button>

            <p style=margin_top_1em()>
                "Focus should be visible: "
                <strong style=move || if focus_should_be_visible.get() { state_active() } else { state_inactive() }>
                    { move || focus_should_be_visible.get().to_string() }
                </strong>
            </p>

            <p>
                "Current modality: "
                <strong>{ move || modality_display.get() }</strong>
            </p>

            <h2 id="modality" class="anchor">
                "Modality Detection"
                <AnchorLink href="#modality" description="Direct link to modality"/>
            </h2>

            <p>"The hook tracks the global interaction modality:"</p>

            <ul>
                <li><strong>"Keyboard"</strong> " — Navigation keys (Tab, Escape, arrows, Enter, Space) were pressed."</li>
                <li><strong>"Pointer"</strong> " — Mouse or touch interaction occurred."</li>
                <li><strong>"Virtual"</strong> " — Screen reader or assistive technology interaction detected."</li>
                <li><strong>"Unknown"</strong> " — Initial state before any interaction."</li>
            </ul>

            <p>"Both Keyboard and Virtual modalities make focus visible, while Pointer hides it."</p>

            <h2 id="input-options" class="anchor">
                "Input Options"
                <AnchorLink href="#input-options" description="Direct link to input options"/>
            </h2>

            <h3>"enabled"</h3>

            <p><code>"enabled: Signal<bool>"</code> " controls whether the hook subscribes to global modality changes. When false, the hook does not update its signals, saving unnecessary reactivity. This is useful when a component is hidden or inactive:"</p>

            <Code>
                {indoc!(r"
                    let focus_visible = use_focus_visible(UseFocusVisibleInput {
                        enabled: Signal::derive(move || is_panel_visible.get()),
                        ..Default::default()
                    });
                ")}
            </Code>

            <h3>"is_text_input"</h3>

            <p><code>"is_text_input: bool"</code> " changes the keyboard filtering logic. When true, only Tab and Escape trigger focus-visible; other keys (arrows, letters, etc.) do not. This is intended for compound components like date pickers where focus sits on a button but the component should use text-input focus rules:"</p>

            <Code>
                {indoc!(r"
                    let focus_visible = use_focus_visible(UseFocusVisibleInput {
                        is_text_input: true,
                        ..Default::default()
                    });
                    // Now typing regular characters won't make focus visible,
                    // only Tab or Escape will.
                ")}
            </Code>

            <h3>"auto_focus"</h3>

            <p><code>"auto_focus: bool"</code> " sets the initial value of " <code>"focus_should_be_visible"</code> " during SSR and before any user interaction. If the element auto-focuses on mount, set this to " <code>"true"</code> " so the focus ring appears immediately:"</p>

            <Code>
                {indoc!(r"
                    let focus_visible = use_focus_visible(UseFocusVisibleInput {
                        auto_focus: true,
                        ..Default::default()
                    });
                    // focus_should_be_visible starts as true.
                ")}
            </Code>

            <h2 id="utility-functions" class="anchor">
                "Utility Functions"
                <AnchorLink href="#utility-functions" description="Direct link to utility functions"/>
            </h2>

            <p>"Two companion functions are available for non-reactive modality access:"</p>

            <Code>
                {indoc!(r"
                    // Read the current modality (non-reactive).
                    let modality: Modality = get_modality();

                    // Programmatically set the modality (e.g., for testing).
                    // Note: will be overwritten on next user interaction.
                    set_modality(Modality::Keyboard);
                ")}
            </Code>

            <h2 id="input" class="anchor">
                "Input"
                <AnchorLink href="#input" description="Direct link to input"/>
            </h2>

            <p><code>"UseFocusVisibleInput"</code> " fields:"</p>

            <TableContainer>
                <Table bordered=true hoverable=true>
                    <TableHeader>
                        <TableRow>
                            <TableHeaderCell min_width=true>"Field"</TableHeaderCell>
                            <TableHeaderCell min_width=true>"Type"</TableHeaderCell>
                            <TableHeaderCell min_width=true>"Default"</TableHeaderCell>
                            <TableHeaderCell>"Description"</TableHeaderCell>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        <TableRow>
                            <TableCell><code>"auto_focus"</code></TableCell>
                            <TableCell><code>"bool"</code></TableCell>
                            <TableCell><code>"false"</code></TableCell>
                            <TableCell>"Initial value of " <code>"focus_should_be_visible"</code> " during SSR."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"enabled"</code></TableCell>
                            <TableCell><code>"Signal<bool>"</code></TableCell>
                            <TableCell><code>"true"</code></TableCell>
                            <TableCell>"Whether the hook subscribes to modality changes."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"is_text_input"</code></TableCell>
                            <TableCell><code>"bool"</code></TableCell>
                            <TableCell><code>"false"</code></TableCell>
                            <TableCell>"When true, only Tab/Escape trigger focus-visible."</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h2 id="return-value" class="anchor">
                "Return Value"
                <AnchorLink href="#return-value" description="Direct link to return value"/>
            </h2>

            <p><code>"UseFocusVisibleReturn"</code> " fields:"</p>

            <TableContainer>
                <Table bordered=true hoverable=true>
                    <TableHeader>
                        <TableRow>
                            <TableHeaderCell min_width=true>"Field"</TableHeaderCell>
                            <TableHeaderCell min_width=true>"Type"</TableHeaderCell>
                            <TableHeaderCell>"Description"</TableHeaderCell>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        <TableRow>
                            <TableCell><code>"focus_should_be_visible"</code></TableCell>
                            <TableCell><code>"Signal<bool>"</code></TableCell>
                            <TableCell>"True when focus should be visually indicated (keyboard/virtual modality)."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"modality"</code></TableCell>
                            <TableCell><code>"Signal<Modality>"</code></TableCell>
                            <TableCell>"The current interaction modality (Keyboard, Pointer, Virtual, or Unknown)."</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h2 id="features" class="anchor">
                "Features"
                <AnchorLink href="#features" description="Direct link to features"/>
            </h2>

            <ul>
                <li>"Reactive " <code>"Signal<bool>"</code> " for focus visibility."</li>
                <li>"Global modality tracking (keyboard, pointer, virtual)."</li>
                <li>"Virtual click detection for screen readers."</li>
                <li>"SSR-safe (returns " <code>"auto_focus"</code> " value during SSR)."</li>
                <li>"Subscription control via " <code>"enabled"</code> " signal."</li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "use_focus_visible", link: "#use_focus_visible" },
                Toc::Leaf { title: "Basic Usage", link: "#basic-usage" },
                Toc::Leaf { title: "Modality Detection", link: "#modality" },
                Toc::Leaf { title: "Input Options", link: "#input-options" },
                Toc::Leaf { title: "Utility Functions", link: "#utility-functions" },
                Toc::Leaf { title: "Input", link: "#input" },
                Toc::Leaf { title: "Return Value", link: "#return-value" },
                Toc::Leaf { title: "Features", link: "#features" },
            ]
        }/>
    }
}
