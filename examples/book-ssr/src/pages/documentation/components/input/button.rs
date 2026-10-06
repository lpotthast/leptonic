use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::{
    button_basic::ButtonBasicDemo, button_colors::ButtonColorsDemo,
    button_disabled::ButtonDisabledDemo, button_group::ButtonGroupDemo,
    button_sizes::ButtonSizesDemo, button_variants::ButtonVariantsDemo,
    button_wrapper::ButtonWrapperDemo,
};
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageButton() -> impl IntoView {
    view! {
        <DocPage title="Button Components">
            <p>
                "The themed "<Code inline=true>"Button"</Code>" with colors, variants and sizes, "
                <Code inline=true>"ButtonGroup"</Code>" and "<Code inline=true>"ButtonWrapper"</Code>" to lay out several "
                "buttons. See the "<Link href=routes::doc::Button.materialize()>"Button overview"</Link>" for concept guidance. "
                "For a link that looks like a button, use the "
                <Link href=format!("{}#linkbutton", routes::doc::link::Component.materialize())><Code inline=true>"LinkButton"</Code></Link>
                " of the "<Link href=routes::doc::link::Component.materialize()>"Link Components"</Link>"."
            </p>

            <Demo description="Button counting presses" source=include_str!("demos/button_basic.rs")>
                <ButtonBasicDemo/>
            </Demo>

            <Section title="Button">
                <p>
                    "A "<Code inline=true>"<button>"</Code>" built on the "
                    <Link href=routes::doc::button::Atom.materialize()>"Button Atom"</Link>", with the theme\u{2019}s look."
                </p>

                <Section title="Props" id="button-props">
                    <ApiTable kind=ApiKind::Props of="components::button::Button">
                        <ApiRow name="on_press" ty="Option<Callback<PressEvent>>" default="None">
                            "Called when the button is pressed. Submit and reset buttons, and buttons inside a "
                            <Link href=routes::doc::interactions::PressResponder.materialize()><Code inline=true>"PressResponder"</Code></Link>
                            " (such as a dialog trigger), work without it."
                        </ApiRow>
                        <ApiRow name="button_type" ty="ButtonType" default="Button">
                            "The button\u{2019}s "<Code inline=true>"type"</Code>": "<Code inline=true>"Submit"</Code>" and "
                            <Code inline=true>"Reset"</Code>" act on their form. The default doesn\u{2019}t submit forms."
                        </ApiRow>
                        <ApiRow name="variant" ty="Signal<ButtonVariant>" default="Filled">
                            "How prominent the button is; see "<AnchorLink href="#variants">"Variants"</AnchorLink>"."
                        </ApiRow>
                        <ApiRow name="color" ty="Signal<ButtonColor>" default="Primary">
                            "What the action means; see "<AnchorLink href="#colors">"Colors"</AnchorLink>"."
                        </ApiRow>
                        <ApiRow name="size" ty="Signal<ButtonSize>" default="Normal">
                            "How large the button is; see "<AnchorLink href="#sizes">"Sizes"</AnchorLink>"."
                        </ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>" default="false">"Whether the button is disabled."</ApiRow>
                        <ApiRow name="aria_haspopup" ty="Signal<Option<AriaHasPopup>>" default="None">
                            "The kind of popup the button opens."
                        </ApiRow>
                        <ApiRow name="aria_expanded" ty="Signal<Option<AriaExpanded>>" default="None">
                            "Whether the element the button controls is expanded."
                        </ApiRow>
                        <ApiRow name="aria_pressed" ty="Signal<Option<AriaPressed>>" default="None">
                            "Whether a toggling button is pressed, e.g. a toolbar\u{2019}s \u{201c}Bold\u{201d} while bold "
                            "text is selected. For a toggle with its own state, use a "
                            <Link href=routes::doc::ToggleButton.materialize()>"toggle button"</Link>"."
                        </ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                            "Classes and styles added to the "<Code inline=true>"<button>"</Code>"."
                        </ApiRow>
                        <ApiRow name="children" ty="Children">"The button content. Required."</ApiRow>
                    </ApiTable>
                    <p>
                        "The component passes the props on to the atom and adds "<Code inline=true>"data-variant"</Code>", "
                        <Code inline=true>"data-color"</Code>" and "<Code inline=true>"data-size"</Code>" for the theme. For "
                        "the other props of the atom, such as "<Code inline=true>"aria_label"</Code>" for an icon-only button, "
                        "use the "<Link href=routes::doc::button::Atom.materialize()>"Button Atom"</Link>"."
                    </p>
                </Section>

                <Section title="Colors">
                    <p>
                        "A "<Code inline=true>"ButtonColor"</Code>" tells what an action means: "<Code inline=true>"Primary"</Code>
                        " for the main action of a view, "<Code inline=true>"Secondary"</Code>" for the others, "
                        <Code inline=true>"Success"</Code>", "<Code inline=true>"Info"</Code>", "<Code inline=true>"Warn"</Code>
                        " and "<Code inline=true>"Danger"</Code>" for actions with that weight, such as "<Code inline=true>"Danger"</Code>
                        " for deleting. Don\u{2019}t rely on the color alone: the label must say what the button does."
                    </p>

                    <Demo description="One button per color, showing the last one pressed" source=include_str!("demos/button_colors.rs")>
                        <ButtonColorsDemo/>
                    </Demo>
                </Section>

                <Section title="Variants">
                    <p>
                        "A "<Code inline=true>"ButtonVariant"</Code>" sets how prominent a button is: "<Code inline=true>"Filled"</Code>
                        " (the default) for the main action, "<Code inline=true>"Outlined"</Code>" for secondary ones and "
                        <Code inline=true>"Flat"</Code>" for actions that shouldn\u{2019}t draw attention."
                    </p>

                    <Demo description="The three button variants" source=include_str!("demos/button_variants.rs")>
                        <ButtonVariantsDemo/>
                    </Demo>
                </Section>

                <Section title="Sizes">
                    <p>
                        "A "<Code inline=true>"ButtonSize"</Code>" is "<Code inline=true>"Small"</Code>", "<Code inline=true>"Normal"</Code>
                        " (the default) or "<Code inline=true>"Big"</Code>"."
                    </p>

                    <Demo description="The three button sizes" source=include_str!("demos/button_sizes.rs")>
                        <ButtonSizesDemo/>
                    </Demo>
                </Section>

                <Section title="Disabled">
                    <p>
                        <Code inline=true>"is_disabled"</Code>" takes a "<Code inline=true>"Signal<bool>"</Code>
                        ", so the button can follow your state. A disabled button ignores presses, can\u{2019}t be focused and "
                        "looks disabled in every variant."
                    </p>

                    <Demo description="Send button with a disabled toggle" source=include_str!("demos/button_disabled.rs")>
                        <ButtonDisabledDemo/>
                    </Demo>
                </Section>
            </Section>

            <Section title="ButtonGroup">
                <p>
                    "Joins adjacent buttons into one seamless row, for closely related actions. Use the "
                    <Code inline=true>"Filled"</Code>" variant inside. For buttons that pick one of several options, use a "
                    <Link href=routes::doc::ToggleButton.materialize()>"Toggle Button"</Link>" group instead: it tells "
                    "screen readers which option is selected."
                </p>

                <Demo description="Cut, copy and paste in a group" source=include_str!("demos/button_group.rs")>
                    <ButtonGroupDemo/>
                </Demo>

                <Section title="Props" id="button-group-props">
                    <ApiTable kind=ApiKind::Props of="ButtonGroup">
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the group\u{2019}s "<Code inline=true>"<div>"</Code>"."</ApiRow>
                        <ApiRow name="children" ty="Children">"The buttons. Required."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="ButtonWrapper">
                <p>
                    "Lays out separate buttons in a row: it keeps their spacing and wraps them onto the next line when space "
                    "runs out, as in the footer of a dialog."
                </p>

                <Demo description="Three actions in a wrapper" source=include_str!("demos/button_wrapper.rs")>
                    <ButtonWrapperDemo/>
                </Demo>

                <Section title="Props" id="button-wrapper-props">
                    <ApiTable kind=ApiKind::Props of="ButtonWrapper">
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the wrapper\u{2019}s "<Code inline=true>"<div>"</Code>"."</ApiRow>
                        <ApiRow name="children" ty="Children">"The buttons. Required."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Styling">
                <p>"Override these CSS variables to adapt the buttons to your design:"</p>
                <CssVariables prefix="--button-" scss=theme_scss!("button")/>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Button.materialize()>"Button overview"</Link></li>
                <li><Link href=routes::doc::button::Hook.materialize()>"use_button"</Link></li>
                <li><Link href=routes::doc::button::Atom.materialize()>"Button Atom"</Link></li>
                <li><Link href=routes::doc::link::Component.materialize()>"Link Components"</Link></li>
                <li><Link href=routes::doc::ToggleButton.materialize()>"Toggle Button overview"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
