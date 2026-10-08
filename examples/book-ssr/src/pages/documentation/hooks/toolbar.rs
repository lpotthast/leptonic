use indoc::indoc;
use leptos::prelude::*;

use super::demos::{
    toolbar_horizontal::ToolbarHorizontalDemo, toolbar_vertical::ToolbarVerticalDemo,
};
use crate::{kit::*, routes};

#[component]
pub fn PageUseToolbar() -> impl IntoView {
    view! {
        <DocPage title="use_toolbar">
            <p>
                "The "<Code inline=true>"use_toolbar"</Code>" hook makes a group of controls one tab stop: "<Keys keys="Tab"/>
                " enters and leaves it, the arrow keys move focus between its controls, and when focus comes back, it returns "
                "to the control focused last. The hook moves focus itself, so the controls need no tabindex management. "
                "See the "<Link href=routes::doc::Toolbar.materialize()>"Toolbar overview"</Link>" for concept guidance and keyboard interaction."
            </p>
            <ReactAria hook="useToolbar"/>

            <Section title="Input">
                <ApiTable kind=ApiKind::Input of="UseToolbarInput">
                    <ApiRow name="orientation" ty="Signal<Orientation>" default="Horizontal">
                        "The axis of the arrow keys: "<Keys keys="ArrowLeft"/>" and "<Keys keys="ArrowRight"/>" for "
                        <Code inline=true>"Horizontal"</Code>", "<Keys keys="ArrowUp"/>" and "<Keys keys="ArrowDown"/>" for "
                        <Code inline=true>"Vertical"</Code>". Also sets "<Code inline=true>"aria-orientation"</Code>"."
                    </ApiRow>
                    <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">"The toolbar\u{2019}s accessible name."</ApiRow>
                    <ApiRow name="aria_labelledby" ty="Option<String>" default="None">
                        "The id of an element naming the toolbar. Ignored when "<Code inline=true>"aria_label"</Code>" is set."
                    </ApiRow>
                </ApiTable>
                <p><Code inline=true>"UseToolbarInput"</Code>" implements "<Code inline=true>"Default"</Code>"."</p>
            </Section>

            <Section title="Return">
                <ApiTable kind=ApiKind::Return of="UseToolbarReturn">
                    <ApiRow name="props" ty="UseToolbarProps">
                        <Code inline=true>"role=\"toolbar\""</Code>", "<Code inline=true>"aria-orientation"</Code>
                        ", the label and the focus handling. Spread with "<Code inline=true>"{..props.into_attrs()}"</Code>"."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::hooks::*;

                        let toolbar = use_toolbar(UseToolbarInput {
                            aria_label: "Text formatting".into(),
                            ..UseToolbarInput::default()
                        });

                        view! {
                            <div {..toolbar.props.into_attrs()}>
                                <button>"Bold"</button>
                                <button>"Italic"</button>
                                <button>"Underline"</button>
                            </div>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <p>
                    "A horizontal toolbar of "<Link href=routes::doc::toggle_button::Hook.materialize()>"toggle buttons"</Link>
                    ", styled through the "<Code inline=true>"aria-pressed"</Code>" attribute they get. Tab into it, move "
                    "with the arrow keys, tab out and back in:"
                </p>
                <Demo
                    description="Horizontal text formatting toolbar with bold, italic and underline toggle buttons and a preview"
                    source=include_str!("demos/toolbar_horizontal.rs")
                >
                    <ToolbarHorizontalDemo/>
                </Demo>

                <p>
                    "A vertical toolbar of "<Link href=routes::doc::button::Hook.materialize()>"buttons"</Link>": "
                    <Keys keys="ArrowUp"/>" and "<Keys keys="ArrowDown"/>" move between them."
                </p>
                <Demo description="Vertical toolbar with three action buttons showing the last action" source=include_str!("demos/toolbar_vertical.rs")>
                    <ToolbarVerticalDemo/>
                </Demo>
            </Section>

            <Section title="Nested Toolbars">
                <p>
                    "A toolbar inside another toolbar becomes a "<Code inline=true>"group"</Code>" of it and leaves the keyboard "
                    "handling to the outer toolbar, so the arrow keys move through all controls of both."
                </p>
            </Section>


            <SeeAlso>
                <li><Link href=routes::doc::Toolbar.materialize()>"Toolbar overview"</Link></li>
                <li><Link href=routes::doc::toolbar::Atom.materialize()>"Toolbar Atom"</Link></li>
                <li><Link href=routes::doc::separator::Hook.materialize()>"use_separator"</Link></li>
                <li><Link href=routes::doc::toggle_button::Hook.materialize()>"Toggle Button Hooks"</Link></li>
                <li><Link href=routes::doc::focus::UseFocusManager.materialize()>"use_focus_manager"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
