use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::interactions::InteractionsQuickStartDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageInteractions() -> impl IntoView {
    view! {
        <DocPage title="Interactions">
            <p>
                "Building blocks that make an element react to the user: pressing, hovering, moving, typing keys, "
                "scrolling and interacting outside of it. Concepts like "<Link href=routes::doc::Button.materialize()>"Button"</Link>
                ", "<Link href=routes::doc::Slider.materialize()>"Slider"</Link>" and "
                <Link href=routes::doc::Menu.materialize()>"Menu"</Link>" are built from them; use them directly when "
                "you give an element of your own one of these behaviors."
            </p>
            <p>
                "They belong together because each one turns the browser\u{2019}s raw events into one consistent event for "
                "every input device: a press is a press whether it comes from a mouse, touch, a pen, the keyboard or a "
                "screen reader."
            </p>

            <Section title="Pages">
                <SectionMembers overview=routes::doc::Interactions.materialize()/>
            </Section>

            <Section title="Relationships">
                <Section title="Hooks and Atoms">
                    <ul>
                        <li>
                            <Link href=routes::doc::interactions::PressResponder.materialize()>"PressResponder"</Link>
                            " hands press handlers to a pressable element inside it, which "
                            <Link href=routes::doc::interactions::UsePress.materialize()>"use_press"</Link>
                            " merges with its own. Triggers of menus and dialogs use it to open their overlay from any "
                            "button."
                        </li>
                        <li>
                            <Link href=routes::doc::interactions::Hoverable.materialize()>"Hoverable"</Link>" applies "
                            <Link href=routes::doc::interactions::UseHover.materialize()>"use_hover"</Link>
                            " to its child, without rendering an element of its own."
                        </li>
                        <li>
                            <Link href=routes::doc::interactions::UseKeyboard.materialize()>"use_keyboard"</Link>
                            " handles any key, while "<Code inline=true>"use_press"</Code>" already handles "
                            <Keys keys="Enter"/>" and "<Keys keys="Space"/>" as presses."
                        </li>
                        <li>
                            <Link href=routes::doc::interactions::UseContextMenu.materialize()>"use_context_menu"</Link>
                            " reports requests for a context menu (right click, "<Keys keys="Shift + F10"/>
                            ", long press). It returns long press callbacks for the element\u{2019}s "
                            <Code inline=true>"use_press"</Code>", because an element has one press handler."
                        </li>
                        <li>
                            <Link href=routes::doc::interactions::UseMove.materialize()>"use_move"</Link>
                            " reports how far the pointer or the arrow keys move a thumb. Moving items from one place to "
                            "another is "<Link href=routes::doc::DragAndDrop.materialize()>"Drag & Drop"</Link>"."
                        </li>
                    </ul>
                </Section>

                <Section title="Combining Hooks">
                    <p>
                        <Code inline=true>"use_press"</Code>" and "<Code inline=true>"use_hover"</Code>" are the most "
                        "commonly paired hooks. Spreading the props of two hooks onto one element would set some event "
                        "handlers twice, so "<Code inline=true>"merge_with"</Code>" (from the "<Code inline=true>"MergeWith"</Code>
                        " trait) combines them into one set of attributes and chains the handlers both hooks set:"
                    </p>

                    <Code language=Language::Rust>
                        {indoc!(r#"
                            use leptonic::{hooks::*, utils::MergeWith};
                            use leptos::prelude::*;

                            let press = use_press(UsePressInput::default());
                            let hover = use_hover(UseHoverInput::default());
                            // `use_press` returns its props with styles; the merge keeps them.
                            let (attrs, styles) = press.props.merge_with(hover.props).into_parts();

                            view! { <button {..attrs} style=styles>"Press or hover me"</button> }
                        "#)}
                    </Code>

                    <p>"These combinations can be merged:"</p>

                    <DocTable headers=&["Merged type", "Combines"]>
                        <TableRow><TableCell><Code inline=true>"MergedPressHoverProps"</Code></TableCell><TableCell>"use_press + use_hover"</TableCell></TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"MergedPressHoverFocusRingProps"</Code></TableCell>
                            <TableCell>"use_press + use_hover + use_focus_ring"</TableCell>
                        </TableRow>
                        <TableRow><TableCell><Code inline=true>"MergedPressFocusRingProps"</Code></TableCell><TableCell>"use_press + use_focus_ring"</TableCell></TableRow>
                        <TableRow><TableCell><Code inline=true>"MergedHoverFocusRingProps"</Code></TableCell><TableCell>"use_hover + use_focus_ring"</TableCell></TableRow>
                        <TableRow><TableCell><Code inline=true>"MergedFocusablePressProps"</Code></TableCell><TableCell>"use_focusable + use_press"</TableCell></TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"MergedFocusablePressFocusRingProps"</Code></TableCell>
                            <TableCell>"use_focusable + use_press + use_focus_ring"</TableCell>
                        </TableRow>
                    </DocTable>

                    <p>
                        <Code inline=true>"MergedPressHoverProps"</Code>" currently drops the "<Code inline=true>"dblclick"</Code>
                        " handler, so "<Code inline=true>"on_double_press"</Code>" doesn\u{2019}t fire on merged props."
                    </p>
                </Section>

                <Section title="Other Areas">
                    <ul>
                        <li>
                            <Link href=routes::doc::interactions::UseInteractOutside.materialize()>"use_interact_outside"</Link>
                            " lets "<Link href=routes::doc::overlay_behavior::UseOverlay.materialize()>"use_overlay"</Link>
                            " close popovers and menus when the user interacts outside of them."
                        </li>
                        <li>
                            <Code inline=true>"use_move"</Code>" drags the thumbs of "
                            <Link href=routes::doc::Slider.materialize()>"sliders"</Link>", "
                            <Link href=routes::doc::ColorArea.materialize()>"color areas"</Link>" and "
                            <Link href=routes::doc::ColorWheel.materialize()>"color wheels"</Link>"."
                        </li>
                        <li>
                            <Link href=routes::doc::interactions::UseScrollWheel.materialize()>"use_scroll_wheel"</Link>
                            " steps the value of "<Link href=routes::doc::NumberField.materialize()>"number fields"</Link>
                            " and "<Link href=routes::doc::ColorField.materialize()>"color fields"</Link>"."
                        </li>
                        <li>
                            "Focus is its own area: "<Link href=routes::doc::Focus.materialize()>"Focus"</Link>
                            " tracks and moves the keyboard focus and shows focus rings."
                        </li>
                    </ul>
                </Section>
            </Section>

            <Section title="Quick Start">
                <p>"The most common interaction: a pressable element that counts presses."</p>

                <Demo description="A button counting presses with use_press" source=include_str!("demos/interactions.rs") source_open=true>
                    <InteractionsQuickStartDemo/>
                </Demo>
            </Section>
        </DocPage>
    }
}
