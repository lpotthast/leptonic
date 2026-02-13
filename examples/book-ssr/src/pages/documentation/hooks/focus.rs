use indoc::indoc;
use leptonic::{components::prelude::*, hooks::*};
use leptos::prelude::*;
use ringbuf::{
    traits::{Consumer, Observer, RingBuffer},
    HeapRb,
};

use crate::pages::documentation::{article::Article, doc_styles::*, toc::Toc};

#[component]
pub fn PageUseFocus() -> impl IntoView {
    let (events, set_events) = signal(HeapRb::<Oco<'static, str>>::new(50));
    let (disabled, set_disabled) = signal(false);
    let (is_focused, set_is_focused) = signal(false);

    let string = Memo::new(move |_| {
        events.with(|events| {
            let mut result = String::new();
            for e in events.iter().rev() {
                result.push_str(e.as_str());
                result.push('\n');
            }
            result
        })
    });

    let UseFocusReturn { props } = use_focus(UseFocusInput {
        disabled: disabled.into(),
        on_focus: Some(Callback::new(move |e| {
            set_events.update(|events| {
                events.push_overwrite(Oco::Owned(format!("Focus: {e:?}")));
            });
        })),
        on_blur: Some(Callback::new(move |e| {
            set_events.update(|events| {
                events.push_overwrite(Oco::Owned(format!("Blur: {e:?}")));
            });
        })),
        on_focus_change: Some(Callback::new(move |focused: bool| {
            set_is_focused.set(focused);
            set_events.update(|events| {
                events.push_overwrite(Oco::Owned(format!("Changed: {focused}")));
            });
        })),
    });

    view! {
        <Article>
            <h1 id="use-focus" class="anchor">
                "use_focus"
                <AnchorLink href="#use-focus" description="Direct link to section: use_focus"/>
            </h1>

            <p>"Track element focus. Fires callbacks when the element itself receives or loses focus (not descendants — see " <code>"use_focus_within"</code> " for that)."</p>

            <h2 id="basic-usage" class="anchor">
                "Basic Usage"
                <AnchorLink href="#basic-usage" description="Direct link to basic usage"/>
            </h2>

            <Code>
                {indoc!(r#"
                    let UseFocusReturn { props } = use_focus(UseFocusInput {
                        disabled: Signal::derive(|| false),
                        on_focus: None,
                        on_blur: None,
                        on_focus_change: Some(Callback::new(|focused: bool| {
                            // React to focus changes
                        })),
                    });

                    view! {
                        <div tabindex=0 {..props.into_attrs()}>
                            "Focusable element"
                        </div>
                    }
                "#)}
            </Code>

            <p>"Click the element below or use Tab to focus it:"</p>

            <div
                tabindex=0
                {..props.into_attrs()}
                style=move || if is_focused.get() { demo_container_active() } else { demo_container_inactive() }
            >
                <strong style=move || if is_focused.get() { state_active() } else { state_inactive() }>
                    { move || if is_focused.get() { "Focused" } else { "Not focused" } }
                </strong>
                " — click here or press Tab"
            </div>

            <h2 id="event-callbacks" class="anchor">
                "Event Callbacks"
                <AnchorLink href="#event-callbacks" description="Direct link to event callbacks"/>
            </h2>

            <p>"The hook provides three callbacks: " <code>"on_focus"</code> " fires when the element receives focus, "
                <code>"on_blur"</code> " fires when focus leaves, and " <code>"on_focus_change"</code>
                " fires on every transition with a " <code>"bool"</code> " indicating the new state. All three are demonstrated in the log below:"</p>

            <p>"Last " { move || events.with(|events| events.occupied_len()) } " events: "</p>

            <pre style=event_log()>
                { move || string.get() }
            </pre>

            <h2 id="disabled" class="anchor">
                "Disabled State"
                <AnchorLink href="#disabled" description="Direct link to disabled state"/>
            </h2>

            <p>"When " <code>"disabled"</code> " is true, all event handlers are suppressed. Toggle the checkbox to see the effect:"</p>

            <FormControl attr:style=form_control_row()>
                <Checkbox checked=disabled set_checked=set_disabled />
                <Label>"Disabled"</Label>
            </FormControl>

            <h2 id="input" class="anchor">
                "Input"
                <AnchorLink href="#input" description="Direct link to input"/>
            </h2>

            <p><code>"UseFocusInput"</code> " fields:"</p>

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
                            <TableCell>"Disables all focus event handling when true."</TableCell>
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
                            <TableCell>"Handler called on every focus state transition."</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h2 id="return-value" class="anchor">
                "Return Value"
                <AnchorLink href="#return-value" description="Direct link to return value"/>
            </h2>

            <p><code>"UseFocusReturn"</code> " fields:"</p>

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
                            <TableCell><code>"UseFocusProps"</code></TableCell>
                            <TableCell>"Spread onto the target element via " <code>"props.into_attrs()"</code> " to wire up focus/blur listeners."</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h2 id="features" class="anchor">
                "Features"
                <AnchorLink href="#features" description="Direct link to features"/>
            </h2>

            <ul>
                <li>"Fires only when the element itself is focused/blurred (not descendants)."</li>
                <li>"Verifies " <code>"document.activeElement"</code> " matches the target before firing."</li>
                <li>"Synthetic blur support for Firefox (form elements disabled while focused)."</li>
                <li>"Respects disabled state — handlers are suppressed when disabled."</li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "use_focus", link: "#use-focus" },
                Toc::Leaf { title: "Basic Usage", link: "#basic-usage" },
                Toc::Leaf { title: "Event Callbacks", link: "#event-callbacks" },
                Toc::Leaf { title: "Disabled State", link: "#disabled" },
                Toc::Leaf { title: "Input", link: "#input" },
                Toc::Leaf { title: "Return Value", link: "#return-value" },
                Toc::Leaf { title: "Features", link: "#features" },
            ]
        }/>
    }
}
