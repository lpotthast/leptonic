use indoc::indoc;
use leptos::prelude::*;

use super::demos::tag::TagDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageUseTag() -> impl IntoView {
    view! {
        <DocPage title="Tag Group Hooks">
            <p>
                "A tag group is a set of tags, such as keywords, active filters or the recipients of a message, that "
                "users navigate with the arrow keys and can select and remove. "<Code inline=true>"use_tag_group"</Code>
                " and "<Code inline=true>"use_tag"</Code>" build one from a list state; you render the tags and their "
                "remove buttons. A tag group is a horizontal "<Link href=routes::doc::GridList.materialize()>"grid list"</Link>
". See the "<Link href=routes::doc::TagGroup.materialize()>"Tag Group overview"</Link>" for concept guidance and keyboard interaction."
            </p>

            <ReactAria hook="useTagGroup"/>

            <Section title="Demo">
                <Demo description="Selectable tags removable with Delete, Backspace or their remove button" source=include_str!("demos/tag.rs")>
                    <TagDemo/>
                </Demo>
            </Section>

            <Section title="use_tag_group">
                <p>
                    "Takes a list state (see "<Link href=routes::doc::CollectionState.materialize()>"Collection State"</Link>
                    ") with the tags. Tags can only be removed if you pass "<Code inline=true>"on_remove"</Code>
                    ", which receives the keys to remove: drop them from your data."
                </p>

                <Section title="Input" id="use-tag-group-input">
                    <p>"Pass a "<Code inline=true>"UseTagGroupInput"</Code>" with every field named; the Default column gives the value for fields you don\u{2019}t need."</p>
                    <ApiTable kind=ApiKind::Input of="UseTagGroupInput">
                        <ApiRow name="state" ty="ListState">"The tags and their selection. Required."</ApiRow>
                        <ApiRow name="element" ty="CapturedElement">"The group element; the props capture it. Required."</ApiRow>
                        <ApiRow name="id" ty="Option<String>" default="None">"The element id, generated when "<Code inline=true>"None"</Code>"."</ApiRow>
                        <ApiRow name="has_label" ty="Signal<bool>" default="false">"Whether you render a visible label with "<Code inline=true>"label_props"</Code>"."</ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">"Labels the tag group without a visible label."</ApiRow>
                        <ApiRow name="aria_labelledby, aria_describedby" ty="Option<String>" default="None">"Further labelling and describing elements."</ApiRow>
                        <ApiRow name="keyboard_delegate" ty="Option<Signal<Arc<dyn KeyboardDelegate>>>" default="None">
                            "Replaces the horizontal "
                            <Link href=format!("{}#use-list-keyboard-delegate", routes::doc::CollectionState.materialize())>"list keyboard delegate"</Link>"."
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

                <Section title="Example" id="use-tag-group-example">
                    <Code language=Language::Rust>
                        {indoc!(r#"
                            use std::collections::HashSet;

                            use leptonic::{
                                hooks::{*, collections::{SelectionOptions, UseListStateInput}},
                                utils::CapturedElement,
                            };
                            use leptos::prelude::*;

                            let tags = RwSignal::new(vec!["Rust", "Leptos"]);
                            let collection = use_list_collection(tags.into(), |tag| Key::from(*tag), |tag| (*tag).to_owned());
                            let state = use_list_state(UseListStateInput { collection, selection: SelectionOptions::default() });

                            let UseTagGroupReturn { grid_props, label_props, data, .. } = use_tag_group(UseTagGroupInput {
                                state,
                                element: CapturedElement::new(),
                                id: None,
                                has_label: true.into(),
                                aria_label: MaybeProp::default(),
                                aria_labelledby: None,
                                aria_describedby: None,
                                keyboard_delegate: None,
                                on_remove: Some(Callback::new(move |keys: HashSet<Key>| {
                                    tags.update(|tags| tags.retain(|tag| !keys.contains(&Key::from(*tag))));
                                })),
                                on_action: None,
                            });

                        "#)}
                    </Code>
                </Section>
            </Section>

            <Section title="use_tag">
                <p>"One tag: a row with a single cell, selected by press and removed with its button or the keyboard."</p>
                <Section title="Input" id="use-tag-input">
                    <ApiTable kind=ApiKind::Input of="UseTagInput">
                        <ApiRow name="group" ty="TagGroupData">"The group, from "<Code inline=true>"use_tag_group"</Code>". Required."</ApiRow>
                        <ApiRow name="key" ty="Key">"The tag\u{2019}s key in the collection. Required."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-tag-return">
                    <ApiTable kind=ApiKind::Return of="UseTagReturn">
                        <ApiRow name="row_props" ty="PropsWithStyles<UseTagRowProps>">
                            "For the tag: "<Code inline=true>"role=\"row\""</Code>", selection and disabled state, press handling, "
                            "and removal with "<Keys keys="Delete"/>" or "<Keys keys="Backspace"/>". Removable tags are described as "
                            "\u{201c}Press Delete or Backspace to remove.\u{201d}"
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


            <SeeAlso>
                <li><Link href=routes::doc::Collections.materialize()>"Collections"</Link></li>
                <li><Link href=routes::doc::grid_list::Hook.materialize()>"Grid List Hooks"</Link></li>
                <li><Link href=routes::doc::TagGroup.materialize()>"Tag Group overview"</Link></li>
                <li><Link href=routes::doc::tag_group::Atom.materialize()>"Tag Group Atoms"</Link></li>
                <li><Link href=routes::doc::CollectionState.materialize()>"Collection State"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
