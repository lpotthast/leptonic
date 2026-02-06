use indoc::indoc;
use leptonic::atoms::focus_scope::FocusScope;
use leptonic::atoms::link::AnchorLink;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::pages::documentation::article::Article;
use crate::pages::documentation::toc::Toc;

#[component]
pub fn PageAtomFocusScope() -> impl IntoView {
    view! {
        <Article>
            <h1 id="focus-scope" class="anchor">
                "FocusScope"
                <AnchorLink href="#focus-scope" description="Direct link to article header"/>
            </h1>

            <p>
                "A FocusScope manages focus containment, restoration, and auto-focus for dialogs, menus, and overlays. "
                "It ensures keyboard navigation stays within the scope and provides a FocusManager through context for programmatic focus control."
            </p>

            <h2 id="props" class="anchor">
                "Props"
                <AnchorLink href="#props" description="Direct link to props section"/>
            </h2>

            <ul>
                <li><Code inline=true>"contain"</Code>" - When true, Tab/Shift+Tab navigation wraps within the scope, preventing focus from leaving."</li>
                <li><Code inline=true>"restore_focus"</Code>" - When true, focus returns to the previously focused element when the scope unmounts."</li>
                <li><Code inline=true>"auto_focus"</Code>" - When true, focus moves to the first focusable element in the scope when it mounts."</li>
            </ul>

            <h2 id="example" class="anchor">
                "Example"
                <AnchorLink href="#example" description="Direct link to example section"/>
            </h2>

            <p>
                "The following example demonstrates a focus-trapped dialog with inputs and a button. "
                "When you Tab through the elements, focus will wrap from the last element back to the first, never leaving the scope."
            </p>

            <Code>
                {indoc!(r#"
                    use leptonic::atoms::focus_scope::FocusScope;

                    view! {
                        <FocusScope contain=true restore_focus=true auto_focus=true>
                            <div style="display: flex; flex-direction: column; gap: 0.5em; padding: 1em; border: 2px solid var(--brand-color); border-radius: 0.5em;">
                                <input type="text" placeholder="First name" />
                                <input type="text" placeholder="Last name" />
                                <button>"Submit"</button>
                            </div>
                        </FocusScope>
                    }
                "#)}
            </Code>

            <h2 id="demo" class="anchor">
                "Demo"
                <AnchorLink href="#demo" description="Direct link to demo section"/>
            </h2>

            <p>
                "Try pressing Tab repeatedly in the demo below. Focus will cycle through the three inputs and the button, never escaping the scope."
            </p>

            <FocusScope contain=true>
                <div style="display: flex; flex-direction: column; gap: 0.5em; padding: 1em; border: 2px solid var(--brand-color); border-radius: 0.5em;">
                    <input type="text" placeholder="First name" />
                    <input type="text" placeholder="Last name" />
                    <button>"Submit"</button>
                </div>
            </FocusScope>

            <h2 id="focus-manager" class="anchor">
                "FocusManager Context"
                <AnchorLink href="#focus-manager" description="Direct link to FocusManager context section"/>
            </h2>

            <p>
                "FocusScope provides a "<Code inline=true>"FocusScopeContext"</Code>" through Leptos context. "
                "Child components can access this context to programmatically move focus using the FocusManager."
            </p>

            <Code>
                {indoc!(r#"
                    use leptonic::atoms::focus_scope::{FocusScope, FocusScopeContext};

                    #[component]
                    fn MyComponent() -> impl IntoView {
                        let ctx = expect_context::<FocusScopeContext>();

                        // Move focus to the next element
                        ctx.focus_manager.focus_next(Default::default());

                        // Move focus to the previous element
                        ctx.focus_manager.focus_previous(Default::default());

                        // Move focus to the first element
                        ctx.focus_manager.focus_first(Default::default());

                        // Move focus to the last element
                        ctx.focus_manager.focus_last(Default::default());
                    }
                "#)}
            </Code>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "FocusScope", link: "#focus-scope" },
                Toc::Leaf { title: "Props", link: "#props" },
                Toc::Leaf { title: "Example", link: "#example" },
                Toc::Leaf { title: "Demo", link: "#demo" },
                Toc::Leaf { title: "FocusManager Context", link: "#focus-manager" },
            ]
        }/>
    }
}
