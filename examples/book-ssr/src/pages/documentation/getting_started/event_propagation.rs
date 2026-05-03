use indoc::indoc;
use leptonic::{components::prelude::*, hooks::LinkTarget};
use leptos::prelude::*;

use crate::pages::documentation::{article::Article, toc::Toc};

#[component]
pub fn PageEventPropagation() -> impl IntoView {
    view! {
        <Article>
            <h1 id="event-propagation">
                "Event Propagation"
                <AnchorLink href="#event-propagation" description="Direct link to section: Event Propagation"/>
            </h1>

            <p>
                "Leptonic's hook layer is based on "
                <LinkExt href="https://react-spectrum.adobe.com/react-aria/" target=LinkTarget::_Blank>"React Aria"</LinkExt>
                " by Adobe. One of its most important design decisions — which Leptonic adopts — is to "
                <b>"invert the default behavior of event propagation"</b>
                ". Instead of events bubbling up the DOM tree by default (the web platform standard), events are "
                <b>"stopped by default"</b>
                ". Handlers must explicitly call "
                <code>"continue_propagation()"</code>
                " if they want the event to bubble."
            </p>

            <h2 id="the-problem">
                "The Problem: Event Bubbling Is a Footgun in Component Libraries"
                <AnchorLink href="#the-problem" description="Direct link to section: The Problem"/>
            </h2>

            <p>
                "The web platform's default is that events bubble up the DOM tree. "
                "In vanilla HTML this is mostly fine, but in component-based UI libraries it creates a class of subtle, hard-to-debug issues:"
            </p>

            <h3 id="accidental-double-handling">"Accidental double handling"</h3>
            <p>
                "A " <code>"<Button>"</code> " inside a " <code>"<ListItem>"</code> " that is itself clickable. "
                "The user clicks the button, and " <i>"both"</i> " the button's " <code>"on_press"</code>
                " and the list item's " <code>"on_press"</code> " fire. "
                "The developer must remember to call " <code>"stop_propagation()"</code>
                " in the button handler — and almost nobody does."
            </p>

            <h3 id="third-party-composition">"Third-party composition breaks"</h3>
            <p>
                "When you compose components from different authors (or different teams), neither side knows what the other is doing. "
                "A menu trigger inside a toolbar, a checkbox inside a draggable row — bubbling means the outer component reacts to events it should not."
            </p>

            <h3 id="wrong-default">"It is an opt-out default for something that is almost always wrong"</h3>
            <p>
                "In practice, " <i>"most"</i> " event handlers handle the event and do "
                <b>"not"</b> " want it to continue propagating. The exceptions are rare."
            </p>

            <h2 id="the-inversion">
                "The Inversion: Stop by Default, Opt In to Propagation"
                <AnchorLink href="#the-inversion" description="Direct link to section: The Inversion"/>
            </h2>

            <p>
                "Leptonic flips the default. When a hook-provided event handler fires, propagation is stopped automatically "
                <i>"after"</i> " the handler returns — unless the handler called "
                <code>"continue_propagation()"</code> " on the event."
            </p>

            <p>"Conceptually, this is what happens inside every hook-provided event handler:"</p>

            <pre>
                <Code language=Language::Rust>
                    {indoc!(r"
                        let should_stop_propagation = true; // default: STOP

                        handler(event); // your handler runs

                        if should_stop_propagation {
                            event.stop_propagation(); // actually stop it on the real DOM event
                        }
                    ")}
                </Code>
            </pre>

            <p>
                "Your handler receives a wrapped event where:"
            </p>
            <ul>
                <li>
                    <code>"stop_propagation()"</code>
                    " is unnecessary — it is already the default."
                </li>
                <li>
                    <code>"continue_propagation()"</code>
                    " is the escape hatch — call it to let the event bubble."
                </li>
            </ul>

            <h2 id="what-this-enables">
                "What This Enables"
                <AnchorLink href="#what-this-enables" description="Direct link to section: What This Enables"/>
            </h2>

            <h3 id="safe-composition">"Safe composition by default"</h3>
            <p>
                "Components are isolated event handlers. "
                "A " <code>"<Button>"</code> " inside a " <code>"<Dialog>"</code> " inside a " <code>"<Popover>"</code>
                " — each layer only sees events directed at it. No accidental cross-talk."
            </p>

            <h3 id="unhandled-event-pattern">"The unhandled event pattern"</h3>
            <p>
                "A handler that does not know what to do with an event can call "
                <code>"continue_propagation()"</code>
                " to delegate upward. "
                "For example, a keyboard handler that receives an unknown key can let it bubble so a parent can handle it."
            </p>
            <p>
                "This creates a " <b>"semantic protocol"</b>
                ": propagation means \"I did not handle this, someone else should.\" "
                "In the standard web model, propagation means nothing — it is just the default. "
                "Leptonic makes propagation " <i>"intentional"</i> " and " <i>"meaningful"</i> "."
            </p>

            <h3 id="no-defensive-calls">"No more defensive stop_propagation() calls"</h3>
            <p>
                "In traditional codebases, developers must remember to add "
                <code>"stop_propagation()"</code>
                " in the right places. Missing one creates a bug. Leptonic eliminates this entire class of errors."
            </p>

            <h3 id="predictable-ownership">"Predictable event ownership"</h3>
            <p>
                "At any point in the tree, you can reason about which component handles an event. "
                "The answer is always \"the closest one with a handler\" — unless that handler explicitly delegates upward. "
                "This makes debugging dramatically easier."
            </p>

            <h3 id="custom-events">"Custom event types become natural"</h3>
            <p>
                "Leptonic's " <code>"PressEvent"</code> ", " <code>"HoverEvent"</code> ", "
                <code>"MoveEvent"</code> " etc. are not DOM events — they are synthetic. "
                "Since they already wrap the native event, there is no cost to also controlling propagation semantics. "
                "The " <code>"PressEvent"</code> " type does not even have " <code>"stop_propagation()"</code>
                " — only " <code>"continue_propagation()"</code> "."
            </p>

            <h2 id="trade-offs">
                "Trade-offs"
                <AnchorLink href="#trade-offs" description="Direct link to section: Trade-offs"/>
            </h2>

            <p>
                "Some patterns rely on bubbling for legitimate purposes: document-level click tracking, idle detection, or analytics. "
                "For these cases, use " <b>"capture-phase listeners"</b> ". "
                "Capture runs " <i>"before"</i> " the target phase and is not affected by "
                <code>"stop_propagation()"</code> "."
            </p>

            <p>
                "The philosophical stance is clear: "
                <b>"modularity and isolation are more important defaults than bubbling"</b>
                ", because the common case is handling events locally, and the rare case (wanting bubbling) can be opted into explicitly."
            </p>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Event Propagation", link: "#event-propagation" },
                Toc::Leaf { title: "The Problem", link: "#the-problem" },
                Toc::Leaf { title: "The Inversion", link: "#the-inversion" },
                Toc::Leaf { title: "What This Enables", link: "#what-this-enables" },
                Toc::Leaf { title: "Trade-offs", link: "#trade-offs" },
            ]
        }/>
    }
}
