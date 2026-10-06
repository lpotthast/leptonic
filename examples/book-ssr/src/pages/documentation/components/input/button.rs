use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::{
    button_basic::ButtonBasicDemo, button_colors::ButtonColorsDemo,
    button_disabled::ButtonDisabledDemo, button_group::ButtonGroupDemo,
    button_variants::ButtonVariantsDemo,
};
use crate::{kit::*, routes};

#[component]
pub fn PageButton() -> impl IntoView {
    view! {
        <DocPage title="Button component">
            <p>
                "The themed "<Code inline=true>"Button"</Code>", with colors, variants, sizes and button groups. "
                "See the "<Link href=routes::doc::Button.materialize()>"Button overview"</Link>" for concept guidance."
            </p>

            <Demo description="Basic button" source=include_str!("demos/button_basic.rs")>
                <ButtonBasicDemo/>
            </Demo>

            <Section title="Props">
                <ApiTable kind=ApiKind::Props of="components::button::Button">
                    <ApiRow name="on_press" ty="Option<Callback<PressEvent>>" default="None">
                        "Called when the button is pressed. Not needed for submit/reset buttons or inside a "
                        <Code inline=true>"PressResponder"</Code>"."
                    </ApiRow>
                    <ApiRow name="button_type" ty="ButtonType" default="Button">
                        "The button\u{2019}s "<Code inline=true>"type"</Code>": "<Code inline=true>"Submit"</Code>" and "
                        <Code inline=true>"Reset"</Code>" act on their form."
                    </ApiRow>
                    <ApiRow name="variant" ty="Signal<ButtonVariant>" default="Filled">
                        <Code inline=true>"Flat"</Code>", "<Code inline=true>"Outlined"</Code>" or "<Code inline=true>"Filled"</Code>"."
                    </ApiRow>
                    <ApiRow name="color" ty="Signal<ButtonColor>" default="Primary">
                        "The color scheme, see "<a href="#colors">"Colors"</a>"."
                    </ApiRow>
                    <ApiRow name="size" ty="Signal<ButtonSize>" default="Normal">
                        <Code inline=true>"Small"</Code>", "<Code inline=true>"Normal"</Code>" or "<Code inline=true>"Big"</Code>"."
                    </ApiRow>
                    <ApiRow name="disabled" ty="Signal<bool>" default="false">"Whether the button is disabled."</ApiRow>
                    <ApiRow name="aria_haspopup" ty="Signal<Option<AriaHasPopup>>" default="None">
                        "The kind of popup the button opens."
                    </ApiRow>
                    <ApiRow name="aria_expanded" ty="Signal<Option<AriaExpanded>>" default="None">
                        "Whether the element the button controls is expanded."
                    </ApiRow>
                    <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                        "Additional classes and styles."
                    </ApiRow>
                    <ApiRow name="children" ty="Children">"The button content."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="Colors">
                <p>"Buttons come in different colors. You can override them with theme variables."</p>

                <Demo description="Button colors" source=include_str!("demos/button_colors.rs")>
                    <ButtonColorsDemo/>
                </Demo>
            </Section>

            <Section title="Variants">
                <p>
                    "Buttons come in three "<Code inline=true>"ButtonVariant"</Code>"s: "<Code inline=true>"Flat"</Code>", "
                    <Code inline=true>"Outlined"</Code>" and "<Code inline=true>"Filled"</Code>". "
                    <Code inline=true>"Filled"</Code>" is the default, which is why the button above looks the way it does."
                </p>

                <Demo description="Button variants" source=include_str!("demos/button_variants.rs")>
                    <ButtonVariantsDemo/>
                </Demo>
            </Section>

            <Section title="Groups">
                <p>
                    "A "<Code inline=true>"ButtonGroup"</Code>" lets adjacent buttons snap to each other, forming a seamless row. "
                    "Use the "<Code inline=true>"Filled"</Code>" variant inside groups. To lay out separate buttons in a "
                    "row instead, wrap them in a "<Code inline=true>"ButtonWrapper"</Code>": it keeps their spacing and "
                    "wraps them onto the next line when space runs out."
                </p>

                <Demo description="Button group" source=include_str!("demos/button_group.rs")>
                    <ButtonGroupDemo/>
                </Demo>
            </Section>

            <Section title="Disabled">
                <p>
                    "Disable a button with the "<Code inline=true>"disabled"</Code>
                    " prop. It accepts anything that converts into a "<Code inline=true>"Signal<bool>"</Code>", including signals."
                </p>

                <Demo description="Toggling a button's disabled state" source=include_str!("demos/button_disabled.rs")>
                    <ButtonDisabledDemo/>
                </Demo>
            </Section>

            <Section title="Styling">
                <p>"Override any of these CSS variables to adapt the buttons to your design:"</p>
                <CssVariables prefix="--button-" scss=theme_scss!("button")/>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Button.materialize()>"Button overview"</Link></li>
                <li><Link href=routes::doc::button::Hook.materialize()>"use_button"</Link></li>
                <li><Link href=routes::doc::button::Atom.materialize()>"Button atom"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
