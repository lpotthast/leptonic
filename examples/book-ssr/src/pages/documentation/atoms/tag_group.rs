use indoc::indoc;
use leptos::prelude::*;

use super::demos::tag_group::TagGroupAtomDemo;
use crate::{kit::*, routes};

/// A section of the tag group hooks page.
fn hook_section(id: &str) -> String {
    format!("{}#{id}", routes::doc::tag_group::Hook.materialize())
}

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageAtomTagGroup() -> impl IntoView {
    view! {
        <DocPage title="Tag Group Atoms">
            <p>
                <Code inline=true>"TagGroup"</Code>", "<Code inline=true>"TagList"</Code>", "<Code inline=true>"Tag"</Code>
                " and "<Code inline=true>"TagRemoveButton"</Code>" render an unstyled tag group: a focusable list of tags "
                "to navigate, select and remove. See the "<Link href=routes::doc::TagGroup.materialize()>"Tag Group overview"</Link>
                " for concept guidance."
            </p>

            <Section title="Hooks Used">
                <ul>
                    <li>
                        <AnchorLink href="#taggroup"><Code inline=true>"TagGroup"</Code></AnchorLink>" builds a list state "
                        "from its collection and calls "<Link href=hook_section("use-tag-group")>"use_tag_group"</Link>"."
                    </li>
                    <li>
                        <AnchorLink href="#tag"><Code inline=true>"Tag"</Code></AnchorLink>" calls "
                        <Link href=hook_section("use-tag")>"use_tag"</Link>", "<AnchorLink href="#tagremovebutton">
                        <Code inline=true>"TagRemoveButton"</Code></AnchorLink>" "
                        <Link href=routes::doc::button::Hook.materialize()>"use_button"</Link>"."
                    </li>
                </ul>
            </Section>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use std::collections::HashSet;

                        use leptonic::{
                            atoms::{field::Label, tag_group::{TagGroup, TagItems, TagList, TagRemoveButton}},
                            hooks::collections::{Key, UseListCollectionInput, use_list_collection},
                        };
                        use leptos::prelude::*;

                        let tags = RwSignal::new(vec!["News", "Travel", "Gaming"]);
                        let collection = use_list_collection(UseListCollectionInput {
                            items: tags.into(),
                            key: |tag| Key::from(*tag),
                            text_value: |tag| (*tag).to_owned(),
                        });

                        view! {
                            <TagGroup
                                collection=collection
                                on_remove={move |keys: HashSet<Key>| {
                                    tags.update(|tags| tags.retain(|tag| !keys.contains(&Key::from(*tag))));
                                }}
                            >
                                <Label>"Categories"</Label>
                                <TagList>
                                    <TagItems let:node>
                                        {node.text_value.to_string()}
                                        <TagRemoveButton>"×"</TagRemoveButton>
                                    </TagItems>
                                </TagList>
                            </TagGroup>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <Demo description="Selectable tags removable with Delete, Backspace or their remove button" source=include_str!("demos/tag_group.rs")>
                    <TagGroupAtomDemo/>
                </Demo>
            </Section>

            <Section title="TagGroup">
                <p>
                    "The group: holds a "<Link href=format!("{}#label", routes::doc::field::Atom.materialize())>"Label"</Link>
                    ", the "<Code inline=true>"TagList"</Code>" and optionally a "<Code inline=true>"Description"</Code>
                    ", and connects them."
                </p>
                <Section title="Props" id="taggroup-props">
                    <ApiTable kind=ApiKind::Props of="TagGroup">
                        <ApiRow name="collection" ty="CollectionMemo">"The tags. Required."</ApiRow>
                        <ApiRow name="selection_mode" ty="Signal<SelectionMode>" default="None">
                            <Code inline=true>"None"</Code>", "<Code inline=true>"Single"</Code>" or "<Code inline=true>"Multiple"</Code>
                            " tags can be selected."
                        </ApiRow>
                        <ApiRow name="selection_behavior" ty="Signal<SelectionBehavior>" default="Toggle">
                            "How pointer presses change the selection: "<Code inline=true>"Toggle"</Code>" the tag, or "
                            <Code inline=true>"Replace"</Code>" the selection with it."
                        </ApiRow>
                        <ApiRow name="default_selection" ty="Selection" default="empty">"The initially selected tags."</ApiRow>
                        <ApiRow name="selection" ty="Option<Signal<Selection>>" default="None">
                            "The selection (controlled), replacing "<Code inline=true>"default_selection"</Code>": a value or any signal."
                        </ApiRow>
                        <ApiRow name="set_selection" ty="Option<Out<Selection>>" default="None">
                            "Receives the new selection: an "<Code inline=true>"RwSignal"</Code>", "<Code inline=true>"WriteSignal"</Code>
                            ", closure, "<Code inline=true>"Callback"</Code>", \u{2026}"
                        </ApiRow>
                        <ApiRow name="on_selection_change" ty="Option<Callback<Selection>>" default="None">"Called when the selection changes."</ApiRow>
                        <ApiRow name="disabled_keys" ty="Option<Signal<HashSet<Key>>>" default="None">"Tags that can\u{2019}t be used."</ApiRow>
                        <ApiRow name="disabled_behavior" ty="DisabledBehavior" default="All">
                            "Whether disabled tags can\u{2019}t be focused or used at all ("<Code inline=true>"All"</Code>
                            "), or only not selected ("<Code inline=true>"Selection"</Code>")."
                        </ApiRow>
                        <ApiRow name="disallow_empty_selection" ty="Signal<bool>" default="false">"Whether the last selected tag can\u{2019}t be deselected."</ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">"Names a group without a visible "<Code inline=true>"Label"</Code>"."</ApiRow>
                        <ApiRow name="aria_labelledby, aria_describedby" ty="Option<String>" default="None">
                            "Further labelling and describing elements."
                        </ApiRow>
                        <ApiRow name="on_remove" ty="Option<Callback<HashSet<Key>>>" default="None">
                            "Called with the keys of the tags to remove: drop them from your data. Without it, tags can\u{2019}t "
                            "be removed and "<Code inline=true>"TagRemoveButton"</Code>" renders nothing."
                        </ApiRow>
                        <ApiRow name="on_action" ty="Option<Callback<Key>>" default="None">"Called with the key of an activated tag."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the group element."</ApiRow>
                        <ApiRow name="children" ty="Children">"The label, the tag list and a description. Required."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="TagList">
                <p>"The list of tags: the "<Code inline=true>"grid"</Code>" element."</p>
                <Section title="Props" id="taglist-props">
                    <ApiTable kind=ApiKind::Props of="TagList">
                        <ApiRow name="empty_state" ty="Option<ViewFn>" default="None">"Shown while the group has no tags."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the list."</ApiRow>
                        <ApiRow name="children" ty="Children">"A "<Code inline=true>"Tag"</Code>" per item, or "<Code inline=true>"TagItems"</Code>". Required."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Tag">
                <p>"A tag for one item of the collection: a "<Code inline=true>"row"</Code>" with a single "<Code inline=true>"gridcell"</Code>" holding its content."</p>
                <Section title="Props" id="tag-props">
                    <ApiTable kind=ApiKind::Props of="Tag">
                        <ApiRow name="key" ty="Key">"The tag\u{2019}s key in the group\u{2019}s collection. Required."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the tag."</ApiRow>
                        <ApiRow name="children" ty="Children">"The tag\u{2019}s content, e.g. its text and a "<Code inline=true>"TagRemoveButton"</Code>". Required."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="TagItems">
                <p>"Renders a "<Code inline=true>"Tag"</Code>" for every item of the collection; "<Code inline=true>"let:node"</Code>" names the item."</p>
                <Section title="Props" id="tagitems-props">
                    <ApiTable kind=ApiKind::Props of="TagItems">
                        <ApiRow name="children" ty="Fn(Node) -> IV">"Renders a tag\u{2019}s content. Required."</ApiRow>
                        <ApiRow name="classes" ty="Classes" default="empty">"Classes of each tag."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="TagRemoveButton">
                <p>
                    "The button removing its tag, rendered only when the group has "<Code inline=true>"on_remove"</Code>
                    ". It is named \u{201c}Remove\u{201d} and the tag\u{2019}s name; keyboard users can also press "
                    <Keys keys="Delete"/>" or "<Keys keys="Backspace"/>" on the tag."
                </p>
                <Section title="Props" id="tagremovebutton-props">
                    <ApiTable kind=ApiKind::Props of="TagRemoveButton">
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the button."</ApiRow>
                        <ApiRow name="children" ty="Option<Children>" default="None">"The button\u{2019}s content, e.g. \u{201c}\u{00d7}\u{201d} or an icon."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Data Attributes">
                <ApiTable kind=ApiKind::DataAttributes>
                    <ApiRow name="data-empty" ty="true">"On "<Code inline=true>"TagList"</Code>": the group has no tags."</ApiRow>
                    <ApiRow name="data-focused, data-focus-visible" ty="true">
                        "On "<Code inline=true>"TagList"</Code>" and "<Code inline=true>"Tag"</Code>": the element has the focus "
                        "(by keyboard). The list itself takes the focus only while it is empty. "<Code inline=true>"data-focus-visible"</Code>
                        " also on "<Code inline=true>"TagRemoveButton"</Code>"."
                    </ApiRow>
                    <ApiRow name="data-selected, data-disabled, data-pressed" ty="true">"On "<Code inline=true>"Tag"</Code>": its state."</ApiRow>
                    <ApiRow name="data-hovered" ty="true">"On "<Code inline=true>"Tag"</Code>" (when it can be selected or activated) and "<Code inline=true>"TagRemoveButton"</Code>"."</ApiRow>
                    <ApiRow name="data-allows-removing" ty="true">"On "<Code inline=true>"Tag"</Code>": the group has "<Code inline=true>"on_remove"</Code>"."</ApiRow>
                    <ApiRow name="data-selection-mode" ty="\"single\" | \"multiple\"">
                        "On "<Code inline=true>"Tag"</Code>": the group\u{2019}s selection mode; absent without selection."
                    </ApiRow>
                </ApiTable>
                <p><Code inline=true>"TagRemoveButton"</Code>" also has "<Code inline=true>"data-pressed"</Code>"."</p>
            </Section>

            <Section title="Styling">
                <p>
                    "The atoms bring no styles. They render the classes "<Code inline=true>"leptonic-TagGroup"</Code>", "
                    <Code inline=true>"leptonic-TagList"</Code>", "<Code inline=true>"leptonic-Tag"</Code>" and "
                    <Code inline=true>"leptonic-TagRemoveButton"</Code>", each followed by the "<Code inline=true>"classes"</Code>
                    " you pass ("<Code inline=true>"TagItems"</Code>" passes its "<Code inline=true>"classes"</Code>" to each "
                    <Code inline=true>"Tag"</Code>"). The remove button\u{2019}s content is yours. The tags draw their focus "
                    "ring themselves, as the grid cell inside is "<Code inline=true>"display: contents"</Code>
                    ". The book\u{2019}s demos use these rules:"
                </p>
                <Code language=Language::Css>
                    {indoc!(r"
                        .my-tag-list { display: flex; flex-wrap: wrap; gap: 0.5em; }
                        .my-tag-list[data-empty] { color: var(--muted); font-style: italic; }
                        .my-tag { display: inline-flex; align-items: center; gap: 0.25em; padding: 0.25em 0.5em; border: 1px solid var(--border); border-radius: 999px; outline: none; }
                        .my-tag[data-selection-mode] { cursor: pointer; }
                        .my-tag[data-hovered] { border-color: var(--accent); }
                        .my-tag[data-selected] { background: var(--accent); color: var(--surface); }
                        .my-tag[data-focus-visible] { box-shadow: 0 0 0 2px var(--focus); }
                        .my-tag-remove { border: none; border-radius: 50%; background: none; color: inherit; }
                        .my-tag-remove[data-focus-visible] { outline: 2px solid var(--focus); }
                    ")}
                </Code>
                <p>
                    "Leptonic also ships an optional atom theme that styles the default classes, for apps that don\u{2019}t "
                    "want to start from scratch: "<Code inline=true>"@use \"leptonic/leptonic-atoms\";"</Code>"."
                </p>
            </Section>

            <Section title="Composition">
                <p>
                    <Code inline=true>"Label"</Code>" and "<Code inline=true>"Description"</Code>" are the "
                    <Link href=routes::doc::field::Atom.materialize()>"Field Atoms"</Link>": inside a "
                    <Code inline=true>"TagGroup"</Code>", they label and describe its list. Render your own "
                    <Code inline=true>"Tag"</Code>"s instead of "<Code inline=true>"TagItems"</Code>" to give single tags "
                    "their own content, one per item of the collection."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::TagGroup.materialize()>"Tag Group overview"</Link></li>
                <li><Link href=routes::doc::tag_group::Hook.materialize()>"Tag Group Hooks"</Link></li>
                <li><Link href=routes::doc::grid_list::Atom.materialize()>"Grid List Atoms"</Link></li>
                <li><Link href=routes::doc::CollectionState.materialize()>"Collection State"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
