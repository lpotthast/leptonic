use indoc::indoc;
use leptos::prelude::*;

use super::demos::{
    toggle_button::ToggleButtonDemo, toggle_button_group::ToggleButtonGroupDemo,
    toggle_button_group_multiple::ToggleButtonGroupMultipleDemo,
};
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageUseToggleButtonHook() -> impl IntoView {
    let toggle_state = format!(
        "{}#use-toggle-state",
        routes::doc::checkbox::Hook.materialize()
    );
    let toggle_state_input = toggle_state.clone();

    view! {
        <DocPage title="Toggle Button Hooks">
            <p>
                "The toggle button hooks make a button stay pressed until it is pressed again: "
                <AnchorLink href="#use-toggle-button"><Code inline=true>"use_toggle_button"</Code></AnchorLink>
                " for a single button, "
                <AnchorLink href="#use-toggle-group-state"><Code inline=true>"use_toggle_group_state"</Code></AnchorLink>", "
                <AnchorLink href="#use-toggle-button-group"><Code inline=true>"use_toggle_button_group"</Code></AnchorLink>" and "
                <AnchorLink href="#use-toggle-button-group-item"><Code inline=true>"use_toggle_button_group_item"</Code></AnchorLink>
                " for a group of them. See the "<Link href=routes::doc::ToggleButton.materialize()>"Toggle Button overview"</Link>
                " for concept guidance."
            </p>
            <ReactAria hook="useToggleButton"/>

            <Section title="Example">
                <p>
                    "The hooks don\u{2019}t render anything: they prepare the input of "
                    <Link href=routes::doc::button::Hook.materialize()>"use_button"</Link>", which makes the element a button. "
                    "A single toggle button keeps its state in "<Link href=toggle_state>"use_toggle_state"</Link>":"
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::hooks::*;
                        use leptos::prelude::*;

                        let state = use_toggle_state(UseToggleStateInput::default());
                        let button = use_button(use_toggle_button(UseToggleButtonInput {
                            state,
                            button: UseButtonInput::default(),
                        }));
                        let (attrs, styles) = button.props.into_parts();

                        view! { <button {..attrs} style=styles>"Pin"</button> }
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <Demo
                    description="Pin toggle button with a disabled toggle"
                    source=include_str!("demos/toggle_button.rs")
                    source_open=true
                >
                    <ToggleButtonDemo/>
                </Demo>

                <p>
                    "A single-selection group that can\u{2019}t be emptied, like a text alignment control. "<Keys keys="Tab"/>
                    " into it, move between the buttons with "<Keys keys="ArrowLeft"/>" and "<Keys keys="ArrowRight"/>
                    " and select one with "<Keys keys="Space"/>":"
                </p>
                <Demo
                    description="Single-selection text alignment group with a disabled toggle"
                    source=include_str!("demos/toggle_button_group.rs")
                >
                    <ToggleButtonGroupDemo/>
                </Demo>

                <p>"A multiple-selection group whose selection is bound to app state:"</p>
                <Demo
                    description="Multiple-selection text formatting group with a disabled toggle"
                    source=include_str!("demos/toggle_button_group_multiple.rs")
                >
                    <ToggleButtonGroupMultipleDemo/>
                </Demo>
            </Section>

            <Section title="use_toggle_button">
                <p>"Makes a button toggle a "<Code inline=true>"ToggleState"</Code>" when pressed."</p>

                <Section title="Input" id="use-toggle-button-input">
                    <p>"Pass a "<Code inline=true>"UseToggleButtonInput"</Code>" with both fields named; the Default column gives the value for a field you don\u{2019}t need."</p>
                    <ApiTable kind=ApiKind::Input of="UseToggleButtonInput">
                        <ApiRow name="state" ty="ToggleState">
                            "Whether the button is pressed, from "<Link href=toggle_state_input>"use_toggle_state"</Link>". Required."
                        </ApiRow>
                        <ApiRow name="button" ty="UseButtonInput" default="UseButtonInput::default()">
                            "The button\u{2019}s further settings, e.g. "<Code inline=true>"is_disabled"</Code>" or "
                            <Code inline=true>"aria_label"</Code>". Its "<Code inline=true>"on_press"</Code>" runs after toggling."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-toggle-button-return">
                    <p>
                        "A "<Code inline=true>"UseButtonInput"</Code>": "<Code inline=true>"button"</Code>" with an "
                        <Code inline=true>"on_press"</Code>" that toggles the state and an "<Code inline=true>"aria_pressed"</Code>
                        " that follows it. Pass it to "<Link href=routes::doc::button::Hook.materialize()>"use_button"</Link>"."
                    </p>
                </Section>
            </Section>

            <Section title="use_toggle_group_state">
                <p>"Creates the state of a toggle button group: which of its buttons, identified by "<Code inline=true>"Key"</Code>"s, are selected."</p>

                <Section title="Input" id="use-toggle-group-state-input">
                    <ApiTable kind=ApiKind::Input of="UseToggleGroupStateInput">
                        <ApiRow name="selection_mode" ty="ToggleGroupSelectionMode" default="Single">
                            <Code inline=true>"Single"</Code>": at most one button is selected; selecting one deselects the other. "
                            <Code inline=true>"Multiple"</Code>": any number."
                        </ApiRow>
                        <ApiRow name="disallow_empty_selection" ty="bool" default="false">
                            "Keeps at least one button selected: the last selected button can\u{2019}t be deselected."
                        </ApiRow>
                        <ApiRow name="default_selected_keys" ty="HashSet<Key>" default="HashSet::new()">
                            "The initially selected buttons. Ignored when "<Code inline=true>"selected_keys"</Code>" is set."
                        </ApiRow>
                        <ApiRow name="selected_keys" ty="Option<ValueBinding<HashSet<Key>>>" default="None">
                            "Binds the selection to app state, e.g. an "<Code inline=true>"RwSignal"</Code>
                            " ("<Code inline=true>"Some(signal.into())"</Code>"), replacing "<Code inline=true>"default_selected_keys"</Code>"."
                        </ApiRow>
                        <ApiRow name="on_selection_change" ty="Option<Callback<HashSet<Key>>>" default="None">
                            "Called with the selected buttons when they change."
                        </ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>" default="false">"Disables all buttons of the group."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-toggle-group-state-return">
                    <ApiTable kind=ApiKind::Fields of="ToggleGroupState">
                        <ApiRow name="selection_mode" ty="ToggleGroupSelectionMode">"Single or multiple selection."</ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>">"Whether the group is disabled."</ApiRow>
                        <ApiRow name="selected_keys" ty="Signal<HashSet<Key>>">"The selected buttons."</ApiRow>
                    </ApiTable>
                    <DocTable headers=&["Method", "Description"]>
                        <TableRow><TableCell><Code inline=true>"is_selected(&Key)"</Code></TableCell><TableCell>"Whether the button is selected. Tracked, so it can drive views."</TableCell></TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"toggle_key(&Key)"</Code></TableCell>
                            <TableCell>"Selects or deselects a button, respecting the selection mode and "<Code inline=true>"disallow_empty_selection"</Code>"."</TableCell>
                        </TableRow>
                        <TableRow><TableCell><Code inline=true>"set_selected(&Key, bool)"</Code></TableCell><TableCell>"Selects or deselects a button, with the same rules."</TableCell></TableRow>
                        <TableRow><TableCell><Code inline=true>"set_selected_keys(HashSet<Key>)"</Code></TableCell><TableCell>"Replaces the selection."</TableCell></TableRow>
                    </DocTable>
                </Section>
            </Section>

            <Section title="use_toggle_button_group">
                <p>
                    "Makes the group element a "<Link href=routes::doc::toolbar::Hook.materialize()>"toolbar"</Link>
                    ": one tab stop, with the arrow keys moving focus between its buttons. With single selection, the group is a "
                    <Code inline=true>"radiogroup"</Code>" instead."
                </p>

                <Section title="Input" id="use-toggle-button-group-input">
                    <p>"Pass a "<Code inline=true>"UseToggleButtonGroupInput"</Code>" with both fields named; the Default column gives the value for a field you don\u{2019}t need."</p>
                    <ApiTable kind=ApiKind::Input of="UseToggleButtonGroupInput">
                        <ApiRow name="state" ty="ToggleGroupState">"From "<AnchorLink href="#use-toggle-group-state"><Code inline=true>"use_toggle_group_state"</Code></AnchorLink>". Required."</ApiRow>
                        <ApiRow name="toolbar" ty="UseToolbarInput" default="UseToolbarInput::default()">
                            "The orientation of the arrow keys and the group\u{2019}s accessible name, as for "
                            <Link href=routes::doc::toolbar::Hook.materialize()>"use_toolbar"</Link>". Give every group a name."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-toggle-button-group-return">
                    <ApiTable kind=ApiKind::Return of="UseToggleButtonGroupReturn">
                        <ApiRow name="props" ty="UseToggleButtonGroupProps">
                            "The toolbar\u{2019}s role, orientation, name and key handling, plus "<Code inline=true>"aria-disabled"</Code>
                            ". Spread them with "<Code inline=true>"{..props.into_attrs()}"</Code>"."
                        </ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="use_toggle_button_group_item">
                <p>
                    "Like "<AnchorLink href="#use-toggle-button"><Code inline=true>"use_toggle_button"</Code></AnchorLink>
                    ", for a button of a group: pressing it selects or deselects its key. In a single-selection group, the button "
                    "is a "<Code inline=true>"radio"</Code>" with "<Code inline=true>"aria-checked"</Code>"; otherwise it has "
                    <Code inline=true>"aria-pressed"</Code>". It is disabled while the group is."
                </p>

                <Section title="Input" id="use-toggle-button-group-item-input">
                    <p>"Pass a "<Code inline=true>"UseToggleButtonGroupItemInput"</Code>" with every field named; the Default column gives the value for fields you don\u{2019}t need."</p>

                    <ApiTable kind=ApiKind::Input of="UseToggleButtonGroupItemInput">
                        <ApiRow name="group" ty="ToggleGroupState">"The group\u{2019}s state. Required."</ApiRow>
                        <ApiRow name="key" ty="Key">"The button\u{2019}s key in the group. Required."</ApiRow>
                        <ApiRow name="button" ty="UseButtonInput" default="UseButtonInput::default()">"The button\u{2019}s further settings."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-toggle-button-group-item-return">
                    <p>
                        "A "<Code inline=true>"UseButtonInput"</Code>" for "<Link href=routes::doc::button::Hook.materialize()>"use_button"</Link>
                        ", as "<AnchorLink href="#use-toggle-button-return">"for a single button"</AnchorLink>"."
                    </p>
                </Section>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::ToggleButton.materialize()>"Toggle Button overview"</Link></li>
                <li><Link href=routes::doc::toggle_button::Atom.materialize()>"Toggle Button Atoms"</Link></li>
                <li><Link href=routes::doc::button::Hook.materialize()>"use_button"</Link></li>
                <li><Link href=routes::doc::toolbar::Hook.materialize()>"use_toolbar"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
