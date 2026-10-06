use leptonic::components::prelude::*;
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

    view! {
        <DocPage title="Toggle Button Hooks">
            <p>
                "The toggle button hooks make a "<Code inline=true>"<button>"</Code>" toggle between pressed and not pressed: "
                <Code inline=true>"use_toggle_button"</Code>" for a single button, "<Code inline=true>"use_toggle_group_state"</Code>
                ", "<Code inline=true>"use_toggle_button_group"</Code>" and "<Code inline=true>"use_toggle_button_group_item"</Code>
                " for a group of them. They prepare the input of "<Link href=routes::doc::button::Hook.materialize()>"use_button"</Link>
                ", which renders the button. See the "<Link href=routes::doc::ToggleButton.materialize()>"Toggle Button overview"</Link>
                " for concept guidance."
            </p>
            <ReactAria hook="useToggleButton"/>

            <Section title="use_toggle_button">
                <p>
                    "Returns a "<Code inline=true>"UseButtonInput"</Code>" whose "<Code inline=true>"on_press"</Code>" toggles the "
                    "state and whose "<Code inline=true>"aria_pressed"</Code>" follows it. Pass it to "<Code inline=true>"use_button"</Code>
                    ". The state comes from "<Link href=toggle_state>"use_toggle_state"</Link>"."
                </p>

                <Section title="Input" id="use-toggle-button-input">
                    <p>"Create it with "<Code inline=true>"UseToggleButtonInput::new(state)"</Code>"."</p>
                    <ApiTable kind=ApiKind::Input of="UseToggleButtonInput">
                        <ApiRow name="state" ty="ToggleState">"Whether the button is pressed."</ApiRow>
                        <ApiRow name="button" ty="UseButtonInput" default="UseButtonInput::default()">
                            "The button\u{2019}s further settings, e.g. "<Code inline=true>"is_disabled"</Code>" or "
                            <Code inline=true>"aria_label"</Code>". Its "<Code inline=true>"on_press"</Code>" runs after toggling."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Example" id="use-toggle-button-example">
                    <Demo
                        description="Pin toggle button with a disabled toggle"
                        source=include_str!("demos/toggle_button.rs")
                        source_open=true
                    >
                        <ToggleButtonDemo/>
                    </Demo>
                </Section>
            </Section>

            <Section title="use_toggle_group_state">
                <p>"Creates the state of a toggle button group: which buttons, identified by "<Code inline=true>"Key"</Code>"s, are selected."</p>

                <Section title="Input" id="use-toggle-group-state-input">
                    <ApiTable kind=ApiKind::Input of="UseToggleGroupStateInput">
                        <ApiRow name="selection_mode" ty="ToggleGroupSelectionMode" default="Single">
                            <Code inline=true>"Single"</Code>": at most one button is selected (selecting one deselects the others). "
                            <Code inline=true>"Multiple"</Code>": any number."
                        </ApiRow>
                        <ApiRow name="disallow_empty_selection" ty="bool" default="false">
                            "Keeps at least one button selected: the last selected button can\u{2019}t be deselected."
                        </ApiRow>
                        <ApiRow name="default_selected_keys" ty="HashSet<Key>" default="empty">"The initially selected buttons."</ApiRow>
                        <ApiRow name="on_selection_change" ty="Option<Callback<HashSet<Key>>>" default="None">
                            "Called with the selected buttons when they change."
                        </ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>" default="false">"Disables all buttons of the group."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="ToggleGroupState">
                    <ApiTable kind=ApiKind::Fields of="ToggleGroupState">
                        <ApiRow name="selection_mode" ty="ToggleGroupSelectionMode">"Single or multiple selection."</ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>">"Whether the group is disabled."</ApiRow>
                        <ApiRow name="selected_keys" ty="Signal<HashSet<Key>>">"The selected buttons."</ApiRow>
                    </ApiTable>
                    <DocTable headers=&["Method", "Description"]>
                        <TableRow><TableCell><Code inline=true>"is_selected(&Key)"</Code></TableCell><TableCell>"Whether the button is selected (tracked)."</TableCell></TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"toggle_key(&Key)"</Code></TableCell>
                            <TableCell>"Selects or deselects a button, respecting the selection mode and "<Code inline=true>"disallow_empty_selection"</Code>"."</TableCell>
                        </TableRow>
                        <TableRow><TableCell><Code inline=true>"set_selected(&Key, bool)"</Code></TableCell><TableCell>"Selects or deselects a button, in the same way."</TableCell></TableRow>
                        <TableRow><TableCell><Code inline=true>"set_selected_keys(HashSet<Key>)"</Code></TableCell><TableCell>"Replaces the selection."</TableCell></TableRow>
                    </DocTable>
                </Section>
            </Section>

            <Section title="use_toggle_button_group">
                <p>
                    "Renders the group as a "<Link href=routes::doc::hooks::UseToolbar.materialize()>"toolbar"</Link>
                    ": it is one tab stop, and the arrow keys move focus between its buttons. With single selection, the group is a "
                    <Code inline=true>"radiogroup"</Code>" instead."
                </p>

                <Section title="Input" id="use-toggle-button-group-input">
                    <p>"Create it with "<Code inline=true>"UseToggleButtonGroupInput::new(state)"</Code>"."</p>
                    <ApiTable kind=ApiKind::Input of="UseToggleButtonGroupInput">
                        <ApiRow name="state" ty="ToggleGroupState">"From "<Code inline=true>"use_toggle_group_state"</Code>"."</ApiRow>
                        <ApiRow name="toolbar" ty="UseToolbarInput" default="UseToolbarInput::default()">
                            "The orientation of the arrow keys and the group\u{2019}s label, as for "
                            <Link href=routes::doc::hooks::UseToolbar.materialize()>"use_toolbar"</Link>"."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-toggle-button-group-return">
                    <ApiTable kind=ApiKind::Return of="UseToggleButtonGroupReturn">
                        <ApiRow name="props" ty="UseToggleButtonGroupProps">
                            "The toolbar\u{2019}s role, orientation, label and key handling, plus "<Code inline=true>"aria-disabled"</Code>
                            ". Spread with "<Code inline=true>"{..props.into_attrs()}"</Code>"."
                        </ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="use_toggle_button_group_item">
                <p>
                    "Like "<Code inline=true>"use_toggle_button"</Code>", for a button of the group: pressing it selects or "
                    "deselects its key. In a single-selection group, the button is a "<Code inline=true>"radio"</Code>" with "
                    <Code inline=true>"aria-checked"</Code>"; otherwise it has "<Code inline=true>"aria-pressed"</Code>
                    ". It is disabled while the group is."
                </p>

                <Section title="Input" id="use-toggle-button-group-item-input">
                    <p>"Create it with "<Code inline=true>"UseToggleButtonGroupItemInput::new(state, key)"</Code>"."</p>
                    <ApiTable kind=ApiKind::Input of="UseToggleButtonGroupItemInput">
                        <ApiRow name="group" ty="ToggleGroupState">"The group state."</ApiRow>
                        <ApiRow name="key" ty="Key">"The button\u{2019}s key in the group."</ApiRow>
                        <ApiRow name="button" ty="UseButtonInput" default="UseButtonInput::default()">"The button\u{2019}s further settings."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Example" id="use-toggle-button-group-item-example">
                    <p>
                        "A single-selection group that can\u{2019}t be emptied, like a text alignment control. Tab into it and use "
                        <Keys keys="ArrowLeft"/>" and "<Keys keys="ArrowRight"/>" to move between the buttons, "<Keys keys="Space"/>
                        " to select."
                    </p>
                    <Demo
                        description="Single-selection text alignment group with a disabled toggle"
                        source=include_str!("demos/toggle_button_group.rs")
                        source_open=true
                    >
                        <ToggleButtonGroupDemo/>
                    </Demo>
                    <p>"A multiple-selection group, reporting its selection through "<Code inline=true>"on_selection_change"</Code>":"</p>
                    <Demo
                        description="Multiple-selection text formatting group"
                        source=include_str!("demos/toggle_button_group_multiple.rs")
                    >
                        <ToggleButtonGroupMultipleDemo/>
                    </Demo>
                </Section>
            </Section>

            <Section title="Keyboard">
                <KeyboardTable>
                    <KeyRow keys="Tab">"Moves focus to the button, or into and out of a group."</KeyRow>
                    <KeyRow keys="Space / Enter">"Toggles the focused button."</KeyRow>
                    <KeyRow keys="ArrowRight / ArrowLeft">
                        "In a horizontal group: focuses the next or previous button (reversed in right-to-left locales)."
                    </KeyRow>
                    <KeyRow keys="ArrowDown / ArrowUp">"In a vertical group: focuses the next or previous button."</KeyRow>
                </KeyboardTable>
                <p>"The arrow keys only move focus; they don\u{2019}t change the selection."</p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::ToggleButton.materialize()>"Toggle Button overview"</Link></li>
                <li><Link href=routes::doc::toggle_button::Atom.materialize()>"Toggle button atoms"</Link></li>
                <li><Link href=routes::doc::button::Hook.materialize()>"use_button"</Link></li>
                <li><Link href=routes::doc::hooks::UseToolbar.materialize()>"use_toolbar"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
