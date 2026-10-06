use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::button_basic::BasicButtonDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageUseButton() -> impl IntoView {
    view! {
        <DocPage title="use_button">
            <p>
                "The "<Code inline=true>"use_button"</Code>" hook makes an element behave and announce itself as a button: "
                "press handling for mouse, touch, keyboard and screen readers, focus management, hover and focus-visible "
                "tracking, and the right attributes for the element you render. "
                "See the "<Link href=routes::doc::Button.materialize()>"Button overview"</Link>" for concept guidance."
            </p>

            <ReactAria hook="useButton"/>

            <Section title="Input">
                <p>
                    <Code inline=true>"UseButtonInput"</Code>" is a flat struct that implements "<Code inline=true>"Default"</Code>
                    ". Everything is optional, so you only name the fields you care about and fill in the rest with "
                    <Code inline=true>"..Default::default()"</Code>"."
                </p>

                <ApiTable kind=ApiKind::Input of="UseButtonInput">
                    <ApiRow name="element_type" ty="ButtonElementType" default="Button">
                        "The kind of element you spread the props onto. Decides which attributes are set, see "
                        <a href="#element-types">"Element types"</a>"."
                    </ApiRow>
                    <ApiRow name="button_type" ty="ButtonType" default="Button">
                        "The "<Code inline=true>"type"</Code>" of a "<Code inline=true>"<button>"</Code>" or "
                        <Code inline=true>"<input>"</Code>": "<Code inline=true>"Button"</Code>", "<Code inline=true>"Submit"</Code>
                        " or "<Code inline=true>"Reset"</Code>". Unlike HTML, the default does not submit forms."
                    </ApiRow>
                    <ApiRow name="id" ty="Option<Oco<'static, str>>" default="None">"The element\u{2019}s id."</ApiRow>
                    <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">
                        "An accessible name for buttons without visible text, such as icon buttons."
                    </ApiRow>
                    <ApiRow name="aria_labelledby" ty="Option<Oco<'static, str>>" default="None">
                        "The id(s) of the element(s) that name the button."
                    </ApiRow>
                    <ApiRow name="is_disabled" ty="Signal<bool>" default="false">"Whether the button is disabled."</ApiRow>
                    <ApiRow name="allow_focus_when_disabled" ty="bool" default="false">
                        "Keep a disabled button focusable (with "<Code inline=true>"tabindex=\"-1\""</Code>
                        "), so focus is not lost when a button becomes disabled while focused."
                    </ApiRow>
                    <ApiRow name="exclude_from_tab_order" ty="Signal<bool>" default="false">
                        "Skip the button when tabbing. It can still be focused by pointer or programmatically."
                    </ApiRow>
                    <ApiRow name="auto_focus" ty="bool" default="false">"Focus the button when it mounts."</ApiRow>
                    <ApiRow name="prevent_focus_on_press" ty="bool" default="false">
                        "Don\u{2019}t move focus to the button when it is pressed, e.g. for toolbar buttons next to a text editor."
                    </ApiRow>
                    <ApiRow name="href" ty="Option<Signal<String>>" default="None">
                        "Link target, only used for "<Code inline=true>"ButtonElementType::Anchor"</Code>
                        ". Removed while the button is disabled."
                    </ApiRow>
                    <ApiRow name="target" ty="Option<LinkTarget>" default="None">
                        "Link target window, only used for "<Code inline=true>"ButtonElementType::Anchor"</Code>"."
                    </ApiRow>
                    <ApiRow name="rel" ty="Option<Oco<'static, str>>" default="None">
                        "Link relationship, only used for "<Code inline=true>"ButtonElementType::Anchor"</Code>"."
                    </ApiRow>
                    <ApiRow name="form" ty="ButtonFormAttributes" default="empty">
                        "Form-related attributes of a native "<Code inline=true>"<button>"</Code>" ("<Code inline=true>"form"</Code>
                        ", "<Code inline=true>"formaction"</Code>", "<Code inline=true>"name"</Code>", "
                        <Code inline=true>"value"</Code>", \u{2026}). Ignored for other element types."
                    </ApiRow>
                    <ApiRow name="aria_haspopup" ty="Signal<Option<AriaHasPopup>>" default="None">
                        "The kind of popup the button opens."
                    </ApiRow>
                    <ApiRow name="aria_expanded" ty="Signal<Option<AriaExpanded>>" default="None">
                        "Whether the element the button controls is expanded."
                    </ApiRow>
                    <ApiRow name="aria_controls" ty="Signal<Option<String>>" default="None">
                        "The id(s) of the element(s) the button controls."
                    </ApiRow>
                    <ApiRow name="aria_describedby" ty="Signal<Option<String>>" default="None">
                        "The id(s) of the element(s) describing the button."
                    </ApiRow>
                    <ApiRow name="aria_pressed" ty="Signal<Option<AriaPressed>>" default="None">
                        "The pressed state of a toggle button."
                    </ApiRow>
                    <ApiRow name="aria_checked" ty="Signal<Option<AriaChecked>>" default="None">
                        "The checked state of a button acting as a checkable item, e.g. "<Code inline=true>"role=\"radio\""</Code>"."
                    </ApiRow>
                    <ApiRow name="role" ty="Option<AriaRole>" default="None">
                        "Overrides the element\u{2019}s role, e.g. "<Code inline=true>"AriaRole::Radio"</Code>
                        " for the buttons of a single-selection "
                        <Link href=routes::doc::toggle_button::Hook.materialize()>"toggle button group"</Link>"."
                    </ApiRow>
                    <ApiRow name="aria_current" ty="Signal<Option<AriaCurrent>>" default="None">
                        "Whether the button represents the current item of a set."
                    </ApiRow>
                    <ApiRow name="on_press, on_press_start, on_press_end, on_press_up" ty="Option<Callback<PressEvent>>" default="None">
                        "Press callbacks, as in "<Link href=routes::doc::interactions::UsePress.materialize()>"use_press"</Link>"."
                    </ApiRow>
                    <ApiRow name="on_press_change" ty="Option<Callback<bool>>" default="None">
                        "Called when the pressed state changes."
                    </ApiRow>
                    <ApiRow name="on_long_press_start, on_long_press, on_long_press_end" ty="Option<Callback<LongPressEvent>>" default="None">
                        "Long press callbacks. Setting any of them enables long press detection."
                    </ApiRow>
                    <ApiRow name="long_press_accessibility_description" ty="Option<Oco<'static, str>>" default="None">
                        "Describes the long press action to assistive technology, e.g. \u{201c}Long press to open menu\u{201d}."
                    </ApiRow>
                    <ApiRow name="on_hover_start, on_hover_end, on_hover_change" ty="Option<Callback<..>>" default="None">
                        "Hover callbacks, as in "<Link href=routes::doc::interactions::UseHover.materialize()>"use_hover"</Link>"."
                    </ApiRow>
                    <ApiRow name="on_focus, on_blur, on_focus_change" ty="Option<Callback<..>>" default="None">
                        "Focus callbacks."
                    </ApiRow>
                    <ApiRow name="on_key_down, on_key_up" ty="Option<Callback<KeyboardEventWrapper>>" default="None">
                        "Raw keyboard callbacks, as in "<Link href=routes::doc::interactions::UseKeyboard.materialize()>"use_keyboard"</Link>"."
                    </ApiRow>
                    <ApiRow name="shortcuts" ty="Option<KeyboardShortcuts>" default="None">
                        "Keyboard shortcuts handled while the button has focus. They run before press handling, so a shortcut for "
                        <Code inline=true>"Enter"</Code>" or "<Code inline=true>"Space"</Code>" can take over the key."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="Return">
                <ApiTable kind=ApiKind::Return of="UseButtonReturn">
                    <ApiRow name="props" ty="PropsWithStyles<UseButtonProps>">
                        "Attributes, event handlers and styles for the button element. Call "
                        <Code inline=true>"props.into_parts()"</Code>" to get "<Code inline=true>"(attrs, styles)"</Code>
                        ", then spread "<Code inline=true>"{..attrs}"</Code>" and set "<Code inline=true>"style=styles"</Code>"."
                    </ApiRow>
                    <ApiRow name="is_pressed" ty="Signal<bool>">"Whether the button is currently pressed."</ApiRow>
                    <ApiRow name="is_hovered" ty="Signal<bool>">"Whether a mouse or pen is over the button."</ApiRow>
                    <ApiRow name="is_focused" ty="Signal<bool>">"Whether the button has focus."</ApiRow>
                    <ApiRow name="is_focus_visible" ty="Signal<bool>">
                        "Whether a focus ring should be shown (keyboard focus only). Also exposed as the "
                        <Code inline=true>"data-focus-visible"</Code>" attribute."
                    </ApiRow>
                    <ApiRow name="focus_handle" ty="FocusHandle">
                        "Focus the button programmatically with "<Code inline=true>"focus_handle.focus()"</Code>"."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        let UseButtonReturn { props, is_pressed, .. } = use_button(UseButtonInput {
                            on_press: Some(Callback::new(move |_| log!("pressed"))),
                            ..Default::default()
                        });
                        let (attrs, styles) = props.into_parts();

                        view! {
                            <button {..attrs} style=styles>"Press me"</button>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <p>
                    "A "<Code inline=true>"<div>"</Code>" turned into a button with "<Code inline=true>"ButtonElementType::Other"</Code>
                    ". Try it with the mouse and with the keyboard."
                </p>

                <Demo description="Press count tracking on a div element" source=include_str!("demos/button_basic.rs")>
                    <BasicButtonDemo/>
                </Demo>
            </Section>

            <Section title="Element types">
                <p>
                    "Native buttons come with a lot of behavior for free, other elements need ARIA to make up for it. "
                    <Code inline=true>"element_type"</Code>" tells the hook what you are rendering, so it only sets what is needed:"
                </p>

                <DocTable headers=&["ButtonElementType", "What you get"]>
                    <TableRow>
                        <TableCell><Code inline=true>"Button"</Code></TableCell>
                        <TableCell>
                            "A native "<Code inline=true>"<button>"</Code>". No role, "<Code inline=true>"type=\"button\""</Code>
                            " unless you pick another "<Code inline=true>"button_type"</Code>", form attributes, and the native "
                            <Code inline=true>"disabled"</Code>" attribute."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"Input"</Code></TableCell>
                        <TableCell>
                            "An "<Code inline=true>"<input>"</Code>". Gets "<Code inline=true>"role=\"button\""</Code>", a "
                            <Code inline=true>"type"</Code>" and the native "<Code inline=true>"disabled"</Code>" attribute."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"Anchor"</Code></TableCell>
                        <TableCell>
                            "An "<Code inline=true>"<a>"</Code>" that acts as a button. Gets "<Code inline=true>"role=\"button\""</Code>
                            " and keeps its "<Code inline=true>"href"</Code>", "<Code inline=true>"target"</Code>" and "
                            <Code inline=true>"rel"</Code>". While disabled, the "<Code inline=true>"href"</Code>" is dropped and "
                            <Code inline=true>"aria-disabled"</Code>" is set."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"Other"</Code></TableCell>
                        <TableCell>
                            "Anything else, like a "<Code inline=true>"<div>"</Code>" or "<Code inline=true>"<span>"</Code>". Gets "
                            <Code inline=true>"role=\"button\""</Code>", a "<Code inline=true>"tabindex"</Code>" and "
                            <Code inline=true>"aria-disabled"</Code>" while disabled."
                        </TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Composition">
                <p>
                    "Some hooks don\u{2019}t render a button themselves but configure one: a menu trigger needs a button that opens a menu, "
                    "a number field needs two stepper buttons. These hooks return a ready-made "<Code inline=true>"UseButtonInput"</Code>
                    " instead of DOM props. You pass it to "<Code inline=true>"use_button"</Code>
                    ", and because it is a plain struct, you can add your own settings with struct update syntax:"
                </p>

                <Code language=Language::Rust>
                    {indoc!(r#"
                        let menu_trigger = use_menu_trigger(UseMenuTriggerInput { /* ... */ });

                        let button = use_button(UseButtonInput {
                            on_hover_start: Some(Callback::new(|_| log!("hovered"))),
                            ..menu_trigger.button
                        });
                        let (attrs, styles) = button.props.into_parts();

                        view! { <button {..attrs} style=styles>"Actions"</button> }
                    "#)}
                </Code>

                <p>
                    "This way, the element ends up with exactly one press, focus and hover state machine. "
                    "See "<Link href=format!("{}#composing-hooks", routes::doc::Architecture.materialize())>"Composing hooks"</Link>
                    " for the full picture. Hooks returning a "<Code inline=true>"UseButtonInput"</Code>" include "
                    <Link href=routes::doc::menu::Hook.materialize()>"use_menu_trigger"</Link>", "
                    <Link href=routes::doc::hooks::UseSpinButton.materialize()>"use_spin_button"</Link>" and "
                    <Link href=routes::doc::text_field::NumberFieldHook.materialize()>"use_number_field"</Link>"."
                </p>
            </Section>

            <Section title="Keyboard">
                <KeyboardTable>
                    <KeyRow keys="Tab">"Focus the button (unless "<Code inline=true>"exclude_from_tab_order"</Code>" is set)."</KeyRow>
                    <KeyRow keys="Enter / Space">"Press the button."</KeyRow>
                </KeyboardTable>

                <p>
                    "Any "<Code inline=true>"shortcuts"</Code>" you pass are handled first. Holding a key down does not re-trigger them; see "
                    <Link href=routes::doc::interactions::UseKeyboard.materialize()>"use_keyboard"</Link>" for how shortcuts work."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Button.materialize()>"Button overview"</Link></li>
                <li><Link href=routes::doc::button::Atom.materialize()>"Button atom"</Link></li>
                <li><Link href=routes::doc::button::Component.materialize()>"Button component"</Link></li>
                <li><Link href=routes::doc::interactions::UsePress.materialize()>"use_press"</Link></li>
                <li><Link href=routes::doc::interactions::UseHover.materialize()>"use_hover"</Link></li>
                <li><Link href=routes::doc::focus::UseFocusable.materialize()>"use_focusable"</Link></li>
                <li><Link href=routes::doc::focus::UseFocusRing.materialize()>"use_focus_ring"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
