use indoc::indoc;
use leptos::prelude::*;

use super::demos::landmark::LandmarkDemo;
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageUseLandmark() -> impl IntoView {
    view! {
        <DocPage title="use_landmark">
            <p>
                "The "<Code inline=true>"use_landmark"</Code>" hook makes an element a landmark: a region of the page, such "
                "as the navigation, a search or the toasts, that keyboard users reach with "<Keys keys="F6"/>". Screen "
                "reader users list and jump between landmarks with their reader\u{2019}s own commands; "<Keys keys="F6"/>
                " gives everyone else the same shortcut through a page. See the "
                <Link href=routes::doc::Focus.materialize()>"Focus overview"</Link>" for the other focus building blocks."
            </p>

            <ReactAria hook="useLandmark"/>

            <Section title="Input">
                <p>"Pass a "<Code inline=true>"UseLandmarkInput"</Code>" with every field named; the Default column gives the value for fields you don\u{2019}t need."</p>
                <ApiTable kind=ApiKind::Input of="UseLandmarkInput">
                    <ApiRow name="element" ty="CapturedElement">"The landmark\u{2019}s element, captured by the caller. Required."</ApiRow>
                    <ApiRow name="role" ty="LandmarkRole">
                        "The landmark\u{2019}s role, see "<AnchorLink href="#roles">"Roles"</AnchorLink>". Required."
                    </ApiRow>
                    <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">
                        "Names the landmark. A page with several landmarks of one role must name each differently."
                    </ApiRow>
                    <ApiRow name="aria_labelledby" ty="Option<String>" default="None">"The id of a visible heading naming the landmark."</ApiRow>
                    <ApiRow name="focus" ty="Option<Callback<LandmarkDirection>>" default="None">
                        "How the landmark takes the focus when navigated to and nothing in it had the focus before, e.g. "
                        "focusing its first or last item depending on the direction. By default, the landmark element "
                        "focuses itself."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="Return">
                <ApiTable kind=ApiKind::Return of="UseLandmarkReturn">
                    <ApiRow name="props" ty="UseLandmarkProps">
                        "Spread on the landmark element: "<Code inline=true>"role"</Code>", "<Code inline=true>"aria-label"</Code>
                        ", "<Code inline=true>"aria-labelledby"</Code>" and, while the landmark itself has the focus, "
                        <Code inline=true>"tabindex=\"-1\""</Code>"."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="Example">
                <p>
                    "Pass the element the props are spread on as a "<Code inline=true>"CapturedElement"</Code>": the hook "
                    "registers it with the page\u{2019}s landmarks once it is rendered."
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::{
                            CapturedElement,
                            hooks::landmark::{LandmarkRole, UseLandmarkInput, UseLandmarkReturn, use_landmark},
                        };

                        let element = CapturedElement::new();
                        let UseLandmarkReturn { props } = use_landmark(UseLandmarkInput {
                            element,
                            role: LandmarkRole::Region,
                            aria_label: "Filters".into(),
                            aria_labelledby: None,
                            focus: None,
                        });

                        view! {
                            <div {..props.into_attrs()} {..element.attr()}>
                                // The filters.
                            </div>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <p>
                    "A mail app with three landmarks. Press "<Keys keys="F6"/>" to move to the next one and "
                    <Keys keys="Shift + F6"/>" to the previous one. A landmark you were in before takes you back to the "
                    "element you left; one you haven\u{2019}t been in takes the focus itself (outlined)."
                </p>
                <Demo
                    description="Mail app with navigation, search and message landmarks reached with F6, showing the landmark the focus is in"
                    source=include_str!("demos/landmark.rs")
                >
                    <LandmarkDemo/>
                </Demo>
            </Section>

            <Section title="Roles">
                <DocTable headers=&["LandmarkRole", "ARIA role", "For"]>
                    <TableRow>
                        <TableCell><Code inline=true>"Main"</Code></TableCell>
                        <TableCell><Code inline=true>"main"</Code></TableCell>
                        <TableCell>"The main content. At most one per page; "<Keys keys="Alt + F6"/>" goes to it."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"Navigation"</Code></TableCell>
                        <TableCell><Code inline=true>"navigation"</Code></TableCell>
                        <TableCell>"Links to the pages or sections of the site."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"Search"</Code></TableCell>
                        <TableCell><Code inline=true>"search"</Code></TableCell>
                        <TableCell>"A search form."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"Form"</Code></TableCell>
                        <TableCell><Code inline=true>"form"</Code></TableCell>
                        <TableCell>"A form that is a region of its own. It needs a name."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"Region"</Code></TableCell>
                        <TableCell><Code inline=true>"region"</Code></TableCell>
                        <TableCell>"Any other important part of the page, such as the toasts. It needs a name."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"Banner"</Code></TableCell>
                        <TableCell><Code inline=true>"banner"</Code></TableCell>
                        <TableCell>"The site header with the logo and the site-wide tools."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"Contentinfo"</Code></TableCell>
                        <TableCell><Code inline=true>"contentinfo"</Code></TableCell>
                        <TableCell>"The site footer."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"Complementary"</Code></TableCell>
                        <TableCell><Code inline=true>"complementary"</Code></TableCell>
                        <TableCell>"Content beside the main content, such as related links."</TableCell>
                    </TableRow>
                </DocTable>
                <p>
                    "In debug builds, the hook warns about a second "<Code inline=true>"Main"</Code>" landmark and about "
                    "landmarks of one role that have no name or the same name, since users couldn\u{2019}t tell them apart."
                </p>
            </Section>

            <Section title="Navigating From Code">
                <p>
                    "A "<Code inline=true>"LandmarkController"</Code>" moves between the landmarks from your code, e.g. for a "
                    "\u{201c}Skip to content\u{201d} button. It listens while it exists; drop it to stop."
                </p>
                <DocTable headers=&["Method", "Description"]>
                    <TableRow>
                        <TableCell><Code inline=true>"focus_next(from: Option<Element>) -> bool"</Code></TableCell>
                        <TableCell>"Moves to the landmark after the one containing "<Code inline=true>"from"</Code>" (default: the focused element), as "<Keys keys="F6"/>" does. Returns whether it moved."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"focus_previous(from: Option<Element>) -> bool"</Code></TableCell>
                        <TableCell>"Moves to the landmark before it, as "<Keys keys="Shift + F6"/>" does."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"navigate(direction: LandmarkDirection, from: Option<Element>) -> bool"</Code></TableCell>
                        <TableCell>"Either of the two, by "<Code inline=true>"LandmarkDirection::Forward"</Code>" or "<Code inline=true>"Backward"</Code>"."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"focus_main() -> bool"</Code></TableCell>
                        <TableCell>"Moves to the "<Code inline=true>"Main"</Code>" landmark, as "<Keys keys="Alt + F6"/>" does."</TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Keyboard">
                <KeyboardTable>
                    <KeyRow keys="F6">
                        "Moves the focus to the next landmark, after the last one back to the first. It lands on the element "
                        "that had the focus there last, else on the landmark."
                    </KeyRow>
                    <KeyRow keys="Shift + F6">"Moves the focus to the previous landmark."</KeyRow>
                    <KeyRow keys="Alt + F6">"Moves the focus to the main landmark."</KeyRow>
                </KeyboardTable>
                <p>
                    "Landmarks hidden with "<Code inline=true>"aria-hidden"</Code>", e.g. behind a modal dialog, are skipped. "
                    "Before wrapping around at the end, the hook dispatches a cancelable "
                    <Code inline=true>"react-aria-landmark-navigation"</Code>" event on the focused element (with the "
                    "direction in its "<Code inline=true>"detail"</Code>"); cancel it to keep the focus where it is."
                </p>
            </Section>

            <Section title="Accessibility">
                <ul>
                    <li>
                        "Only landmarks registered with the hook take part in "<Keys keys="F6"/>" navigation. Plain "
                        <Code inline=true>"<main>"</Code>" or "<Code inline=true>"<nav>"</Code>" elements are landmarks for "
                        "screen readers, but "<Keys keys="F6"/>" doesn\u{2019}t reach them: give each landmark of your page "
                        "the hook."
                    </li>
                    <li>
                        "Name landmarks by their purpose (\u{201c}Filters\u{201d}, not \u{201c}Filter region\u{201d}): "
                        "screen readers announce the role with the name."
                    </li>
                    <li>
                        "Use landmarks for the few large parts of a page. Too many of them make "<Keys keys="F6"/>
                        " no faster than "<Keys keys="Tab"/>"."
                    </li>
                </ul>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Focus.materialize()>"Focus overview"</Link></li>
                <li><Link href=routes::doc::Toast.materialize()>"Toast"</Link></li>
                <li><Link href=routes::doc::focus::FocusScope.materialize()>"FocusScope"</Link></li>
                <li><Link href=routes::doc::Accessibility.materialize()>"Accessibility"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
