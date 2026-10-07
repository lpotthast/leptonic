use indoc::indoc;
use leptos::prelude::*;

use super::demos::visually_hidden::VisuallyHiddenDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageUseVisuallyHidden() -> impl IntoView {
    view! {
        <DocPage title="use_visually_hidden">
            <p>
                "Some content only screen reader users need: the name of an icon-only button, the context of a "
                "\u{201c}Read more\u{201d} link, a skip link. Hiding it with "<Code inline=true>"display: none"</Code>
                " would hide it from screen readers too. The "<Code inline=true>"use_visually_hidden"</Code>
                " hook hides an element visually while keeping it in the accessibility tree, and can show a focusable "
                "element while it has focus. The "<Link href=routes::doc::screen_readers::VisuallyHidden.materialize()>"VisuallyHidden"</Link>
                " atom renders such an element for you. Together with the "
                <Link href=routes::doc::screen_readers::LiveAnnouncer.materialize()>"live_announcer"</Link>
                " and "<Link href=routes::doc::screen_readers::UseDescription.materialize()>"use_description"</Link>
                ", they are the building blocks for screen readers."
            </p>

            <ReactAria hook="VisuallyHidden"/>

            <Section title="Input">
                <ApiTable kind=ApiKind::Input of="UseVisuallyHiddenInput">
                    <ApiRow name="is_focusable" ty="bool" default="false">
                        "Show the element while focus is within it, e.g. for a skip link."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="Return">
                <ApiTable kind=ApiKind::Return of="UseVisuallyHiddenReturn">
                    <ApiRow name="props" ty="UseVisuallyHiddenProps">
                        "Attributes for the hidden element. Spread with "<Code inline=true>"{..props.into_attrs()}"</Code>"."
                    </ApiRow>
                </ApiTable>

                <ApiTable kind=ApiKind::Fields of="UseVisuallyHiddenProps">
                    <ApiRow name="style" ty="Signal<Option<&'static str>>">
                        "The hiding styles, rendered as the element\u{2019}s "<Code inline=true>"style"</Code>
                        " attribute. "<Code inline=true>"None"</Code>" while a focusable element shows."
                    </ApiRow>
                    <ApiRow name="on_focusin, on_focusout" ty="EventHandler<FocusEvent>">
                        "Track whether focus is within the element. They do nothing unless "
                        <Code inline=true>"is_focusable"</Code>" is set."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::hooks::{IntoAttrs, UseVisuallyHiddenInput, use_visually_hidden};

                        let hidden = use_visually_hidden(UseVisuallyHiddenInput::default());

                        view! {
                            <button>
                                <svg aria-hidden="true">/* trash icon */</svg>
                                <span {..hidden.props.into_attrs()}>"Delete"</span>
                            </button>
                        }
                    "#)}
                </Code>
                <p>
                    "The props set the element\u{2019}s "<Code inline=true>"style"</Code>" attribute: don\u{2019}t give it a "
                    <Code inline=true>"style"</Code>" of your own. Classes are fine, but rules that change its size, position "
                    "or overflow undo the hiding."
                </p>
            </Section>

            <Section title="Demo">
                <p>
                    "The archive button shows only an icon; screen readers announce it as \u{201c}Archive message\u{201d}. "
                    "Tab into the demo: the skip link above the button appears while it has focus."
                </p>

                <Demo description="Icon button named by visually hidden text and a skip link shown on focus" source=include_str!("demos/visually_hidden.rs")>
                    <VisuallyHiddenDemo/>
                </Demo>
            </Section>

            <Section title="Focusable Content">
                <p>
                    "With "<Code inline=true>"is_focusable: true"</Code>", the element shows while it or one of its "
                    "descendants has focus, and hides again when focus leaves it. Without it, a focused element stays "
                    "invisible: keyboard users would lose track of the focus, so set it whenever the hidden content contains "
                    "something focusable."
                </p>
            </Section>

            <Section title="Hiding Styles">
                <p>
                    "The hook hides elements with "<Code inline=true>"VISUALLY_HIDDEN_STYLE"</Code>" from "
                    <Code inline=true>"leptonic::utils::visually_hidden"</Code>": a 1\u{d7}1 pixel, absolutely positioned, "
                    "clipped box. Use the constant (or "<Code inline=true>"visually_hidden_styles()"</Code>", the same rules as "
                    <Code inline=true>"Styles"</Code>" to merge with others) for elements that are always hidden and need no "
                    "hook, such as a native "<Code inline=true>"<input>"</Code>" behind a custom checkbox."
                </p>
                <Code language=Language::Css>{leptonic::utils::visually_hidden::VISUALLY_HIDDEN_STYLE}</Code>
            </Section>

            <Section title="Hiding Techniques">
                <DocTable headers=&["Technique", "Use it for"]>
                    <TableRow>
                        <TableCell><Code inline=true>"aria-label"</Code></TableCell>
                        <TableCell>"Naming an element with plain text when nothing visible names it."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"use_visually_hidden"</Code></TableCell>
                        <TableCell>
                            "Content screen readers read as part of the page: text inside a name, context for a link, a "
                            "skip link, a dismiss button."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>
                            <Link href=routes::doc::screen_readers::UseDescription.materialize()>
                                <Code inline=true>"use_description"</Code>
                            </Link>
                        </TableCell>
                        <TableCell>"A description screen readers read after the element\u{2019}s name, e.g. usage instructions."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"aria-hidden=\"true\""</Code></TableCell>
                        <TableCell>"The opposite: visible decoration screen readers should skip, e.g. an icon next to its label."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"display: none"</Code>", "<Code inline=true>"hidden"</Code></TableCell>
                        <TableCell>"Content hidden from everyone."</TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Keyboard">
                <KeyboardTable>
                    <KeyRow keys="Tab">"Moves focus into the hidden element, which shows it (with "<Code inline=true>"is_focusable"</Code>")."</KeyRow>
                    <KeyRow keys="Tab / Shift + Tab">"Moves focus out again, which hides it."</KeyRow>
                </KeyboardTable>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::screen_readers::VisuallyHidden.materialize()>"VisuallyHidden"</Link></li>
                <li><Link href=routes::doc::screen_readers::LiveAnnouncer.materialize()>"live_announcer"</Link></li>
                <li><Link href=routes::doc::screen_readers::UseDescription.materialize()>"use_description"</Link></li>
                <li><Link href=routes::doc::overlay_behavior::DismissButton.materialize()>"DismissButton"</Link></li>
                <li><Link href=routes::doc::focus::UseFocusWithin.materialize()>"use_focus_within"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
