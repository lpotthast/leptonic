use std::sync::Arc;

use leptonic::{
    RGB8,
    atoms::{
        color_swatch::ColorSwatch,
        combobox::{ComboBox, ComboBoxPopover},
        field::Label,
        grid::{Grid, GridCell, GridRow},
        grid_list::{GridList, GridListItem},
        input::Input,
        listbox::{ListBox, ListBoxItems},
        select::{Select, SelectPopover, SelectTrigger, SelectValue},
        table::{Table, TableBody, TableCell, TableHeader, TableRow},
        tag_group::{TagGroup, TagItems, TagList},
    },
    hooks::{
        collections::{
            Collection, CollectionMemo, Key, UseListCollectionInput, use_list_collection,
        },
        table::TableCollection,
    },
};
use leptos::prelude::*;

const ANIMALS: [&str; 2] = ["Cat", "Dog"];

fn animals() -> CollectionMemo {
    use_list_collection(UseListCollectionInput {
        items: Signal::stored(ANIMALS.to_vec()),
        key: |animal| Key::from(*animal),
        text_value: |animal| (*animal).to_owned(),
    })
}

/// Atoms whose element has an id of its own, given an id by the app (`id`, as
/// react-aria-components' `id`): the grid list `own-grid-list`, the grid `own-grid`, the table
/// `own-table`, the tag list `own-tag-list`, the combo box input `own-combo-box`, the select
/// trigger `own-select` and the color swatch `own-color-swatch`.
#[component]
pub fn PageAtomOwnIds() -> impl IntoView {
    let grid: CollectionMemo = Memo::new(|_| {
        Arc::new(Collection::build(|b| {
            b.row("row", "Row", |r| {
                r.cell("Cell");
            });
        }))
    });
    let table = Memo::new(|_| {
        Arc::new(TableCollection::build(|t| {
            t.column("name", "Name");
            t.row("cat", "Cat", |r| {
                r.cell("Cat");
            });
        }))
    });
    view! {
        <h1>"Own ids"</h1>
        <GridList collection=animals() aria_label="Grid list" id="own-grid-list">
            <GridListItem key="Cat">"Cat"</GridListItem>
            <GridListItem key="Dog">"Dog"</GridListItem>
        </GridList>
        <Grid collection=grid aria_label="Grid" id="own-grid">
            <GridRow key="row">
                <GridCell key=Key::cell(&Key::from("row"), 0)>"Cell"</GridCell>
            </GridRow>
        </Grid>
        <Table table=table aria_label="Table" id="own-table">
            <TableHeader />
            <TableBody>
                <TableRow key="cat">
                    <TableCell column="name">"Cat"</TableCell>
                </TableRow>
            </TableBody>
        </Table>
        <TagGroup collection=animals() aria_label="Tags" id="own-tag-list">
            <TagList>
                <TagItems let:node>{node.text_value.to_string()}</TagItems>
            </TagList>
        </TagGroup>
        <ComboBox<Option<Key>> collection=animals() id="own-combo-box">
            <Label>"Combo box"</Label>
            <Input />
            <ComboBoxPopover>
                <ListBox>
                    <ListBoxItems let:node>{node.text_value.to_string()}</ListBoxItems>
                </ListBox>
            </ComboBoxPopover>
        </ComboBox<Option<Key>>>
        <Select<Option<Key>> collection=animals() id="own-select">
            <Label>"Select"</Label>
            <SelectTrigger>
                <SelectValue />
            </SelectTrigger>
            <SelectPopover>
                <ListBox>
                    <ListBoxItems let:node>{node.text_value.to_string()}</ListBoxItems>
                </ListBox>
            </SelectPopover>
        </Select<Option<Key>>>
        <ColorSwatch color=RGB8 { r: 255, g: 0, b: 0 } id="own-color-swatch" />
    }
}
