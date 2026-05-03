use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::pages::documentation::{article::Article, demo_shell::DemoShell, toc::Toc};

use super::demos::{
    focus_manager_basic::FocusManagerBasicDemo, focus_manager_scope::FocusManagerScopeDemo,
};

#[component]
pub fn PageUseFocusManager() -> impl IntoView {
    view! {
        <Article>
            <h1 id="use_focus_manager" class="anchor">
                "use_focus_manager"
                <AnchorLink href="#use_focus_manager" description="Direct link to article header"/>
            </h1>

            <p>
                "The "<code>"use_focus_manager"</code>" hook programmatically navigates focus within a container. "
                "Provides methods to move focus to next, previous, first, or last focusable element. "
                "See the "<Link href=crate::routes::doc::Focus.materialize()>"Focus overview"</Link>" for domain guidance."
            </p>

            <p>
                "Based on react-aria\u{2019}s "
                <LinkExt href="https://react-spectrum.adobe.com/react-aria/FocusScope.html" target=LinkTarget::_Blank>
                    "useFocusManager"
                </LinkExt>
                "."
            </p>

            <h2 id="demo" class="anchor">
                "Interactive Demo"
                <AnchorLink href="#demo" description="Direct link to demo"/>
            </h2>

            <p>"Use the control buttons to move focus within the scope container:"</p>

            <DemoShell
                source=include_str!("demos/focus_manager_basic.rs")
                description="Programmatic focus navigation"
            >
                <FocusManagerBasicDemo />
            </DemoShell>

            <h2 id="tabbable" class="anchor">
                "Tabbable Option"
                <AnchorLink href="#tabbable" description="Direct link to tabbable option"/>
            </h2>

            <p>"By default, " <code>"focus_next"</code> "/" <code>"focus_previous"</code> " navigate to all focusable elements including those with " <code>"tabindex=\"-1\""</code> ". Set " <code>"tabbable: true"</code> " to restrict navigation to elements with " <code>"tabindex >= 0"</code> " (those reachable via Tab):"</p>

            <Code language=Language::Rust>
                {indoc!(r"
                    focus_manager.focus_next(FocusManagerOptions {
                        tabbable: true, // Skip elements with tabindex=-1
                        ..Default::default()
                    });
                ")}
            </Code>

            <p>"Toggle the \"Tabbable only\" checkbox in the demo above to see the difference."</p>

            <h2 id="accept" class="anchor">
                "Custom Filter (accept)"
                <AnchorLink href="#accept" description="Direct link to custom filter"/>
            </h2>

            <p>"The " <code>"accept"</code> " option takes a filter function to skip specific elements during navigation. This is useful when you need to exclude certain elements programmatically:"</p>

            <Code language=Language::Rust>
                {indoc!(r#"
                    use std::sync::Arc;

                    focus_manager.focus_next(FocusManagerOptions {
                        accept: Some(Arc::new(|el: &web_sys::Element| {
                            // Skip disabled-looking elements
                            !el.class_list().contains("skip-focus")
                        })),
                        ..Default::default()
                    });
                "#)}
            </Code>

            <h2 id="focus-trapping" class="anchor">
                "Focus Trapping with FocusScope"
                <AnchorLink href="#focus-trapping" description="Direct link to focus trapping"/>
            </h2>

            <p>"The " <code>"use_focus_manager"</code> " hook provides " <em>"programmatic"</em> " focus control only. It does not trap focus or intercept Tab key presses. For focus trapping (preventing Tab from leaving the container), use the " <code>"FocusScope"</code> " component which combines focus management with keyboard event handling."</p>

            <Code language=Language::Rust>
                {indoc!(r#"
                    <FocusScope contain=true auto_focus=true restore_focus=true>
                        <button>"First"</button>
                        <input type="text" placeholder="Middle" />
                        <button>"Last"</button>
                    </FocusScope>
                "#)}
            </Code>

            <p>"Try tabbing through the container below. Focus will wrap from the last element back to the first, and vice versa with Shift+Tab:"</p>

            <DemoShell
                source=include_str!("demos/focus_manager_scope.rs")
                description="Focus trapping with FocusScope"
            >
                <FocusManagerScopeDemo />
            </DemoShell>

            <h3>"FocusScope Props"</h3>

            <TableContainer>
                <Table bordered=true hoverable=true>
                    <TableHeader>
                        <TableRow>
                            <TableHeaderCell min_width=true>"Prop"</TableHeaderCell>
                            <TableHeaderCell min_width=true>"Type"</TableHeaderCell>
                            <TableHeaderCell min_width=true>"Default"</TableHeaderCell>
                            <TableHeaderCell>"Description"</TableHeaderCell>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        <TableRow>
                            <TableCell><code>"contain"</code></TableCell>
                            <TableCell><code>"bool"</code></TableCell>
                            <TableCell><code>"false"</code></TableCell>
                            <TableCell>"When true, Tab/Shift+Tab navigation wraps within the scope."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"auto_focus"</code></TableCell>
                            <TableCell><code>"bool"</code></TableCell>
                            <TableCell><code>"false"</code></TableCell>
                            <TableCell>"When true, focuses the first focusable element on mount."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"restore_focus"</code></TableCell>
                            <TableCell><code>"bool"</code></TableCell>
                            <TableCell><code>"false"</code></TableCell>
                            <TableCell>"When true, restores focus to the previously focused element when unmounted."</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h2 id="input" class="anchor">
                "Input"
                <AnchorLink href="#input" description="Direct link to input"/>
            </h2>

            <p><code>"UseFocusManagerInput"</code> " is an empty struct — all configuration is passed per-call via " <code>"FocusManagerOptions"</code> ":"</p>

            <h3>"FocusManagerOptions"</h3>

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
                            <TableCell><code>"from"</code></TableCell>
                            <TableCell><code>"Option<web_sys::Element>"</code></TableCell>
                            <TableCell><code>"document.activeElement"</code></TableCell>
                            <TableCell>"Element to start navigation from."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"wrap"</code></TableCell>
                            <TableCell><code>"bool"</code></TableCell>
                            <TableCell><code>"false"</code></TableCell>
                            <TableCell>"Whether to wrap around when reaching the end/beginning."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"tabbable"</code></TableCell>
                            <TableCell><code>"bool"</code></TableCell>
                            <TableCell><code>"false"</code></TableCell>
                            <TableCell>"Only consider elements with tabindex >= 0."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"accept"</code></TableCell>
                            <TableCell><code>"Option<Arc<dyn Fn(&Element) -> bool>>"</code></TableCell>
                            <TableCell><code>"None"</code></TableCell>
                            <TableCell>"Custom filter function to skip specific elements."</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h2 id="return-value" class="anchor">
                "Return Value"
                <AnchorLink href="#return-value" description="Direct link to return value"/>
            </h2>

            <p><code>"UseFocusManagerReturn"</code> " fields:"</p>

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
                            <TableCell><code>"focus_manager"</code></TableCell>
                            <TableCell><code>"FocusManager"</code></TableCell>
                            <TableCell>"Provides " <code>"focus_next"</code> ", " <code>"focus_previous"</code> ", " <code>"focus_first"</code> ", and " <code>"focus_last"</code> " methods."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"props"</code></TableCell>
                            <TableCell><code>"UseFocusManagerProps"</code></TableCell>
                            <TableCell>"Spread onto the container element via " <code>"props.into_attrs()"</code> " to define the scope boundary."</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h2 id="features" class="anchor">
                "Features"
                <AnchorLink href="#features" description="Direct link to features"/>
            </h2>

            <ul>
                <li>"Navigate to next/previous/first/last focusable element."</li>
                <li>"Respects tabindex order (positive tabindex elements come first)."</li>
                <li>"Filters out hidden and disabled elements."</li>
                <li>"Optional wrap-around, tabbable-only, and custom filter support."</li>
                <li>"Automatic element capture via prop spreading."</li>
            </ul>

            <h2 id="see-also" class="anchor">
                "See Also"
                <AnchorLink href="#see-also" description="Direct link to section: See Also"/>
            </h2>

            <ul>
                <li><Link href=crate::routes::doc::Focus.materialize()>"Focus overview"</Link></li>
                <li><Link href=crate::routes::doc::focus::FocusScope.materialize()>"FocusScope atom"</Link>" \u{2014} provides focus containment and restoration"</li>
                <li><Link href=crate::routes::doc::focus::UseHasTabbableChild.materialize()>"use_has_tabbable_child"</Link></li>
                <li><Link href=crate::routes::doc::focus::UseFocusable.materialize()>"use_focusable"</Link></li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "use_focus_manager", link: "#use_focus_manager" },
                Toc::Leaf { title: "Interactive Demo", link: "#demo" },
                Toc::Leaf { title: "Tabbable Option", link: "#tabbable" },
                Toc::Leaf { title: "Custom Filter (accept)", link: "#accept" },
                Toc::Leaf { title: "Focus Trapping", link: "#focus-trapping" },
                Toc::Leaf { title: "Input", link: "#input" },
                Toc::Leaf { title: "Return Value", link: "#return-value" },
                Toc::Leaf { title: "Features", link: "#features" },
                Toc::Leaf { title: "See Also", link: "#see-also" },
            ]
        }/>
    }
}
