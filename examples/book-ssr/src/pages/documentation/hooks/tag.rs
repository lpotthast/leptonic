use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::tag::TagDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageUseTag() -> impl IntoView {
    view! {
        <DocPage title="Tag Group Hooks">
            <p>
                <Code inline=true>"use_tag_group"</Code>" and "<Code inline=true>"use_tag"</Code>
                " build a group of tags (keywords, filters, recipients) that can be navigated with the arrow keys, selected and "
                "removed. A tag group is a horizontal "<Link href=routes::doc::grid::Hook.materialize()>"grid list"</Link>
                ". See the "<Link href=routes::doc::Chip.materialize()>"Chip overview"</Link>" for concept guidance."
            </p>

            <ReactAria hook="useTagGroup"/>

            <Section title="Demo">
                <Demo description="Selectable tags removable with Delete, Backspace or their remove button" source=include_str!("demos/tag.rs")>
                    <TagDemo/>
                </Demo>
            </Section>

            <Section title="use_tag_group">
                <p>
                    "Takes a list state (see "<Link href=routes::doc::Collections.materialize()>"Collections"</Link>
                    ") with the tags. Tags can only be removed if you pass "<Code inline=true>"on_remove"</Code>
                    ", which receives the keys to remove: drop them from your data."
                </p>

                <Code language=Language::Rust>
                    {indoc!(r"
                        let collection = use_list_collection(tags.into(), |tag| Key::from(*tag), |tag| (*tag).to_owned());
                        let state = use_list_state(UseListStateInput { collection, selection: SelectionOptions::default() });

                        let UseTagGroupReturn { grid_props, label_props, data, .. } = use_tag_group(UseTagGroupInput {
                            has_label: true,
                            on_remove: Some(Callback::new(move |keys: HashSet<Key>| {
                                tags.update(|tags| tags.retain(|tag| !keys.contains(&Key::from(*tag))));
                            })),
                            ..UseTagGroupInput::new(state, CapturedElement::new())
                        });
                    ")}
                </Code>

                <Section title="Input" id="use-tag-group-input">
                    <p>"Create the input with "<Code inline=true>"UseTagGroupInput::new(state, element)"</Code>"."</p>
                    <ApiTable kind=ApiKind::Input of="UseTagGroupInput">
                        <ApiRow name="state" ty="ListState">"The tags and their selection."</ApiRow>
                        <ApiRow name="element" ty="CapturedElement">"The group element. The props capture it."</ApiRow>
                        <ApiRow name="id" ty="Option<String>" default="None">"The element id, generated when "<Code inline=true>"None"</Code>"."</ApiRow>
                        <ApiRow name="has_label" ty="bool" default="false">"Whether you render a visible label with "<Code inline=true>"label_props"</Code>"."</ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">"Labels the tag group without a visible label."</ApiRow>
                        <ApiRow name="aria_labelledby, aria_describedby" ty="Option<String>" default="None">"Further labelling and describing elements."</ApiRow>
                        <ApiRow name="keyboard_delegate" ty="Option<Signal<Arc<dyn KeyboardDelegate>>>" default="None">
                            "Replaces the horizontal list keyboard navigation."
                        </ApiRow>
                        <ApiRow name="on_remove" ty="Option<Callback<HashSet<Key>>>" default="None">
                            "Called with the keys of tags to remove. Without it, tags can\u{2019}t be removed."
                        </ApiRow>
                        <ApiRow name="on_action" ty="Option<Callback<Key>>" default="None">"Called with the key of an activated tag."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-tag-group-return">
                    <ApiTable kind=ApiKind::Return of="UseTagGroupReturn">
                        <ApiRow name="grid_props" ty="UseTagGroupProps">
                            "For the group element: "<Code inline=true>"role=\"grid\""</Code>" ("<Code inline=true>"group"</Code>
                            " while empty), labelling, and a polite live region announcing added tags while focus is in the group."
                        </ApiRow>
                        <ApiRow name="label_props" ty="UseLabelProps">"For the label."</ApiRow>
                        <ApiRow name="description_props, error_message_props" ty="SlotProps">
                            "For a description and an error message, referenced only while rendered."
                        </ApiRow>
                        <ApiRow name="data" ty="TagGroupData">"Hand this to "<Code inline=true>"use_tag"</Code>" for every tag."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="use_tag">
                <Section title="Input" id="use-tag-input">
                    <ApiTable kind=ApiKind::Input of="UseTagInput">
                        <ApiRow name="group" ty="TagGroupData">"The group, from "<Code inline=true>"use_tag_group"</Code>"."</ApiRow>
                        <ApiRow name="key" ty="Key">"The tag\u{2019}s key in the collection."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-tag-return">
                    <ApiTable kind=ApiKind::Return of="UseTagReturn">
                        <ApiRow name="row_props" ty="PropsWithStyles<UseTagRowProps>">
                            "For the tag: "<Code inline=true>"role=\"row\""</Code>", selection and disabled state, press handling, "
                            "and Delete/Backspace removal. Removable tags are described as \u{201c}Press Delete or Backspace to remove.\u{201d}"
                        </ApiRow>
                        <ApiRow name="grid_cell_props" ty="UseGridListItemCellProps">"For the tag\u{2019}s content: "<Code inline=true>"role=\"gridcell\""</Code>"."</ApiRow>
                        <ApiRow name="remove_button" ty="Option<UseButtonInput>">
                            "The remove button\u{2019}s configuration, for "<Link href=routes::doc::button::Hook.materialize()>"use_button"</Link>
                            ". "<Code inline=true>"None"</Code>" when the group has no "<Code inline=true>"on_remove"</Code>"."
                        </ApiRow>
                        <ApiRow name="is_selected, is_focused, is_focus_visible, is_disabled, is_pressed" ty="Signal<bool>">"The tag\u{2019}s state."</ApiRow>
                        <ApiRow name="allows_selection" ty="Signal<bool>">"Whether the tag can be selected."</ApiRow>
                        <ApiRow name="allows_removing" ty="bool">"Whether the tag can be removed."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Keyboard">
                <KeyboardTable>
                    <KeyRow keys="ArrowLeft / ArrowRight">"Focus the previous or next tag, wrapping around."</KeyRow>
                    <KeyRow keys="Home / End">"Focus the first or last tag."</KeyRow>
                    <KeyRow keys="Space">"Toggle the selection of the focused tag (when selection is enabled)."</KeyRow>
                    <KeyRow keys="Delete / Backspace">
                        "Remove the focused tag, or all selected tags if it is selected. Focus moves to a neighbor, or stays in the "
                        "empty group."
                    </KeyRow>
                </KeyboardTable>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Chip.materialize()>"Chip overview"</Link></li>
                <li><Link href=routes::doc::chip::Component.materialize()>"Chip component"</Link></li>
                <li><Link href=routes::doc::grid::Hook.materialize()>"Grid list hooks"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
