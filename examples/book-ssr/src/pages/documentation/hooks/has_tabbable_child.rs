use indoc::indoc;
use leptonic::{components::prelude::*, hooks::*, prelude::Size};
use leptos::prelude::*;

use crate::pages::documentation::{article::Article, doc_styles::*, toc::Toc};

#[component]
pub fn PageUseHasTabbableChild() -> impl IntoView {
    let (show_button, set_show_button) = signal(true);
    let (show_input, set_show_input) = signal(true);
    let (disabled, set_disabled) = signal(false);

    let UseHasTabbableChildReturn {
        has_tabbable_child,
        props,
    } = use_has_tabbable_child(UseHasTabbableChildInput {
        disabled: disabled.into(),
    });

    view! {
        <Article>
            <h1 id="use_has_tabbable_child" class="anchor">
                "use_has_tabbable_child"
                <AnchorLink href="#use_has_tabbable_child" description="Direct link to article header"/>
            </h1>

            <p>"Detects whether an element contains any tabbable child elements. Useful for deciding whether a container should itself be focusable via Tab, ensuring focus trapping in modals has something to focus, or controlling skip-link visibility."</p>

            <h2 id="basic-usage" class="anchor">
                "Basic Usage"
                <AnchorLink href="#basic-usage" description="Direct link to basic usage"/>
            </h2>

            <p>"A common pattern is to make a container focusable only when it has no tabbable children. When children are tabbable, the container uses " <code>"tabindex=\"-1\""</code> " so it can receive programmatic focus but is skipped during Tab navigation:"</p>

            <Code>
                {indoc!(r#"
                    let UseHasTabbableChildReturn { has_tabbable_child, props } =
                        use_has_tabbable_child(UseHasTabbableChildInput::default());

                    view! {
                        <div
                            {..props.into_attrs()}
                            tabindex=move || if has_tabbable_child.get() { -1 } else { 0 }
                        >
                            <button>"Child button"</button>
                        </div>
                    }
                "#)}
            </Code>

            <h2 id="demo" class="anchor">
                "Dynamic Content"
                <AnchorLink href="#demo" description="Direct link to demo"/>
            </h2>

            <p>"Toggle the elements below to see how the tabbable child detection changes dynamically:"</p>

            <Stack orientation=StackOrientation::Vertical spacing=Size::Em(0.5) attr:style="margin: 1em 0;">
                <FormControl attr:style=form_control_row()>
                    <Checkbox checked=show_button set_checked=set_show_button />
                    <Label>"Show button"</Label>
                </FormControl>
                <FormControl attr:style=form_control_row()>
                    <Checkbox checked=show_input set_checked=set_show_input />
                    <Label>"Show input"</Label>
                </FormControl>
            </Stack>

            <div
                {..props.into_attrs()}
                style=demo_container()
            >
                <p style="margin: 0 0 1em 0; font-weight: bold;">"Container"</p>
                <Stack orientation=StackOrientation::Horizontal spacing=Size::Em(0.5)>
                    <Show when=move || show_button.get()>
                        <button style=demo_button()>
                            "Tabbable Button"
                        </button>
                    </Show>
                    <Show when=move || show_input.get()>
                        <input
                            type="text"
                            placeholder="Tabbable input"
                            style=demo_input()
                        />
                    </Show>
                </Stack>
            </div>

            <p>
                "Has tabbable child: "
                <strong style=move || if has_tabbable_child.get() { state_active() } else { state_inactive() }>
                    { move || if has_tabbable_child.get() { "true" } else { "false" } }
                </strong>
            </p>

            <h2 id="disabled" class="anchor">
                "Disabled State"
                <AnchorLink href="#disabled" description="Direct link to disabled state"/>
            </h2>

            <p>"When " <code>"disabled"</code> " is true, the hook stops observing and " <code>"has_tabbable_child"</code> " returns false regardless of actual children:"</p>

            <FormControl attr:style=form_control_row()>
                <Checkbox checked=disabled set_checked=set_disabled />
                <Label>"Disabled"</Label>
            </FormControl>

            <h2 id="how-it-works" class="anchor">
                "How It Works"
                <AnchorLink href="#how-it-works" description="Direct link to how it works"/>
            </h2>

            <p>"The hook uses a " <code>"MutationObserver"</code> " to watch for changes in the container's subtree. It observes:"</p>
            <ul>
                <li><code>"childList"</code> " — detects added/removed child elements"</li>
                <li><code>"subtree"</code> " — watches the entire descendant tree, not just direct children"</li>
                <li><code>"attributes"</code> " — watches for " <code>"tabindex"</code> " and " <code>"disabled"</code> " attribute changes that affect tabbability"</li>
            </ul>

            <p>"When any mutation is observed, the hook rescans the container for tabbable elements using the same focusability logic as the focus management system."</p>

            <h2 id="input" class="anchor">
                "Input"
                <AnchorLink href="#input" description="Direct link to input"/>
            </h2>

            <p><code>"UseHasTabbableChildInput"</code> " fields:"</p>

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
                            <TableCell>"Disables observation when true."</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h2 id="return-value" class="anchor">
                "Return Value"
                <AnchorLink href="#return-value" description="Direct link to return value"/>
            </h2>

            <p><code>"UseHasTabbableChildReturn"</code> " fields:"</p>

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
                            <TableCell><code>"has_tabbable_child"</code></TableCell>
                            <TableCell><code>"Signal<bool>"</code></TableCell>
                            <TableCell>"True when the container has at least one tabbable descendant."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"props"</code></TableCell>
                            <TableCell><code>"UseHasTabbableChildProps"</code></TableCell>
                            <TableCell>"Spread onto the container element via " <code>"props.into_attrs()"</code> " to enable subtree observation."</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h2 id="features" class="anchor">
                "Features"
                <AnchorLink href="#features" description="Direct link to features"/>
            </h2>

            <ul>
                <li>"Reactive detection of tabbable children via " <code>"MutationObserver"</code> "."</li>
                <li>"Considers " <code>"tabindex"</code> " values and filters out disabled elements."</li>
                <li>"Handles dynamic content changes (added/removed children, attribute mutations)."</li>
                <li>"Automatic element capture via prop spreading."</li>
                <li>"Respects disabled state — stops observation when disabled."</li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "use_has_tabbable_child", link: "#use_has_tabbable_child" },
                Toc::Leaf { title: "Basic Usage", link: "#basic-usage" },
                Toc::Leaf { title: "Dynamic Content", link: "#demo" },
                Toc::Leaf { title: "Disabled State", link: "#disabled" },
                Toc::Leaf { title: "How It Works", link: "#how-it-works" },
                Toc::Leaf { title: "Input", link: "#input" },
                Toc::Leaf { title: "Return Value", link: "#return-value" },
                Toc::Leaf { title: "Features", link: "#features" },
            ]
        }/>
    }
}
