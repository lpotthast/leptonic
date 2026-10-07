use indoc::indoc;
use leptos::prelude::*;

use super::demos::toolbar::ToolbarAtomDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageAtomToolbar() -> impl IntoView {
    view! {
        <DocPage title="Toolbar Atom">
            <p>
                "The "<Code inline=true>"Toolbar"</Code>" atom renders an unstyled toolbar: a "<Code inline=true>"<div>"</Code>
                " whose controls are a single tab stop, between which the arrow keys move focus. See the "
                <Link href=routes::doc::Toolbar.materialize()>"Toolbar overview"</Link>" for concept guidance."
            </p>

            <Section title="Hooks Used">
                <p>
                    <Link href=routes::doc::toolbar::Hook.materialize()>"use_toolbar"</Link>
                    ", for the role, the orientation, the label and the focus handling."
                </p>
            </Section>

            <Section title="Props">
                <ApiTable kind=ApiKind::Props of="atoms::toolbar::Toolbar">
                    <ApiRow name="orientation" ty="Signal<Orientation>" default="Horizontal">
                        "The axis of the arrow keys: "<Keys keys="ArrowLeft"/>" and "<Keys keys="ArrowRight"/>" for "
                        <Code inline=true>"Horizontal"</Code>", "<Keys keys="ArrowUp"/>" and "<Keys keys="ArrowDown"/>" for "
                        <Code inline=true>"Vertical"</Code>"."
                    </ApiRow>
                    <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">"Names the toolbar."</ApiRow>
                    <ApiRow name="aria_labelledby" ty="Option<String>" default="None">
                        "The ids of the elements naming the toolbar, unless "<Code inline=true>"aria_label"</Code>" is set."
                    </ApiRow>
                    <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the toolbar."</ApiRow>
                    <ApiRow name="children" ty="Children">
                        "The controls, and "<Link href=routes::doc::separator::Atom.materialize()>"Separator"</Link>
                        "s between groups of them."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::atoms::prelude as atoms;

                        let bold = RwSignal::new(false);
                        let italic = RwSignal::new(false);

                        view! {
                            <atoms::Toolbar aria_label="Text formatting">
                                <atoms::ToggleButton is_selected=bold set_selected=bold>"Bold"</atoms::ToggleButton>
                                <atoms::ToggleButton is_selected=italic set_selected=italic>"Italic"</atoms::ToggleButton>
                            </atoms::Toolbar>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <p>
                    "Toggle buttons for the formats and a button clearing them, divided by a separator. The buttons are "
                    "styled through their data attributes ("<Code inline=true>"data-selected"</Code>", "
                    <Code inline=true>"data-focus-visible"</Code>", \u{2026}):"
                </p>
                <Demo
                    description="Text formatting toolbar with bold and italic toggle buttons, a separator and a clear button, with a preview"
                    source=include_str!("demos/toolbar.rs")
                >
                    <ToolbarAtomDemo/>
                </Demo>
            </Section>

            <Section title="Data Attributes">
                <ApiTable kind=ApiKind::DataAttributes>
                    <ApiRow name="data-orientation" ty="\"horizontal\" | \"vertical\"">"The toolbar\u{2019}s orientation."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="Styling">
                <p>
                    "The atom brings no styles. It renders the class "<Code inline=true>"leptonic-Toolbar"</Code>
                    " followed by the "<Code inline=true>"classes"</Code>" you pass. Lay the controls out along the "
                    "toolbar\u{2019}s "<Code inline=true>"data-orientation"</Code>", and style the controls through their "
                    "own data attributes. The book\u{2019}s demos use these rules:"
                </p>
                <Code language=Language::Css>
                    {indoc!(r#"
                        .my-toolbar { display: flex; gap: 0.5em; width: fit-content; padding: 0.5em; border-radius: 4px; background: var(--surface); }
                        .my-toolbar[data-orientation="vertical"] { flex-direction: column; }
                        .my-toolbar-separator { align-self: stretch; margin: 0 0.25em; border: none; border-left: 1px solid var(--border); }
                        .my-toolbar-button { padding: 0.5em 1em; border: 1px solid var(--border); border-radius: 4px; background: var(--surface); }
                        .my-toolbar-button[data-selected] { border-color: var(--accent); }
                        .my-toolbar-button[data-focus-visible] { outline: 2px solid var(--focus); outline-offset: 2px; }
                    "#)}
                </Code>
                <p>
                    "Leptonic also ships an optional atom theme that styles the default classes, for apps that don\u{2019}t "
                    "want to start from scratch: "<Code inline=true>"@use \"leptonic/leptonic-atoms\";"</Code>"."
                </p>
            </Section>

            <Section title="Composition">
                <ul>
                    <li>
                        "Any focusable control works inside a toolbar: "
                        <Link href=routes::doc::button::Atom.materialize()>"Button"</Link>", "
                        <Link href=routes::doc::toggle_button::Atom.materialize()>"ToggleButton"</Link>
                        " (a "<Code inline=true>"ToggleButtonGroup"</Code>" is a toolbar itself), "
                        <Link href=routes::doc::menu::Atom.materialize()>"MenuTrigger"</Link>" buttons and "
                        <Link href=routes::doc::select::Atom.materialize()>"Select"</Link>" triggers."
                    </li>
                    <li>
                        "Divide groups of controls with a "<Link href=routes::doc::separator::Atom.materialize()>"Separator"</Link>
                        ". Set its orientation yourself, across the toolbar\u{2019}s: "
                        <Code inline=true>"Orientation::Vertical"</Code>" in a horizontal toolbar. The toolbar doesn\u{2019}t "
                        "set it for you."
                    </li>
                    <li>
                        "A toolbar inside another toolbar becomes a group of it, and the outer toolbar\u{2019}s arrow keys "
                        "move through the controls of both."
                    </li>
                </ul>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Toolbar.materialize()>"Toolbar overview"</Link></li>
                <li><Link href=routes::doc::toolbar::Hook.materialize()>"use_toolbar"</Link></li>
                <li><Link href=routes::doc::separator::Atom.materialize()>"Separator Atom"</Link></li>
                <li><Link href=routes::doc::toggle_button::Atom.materialize()>"Toggle Button Atoms"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
