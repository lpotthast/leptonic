use indoc::indoc;
use leptos::prelude::*;

use super::demos::{
    transitions_collapse::TransitionsCollapseDemo,
    transitions_collapse_x::TransitionsCollapseXDemo, transitions_compare::TransitionsCompareDemo,
    transitions_fade::TransitionsFadeDemo, transitions_grow::TransitionsGrowDemo,
    transitions_slide::TransitionsSlideDemo, transitions_zoom::TransitionsZoomDemo,
};
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageTransitions() -> impl IntoView {
    view! {
        <DocPage title="Transitions">
            <p>
                "Transitions animate content between a shown and a hidden state, so that a change on screen is easy to "
                "follow instead of happening in a single frame. leptonic has five transition components: "
                <Code inline=true>"Collapse"</Code>", "<Code inline=true>"Fade"</Code>", "<Code inline=true>"Grow"</Code>
                ", "<Code inline=true>"Slide"</Code>" and "<Code inline=true>"Zoom"</Code>". Each wraps its children in "
                "a "<Code inline=true>"<div>"</Code>" that has the "<Code inline=true>"data-shown"</Code>" attribute "
                "while "<Code inline=true>"is_shown"</Code>" is true; the theme\u{2019}s CSS transitions do the "
                "animating. See the "<Link href=routes::doc::Animation.materialize()>"Animation overview"</Link>
                " for the other animation building blocks."
            </p>
            <p>
                "The children always stay mounted. While hidden, the wrapper is "<Code inline=true>"inert"</Code>
                ", so its content can\u{2019}t be focused, clicked or read by screen readers, and once the transition "
                "ended, the theme makes it invisible. Pass "<Code inline=true>"is_shown"</Code>" as a plain value or any "
                "signal."
            </p>

            <Demo description="All five transitions toggled together" source=include_str!("demos/transitions_compare.rs")>
                <TransitionsCompareDemo/>
            </Demo>

            <p>
                "For elements that are added to and removed from the DOM, such as overlays, use the animation hooks "
                "instead; the "<Link href=format!("{}#relationships", routes::doc::Animation.materialize())>"Animation overview"</Link>
                " compares both."
            </p>

            <Section title="Collapse">
                <p>
                    "Collapse animates the size of its content along one axis, so content around it moves smoothly instead "
                    "of jumping. The theme makes the wrapper a grid whose single track holds the content and transitions "
                    "that track between "<Code inline=true>"0fr"</Code>" (nothing) and "<Code inline=true>"1fr"</Code>
                    " (the content\u{2019}s full size); the content clips what doesn\u{2019}t fit yet."
                </p>
                <p>
                    "As the track follows the content, an expanded Collapse grows and shrinks with it, for example when "
                    "text wraps in a narrower window. An initially expanded Collapse is rendered at its full size on the "
                    "server already."
                </p>
                <Demo description="Details panel expanding and collapsing vertically" source=include_str!("demos/transitions_collapse.rs")>
                    <TransitionsCollapseDemo/>
                </Demo>

                <Section title="Props" id="collapse-props">
                    <ApiTable kind=ApiKind::Props of="Collapse">
                        <ApiRow name="is_shown" ty="Signal<bool>">
                            "Whether the content is expanded: a value or any signal. Required."
                        </ApiRow>
                        <ApiRow name="axis" ty="CollapseAxis" default="Y">
                            <Code inline=true>"Y"</Code>" animates the height, "<Code inline=true>"X"</Code>" the width."
                        </ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                            "Additional classes and styles of the wrapper."
                        </ApiRow>
                        <ApiRow name="children" ty="Children">"The collapsible content. Required."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Horizontal Axis">
                    <p>
                        "With "<Code inline=true>"axis=CollapseAxis::X"</Code>", Collapse animates the width. Give the "
                        "content a fixed width, as the sidebar in the demo has: content without one is as wide as the "
                        "animated track, so its text wraps anew at every step of the animation."
                    </p>
                    <Demo description="Sidebar collapsing horizontally" source=include_str!("demos/transitions_collapse_x.rs")>
                        <TransitionsCollapseXDemo/>
                    </Demo>
                </Section>
            </Section>

            <Section title="Fade">
                <p>
                    "Fade animates the opacity of its content: fully opaque while "<Code inline=true>"is_shown"</Code>
                    " is true, transparent otherwise. The content keeps its space while hidden, so nothing around it moves."
                </p>
                <Demo description="Panel fading out and in" source=include_str!("demos/transitions_fade.rs")>
                    <TransitionsFadeDemo/>
                </Demo>

                <Section title="Props" id="fade-props">
                    <ApiTable kind=ApiKind::Props of="Fade">
                        <ApiRow name="is_shown" ty="Signal<bool>">"Whether the content is shown: a value or any signal. Required."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Additional classes and styles of the wrapper."</ApiRow>
                        <ApiRow name="children" ty="Children">"The faded content. Required."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Grow">
                <p>
                    "Grow scales its content up from its center while fading it in, and back down while fading it out. "
                    "Like Fade, it keeps the content\u{2019}s space while hidden."
                </p>
                <Demo description="Panel growing in and shrinking out" source=include_str!("demos/transitions_grow.rs")>
                    <TransitionsGrowDemo/>
                </Demo>

                <Section title="Props" id="grow-props">
                    <ApiTable kind=ApiKind::Props of="Grow">
                        <ApiRow name="is_shown" ty="Signal<bool>">"Whether the content is shown: a value or any signal. Required."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Additional classes and styles of the wrapper."</ApiRow>
                        <ApiRow name="children" ty="Children">"The animated content. Required."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Slide">
                <p>
                    "Slide moves its content in from below while fading it in. While hidden, the content is moved down by "
                    "its own height and may overlap what follows it; the demo clips it with an "
                    <Code inline=true>"overflow: hidden"</Code>" container."
                </p>
                <Demo description="Panel sliding in from below and out again" source=include_str!("demos/transitions_slide.rs")>
                    <TransitionsSlideDemo/>
                </Demo>

                <Section title="Props" id="slide-props">
                    <ApiTable kind=ApiKind::Props of="Slide">
                        <ApiRow name="is_shown" ty="Signal<bool>">"Whether the content is shown: a value or any signal. Required."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Additional classes and styles of the wrapper."</ApiRow>
                        <ApiRow name="children" ty="Children">"The animated content. Required."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Zoom">
                <p>
                    "Zoom scales its content up from nothing to its full size, and back to nothing. Unlike Grow, it "
                    "doesn\u{2019}t fade."
                </p>
                <Demo description="Panel zooming in and out" source=include_str!("demos/transitions_zoom.rs")>
                    <TransitionsZoomDemo/>
                </Demo>

                <Section title="Props" id="zoom-props">
                    <ApiTable kind=ApiKind::Props of="Zoom">
                        <ApiRow name="is_shown" ty="Signal<bool>">"Whether the content is shown: a value or any signal. Required."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Additional classes and styles of the wrapper."</ApiRow>
                        <ApiRow name="children" ty="Children">"The animated content. Required."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Accessibility">
                <ul>
                    <li>
                        <b>"Hidden content is unreachable."</b>" A hidden transition is "<Code inline=true>"inert"</Code>
                        ": its content leaves the tab order and the accessibility tree and ignores clicks, even while it is "
                        "still animating out. Once the transition ended, it is also "<Code inline=true>"visibility: hidden"</Code>"."
                    </li>
                    <li>
                        <b>"Reduced motion is respected."</b>" When the user prefers reduced motion, the theme turns the "
                        "transitions off: content appears and disappears at once."
                    </li>
                    <li>
                        <b>"Announce the state on the trigger."</b>" The transitions render no ARIA attributes. A button "
                        "that expands content should set "<Code inline=true>"aria-expanded"</Code>
                        ", as the Collapse demos do with "<Code inline=true>"Button"</Code>"\u{2019}s "
                        <Code inline=true>"aria_expanded"</Code>" prop. For the complete disclosure ARIA pattern, use a "
                        <Link href=routes::doc::Disclosure.materialize()>"Disclosure"</Link>"."
                    </li>
                </ul>
            </Section>

            <Section title="Data Attributes">
                <ApiTable kind=ApiKind::DataAttributes>
                    <ApiRow name="data-shown" ty="true">"Set on the wrapper while the content is shown."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="Styling">
                <p>
                    "The wrappers have the classes "<Code inline=true>"leptonic-fade"</Code>", "
                    <Code inline=true>"leptonic-grow"</Code>", "<Code inline=true>"leptonic-slide"</Code>", "
                    <Code inline=true>"leptonic-zoom"</Code>" and "<Code inline=true>"leptonic-collapse"</Code>
                    " (with "<Code inline=true>"leptonic-collapse-y"</Code>" or "<Code inline=true>"leptonic-collapse-x"</Code>
                    " for the axis). Collapse wraps its children in a second "<Code inline=true>"<div>"</Code>", "
                    <Code inline=true>".leptonic-collapse-content"</Code>", which clips them."
                </p>
                <p>
                    "The theme has no CSS variables: every transition takes 0.2s. It ends with a delayed "
                    <Code inline=true>"visibility"</Code>" change, which hides the content only after it animated out. "
                    "When you change a duration through "<Code inline=true>"classes"</Code>", delay the "
                    <Code inline=true>"visibility"</Code>" by as much:"
                </p>
                <Code language=Language::Css>
                    {indoc!(r"
                        .slow.leptonic-fade { transition: opacity 0.6s, visibility 0s linear 0.6s; }
                        .slow.leptonic-fade[data-shown] { transition: opacity 0.6s, visibility 0s; }
                        @media (prefers-reduced-motion: reduce) {
                            .slow.leptonic-fade, .slow.leptonic-fade[data-shown] { transition: none; }
                        }
                    ")}
                </Code>
                <p>"This is the complete theme stylesheet:"</p>
                <Code language=Language::Css>{theme_scss!("transition")}</Code>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Animation.materialize()>"Animation"</Link></li>
                <li><Link href=routes::doc::Disclosure.materialize()>"Disclosure"</Link></li>
                <li><Link href=routes::doc::Drawer.materialize()>"Drawer"</Link></li>
                <li><Link href=routes::doc::OverlayBehavior.materialize()>"Overlay Behavior"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
