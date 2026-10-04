use indoc::indoc;
use leptonic::{components::prelude::*, utils::key::Key};
use leptos::prelude::*;

use crate::pages::documentation::{article::Article, demo_shell::DemoShell, toc::Toc};

use super::demos::button_basic::BasicButtonDemo;

#[component]
pub fn PageUseButton() -> impl IntoView {
    view! {
        <Article>
            <h1 id="use-button" class="anchor">
                "use_button"
                <AnchorLink href="#use-button" description="Direct link to article header"/>
            </h1>

            <p>
                "The "<Code inline=true>"use_button"</Code>" hook makes an element behave and announce itself as a button: "
                "press handling for mouse, touch, keyboard and screen readers, focus management, hover and focus-visible "
                "tracking, and the right attributes for the element you render. "
                "See the "<Link href=crate::routes::doc::Button.materialize()>"Button overview"</Link>" for concept guidance."
            </p>

            <p>
                "Based on react-aria\u{2019}s "
                <LinkExt href="https://react-spectrum.adobe.com/react-aria/useButton.html" target=LinkTarget::_Blank>
                    "useButton"
                </LinkExt>
                "."
            </p>

            <h2 id="input" class="anchor">
                "Input"
                <AnchorLink href="#input" description="Direct link to section: Input"/>
            </h2>

            <p>
                <Code inline=true>"UseButtonInput"</Code>" is a flat struct that implements "<Code inline=true>"Default"</Code>
                ". Everything is optional, so you only name the fields you care about and fill in the rest with "
                <Code inline=true>"..Default::default()"</Code>"."
            </p>

            <TableContainer>
                <Table bordered=true hoverable=true>
                    <TableHeader>
                        <TableRow>
                            <TableHeaderCell min_width=true>"Field"</TableHeaderCell>
                            <TableHeaderCell min_width=true>"Type"</TableHeaderCell>
                            <TableHeaderCell min_width=true>"Default"</TableHeaderCell>
                            <TableHeaderCell>"Description"</TableHeaderCell>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        <TableRow>
                            <TableCell><Code inline=true>"element_type"</Code></TableCell>
                            <TableCell><Code inline=true>"ButtonElementType"</Code></TableCell>
                            <TableCell><Code inline=true>"Button"</Code></TableCell>
                            <TableCell>"The kind of element you spread the props onto. Decides which attributes are set, see "<a href="#element-types">"Element types"</a>"."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"button_type"</Code></TableCell>
                            <TableCell><Code inline=true>"ButtonType"</Code></TableCell>
                            <TableCell><Code inline=true>"Button"</Code></TableCell>
                            <TableCell>"The "<Code inline=true>"type"</Code>" of a "<Code inline=true>"<button>"</Code>" or "<Code inline=true>"<input>"</Code>": "<Code inline=true>"Button"</Code>", "<Code inline=true>"Submit"</Code>" or "<Code inline=true>"Reset"</Code>". Unlike HTML, the default does not submit forms."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"id"</Code></TableCell>
                            <TableCell><Code inline=true>"Option<Oco<'static, str>>"</Code></TableCell>
                            <TableCell><Code inline=true>"None"</Code></TableCell>
                            <TableCell>"The element\u{2019}s id."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"aria_label"</Code></TableCell>
                            <TableCell><Code inline=true>"Option<Oco<'static, str>>"</Code></TableCell>
                            <TableCell><Code inline=true>"None"</Code></TableCell>
                            <TableCell>"An accessible name for buttons without visible text, such as icon buttons."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"aria_labelledby"</Code></TableCell>
                            <TableCell><Code inline=true>"Option<Oco<'static, str>>"</Code></TableCell>
                            <TableCell><Code inline=true>"None"</Code></TableCell>
                            <TableCell>"The id(s) of the element(s) that name the button."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"disabled"</Code></TableCell>
                            <TableCell><Code inline=true>"Signal<bool>"</Code></TableCell>
                            <TableCell><Code inline=true>"false"</Code></TableCell>
                            <TableCell>"Whether the button is disabled."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"allow_focus_when_disabled"</Code></TableCell>
                            <TableCell><Code inline=true>"bool"</Code></TableCell>
                            <TableCell><Code inline=true>"false"</Code></TableCell>
                            <TableCell>"Keep a disabled button focusable (with "<Code inline=true>"tabindex=\"-1\""</Code>"), so focus is not lost when a button becomes disabled while focused."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"exclude_from_tab_order"</Code></TableCell>
                            <TableCell><Code inline=true>"Signal<bool>"</Code></TableCell>
                            <TableCell><Code inline=true>"false"</Code></TableCell>
                            <TableCell>"Skip the button when tabbing. It can still be focused by pointer or programmatically."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"auto_focus"</Code></TableCell>
                            <TableCell><Code inline=true>"bool"</Code></TableCell>
                            <TableCell><Code inline=true>"false"</Code></TableCell>
                            <TableCell>"Focus the button when it mounts."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"prevent_focus_on_press"</Code></TableCell>
                            <TableCell><Code inline=true>"bool"</Code></TableCell>
                            <TableCell><Code inline=true>"false"</Code></TableCell>
                            <TableCell>"Don\u{2019}t move focus to the button when it is pressed, e.g. for toolbar buttons next to a text editor."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"href"</Code>", "<Code inline=true>"target"</Code>", "<Code inline=true>"rel"</Code></TableCell>
                            <TableCell><Code inline=true>"Option<Signal<String>>"</Code>", "<Code inline=true>"Option<LinkTarget>"</Code>", "<Code inline=true>"Option<Oco<'static, str>>"</Code></TableCell>
                            <TableCell><Code inline=true>"None"</Code></TableCell>
                            <TableCell>"Link attributes, only used for "<Code inline=true>"ButtonElementType::Anchor"</Code>". The "<Code inline=true>"href"</Code>" is removed while the button is disabled."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"form"</Code></TableCell>
                            <TableCell><Code inline=true>"ButtonFormAttributes"</Code></TableCell>
                            <TableCell>"empty"</TableCell>
                            <TableCell>"Form-related attributes of a native "<Code inline=true>"<button>"</Code>" ("<Code inline=true>"form"</Code>", "<Code inline=true>"formaction"</Code>", "<Code inline=true>"name"</Code>", "<Code inline=true>"value"</Code>", \u{2026}). Ignored for other element types."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"aria_haspopup"</Code></TableCell>
                            <TableCell><Code inline=true>"Signal<Option<AriaHasPopup>>"</Code></TableCell>
                            <TableCell><Code inline=true>"None"</Code></TableCell>
                            <TableCell>"The kind of popup the button opens."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"aria_expanded"</Code></TableCell>
                            <TableCell><Code inline=true>"Signal<Option<AriaExpanded>>"</Code></TableCell>
                            <TableCell><Code inline=true>"None"</Code></TableCell>
                            <TableCell>"Whether the element the button controls is expanded."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"aria_controls"</Code></TableCell>
                            <TableCell><Code inline=true>"Signal<Option<String>>"</Code></TableCell>
                            <TableCell><Code inline=true>"None"</Code></TableCell>
                            <TableCell>"The id(s) of the element(s) the button controls."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"aria_pressed"</Code></TableCell>
                            <TableCell><Code inline=true>"Signal<Option<AriaPressed>>"</Code></TableCell>
                            <TableCell><Code inline=true>"None"</Code></TableCell>
                            <TableCell>"The pressed state of a toggle button."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"aria_current"</Code></TableCell>
                            <TableCell><Code inline=true>"Signal<Option<AriaCurrent>>"</Code></TableCell>
                            <TableCell><Code inline=true>"None"</Code></TableCell>
                            <TableCell>"Whether the button represents the current item of a set."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>
                                <Code inline=true>"on_press"</Code>", "<Code inline=true>"on_press_start"</Code>", "
                                <Code inline=true>"on_press_end"</Code>", "<Code inline=true>"on_press_up"</Code>", "
                                <Code inline=true>"on_press_change"</Code>
                            </TableCell>
                            <TableCell><Code inline=true>"Option<Callback<PressEvent>>"</Code>" ("<Code inline=true>"bool"</Code>" for "<Code inline=true>"on_press_change"</Code>")"</TableCell>
                            <TableCell><Code inline=true>"None"</Code></TableCell>
                            <TableCell>"Press callbacks, as in "<Link href=crate::routes::doc::interactions::UsePress.materialize()>"use_press"</Link>"."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>
                                <Code inline=true>"on_long_press_start"</Code>", "<Code inline=true>"on_long_press"</Code>", "
                                <Code inline=true>"on_long_press_end"</Code>
                            </TableCell>
                            <TableCell><Code inline=true>"Option<Callback<LongPressEvent>>"</Code></TableCell>
                            <TableCell><Code inline=true>"None"</Code></TableCell>
                            <TableCell>"Long press callbacks. Setting any of them enables long press detection."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"long_press_accessibility_description"</Code></TableCell>
                            <TableCell><Code inline=true>"Option<Oco<'static, str>>"</Code></TableCell>
                            <TableCell><Code inline=true>"None"</Code></TableCell>
                            <TableCell>"Describes the long press action to assistive technology, e.g. \u{201c}Long press to open menu\u{201d}."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"on_hover_start"</Code>", "<Code inline=true>"on_hover_end"</Code>", "<Code inline=true>"on_hover_change"</Code></TableCell>
                            <TableCell><Code inline=true>"Option<Callback<..>>"</Code></TableCell>
                            <TableCell><Code inline=true>"None"</Code></TableCell>
                            <TableCell>"Hover callbacks, as in "<Link href=crate::routes::doc::interactions::UseHover.materialize()>"use_hover"</Link>"."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"on_focus"</Code>", "<Code inline=true>"on_blur"</Code>", "<Code inline=true>"on_focus_change"</Code></TableCell>
                            <TableCell><Code inline=true>"Option<Callback<..>>"</Code></TableCell>
                            <TableCell><Code inline=true>"None"</Code></TableCell>
                            <TableCell>"Focus callbacks."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"on_key_down"</Code>", "<Code inline=true>"on_key_up"</Code></TableCell>
                            <TableCell><Code inline=true>"Option<Callback<KeyboardEventWrapper>>"</Code></TableCell>
                            <TableCell><Code inline=true>"None"</Code></TableCell>
                            <TableCell>"Raw keyboard callbacks, as in "<Link href=crate::routes::doc::interactions::UseKeyboard.materialize()>"use_keyboard"</Link>"."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"shortcuts"</Code></TableCell>
                            <TableCell><Code inline=true>"Option<KeyboardShortcuts>"</Code></TableCell>
                            <TableCell><Code inline=true>"None"</Code></TableCell>
                            <TableCell>"Keyboard shortcuts handled while the button has focus. They run before press handling, so a shortcut for "<Code inline=true>"Enter"</Code>" or "<Code inline=true>"Space"</Code>" can take over the key."</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h2 id="return" class="anchor">
                "Return"
                <AnchorLink href="#return" description="Direct link to section: Return"/>
            </h2>

            <p><Code inline=true>"UseButtonReturn"</Code>" fields:"</p>

            <TableContainer>
                <Table bordered=true hoverable=true>
                    <TableHeader>
                        <TableRow>
                            <TableHeaderCell min_width=true>"Field"</TableHeaderCell>
                            <TableHeaderCell min_width=true>"Type"</TableHeaderCell>
                            <TableHeaderCell>"Description"</TableHeaderCell>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        <TableRow>
                            <TableCell><Code inline=true>"props"</Code></TableCell>
                            <TableCell><Code inline=true>"PropsWithStyles<UseButtonProps>"</Code></TableCell>
                            <TableCell>
                                "Attributes, event handlers and styles for the button element. Call "
                                <Code inline=true>"props.into_parts()"</Code>" to get "<Code inline=true>"(attrs, styles)"</Code>
                                ", then spread "<Code inline=true>"{..attrs}"</Code>" and set "<Code inline=true>"style=styles"</Code>"."
                            </TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"is_pressed"</Code></TableCell>
                            <TableCell><Code inline=true>"Signal<bool>"</Code></TableCell>
                            <TableCell>"Whether the button is currently pressed."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"is_hovered"</Code></TableCell>
                            <TableCell><Code inline=true>"Signal<bool>"</Code></TableCell>
                            <TableCell>"Whether a mouse or pen is over the button."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"is_focus_visible"</Code></TableCell>
                            <TableCell><Code inline=true>"Signal<bool>"</Code></TableCell>
                            <TableCell>"Whether a focus ring should be shown (keyboard focus only). Also exposed as the "<Code inline=true>"data-focus-visible"</Code>" attribute."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"focus_handle"</Code></TableCell>
                            <TableCell><Code inline=true>"FocusHandle"</Code></TableCell>
                            <TableCell>"Focus the button programmatically with "<Code inline=true>"focus_handle.focus()"</Code>"."</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h2 id="example" class="anchor">
                "Example"
                <AnchorLink href="#example" description="Direct link to section: Example"/>
            </h2>

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

            <h2 id="demo" class="anchor">
                "Demo"
                <AnchorLink href="#demo" description="Direct link to section: Demo"/>
            </h2>

            <p>"A "<Code inline=true>"<div>"</Code>" turned into a button with "<Code inline=true>"ButtonElementType::Other"</Code>". Try it with the mouse and with the keyboard."</p>

            <DemoShell
                source=include_str!("demos/button_basic.rs")
                description="Press count tracking on a div element"
            >
                <BasicButtonDemo />
            </DemoShell>

            <h2 id="element-types" class="anchor">
                "Element types"
                <AnchorLink href="#element-types" description="Direct link to section: Element types"/>
            </h2>

            <p>
                "Native buttons come with a lot of behavior for free, other elements need ARIA to make up for it. "
                <Code inline=true>"element_type"</Code>" tells the hook what you are rendering, so it only sets what is needed:"
            </p>

            <TableContainer>
                <Table bordered=true hoverable=true>
                    <TableHeader>
                        <TableRow>
                            <TableHeaderCell min_width=true>"ButtonElementType"</TableHeaderCell>
                            <TableHeaderCell>"What you get"</TableHeaderCell>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        <TableRow>
                            <TableCell><Code inline=true>"Button"</Code></TableCell>
                            <TableCell>"A native "<Code inline=true>"<button>"</Code>". No role, "<Code inline=true>"type=\"button\""</Code>" unless you pick another "<Code inline=true>"button_type"</Code>", form attributes, and the native "<Code inline=true>"disabled"</Code>" attribute."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"Input"</Code></TableCell>
                            <TableCell>"An "<Code inline=true>"<input>"</Code>". Gets "<Code inline=true>"role=\"button\""</Code>", a "<Code inline=true>"type"</Code>" and the native "<Code inline=true>"disabled"</Code>" attribute."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"Anchor"</Code></TableCell>
                            <TableCell>"An "<Code inline=true>"<a>"</Code>" that acts as a button. Gets "<Code inline=true>"role=\"button\""</Code>" and keeps its "<Code inline=true>"href"</Code>", "<Code inline=true>"target"</Code>" and "<Code inline=true>"rel"</Code>". While disabled, the "<Code inline=true>"href"</Code>" is dropped and "<Code inline=true>"aria-disabled"</Code>" is set."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"Other"</Code></TableCell>
                            <TableCell>"Anything else, like a "<Code inline=true>"<div>"</Code>" or "<Code inline=true>"<span>"</Code>". Gets "<Code inline=true>"role=\"button\""</Code>", a "<Code inline=true>"tabindex"</Code>" and "<Code inline=true>"aria-disabled"</Code>" while disabled."</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h2 id="composition" class="anchor">
                "Composition"
                <AnchorLink href="#composition" description="Direct link to section: Composition"/>
            </h2>

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
                "See "<Link href=format!("{}#composing-hooks", crate::routes::doc::Architecture.materialize())>"Composing hooks"</Link>
                " for the full picture. Hooks returning a "<Code inline=true>"UseButtonInput"</Code>" include "
                <Link href=crate::routes::doc::menu::Hook.materialize()>"use_menu_trigger"</Link>", "
                <Link href=crate::routes::doc::hooks::UseSpinButton.materialize()>"use_spin_button"</Link>" and "
                <Link href=crate::routes::doc::text_field::NumberFieldHook.materialize()>"use_number_field"</Link>"."
            </p>

            <h2 id="keyboard" class="anchor">
                "Keyboard"
                <AnchorLink href="#keyboard" description="Direct link to section: Keyboard"/>
            </h2>

            <ul>
                <li><KbdKey key=Key::Tab/>" \u{2014} Focus the button (unless "<Code inline=true>"exclude_from_tab_order"</Code>" is set)"</li>
                <li><KbdKey key=Key::Enter/>" / "<KbdKey key=Key::Space/>" \u{2014} Press the button"</li>
            </ul>

            <p>
                "Any "<Code inline=true>"shortcuts"</Code>" you pass are handled first. Holding a key down does not re-trigger them; see "
                <Link href=crate::routes::doc::interactions::UseKeyboard.materialize()>"use_keyboard"</Link>" for how shortcuts work."
            </p>

            <h2 id="deviations" class="anchor">
                "Deviations from react-aria"
                <AnchorLink href="#deviations" description="Direct link to section: Deviations"/>
            </h2>

            <ul>
                <li>
                    <b>"Hover and focus ring built in."</b>" React-aria leaves hover and focus-visible tracking to the "
                    <Code inline=true>"Button"</Code>" of react-aria-components, which combines "<Code inline=true>"useButton"</Code>", "
                    <Code inline=true>"useHover"</Code>" and "<Code inline=true>"useFocusRing"</Code>
                    ". Every leptonic button needs them, so "<Code inline=true>"use_button"</Code>" returns "
                    <Code inline=true>"is_hovered"</Code>" and "<Code inline=true>"is_focus_visible"</Code>" and sets "
                    <Code inline=true>"data-focus-visible"</Code>" itself."
                </li>
                <li>
                    <b>"Flat input with Default."</b>" Hooks that configure a button return a "<Code inline=true>"UseButtonInput"</Code>
                    " instead of DOM props, mirroring how react-aria passes "<Code inline=true>"AriaButtonProps"</Code>" around."
                </li>
                <li>
                    <b>"Long press is part of the input."</b>" Leptonic\u{2019}s "<Code inline=true>"use_press"</Code>
                    " detects long presses itself, so there is no separate "<Code inline=true>"useLongPress"</Code>"."
                </li>
                <li>
                    <b>"Few DOM passthroughs."</b>" Of the DOM props react-aria forwards, only "<Code inline=true>"id"</Code>", "
                    <Code inline=true>"aria-label"</Code>" and "<Code inline=true>"aria-labelledby"</Code>
                    " are part of the input, because hooks that configure a button need them. Set anything else directly on the element."
                </li>
                <li>
                    <b>"Anchor clicks bubble."</b>" Leptonic stops event propagation by default. Press events on "
                    <Code inline=true>"ButtonElementType::Anchor"</Code>" always propagate, so client-side routers like "
                    <Code inline=true>"leptos_router"</Code>", which listen on the document, still see the click."
                </li>
                <li>
                    <b>"No onClick."</b>" It is deprecated in react-aria; use "<Code inline=true>"on_press"</Code>"."
                </li>
            </ul>

            <h2 id="see-also" class="anchor">
                "See Also"
                <AnchorLink href="#see-also" description="Direct link to section: See Also"/>
            </h2>

            <ul>
                <li><Link href=crate::routes::doc::Button.materialize()>"Button overview"</Link></li>
                <li><Link href=crate::routes::doc::button::Atom.materialize()>"Button atom"</Link></li>
                <li><Link href=crate::routes::doc::button::Component.materialize()>"Button component"</Link></li>
                <li><Link href=crate::routes::doc::interactions::UsePress.materialize()>"use_press"</Link></li>
                <li><Link href=crate::routes::doc::interactions::UseHover.materialize()>"use_hover"</Link></li>
                <li><Link href=crate::routes::doc::focus::UseFocusable.materialize()>"use_focusable"</Link></li>
                <li><Link href=crate::routes::doc::focus::UseFocusRing.materialize()>"use_focus_ring"</Link></li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "use_button", link: "#use-button" },
                Toc::Leaf { title: "Input", link: "#input" },
                Toc::Leaf { title: "Return", link: "#return" },
                Toc::Leaf { title: "Example", link: "#example" },
                Toc::Leaf { title: "Demo", link: "#demo" },
                Toc::Leaf { title: "Element types", link: "#element-types" },
                Toc::Leaf { title: "Composition", link: "#composition" },
                Toc::Leaf { title: "Keyboard", link: "#keyboard" },
                Toc::Leaf { title: "Deviations", link: "#deviations" },
                Toc::Leaf { title: "See Also", link: "#see-also" },
            ]
        }/>
    }
}
