use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::button_basic::BasicButtonDemo;
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageUseButton() -> impl IntoView {
    view! {
        <DocPage title="use_button">
            <p>
                "The "<Code inline=true>"use_button"</Code>" hook makes an element behave and announce itself as a button: "
                "presses from mouse, touch, keyboard and screen readers, focus, hover and focus-visible state, and the "
                "attributes the element you render needs. See the "<Link href=routes::doc::Button.materialize()>"Button overview"</Link>
                " for concept guidance."
            </p>

            <ReactAria hook="useButton"/>

            <Section title="Input">
                <p>
                    <Code inline=true>"UseButtonInput"</Code>" implements "<Code inline=true>"Default"</Code>
                    ". Every field is optional: name the ones you need and fill in the rest with "
                    <Code inline=true>"..Default::default()"</Code>"."
                </p>

                <ApiTable kind=ApiKind::Input of="UseButtonInput">
                    <ApiRow name="element_type" ty="ButtonElementType" default="Button">
                        "The kind of element you spread the props onto. Decides which attributes are set; see "
                        <AnchorLink href="#element-types">"Element Types"</AnchorLink>"."
                    </ApiRow>
                    <ApiRow name="button_type" ty="ButtonType" default="Button">
                        "The "<Code inline=true>"type"</Code>" of a "<Code inline=true>"<button>"</Code>" or "
                        <Code inline=true>"<input>"</Code>": "<Code inline=true>"Button"</Code>", "<Code inline=true>"Submit"</Code>
                        " or "<Code inline=true>"Reset"</Code>". The default doesn\u{2019}t submit forms, so a button inside a "
                        "form submits it only when you ask for it."
                    </ApiRow>
                    <ApiRow name="id" ty="Option<Oco<'static, str>>" default="None">"The element\u{2019}s id."</ApiRow>
                    <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">
                        "An accessible name for buttons without visible text, such as icon buttons."
                    </ApiRow>
                    <ApiRow name="aria_labelledby" ty="Signal<Option<String>>" default="None">
                        "The ids of the elements that name the button."
                    </ApiRow>
                    <ApiRow name="is_disabled" ty="Signal<bool>" default="false">"Whether the button is disabled."</ApiRow>
                    <ApiRow name="allow_focus_when_disabled" ty="bool" default="false">
                        "Keeps a disabled button focusable, but out of the tab order, so that focus isn\u{2019}t lost when the "
                        "focused button becomes disabled."
                    </ApiRow>
                    <ApiRow name="exclude_from_tab_order" ty="Signal<bool>" default="false">
                        "Skips the button when tabbing. Pointers and code can still focus it."
                    </ApiRow>
                    <ApiRow name="auto_focus" ty="bool" default="false">"Focuses the button when it mounts."</ApiRow>
                    <ApiRow name="prevent_focus_on_press" ty="bool" default="false">
                        "Doesn\u{2019}t move focus to the button when it is pressed, e.g. for toolbar buttons next to a text editor."
                    </ApiRow>
                    <ApiRow name="href" ty="Signal<Option<String>>" default="None">
                        "The link target of a "<Code inline=true>"ButtonElementType::Anchor"</Code>
                        ". Removed while the button is disabled."
                    </ApiRow>
                    <ApiRow name="target" ty="LinkTarget" default="Same">
                        "Where a "<Code inline=true>"ButtonElementType::Anchor"</Code>" opens its link."
                    </ApiRow>
                    <ApiRow name="rel" ty="Vec<LinkRel>" default="Vec::new()">
                        "The link relationship of a "<Code inline=true>"ButtonElementType::Anchor"</Code>". "
                        <Code inline=true>"LinkRel::NoOpener"</Code>" is added for "<Code inline=true>"LinkTarget::Blank"</Code>"."
                    </ApiRow>
                    <ApiRow name="form" ty="ButtonFormAttributes" default="ButtonFormAttributes::default()">
                        "Form attributes of a native "<Code inline=true>"<button>"</Code>": "<Code inline=true>"form"</Code>
                        ", "<Code inline=true>"form_action"</Code>", "<Code inline=true>"form_method"</Code>", "
                        <Code inline=true>"name"</Code>", "<Code inline=true>"value"</Code>" and more. Ignored for other element types."
                    </ApiRow>
                    <ApiRow name="aria_haspopup" ty="Signal<Option<AriaHasPopup>>" default="None">
                        "The kind of popup the button opens."
                    </ApiRow>
                    <ApiRow name="aria_expanded" ty="Signal<Option<AriaExpanded>>" default="None">
                        "Whether the element the button controls is expanded."
                    </ApiRow>
                    <ApiRow name="aria_controls" ty="Signal<Option<String>>" default="None">
                        "The ids of the elements the button controls."
                    </ApiRow>
                    <ApiRow name="aria_describedby" ty="Signal<Option<String>>" default="None">
                        "The ids of the elements describing the button."
                    </ApiRow>
                    <ApiRow name="aria_pressed" ty="Signal<Option<AriaPressed>>" default="None">
                        "The pressed state of a toggle button; "
                        <Link href=routes::doc::toggle_button::Hook.materialize()>"use_toggle_button"</Link>" sets it for you."
                    </ApiRow>
                    <ApiRow name="aria_checked" ty="Signal<Option<AriaChecked>>" default="None">
                        "The checked state of a button acting as a checkable item, e.g. "<Code inline=true>"role=\"radio\""</Code>"."
                    </ApiRow>
                    <ApiRow name="role" ty="Option<AriaRole>" default="None">
                        "Overrides the element\u{2019}s role, e.g. "<Code inline=true>"AriaRole::Radio"</Code>
                        " for the buttons of a single-selection toggle button group."
                    </ApiRow>
                    <ApiRow name="aria_current" ty="Signal<Option<AriaCurrent>>" default="None">
                        "Whether the button represents the current item of a set, such as the current page of a pagination."
                    </ApiRow>
                    <ApiRow name="on_press, on_press_start, on_press_end, on_press_up" ty="Option<Callback<PressEvent>>" default="None">
                        "Press callbacks, as in "<Link href=routes::doc::interactions::UsePress.materialize()>"use_press"</Link>"."
                    </ApiRow>
                    <ApiRow name="on_press_change" ty="Option<Callback<bool>>" default="None">
                        "Called when the pressed state changes."
                    </ApiRow>
                    <ApiRow name="on_long_press_start, on_long_press, on_long_press_end" ty="Option<Callback<LongPressEvent>>" default="None">
                        "Long press callbacks. Setting any of them turns on long press detection."
                    </ApiRow>
                    <ApiRow name="long_press_accessibility_description" ty="MaybeProp<String>" default="None">
                        "Describes the long press action to assistive technology, e.g. \u{201c}Long press to open menu\u{201d}."
                    </ApiRow>
                    <ApiRow name="on_hover_start, on_hover_end" ty="Option<Callback<HoverStartEvent>>, Option<Callback<HoverEndEvent>>" default="None">
                        "Called when a mouse or pen starts or stops hovering the button, as in "
                        <Link href=routes::doc::interactions::UseHover.materialize()>"use_hover"</Link>"."
                    </ApiRow>
                    <ApiRow name="on_hover_change" ty="Option<Callback<bool>>" default="None">
                        "Called when the hover state changes."
                    </ApiRow>
                    <ApiRow name="on_focus, on_blur" ty="Option<Callback<FocusEvent>>" default="None">
                        "Called when the button gains or loses focus."
                    </ApiRow>
                    <ApiRow name="on_focus_change" ty="Option<Callback<bool>>" default="None">
                        "Called when the focus state changes."
                    </ApiRow>
                    <ApiRow name="on_key_down, on_key_up" ty="Option<Callback<KeyboardEventWrapper>>" default="None">
                        "Raw keyboard callbacks, as in "<Link href=routes::doc::interactions::UseKeyboard.materialize()>"use_keyboard"</Link>"."
                    </ApiRow>
                    <ApiRow name="shortcuts" ty="Option<KeyboardShortcuts>" default="None">
                        "Keyboard shortcuts handled while the button has focus. They run before press handling, so a shortcut for "
                        <Keys keys="Enter"/>" or "<Keys keys="Space"/>" takes over the key."
                    </ApiRow>
                    <ApiRow name="on_context_menu" ty="Option<Callback<ContextMenuEvent>>" default="None">
                        "Called when the user requests a context menu on the button: a right click, "<Keys keys="Shift + F10"/>
                        ", the context menu key or a long press on a touch screen. See "
                        <Link href=routes::doc::interactions::UseContextMenu.materialize()>"use_context_menu"</Link>"."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="Return">
                <ApiTable kind=ApiKind::Return of="UseButtonReturn">
                    <ApiRow name="props" ty="PropsWithStyles<UseButtonProps>">
                        "Attributes, event handlers and styles of the button element. "
                        <Code inline=true>"props.into_parts()"</Code>" returns "<Code inline=true>"(attrs, styles)"</Code>
                        ": spread "<Code inline=true>"{..attrs}"</Code>" and set "<Code inline=true>"style=styles"</Code>
                        ". The attributes include "<Code inline=true>"data-focus-visible"</Code>"."
                    </ApiRow>
                    <ApiRow name="is_disabled" ty="Signal<bool>">
                        "Whether the button is disabled: by "<Code inline=true>"is_disabled"</Code>", or by a surrounding "
                        <Link href=routes::doc::interactions::PressResponder.materialize()><Code inline=true>"PressResponder"</Code></Link>
                        " (a disabled menu trigger or disclosure)."
                    </ApiRow>
                    <ApiRow name="is_pressed" ty="Signal<bool>">"Whether the button is being pressed right now."</ApiRow>
                    <ApiRow name="is_hovered" ty="Signal<bool>">"Whether a mouse or pen is over the button."</ApiRow>
                    <ApiRow name="is_focused" ty="Signal<bool>">"Whether the button has focus."</ApiRow>
                    <ApiRow name="is_focus_visible" ty="Signal<bool>">
                        "Whether the button has keyboard focus and should show a focus ring."
                    </ApiRow>
                    <ApiRow name="focus_handle" ty="FocusHandle">
                        "Focuses the button from code: "<Code inline=true>"focus_handle.focus()"</Code>"."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::hooks::*;
                        use leptos::{logging::log, prelude::*};

                        let UseButtonReturn { props, .. } = use_button(UseButtonInput {
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
                    ". Press it with the mouse, then "<Keys keys="Tab"/>" to it and press "<Keys keys="Enter"/>" or "
                    <Keys keys="Space"/>"."
                </p>

                <Demo description="A div made a button, counting presses, with a disabled toggle" source=include_str!("demos/button_basic.rs")>
                    <BasicButtonDemo/>
                </Demo>
            </Section>

            <Section title="Element Types">
                <p>
                    "A native "<Code inline=true>"<button>"</Code>" brings much of its behavior along; other elements need ARIA to "
                    "make up for it. "<Code inline=true>"element_type"</Code>" tells the hook what you render, so that it sets only "
                    "what is needed:"
                </p>

                <DocTable headers=&["ButtonElementType", "What you get"]>
                    <TableRow>
                        <TableCell><Code inline=true>"Button"</Code></TableCell>
                        <TableCell>
                            "A native "<Code inline=true>"<button>"</Code>": no role, "<Code inline=true>"type=\"button\""</Code>
                            " unless you pick another "<Code inline=true>"button_type"</Code>", the form attributes and the native "
                            <Code inline=true>"disabled"</Code>" attribute."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"Input"</Code></TableCell>
                        <TableCell>
                            "An "<Code inline=true>"<input>"</Code>": "<Code inline=true>"role=\"button\""</Code>", a "
                            <Code inline=true>"type"</Code>" and the native "<Code inline=true>"disabled"</Code>" attribute."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"Anchor"</Code></TableCell>
                        <TableCell>
                            "An "<Code inline=true>"<a>"</Code>" that acts as a button: "<Code inline=true>"role=\"button\""</Code>
                            ", its "<Code inline=true>"href"</Code>", "<Code inline=true>"target"</Code>" and "
                            <Code inline=true>"rel"</Code>". While it is disabled, the "<Code inline=true>"href"</Code>" is dropped and "
                            <Code inline=true>"aria-disabled"</Code>" is set. Its presses let the click bubble, so that a router "
                            "listening on the document sees it."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"Other"</Code></TableCell>
                        <TableCell>
                            "Any other element, like a "<Code inline=true>"<div>"</Code>" or "<Code inline=true>"<span>"</Code>": "
                            <Code inline=true>"role=\"button\""</Code>", a "<Code inline=true>"tabindex"</Code>" and "
                            <Code inline=true>"aria-disabled"</Code>" while disabled."
                        </TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Composition">
                <p>
                    "Some hooks don\u{2019}t render a button themselves but configure one: a menu trigger needs a button that opens "
                    "a menu, a number field needs two stepper buttons. These hooks return a "<Code inline=true>"UseButtonInput"</Code>
                    " instead of DOM props. You pass it to "<Code inline=true>"use_button"</Code>
                    ", and because it is a plain struct, you can add your own settings with struct update syntax:"
                </p>

                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::hooks::*;
                        use leptos::{logging::log, prelude::*};

                        let state = use_menu_trigger_state(UseMenuTriggerStateInput::default());
                        let menu_trigger = use_menu_trigger(UseMenuTriggerInput {
                            menu_type: OverlayTriggerType::Menu,
                            is_disabled: false.into(),
                            trigger: MenuTriggerType::Press,
                            state,
                        });

                        let button = use_button(UseButtonInput {
                            on_hover_start: Some(Callback::new(|_| log!("hovered"))),
                            ..menu_trigger.button
                        });
                        let (attrs, styles) = button.props.into_parts();

                        view! { <button {..attrs} style=styles>"Actions"</button> }
                    "#)}
                </Code>

                <p>
                    "The element ends up with exactly one press, focus and hover state machine. See "
                    <Link href=format!("{}#composing-hooks", routes::doc::Architecture.materialize())>"Composing Hooks"</Link>
                    " for the full picture. Hooks returning a "<Code inline=true>"UseButtonInput"</Code>" include "
                    <Link href=routes::doc::menu::Hook.materialize()>"use_menu_trigger"</Link>", "
                    <Link href=routes::doc::toggle_button::Hook.materialize()>"use_toggle_button"</Link>", "
                    <Link href=routes::doc::disclosure::Hook.materialize()>"use_disclosure"</Link>", "
                    <Link href=routes::doc::utilities::UseSpinButton.materialize()>"use_spin_button"</Link>" and "
                    <Link href=routes::doc::number_field::Hook.materialize()>"use_number_field"</Link>"."
                </p>
            </Section>

            <Section title="Keyboard">
                <KeyboardTable>
                    <KeyRow keys="Tab">"Focuses the button, unless "<Code inline=true>"exclude_from_tab_order"</Code>" is set."</KeyRow>
                    <KeyRow keys="Enter / Space">"Presses the button."</KeyRow>
                    <KeyRow keys="Shift + F10 / ContextMenu">
                        "Requests a context menu, when "<Code inline=true>"on_context_menu"</Code>" is set. On macOS, "
                        <Keys keys="Control + Enter"/>" does too."
                    </KeyRow>
                </KeyboardTable>

                <p>
                    "The "<Code inline=true>"shortcuts"</Code>" you pass are handled first. Holding a key down doesn\u{2019}t "
                    "trigger them again; see "<Link href=routes::doc::interactions::UseKeyboard.materialize()>"use_keyboard"</Link>
                    " for how shortcuts work."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Button.materialize()>"Button overview"</Link></li>
                <li><Link href=routes::doc::button::Atom.materialize()>"Button Atom"</Link></li>
                <li><Link href=routes::doc::button::Component.materialize()>"Button Components"</Link></li>
                <li><Link href=routes::doc::toggle_button::Hook.materialize()>"Toggle Button Hooks"</Link></li>
                <li><Link href=routes::doc::interactions::UsePress.materialize()>"use_press"</Link></li>
                <li><Link href=routes::doc::interactions::UseHover.materialize()>"use_hover"</Link></li>
                <li><Link href=routes::doc::focus::UseFocusable.materialize()>"use_focusable"</Link></li>
                <li><Link href=routes::doc::focus::UseFocusRing.materialize()>"use_focus_ring"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
