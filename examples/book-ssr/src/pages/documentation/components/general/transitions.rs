use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::{
    transitions_collapse::TransitionsCollapseDemo,
    transitions_collapse_x::TransitionsCollapseXDemo, transitions_compare::TransitionsCompareDemo,
    transitions_fade::TransitionsFadeDemo, transitions_grow::TransitionsGrowDemo,
    transitions_slide::TransitionsSlideDemo, transitions_zoom::TransitionsZoomDemo,
};
use crate::{kit::*, routes};

#[component]
pub fn PageTransitions() -> impl IntoView {
    view! {
        <DocPage title="Transitions">
            <p>
                "Transitions animate content between a shown and a hidden state, so that a change on screen is easy to "
                "follow instead of happening in a single frame. leptonic has five transition components: "
                <Code inline=true>"Collapse"</Code>", "<Code inline=true>"Fade"</Code>", "<Code inline=true>"Grow"</Code>
                ", "<Code inline=true>"Slide"</Code>" and "<Code inline=true>"Zoom"</Code>". Each wraps its children in "
                "a "<Code inline=true>"<div>"</Code>" and switches that element between both states when a "
                <Code inline=true>"Signal<bool>"</Code>" changes; the CSS transitions of the element do the animating. See the "
                <Link href=routes::doc::Animation.materialize()>"Animation overview"</Link>" for the other animation building blocks."
            </p>
            <p>
                "The children always stay mounted: a hidden transition is still in the DOM, only collapsed, transparent "
                "or transformed. The default theme styles "<Code inline=true>"Collapse"</Code>" and "
                <Code inline=true>"Fade"</Code>". "<Code inline=true>"Grow"</Code>", "<Code inline=true>"Slide"</Code>
                " and "<Code inline=true>"Zoom"</Code>" only render a "<Code inline=true>"data-in"</Code>
                " attribute and have no styles in the theme yet, so they don\u{2019}t animate until you style them. The "
                "demos on this page do that with their own classes."
            </p>

            <Demo description="All five transitions toggled together" source=include_str!("demos/transitions_compare.rs")>
                <TransitionsCompareDemo/>
            </Demo>

            <p>
                "Transitions keep their children mounted. For elements that are added to and removed from the DOM, such "
                "as overlays, use the animation hooks instead; the "
                <Link href=format!("{}#relationships", routes::doc::Animation.materialize())>"Animation overview"</Link>
                " compares both."
            </p>

            <Section title="Collapse">
                <p>
                    "Collapse animates the size of its content along one axis, so content around it moves smoothly instead "
                    "of jumping. When "<Code inline=true>"show"</Code>" turns true, it measures the content\u{2019}s "
                    <Code inline=true>"scrollHeight"</Code>" (or "<Code inline=true>"scrollWidth"</Code>") and sets it as "
                    "the wrapper\u{2019}s "<Code inline=true>"height"</Code>" (or "<Code inline=true>"width"</Code>
                    "); when it turns false, the size goes back to "<Code inline=true>"0px"</Code>". The theme transitions "
                    "that property over 0.3s and clips the overflow."
                </p>
                <p>
                    "While collapsed, the content is hidden with "<Code inline=true>"visibility: hidden"</Code>". The theme "
                    "switches the visibility without a delay, so when the content collapses, it disappears at once and only "
                    "the empty space animates closed. Expanding shows the content right away and reveals it as the wrapper grows."
                </p>
                <Demo description="Details panel expanding and collapsing vertically" source=include_str!("demos/transitions_collapse.rs")>
                    <TransitionsCollapseDemo/>
                </Demo>

                <Section title="Props" id="collapse-props">
                    <ApiTable kind=ApiKind::Props of="Collapse">
                        <ApiRow name="show" ty="Signal<bool>">"Whether the content is expanded. Required."</ApiRow>
                        <ApiRow name="axis" ty="CollapseAxis" default="Y">
                            <Code inline=true>"Y"</Code>" animates the height, "<Code inline=true>"X"</Code>" the width."
                        </ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                            "Additional classes and styles of the wrapper. Collapse adds its own "
                            <Code inline=true>"height"</Code>" / "<Code inline=true>"min-height"</Code>" (or width) declarations."
                        </ApiRow>
                        <ApiRow name="children" ty="Children">"The collapsible content."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Horizontal Axis">
                    <p>
                        "With "<Code inline=true>"axis=CollapseAxis::X"</Code>", Collapse animates the width. Give the content "
                        "a fixed width: inside a zero-width wrapper, content without one shrinks to its narrowest layout "
                        "and is measured that way."
                    </p>
                    <Demo description="Sidebar collapsing horizontally" source=include_str!("demos/transitions_collapse_x.rs")>
                        <TransitionsCollapseXDemo/>
                    </Demo>
                </Section>

                <Section title="Measured Size">
                    <p>
                        "Collapse measures the content only when "<Code inline=true>"show"</Code>" changes (and once its "
                        "element is mounted). The expanded size is a fixed pixel value, not "<Code inline=true>"auto"</Code>
                        ": if the content grows or shrinks while expanded, for example because the window became narrower "
                        "and text wraps, the wrapper keeps its old size and clips or leaves empty space until the next toggle."
                    </p>
                    <p>
                        "During server-side rendering there is nothing to measure, so an initially expanded Collapse renders "
                        "with a size of "<Code inline=true>"0px"</Code>" and expands once the page is hydrated."
                    </p>
                </Section>
            </Section>

            <Section title="Fade">
                <p>
                    "Fade animates the opacity of its content: fully opaque while "<Code inline=true>"inn"</Code>
                    " is true, transparent otherwise. The theme transitions the opacity over 0.2s. The content keeps its "
                    "space while hidden, so nothing around it moves."
                </p>
                <Demo description="Panel fading out and in" source=include_str!("demos/transitions_fade.rs")>
                    <TransitionsFadeDemo/>
                </Demo>

                <Section title="Props" id="fade-props">
                    <ApiTable kind=ApiKind::Props of="Fade">
                        <ApiRow name="inn" ty="Signal<bool>">
                            "Whether the content is shown, rendered as the "<Code inline=true>"data-in"</Code>
                            " attribute. Required. ("<Code inline=true>"in"</Code>" is a Rust keyword.)"
                        </ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Additional classes and styles of the wrapper."</ApiRow>
                        <ApiRow name="children" ty="Children">"The faded content."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Grow">
                <p>
                    "Grow is meant to scale its content up while fading it in. It renders "<Code inline=true>"data-in"</Code>
                    " while "<Code inline=true>"inn"</Code>" is true and has the class "<Code inline=true>"leptonic-grow"</Code>
                    ", which the theme doesn\u{2019}t style yet. The demo animates "<Code inline=true>"opacity"</Code>" and "
                    <Code inline=true>"transform: scale(..)"</Code>" through its own class; open \u{201c}View styles\u{201d} "
                    "to copy the CSS."
                </p>
                <Demo description="Panel growing in and shrinking out" source=include_str!("demos/transitions_grow.rs")>
                    <TransitionsGrowDemo/>
                </Demo>

                <Section title="Props" id="grow-props">
                    <ApiTable kind=ApiKind::Props of="Grow">
                        <ApiRow name="inn" ty="Signal<bool>">
                            "Whether the content is shown, rendered as the "<Code inline=true>"data-in"</Code>" attribute. "
                            "Required. Unlike Fade\u{2019}s, this prop doesn\u{2019}t convert: pass a "
                            <Code inline=true>"Signal<bool>"</Code>", e.g. "<Code inline=true>"inn=visible.into()"</Code>"."
                        </ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Additional classes and styles of the wrapper."</ApiRow>
                        <ApiRow name="children" ty="Children">"The animated content."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Slide">
                <p>
                    "Slide is meant to move its content in from outside its container. Like Grow, it renders "
                    <Code inline=true>"data-in"</Code>" and the class "<Code inline=true>"leptonic-slide"</Code>
                    " without theme styles. The demo translates the panel horizontally and clips it with an "
                    <Code inline=true>"overflow: hidden"</Code>" container."
                </p>
                <Demo description="Panel sliding in from the left and out again" source=include_str!("demos/transitions_slide.rs")>
                    <TransitionsSlideDemo/>
                </Demo>

                <Section title="Props" id="slide-props">
                    <ApiTable kind=ApiKind::Props of="Slide">
                        <ApiRow name="inn" ty="Signal<bool>">
                            "Whether the content is shown, rendered as the "<Code inline=true>"data-in"</Code>" attribute. "
                            "Required. Pass a "<Code inline=true>"Signal<bool>"</Code>" (no conversion)."
                        </ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Additional classes and styles of the wrapper."</ApiRow>
                        <ApiRow name="children" ty="Children">"The animated content."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Zoom">
                <p>
                    "Zoom is meant to scale its content up from nothing. It renders "<Code inline=true>"data-in"</Code>
                    " and the class "<Code inline=true>"leptonic-zoom"</Code>" without theme styles; the demo scales the "
                    "panel from "<Code inline=true>"scale(0)"</Code>" to its full size."
                </p>
                <Demo description="Panel zooming in and out" source=include_str!("demos/transitions_zoom.rs")>
                    <TransitionsZoomDemo/>
                </Demo>

                <Section title="Props" id="zoom-props">
                    <ApiTable kind=ApiKind::Props of="Zoom">
                        <ApiRow name="inn" ty="Signal<bool>">
                            "Whether the content is shown, rendered as the "<Code inline=true>"data-in"</Code>" attribute. "
                            "Required. Pass a "<Code inline=true>"Signal<bool>"</Code>" (no conversion)."
                        </ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Additional classes and styles of the wrapper."</ApiRow>
                        <ApiRow name="children" ty="Children">"The animated content."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Accessibility">
                <ul>
                    <li>
                        <b>"Hidden content is not always hidden from everyone."</b>" Collapse hides its content with "
                        <Code inline=true>"visibility: hidden"</Code>" while collapsed, which removes it from the tab order "
                        "and the accessibility tree. Fade only makes it transparent: a faded-out element can still be "
                        "focused, clicked and read by screen readers. Add "<Code inline=true>"visibility"</Code>
                        " to the transition, as the Grow, Slide and Zoom demos do, if hidden content must be unreachable."
                    </li>
                    <li>
                        <b>"Reduced motion is not handled."</b>" Neither the components nor the theme respect "
                        <Code inline=true>"prefers-reduced-motion"</Code>". Turn the transitions off in your own "
                        "stylesheet for users who ask for less motion (the Grow, Slide and Zoom demo styles do the same)."
                    </li>
                    <li>
                        <b>"Announce the state on the trigger."</b>" The transitions render no ARIA attributes. A button "
                        "that expands content should set "<Code inline=true>"aria-expanded"</Code>
                        ", as the Collapse demos do with "<Code inline=true>"Button"</Code>"\u{2019}s "
                        <Code inline=true>"aria_expanded"</Code>" prop. For the complete disclosure ARIA pattern, use a "
                        <Link href=routes::doc::Disclosure.materialize()>"Disclosure"</Link>"."
                    </li>
                </ul>
                <Code language=Language::Css>
                    {indoc!(r"
                        .leptonic-fade {
                            visibility: hidden;
                            transition: opacity 0.2s cubic-bezier(0.4, 0, 0.2, 1), visibility 0s 0.2s;
                        }
                        .leptonic-fade[data-in] {
                            visibility: visible;
                            transition: opacity 0.2s cubic-bezier(0.4, 0, 0.2, 1), visibility 0s;
                        }
                        @media (prefers-reduced-motion: reduce) {
                            .leptonic-collapse.width,
                            .leptonic-collapse.height,
                            .leptonic-fade,
                            .leptonic-fade[data-in] {
                                transition: none;
                            }
                        }
                    ")}
                </Code>
            </Section>

            <Section title="Styling">
                <p>
                    "The wrappers have the classes "<Code inline=true>"leptonic-collapse"</Code>" (plus "
                    <Code inline=true>"height"</Code>" or "<Code inline=true>"width"</Code>" for the axis), "
                    <Code inline=true>"leptonic-fade"</Code>", "<Code inline=true>"leptonic-grow"</Code>", "
                    <Code inline=true>"leptonic-slide"</Code>" and "<Code inline=true>"leptonic-zoom"</Code>
                    ". Collapse wraps its children in a second "<Code inline=true>"<div class=\"content\">"</Code>
                    " that has the class "<Code inline=true>"show"</Code>" while expanded; the others put "
                    <Code inline=true>"data-in"</Code>" on the wrapper while shown."
                </p>
                <p>
                    "The transition theme has no CSS variables; durations and easing are fixed in its stylesheet. Override "
                    "the "<Code inline=true>"transition"</Code>" property in your own stylesheet, or style a single "
                    "instance through its "<Code inline=true>"classes"</Code>". This is the complete theme stylesheet:"
                </p>
                <Code language=Language::Css>{theme_scss!("transition")}</Code>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Animation.materialize()>"Animation"</Link></li>
                <li><Link href=routes::doc::Disclosure.materialize()>"Disclosure"</Link></li>
                <li><Link href=routes::doc::OverlayBehavior.materialize()>"Overlay Behavior"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
