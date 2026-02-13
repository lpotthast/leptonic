use indoc::indoc;
use leptonic::{components::prelude::*, hooks::*};
use leptos::prelude::*;

use crate::pages::documentation::{article::Article, doc_styles::*, toc::Toc};

#[component]
pub fn PageUseFocusRing() -> impl IntoView {
    let (disabled, set_disabled) = signal(false);
    let (focus_count, set_focus_count) = signal(0);
    let (blur_count, set_blur_count) = signal(0);

    let focus_ring = use_focus_ring(UseFocusRingInput::default());

    let focus_ring_custom = use_focus_ring(UseFocusRingInput {
        disabled: disabled.into(),
        on_focus: Some(Callback::new(move |_| {
            set_focus_count.update(|c| *c += 1);
        })),
        on_blur: Some(Callback::new(move |_| {
            set_blur_count.update(|c| *c += 1);
        })),
        ..Default::default()
    });

    let focus_ring_within = use_focus_ring(UseFocusRingInput {
        within: true,
        ..Default::default()
    });

    view! {
        <Article>
            <h1 id="use_focus_ring" class="anchor">
                "use_focus_ring"
                <AnchorLink href="#use_focus_ring" description="Direct link to article header"/>
            </h1>

            <p>"Determine when to show a focus ring for accessibility. The focus ring should only be visible during keyboard navigation, not when using mouse or touch."</p>

            <h2 id="basic-usage" class="anchor">
                "Basic Usage"
                <AnchorLink href="#basic-usage" description="Direct link to basic usage"/>
            </h2>

            <Code>
                {indoc!(r#"
                    let focus_ring = use_focus_ring(UseFocusRingInput::default());

                    view! {
                        <button
                            {..focus_ring.props.into_attrs()}
                            tabindex="0"
                        >
                            "Focus me"
                        </button>
                    }

                    // CSS handles styling automatically:
                    // [data-focus-visible="true"] {
                    //     outline: 3px solid var(--brand-color);
                    //     outline-offset: 2px;
                    // }
                "#)}
            </Code>

            <h2 id="keyboard-vs-mouse" class="anchor">
                "Keyboard vs Mouse Focus"
                <AnchorLink href="#keyboard-vs-mouse" description="Direct link to keyboard vs mouse"/>
            </h2>

            <p>"Try both clicking and tabbing to the button below. The focus ring only appears when using keyboard navigation. The " <code>"data-focus-visible"</code> " attribute is automatically added when the focus ring should be visible, and CSS handles the styling:"</p>

            <button
                {..focus_ring.props.into_attrs()}
                tabindex="0"
                style=demo_button()
            >
                "Tab to me (keyboard) or click me (mouse)"
            </button>

            <p style=margin_top_1em()>
                "Is focused: " <strong>{ move || focus_ring.is_focused.get().to_string() }</strong>
                " | Focus ring visible: " <strong>{ move || focus_ring.is_focus_visible.get().to_string() }</strong>
            </p>

            <h2 id="within-mode" class="anchor">
                "Within Mode"
                <AnchorLink href="#within-mode" description="Direct link to within mode"/>
            </h2>

            <p>"With " <code>"within: true"</code> ", the focus ring tracks focus within the element's subtree using " <code>"focusin"</code> "/" <code>"focusout"</code> " events. The " <code>"data-focus-visible"</code> " attribute is set on the container when any descendant is focused via keyboard:"</p>

            <Code>
                {indoc!(r#"
                    let focus_ring_within = use_focus_ring(UseFocusRingInput {
                        within: true,
                        ..Default::default()
                    });

                    view! {
                        <div {..focus_ring_within.props.into_attrs()}>
                            <input type="text" placeholder="Tab here..." />
                            <button>"Or here"</button>
                        </div>
                    }
                "#)}
            </Code>

            <div
                {..focus_ring_within.props.into_attrs()}
                style="padding: 1em; border-radius: 8px; border: 2px solid #ccc; display: flex; gap: 0.5em; align-items: center; transition: all 0.2s;"
            >
                <input
                    type="text"
                    placeholder="Tab here..."
                    style=demo_input()
                />
                <button style=demo_button()>
                    "Or here"
                </button>
            </div>

            <p style=margin_top_1em()>
                "Focus within: " <strong>{ move || focus_ring_within.is_focused.get().to_string() }</strong>
                " | Focus ring (within) visible: " <strong>{ move || focus_ring_within.is_focus_visible.get().to_string() }</strong>
            </p>

            <h2 id="custom-options" class="anchor">
                "Custom Options"
                <AnchorLink href="#custom-options" description="Direct link to custom options"/>
            </h2>

            <p>"The hook accepts optional callbacks and a disabled signal. The demo below wires up " <code>"on_focus"</code> " and " <code>"on_blur"</code> " callbacks with counters:"</p>

            <Code>
                {indoc!(r#"
                    let focus_ring = use_focus_ring(UseFocusRingInput {
                        disabled: disabled.into(),
                        on_focus: Some(Callback::new(|_| { /* focused */ })),
                        on_blur: Some(Callback::new(|_| { /* blurred */ })),
                        on_focus_change: Some(Callback::new(|focused: bool| { /* changed */ })),
                        ..Default::default()
                    });
                "#)}
            </Code>

            <button
                {..focus_ring_custom.props.into_attrs()}
                tabindex="0"
                style=demo_button()
            >
                "Focus ring with callbacks"
            </button>

            <div style=flex_row_gap()>
                <p>"Focus count: " <strong>{ move || focus_count.get() }</strong></p>
                <p>"Blur count: " <strong>{ move || blur_count.get() }</strong></p>
            </div>

            <FormControl attr:style=form_control_row()>
                <Checkbox checked=disabled set_checked=set_disabled />
                <Label>"Disabled"</Label>
            </FormControl>

            <h2 id="text-input-mode" class="anchor">
                "Text Input Mode"
                <AnchorLink href="#text-input-mode" description="Direct link to text input mode"/>
            </h2>

            <p>"When " <code>"is_text_input: true"</code> ", only Tab and Escape trigger the focus ring. Other keyboard events (arrow keys, letters) do not make focus visible. This matches the behavior of native text inputs where typing shouldn't trigger a focus ring:"</p>

            <Code>
                {indoc!(r"
                    let focus_ring = use_focus_ring(UseFocusRingInput {
                        is_text_input: true,
                        ..Default::default()
                    });
                ")}
            </Code>

            <h2 id="data-attribute" class="anchor">
                "Data Attribute Styling"
                <AnchorLink href="#data-attribute" description="Direct link to data attribute"/>
            </h2>

            <p>"The hook automatically adds a " <code>"data-focus-visible"</code> " attribute when the focus ring should be visible. This allows centralized CSS styling:"</p>

            <Code>
                {indoc!(r#"
                    /* In your CSS */
                    [data-focus-visible="true"] {
                        outline: 3px solid var(--brand-color);
                        outline-offset: 2px;
                    }
                "#)}
            </Code>

            <p>"This approach has several benefits:"</p>

            <ul>
                <li>"No manual style computation in your components"</li>
                <li>"Consistent focus ring styling across your entire application"</li>
                <li>"Easy to customize in one place"</li>
                <li>"The signal " <code>"is_focus_visible"</code> " is still available if you need programmatic access"</li>
            </ul>

            <h2 id="input" class="anchor">
                "Input"
                <AnchorLink href="#input" description="Direct link to input"/>
            </h2>

            <p><code>"UseFocusRingInput"</code> " fields:"</p>

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
                            <TableCell><code>"disabled"</code></TableCell>
                            <TableCell><code>"Signal<bool>"</code></TableCell>
                            <TableCell><code>"false"</code></TableCell>
                            <TableCell>"Whether the focus ring is disabled."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"within"</code></TableCell>
                            <TableCell><code>"bool"</code></TableCell>
                            <TableCell><code>"false"</code></TableCell>
                            <TableCell>"Track focus within descendants instead of the element itself."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"auto_focus"</code></TableCell>
                            <TableCell><code>"bool"</code></TableCell>
                            <TableCell><code>"false"</code></TableCell>
                            <TableCell>"Whether to auto-focus the element."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"is_text_input"</code></TableCell>
                            <TableCell><code>"bool"</code></TableCell>
                            <TableCell><code>"false"</code></TableCell>
                            <TableCell>"Only Tab/Escape trigger focus-visible."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"on_focus"</code></TableCell>
                            <TableCell><code>"Option<Callback<FocusEvent>>"</code></TableCell>
                            <TableCell><code>"None"</code></TableCell>
                            <TableCell>"Handler called on focus."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"on_blur"</code></TableCell>
                            <TableCell><code>"Option<Callback<FocusEvent>>"</code></TableCell>
                            <TableCell><code>"None"</code></TableCell>
                            <TableCell>"Handler called on blur."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"on_focus_change"</code></TableCell>
                            <TableCell><code>"Option<Callback<bool>>"</code></TableCell>
                            <TableCell><code>"None"</code></TableCell>
                            <TableCell>"Handler called on focus state change."</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h2 id="return-value" class="anchor">
                "Return Value"
                <AnchorLink href="#return-value" description="Direct link to return value"/>
            </h2>

            <p><code>"UseFocusRingReturn"</code> " fields:"</p>

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
                            <TableCell><code>"props"</code></TableCell>
                            <TableCell><code>"UseFocusRingProps"</code></TableCell>
                            <TableCell>"Spread onto the target element via " <code>"props.into_attrs()"</code> ". Includes the " <code>"data-focus-visible"</code> " attribute."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"is_focused"</code></TableCell>
                            <TableCell><code>"Signal<bool>"</code></TableCell>
                            <TableCell>"True when the element (or a descendant in within mode) is focused."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"is_focus_visible"</code></TableCell>
                            <TableCell><code>"Signal<bool>"</code></TableCell>
                            <TableCell>"True when the focus ring should be shown (keyboard/virtual modality)."</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h2 id="features" class="anchor">
                "Features"
                <AnchorLink href="#features" description="Direct link to features"/>
            </h2>

            <ul>
                <li>"Distinguishes between keyboard and pointer focus."</li>
                <li>"Tracks both focus state and focus visibility separately."</li>
                <li>"Automatic " <code>"data-focus-visible"</code> " attribute for CSS styling."</li>
                <li><code>"within"</code> " mode tracks focus within descendants (for containers)."</li>
                <li>"Optional focus/blur/change callbacks with disabled state support."</li>
                <li>"Text input mode for compound components."</li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "use_focus_ring", link: "#use_focus_ring" },
                Toc::Leaf { title: "Basic Usage", link: "#basic-usage" },
                Toc::Leaf { title: "Keyboard vs Mouse Focus", link: "#keyboard-vs-mouse" },
                Toc::Leaf { title: "Within Mode", link: "#within-mode" },
                Toc::Leaf { title: "Custom Options", link: "#custom-options" },
                Toc::Leaf { title: "Text Input Mode", link: "#text-input-mode" },
                Toc::Leaf { title: "Data Attribute Styling", link: "#data-attribute" },
                Toc::Leaf { title: "Input", link: "#input" },
                Toc::Leaf { title: "Return Value", link: "#return-value" },
                Toc::Leaf { title: "Features", link: "#features" },
            ]
        }/>
    }
}
