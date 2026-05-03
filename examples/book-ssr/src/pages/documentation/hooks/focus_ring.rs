use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::pages::documentation::{article::Article, demo_shell::DemoShell, toc::Toc};

use super::demos::{
    focus_ring_keyboard::FocusRingKeyboardDemo, focus_ring_within::FocusRingWithinDemo,
};

#[component]
pub fn PageUseFocusRing() -> impl IntoView {
    view! {
        <Article>
            <h1 id="use_focus_ring" class="anchor">
                "use_focus_ring"
                <AnchorLink href="#use_focus_ring" description="Direct link to article header"/>
            </h1>

            <p>
                "The "<code>"use_focus_ring"</code>" hook determines when to show a focus ring for accessibility. "
                "The focus ring should only be visible during keyboard navigation, not when using mouse or touch. "
                "See the "<Link href=crate::routes::doc::Focus.materialize()>"Focus overview"</Link>" for domain guidance."
            </p>

            <p>
                "Based on react-aria\u{2019}s "
                <LinkExt href="https://react-spectrum.adobe.com/react-aria/useFocusRing.html" target=LinkTarget::_Blank>
                    "useFocusRing"
                </LinkExt>
                "."
            </p>

            <h2 id="keyboard-vs-mouse" class="anchor">
                "Keyboard vs Mouse Focus"
                <AnchorLink href="#keyboard-vs-mouse" description="Direct link to keyboard vs mouse"/>
            </h2>

            <p>"Try both clicking and tabbing to the buttons below. The focus ring only appears when using keyboard navigation. The " <code>"data-focus-visible"</code> " attribute is automatically added when the focus ring should be visible, and CSS handles the styling:"</p>

            <DemoShell
                source=include_str!("demos/focus_ring_keyboard.rs")
                description="Keyboard-only focus ring visibility"
            >
                <FocusRingKeyboardDemo />
            </DemoShell>

            <h2 id="within-mode" class="anchor">
                "Within Mode"
                <AnchorLink href="#within-mode" description="Direct link to within mode"/>
            </h2>

            <p>"With " <code>"within: true"</code> ", the focus ring tracks focus within the element's subtree using " <code>"focusin"</code> "/" <code>"focusout"</code> " events. The " <code>"data-focus-visible"</code> " attribute is set on the container when any descendant is focused via keyboard:"</p>

            <DemoShell
                source=include_str!("demos/focus_ring_within.rs")
                description="Focus ring tracking within descendants"
            >
                <FocusRingWithinDemo />
            </DemoShell>

            <h2 id="text-input-mode" class="anchor">
                "Text Input Mode"
                <AnchorLink href="#text-input-mode" description="Direct link to text input mode"/>
            </h2>

            <p>"When " <code>"is_text_input: true"</code> ", only Tab and Escape trigger the focus ring. Other keyboard events (arrow keys, letters) do not make focus visible. This matches the behavior of native text inputs where typing shouldn't trigger a focus ring:"</p>

            <Code language=Language::Rust>
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

            <Code language=Language::Rust>
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

            <h2 id="see-also" class="anchor">
                "See Also"
                <AnchorLink href="#see-also" description="Direct link to section: See Also"/>
            </h2>

            <ul>
                <li><Link href=crate::routes::doc::Focus.materialize()>"Focus overview"</Link></li>
                <li><Link href=crate::routes::doc::focus::UseFocusVisible.materialize()>"use_focus_visible"</Link>" \u{2014} lower-level modality tracking used internally"</li>
                <li><Link href=crate::routes::doc::focus::UseFocusable.materialize()>"use_focusable"</Link></li>
                <li><Link href=crate::routes::doc::focus::FocusRing.materialize()>"FocusRing atom"</Link>" \u{2014} wraps this hook"</li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "use_focus_ring", link: "#use_focus_ring" },
                Toc::Leaf { title: "Keyboard vs Mouse Focus", link: "#keyboard-vs-mouse" },
                Toc::Leaf { title: "Within Mode", link: "#within-mode" },
                Toc::Leaf { title: "Text Input Mode", link: "#text-input-mode" },
                Toc::Leaf { title: "Data Attribute Styling", link: "#data-attribute" },
                Toc::Leaf { title: "Input", link: "#input" },
                Toc::Leaf { title: "Return Value", link: "#return-value" },
                Toc::Leaf { title: "Features", link: "#features" },
                Toc::Leaf { title: "See Also", link: "#see-also" },
            ]
        }/>
    }
}
