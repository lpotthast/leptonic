// Upstream: react-aria/src/tag/useTag.ts @ 99e6102368
use std::collections::HashSet;

use leptos::{
    attr::{self, Attr},
    prelude::*,
};

use super::TagGroupData;
use crate::{
    hooks::{
        FocusMode, IntoAttrs, PropsWithStyles,
        button::use_button::UseButtonInput,
        collections::Key,
        focus::use_focus_visible::{Modality, UseFocusVisibleInput, use_focus_visible},
        gridlist::{
            UseGridListItemCellProps, UseGridListItemInput, UseGridListItemReturn,
            UseGridListItemRowAttrs, UseGridListItemRowProps, grid_list_row_id, use_grid_list_item,
        },
        interactions::{
            use_keyboard::{UseKeyboardAttrs, UseKeyboardInput, UseKeyboardProps, use_keyboard},
            use_press::PressEvent,
        },
    },
    utils::{
        id::use_id,
        keyboard_shortcut::{KeyboardShortcuts, Shortcut},
        use_description::use_description,
    },
};
use crate::utils::intl_strings::{TagStrings, use_localized_strings};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - The remove button is configured, not rendered: `remove_button` is the `UseButtonInput` for
//   `use_button`.
//
// =============================================================================

/// Input of [`use_tag`].
#[derive(Debug, Clone)]
pub struct UseTagInput {
    /// The tag group (from `use_tag_group`).
    pub group: TagGroupData,
    /// The tag's key in the group's collection.
    pub key: Key,
}

/// Return value of [`use_tag`].
pub struct UseTagReturn {
    pub row_props: PropsWithStyles<UseTagRowProps>,
    pub grid_cell_props: UseGridListItemCellProps,
    /// The remove button's configuration, for `use_button` (only if the group has `on_remove`).
    pub remove_button: Option<UseButtonInput>,
    pub is_selected: Signal<bool>,
    pub is_focused: Signal<bool>,
    pub is_focus_visible: Signal<bool>,
    pub is_disabled: Signal<bool>,
    pub is_pressed: Signal<bool>,
    pub allows_selection: Signal<bool>,
    pub allows_removing: bool,
}

/// Props for the tag (row) element.
#[derive(Debug)]
pub struct UseTagRowProps {
    pub row: UseGridListItemRowProps,
    /// Delete/Backspace remove the tag (or all selected tags, if it is selected).
    pub keyboard: UseKeyboardProps,
    pub aria_describedby: Signal<Option<String>>,
}

pub type UseTagRowAttrs = (
    UseGridListItemRowAttrs,
    UseKeyboardAttrs,
    Attr<attr::AriaDescribedby, Signal<Option<String>>>,
);

impl IntoAttrs for UseTagRowProps {
    type Attrs = UseTagRowAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            self.row.into_attrs(),
            self.keyboard.into_attrs(),
            Attr(attr::AriaDescribedby, self.aria_describedby),
        )
    }
}

/// A tag of a tag group: a grid list row that is the group's tab stop while nothing is focused,
/// and can be removed with Delete/Backspace or its remove button.
pub fn use_tag(input: UseTagInput) -> UseTagReturn {
    let UseTagInput { group, key } = input;
    let TagGroupData { list, on_remove } = group;
    let selection = list.state.selection;
    let row_id = grid_list_row_id(&list.id, &key);
    let button_id = use_id("tag-remove");

    let UseGridListItemReturn {
        row_props,
        grid_cell_props,
        is_selected,
        is_focused,
        is_pressed,
        allows_selection,
        ..
    } = use_grid_list_item(UseGridListItemInput {
        list,
        key: key.clone(),
        focus_mode: FocusMode::Row,
        allows_arrow_navigation: false,
        on_context_menu: None,
    });
    let (mut row, row_styles) = row_props.into_inner();

    let key = StoredValue::new(key);
    // A disabled tag can't be removed or focused by Tab, also with `DisabledBehavior::Selection`
    // (react-aria: `disabledKeys` or the item's `isDisabled`).
    let is_disabled = Signal::derive(move || key.with_value(|k| selection.is_item_disabled(k)));
    // The keys to remove: all selected tags if this one is selected, else just this one.
    let keys_to_remove = move || {
        let key = key.get_value();
        if untrack(|| selection.is_selected(&key)) {
            untrack(|| selection.selected_keys())
        } else {
            HashSet::from([key])
        }
    };
    let remove = move || {
        if let Some(on_remove) = on_remove {
            on_remove.run(keys_to_remove());
        }
    };
    let keyboard = if on_remove.is_some() {
        use_keyboard(UseKeyboardInput {
            is_disabled,
            shortcuts: Some(
                KeyboardShortcuts::new()
                    .on(Shortcut::key("Delete"), move |_| remove())
                    .on(Shortcut::key("Backspace"), move |_| remove()),
            ),
            allow_repeats: true,
            ..UseKeyboardInput::default()
        })
        .props
    } else {
        use_keyboard(UseKeyboardInput::default()).props
    };

    // The first tag is the tab stop until a tag gets focus.
    row.item.tabindex = Signal::derive(move || {
        let focused_key = selection.focused_key();
        let is_this = key.with_value(|k| focused_key.as_ref() == Some(k));
        Some(
            if !is_disabled.get() && (is_this || focused_key.is_none()) {
                0
            } else {
                -1
            },
        )
    });

    // Keyboard and screen reader users learn how to remove tags.
    let focus_visible = use_focus_visible(UseFocusVisibleInput::default());
    let modality = focus_visible.modality;
    let has_remove = on_remove.is_some();
    let strings = use_localized_strings::<TagStrings>();
    let aria_describedby = use_description(Signal::derive(move || {
        (has_remove && matches!(modality.get(), Modality::Keyboard | Modality::Virtual))
            .then(|| strings.read().remove_description())
    }));

    let remove_button = on_remove.map(|on_remove| UseButtonInput {
        id: Some(button_id.clone()),
        aria_label: Signal::derive(move || Some(strings.read().remove_button_label())).into(),
        aria_labelledby: Signal::stored(Some(format!("{button_id} {row_id}"))),
        is_disabled,
        on_press: Some(Callback::new(move |_: PressEvent| {
            on_remove.run(HashSet::from([key.get_value()]));
        })),
        ..UseButtonInput::default()
    });

    UseTagReturn {
        row_props: PropsWithStyles::new(
            UseTagRowProps {
                row,
                keyboard,
                aria_describedby,
            },
            row_styles,
        ),
        grid_cell_props,
        remove_button,
        is_selected,
        is_focused,
        is_focus_visible: Signal::derive(move || {
            is_focused.get() && focus_visible.focus_should_be_visible.get()
        }),
        is_disabled,
        is_pressed,
        allows_selection,
        allows_removing: on_remove.is_some(),
    }
}
