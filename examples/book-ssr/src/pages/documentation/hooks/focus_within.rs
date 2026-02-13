use indoc::indoc;
use leptonic::{components::prelude::*, hooks::*, prelude::Size};
use leptos::prelude::*;

use crate::pages::documentation::{article::Article, doc_styles::*, toc::Toc};

#[component]
pub fn PageUseFocusWithin() -> impl IntoView {
    let (disabled, set_disabled) = signal(false);
    let (focus_count, set_focus_count) = signal(0);
    let (blur_count, set_blur_count) = signal(0);
    let (change_count, set_change_count) = signal(0);

    let UseFocusWithinReturn {
        props,
        is_focus_within,
    } = use_focus_within(UseFocusWithinInput {
        disabled: disabled.into(),
        on_focus_within: Some(Callback::new(move |_| {
            set_focus_count.update(|c| *c += 1);
        })),
        on_blur_within: Some(Callback::new(move |_| {
            set_blur_count.update(|c| *c += 1);
        })),
        on_focus_within_change: Some(Callback::new(move |_focused: bool| {
            set_change_count.update(|c| *c += 1);
        })),
    });

    view! {
        <Article>
            <h1 id="use_focus_within" class="anchor">
                "use_focus_within"
                <AnchorLink href="#use_focus_within" description="Direct link to article header"/>
            </h1>

            <p>"Track when focus is anywhere within an element tree. Unlike " <code>"use_focus"</code> " which only fires when the element itself receives focus, " <code>"use_focus_within"</code> " fires when focus enters or leaves the entire element tree. Common use cases include form groups that highlight when any field is focused, dropdown menus that stay open while navigating between items, and card components that respond to child focus."</p>

            <h2 id="basic-usage" class="anchor">
                "Basic Usage"
                <AnchorLink href="#basic-usage" description="Direct link to basic usage"/>
            </h2>

            <Code>
                {indoc!(r#"
                    let UseFocusWithinReturn { props, is_focus_within } = use_focus_within(
                        UseFocusWithinInput {
                            disabled: Signal::derive(|| false),
                            on_focus_within: Some(Callback::new(|_| { /* focus entered */ })),
                            on_blur_within: Some(Callback::new(|_| { /* focus left */ })),
                            on_focus_within_change: Some(Callback::new(|is_focused: bool| {
                                // Fires whenever focus-within state changes
                            })),
                        }
                    );

                    view! {
                        <div {..props.into_attrs()}>
                            <input type="text" />
                            <button>"Submit"</button>
                        </div>
                    }
                "#)}
            </Code>

            <p>"Click on any element inside the container below. Focus within is tracked even as you move between different focusable children:"</p>

            <div
                {..props.into_attrs()}
                style=move || if is_focus_within.get() { demo_container_active() } else { demo_container_inactive() }
            >
                <Stack orientation=StackOrientation::Vertical spacing=Size::Em(1.0)>
                    <p style="margin: 0;">
                        "Focus within: "
                        <strong style=move || if is_focus_within.get() { state_active() } else { state_inactive() }>
                            { move || if is_focus_within.get() { "Yes" } else { "No" } }
                        </strong>
                    </p>

                    <Stack orientation=StackOrientation::Horizontal spacing=Size::Em(1.0)>
                        <input
                            type="text"
                            placeholder="Click me..."
                            style=demo_input()
                        />
                        <button style=demo_button()>
                            "Button 1"
                        </button>
                        <button style=demo_button()>
                            "Button 2"
                        </button>
                    </Stack>

                    <p style="margin: 0; font-size: 0.85em; opacity: 0.7;">
                        "Tab between elements - focus stays \"within\" the container"
                    </p>
                </Stack>
            </div>

            <h2 id="event-callbacks" class="anchor">
                "Event Callbacks"
                <AnchorLink href="#event-callbacks" description="Direct link to event callbacks"/>
            </h2>

            <p>"Three callbacks report focus activity within the container: " <code>"on_focus_within"</code>
                " fires when focus enters, " <code>"on_blur_within"</code> " fires when focus leaves, and "
                <code>"on_focus_within_change"</code> " fires on every transition."</p>

            <div style=flex_row_gap()>
                <p>"Focus within: " <strong>{ move || focus_count.get() }</strong></p>
                <p>"Blur within: " <strong>{ move || blur_count.get() }</strong></p>
                <p>"Change: " <strong>{ move || change_count.get() }</strong></p>
            </div>

            <h2 id="is-focus-within" class="anchor">
                "is_focus_within Signal"
                <AnchorLink href="#is-focus-within" description="Direct link to is_focus_within signal"/>
            </h2>

            <p>"The hook returns a reactive " <code>"Signal<bool>"</code> " named " <code>"is_focus_within"</code>
                " that is true whenever any descendant is focused. Use it for conditional styling:"</p>

            <Code>
                {indoc!(r#"
                    let UseFocusWithinReturn { props, is_focus_within } = use_focus_within(input);

                    view! {
                        <div
                            {..props.into_attrs()}
                            style=move || if is_focus_within.get() {
                                "border: 2px solid blue; background: lightblue;"
                            } else {
                                "border: 2px solid #ccc; background: transparent;"
                            }
                        >
                            <input type="text" />
                        </div>
                    }
                "#)}
            </Code>

            <h2 id="disabled" class="anchor">
                "Disabled State"
                <AnchorLink href="#disabled" description="Direct link to disabled state"/>
            </h2>

            <p>"When " <code>"disabled"</code> " is true, all event handlers are suppressed and " <code>"is_focus_within"</code> " remains false. Toggle the checkbox to see the effect on the demo above:"</p>

            <FormControl attr:style=form_control_row()>
                <Checkbox checked=disabled set_checked=set_disabled />
                <Label>"Disabled"</Label>
            </FormControl>

            <h2 id="input" class="anchor">
                "Input"
                <AnchorLink href="#input" description="Direct link to input"/>
            </h2>

            <p><code>"UseFocusWithinInput"</code> " fields:"</p>

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
                            <TableCell>"Disables all focus-within event handling when true."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"on_focus_within"</code></TableCell>
                            <TableCell><code>"Option<Callback<FocusWithinEvent>>"</code></TableCell>
                            <TableCell><code>"None"</code></TableCell>
                            <TableCell>"Handler called when focus enters the element tree."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"on_blur_within"</code></TableCell>
                            <TableCell><code>"Option<Callback<FocusWithinEvent>>"</code></TableCell>
                            <TableCell><code>"None"</code></TableCell>
                            <TableCell>"Handler called when focus leaves the element tree."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"on_focus_within_change"</code></TableCell>
                            <TableCell><code>"Option<Callback<bool>>"</code></TableCell>
                            <TableCell><code>"None"</code></TableCell>
                            <TableCell>"Handler called on every focus-within state transition."</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h2 id="return-value" class="anchor">
                "Return Value"
                <AnchorLink href="#return-value" description="Direct link to return value"/>
            </h2>

            <p><code>"UseFocusWithinReturn"</code> " fields:"</p>

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
                            <TableCell><code>"UseFocusWithinProps"</code></TableCell>
                            <TableCell>"Spread onto the container element via " <code>"props.into_attrs()"</code> " to wire up focusin/focusout listeners."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"is_focus_within"</code></TableCell>
                            <TableCell><code>"Signal<bool>"</code></TableCell>
                            <TableCell>"True whenever any descendant of the container is focused."</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h2 id="features" class="anchor">
                "Features"
                <AnchorLink href="#features" description="Direct link to features"/>
            </h2>

            <ul>
                <li>"Tracks " <code>"focusin"</code> "/" <code>"focusout"</code> " events for the entire element tree."</li>
                <li>"Handles focus moving between children without triggering blur."</li>
                <li>"Ignores events bubbling through portals."</li>
                <li>"Global focus listener detects DOM-removal edge cases."</li>
                <li>"Respects disabled state — handlers are suppressed when disabled."</li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "use_focus_within", link: "#use_focus_within" },
                Toc::Leaf { title: "Basic Usage", link: "#basic-usage" },
                Toc::Leaf { title: "Event Callbacks", link: "#event-callbacks" },
                Toc::Leaf { title: "is_focus_within Signal", link: "#is-focus-within" },
                Toc::Leaf { title: "Disabled State", link: "#disabled" },
                Toc::Leaf { title: "Input", link: "#input" },
                Toc::Leaf { title: "Return Value", link: "#return-value" },
                Toc::Leaf { title: "Features", link: "#features" },
            ]
        }/>
    }
}
