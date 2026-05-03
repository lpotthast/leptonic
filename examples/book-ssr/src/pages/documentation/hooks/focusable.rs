use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::pages::documentation::{article::Article, demo_shell::DemoShell, toc::Toc};

use super::demos::focusable::FocusableDemo;

#[component]
pub fn PageUseFocusable() -> impl IntoView {
    view! {
        <Article>
            <h1 id="use_focusable" class="anchor">
                "use_focusable"
                <AnchorLink href="#use_focusable" description="Direct link to article header"/>
            </h1>

            <p>
                "The "<code>"use_focusable"</code>" hook makes any element focusable with proper keyboard event handling. "
                "Combines "<code>"use_focus"</code>" and "<code>"use_keyboard"</code>" for a complete focusable element solution. "
                "See the "<Link href=crate::routes::doc::Focus.materialize()>"Focus overview"</Link>" for domain guidance."
            </p>

            <p>
                "Based on react-aria\u{2019}s "
                <LinkExt href="https://react-spectrum.adobe.com/react-aria/useFocusable.html" target=LinkTarget::_Blank>
                    "useFocusable"
                </LinkExt>
                "."
            </p>

            <h2 id="demo" class="anchor">
                "Interactive Demo"
                <AnchorLink href="#demo" description="Direct link to demo"/>
            </h2>

            <p>"Focus the custom element below using Tab, click, or the button. The demo uses " <code>"FocusRing"</code> " (powered by " <code>"use_focus_ring"</code> ") to show a visible ring on keyboard focus:"</p>

            <DemoShell
                source=include_str!("demos/focusable.rs")
                description="Custom focusable element with keyboard events"
            >
                <FocusableDemo />
            </DemoShell>

            <h2 id="programmatic-focus" class="anchor">
                "Programmatic Focus"
                <AnchorLink href="#programmatic-focus" description="Direct link to programmatic focus"/>
            </h2>

            <p>"The hook returns a " <code>"FocusHandle"</code> " that allows you to programmatically focus the element. The " <code>"focus()"</code> " method uses " <code>"focus_safely"</code> " which defers focus during screen reader (virtual) modality to avoid VoiceOver scroll issues during CSS transitions:"</p>

            <Code language=Language::Rust>
                {indoc!(r"
                    let UseFocusableReturn { props, focus_handle } = use_focusable(input);

                    // Focus the element from anywhere
                    focus_handle.focus();

                    // Check if element has been captured (false during SSR)
                    if focus_handle.has_element() {
                        focus_handle.focus();
                    }
                ")}
            </Code>

            <p>"The " <code>"FocusHandle"</code> " is useful for:"</p>
            <ul>
                <li>"Focusing elements in response to user actions"</li>
                <li>"Implementing keyboard navigation in custom components"</li>
                <li>"Focus restoration after dialogs close"</li>
            </ul>

            <h2 id="tab-index" class="anchor">
                "Tab Index Management"
                <AnchorLink href="#tab-index" description="Direct link to tab index"/>
            </h2>

            <p>"The hook automatically manages the tabindex attribute:"</p>
            <ul>
                <li><code>"disabled=true"</code> " → no tabindex (element not focusable)"</li>
                <li><code>"exclude_from_tab_order=true"</code> " → tabindex=\"-1\" (focusable but not via Tab)"</li>
                <li>"Default → tabindex=\"0\" (focusable via Tab in normal order)"</li>
            </ul>

            <h2 id="auto-focus" class="anchor">
                "Auto Focus"
                <AnchorLink href="#auto-focus" description="Direct link to auto focus"/>
            </h2>

            <p>"Set " <code>"auto_focus: true"</code> " to automatically focus the element when it mounts:"</p>

            <Code language=Language::Rust>
                {indoc!(r"
                    let focusable = use_focusable(UseFocusableInput {
                        auto_focus: true,  // Element will be focused on mount
                        ..Default::default()
                    });
                ")}
            </Code>

            <h2 id="context" class="anchor">
                "FocusableContext"
                <AnchorLink href="#context" description="Direct link to FocusableContext"/>
            </h2>

            <p>"Parent components (e.g., " <code>"TooltipTrigger"</code> ") can inject additional event handlers into a focusable child via " <code>"FocusableContext"</code> ". The child's " <code>"use_focusable"</code> " automatically reads the context and chains the parent's handlers with its own."</p>

            <Code language=Language::Rust>
                {indoc!(r"
                    // Parent provides context:
                    provide_context(FocusableContext {
                        on_focus: Some(EventHandler::new(|_| { /* parent focus handler */ })),
                        ..Default::default()
                    });

                    // Child's use_focusable automatically chains context handlers.
                    // Context handlers are guarded by the disabled state — they are
                    // skipped when the focusable element is disabled.
                ")}
            </Code>

            <h2 id="input" class="anchor">
                "Input"
                <AnchorLink href="#input" description="Direct link to input"/>
            </h2>

            <p><code>"UseFocusableInput"</code> " fields:"</p>

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
                            <TableCell>"Whether focus should be disabled."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"auto_focus"</code></TableCell>
                            <TableCell><code>"bool"</code></TableCell>
                            <TableCell><code>"false"</code></TableCell>
                            <TableCell>"Whether the element should be focused on mount."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"exclude_from_tab_order"</code></TableCell>
                            <TableCell><code>"Signal<bool>"</code></TableCell>
                            <TableCell><code>"false"</code></TableCell>
                            <TableCell>"When true, sets tabindex=\"-1\"."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"on_focus"</code></TableCell>
                            <TableCell><code>"Option<Callback<FocusEvent>>"</code></TableCell>
                            <TableCell><code>"None"</code></TableCell>
                            <TableCell>"Handler called when the element receives focus."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"on_blur"</code></TableCell>
                            <TableCell><code>"Option<Callback<FocusEvent>>"</code></TableCell>
                            <TableCell><code>"None"</code></TableCell>
                            <TableCell>"Handler called when the element loses focus."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"on_focus_change"</code></TableCell>
                            <TableCell><code>"Option<Callback<bool>>"</code></TableCell>
                            <TableCell><code>"None"</code></TableCell>
                            <TableCell>"Handler called when focus state changes."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"on_key_down"</code></TableCell>
                            <TableCell><code>"Option<Callback<KeyboardEventWrapper>>"</code></TableCell>
                            <TableCell><code>"None"</code></TableCell>
                            <TableCell>"Handler called when a key is pressed."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"on_key_up"</code></TableCell>
                            <TableCell><code>"Option<Callback<KeyboardEventWrapper>>"</code></TableCell>
                            <TableCell><code>"None"</code></TableCell>
                            <TableCell>"Handler called when a key is released."</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h2 id="return-value" class="anchor">
                "Return Value"
                <AnchorLink href="#return-value" description="Direct link to return value"/>
            </h2>

            <p><code>"UseFocusableReturn"</code> " fields:"</p>

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
                            <TableCell><code>"UseFocusableProps"</code></TableCell>
                            <TableCell>"Spread onto the target element via " <code>"props.into_attrs()"</code> ". Manages tabindex, focus, blur, and keyboard listeners."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"focus_handle"</code></TableCell>
                            <TableCell><code>"FocusHandle"</code></TableCell>
                            <TableCell>"Allows programmatic focus via " <code>"focus_handle.focus()"</code> "."</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h2 id="see-also" class="anchor">
                "See Also"
                <AnchorLink href="#see-also" description="Direct link to section: See Also"/>
            </h2>

            <ul>
                <li><Link href=crate::routes::doc::Focus.materialize()>"Focus overview"</Link></li>
                <li><Link href=crate::routes::doc::focus::UseFocus.materialize()>"use_focus"</Link>" \u{2014} low-level focus/blur, used internally"</li>
                <li><Link href=crate::routes::doc::interactions::UseKeyboard.materialize()>"use_keyboard"</Link>" \u{2014} low-level keyboard, used internally"</li>
                <li><Link href=crate::routes::doc::focus::UseFocusRing.materialize()>"use_focus_ring"</Link>" \u{2014} tracks whether a focus ring should be visible"</li>
                <li><Link href=crate::routes::doc::focus::UseFocusVisible.materialize()>"use_focus_visible"</Link>" \u{2014} tracks current input modality"</li>
            </ul>

            <h2 id="deviations" class="anchor">
                "Deviations from react-aria"
                <AnchorLink href="#deviations" description="Direct link to deviations"/>
            </h2>

            <ul>
                <li><b>"Handler optimization"</b> " — React-aria returns " <code>"undefined"</code> " props when no callbacks are provided. Our " <code>"EventHandler"</code> " always attaches a listener but checks the disabled state inside. The overhead is negligible."</li>
                <li><b>"Context handler guard"</b> " — React-aria discards all interaction props when disabled (" <code>"isDisabled ? {} : domProps"</code> "). Leptonic wraps context-provided handlers with a disabled check so they are skipped when the focusable element is disabled."</li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "use_focusable", link: "#use_focusable" },
                Toc::Leaf { title: "Interactive Demo", link: "#demo" },
                Toc::Leaf { title: "Programmatic Focus", link: "#programmatic-focus" },
                Toc::Leaf { title: "Tab Index Management", link: "#tab-index" },
                Toc::Leaf { title: "Auto Focus", link: "#auto-focus" },
                Toc::Leaf { title: "FocusableContext", link: "#context" },
                Toc::Leaf { title: "Input", link: "#input" },
                Toc::Leaf { title: "Return Value", link: "#return-value" },
                Toc::Leaf { title: "See Also", link: "#see-also" },
                Toc::Leaf { title: "Deviations from react-aria", link: "#deviations" },
            ]
        }/>
    }
}
