// Upstream: react-aria/src/tag/useTag.ts @ 99e6102368
// Upstream: react-aria/test/tag/useTagGroup.test.js @ 99e6102368
// Upstream: react-aria-components/test/TagGroup.test.js @ 99e6102368
// Upstream: @adobe/react-spectrum/test/tag/TagGroup.test.js @ 99e6102368
use std::collections::HashSet;

use leptos::prelude::*;

use super::TagGroupData;
use crate::{
    EventHandler, Id, IntoAttrs, PropsWithStyles,
    hooks::{
        button::use_button::UseButtonInput,
        collections::Key,
        focus::use_focus_visible::{Modality, use_interaction_modality},
        gridlist::{
            FocusMode, UseGridListItemCellProps, UseGridListItemInput, UseGridListItemReturn,
            UseGridListItemRowAttrs, UseGridListItemRowProps, grid_list_row_id, use_grid_list_item,
        },
        interactions::{
            use_keyboard::{UseKeyboardAttrs, UseKeyboardInput, UseKeyboardProps, use_keyboard},
            use_press::PressEvent,
        },
    },
    utils::{
        id::use_id,
        intl_strings::{TagStrings, use_localized_strings},
        key::KeyboardKey,
        keyboard_shortcut::{KeyboardShortcuts, Shortcut},
        part::{Part, Remount},
        platform::device::has_touch_events,
        signal_attr::{SignalAttr, signal_attr},
        use_description::use_description,
    },
};

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

/// The return value of [`use_tag`].
pub struct UseTagReturn {
    /// The tag's row element: `let (attrs, styles) = row.attrs();`.
    pub row: Part<PropsWithStyles<UseTagRowProps>>,
    /// The tag's cell element (its content).
    pub cell: Part<UseGridListItemCellProps>,
    /// The remove button's configuration, for `use_button` (only if the group has `on_remove`).
    pub remove_button: Option<UseButtonInput>,
    pub is_selected: Signal<bool>,
    pub is_focused: Signal<bool>,
    /// Whether the tag element itself has keyboard focus (not its remove button).
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
    pub aria_describedby: Signal<Option<Id>>,
}

impl Remount for UseTagRowProps {
    fn remount(&self) -> Self {
        Self {
            row: self.row.remount(),
            keyboard: self.keyboard.remount(),
            aria_describedby: self.aria_describedby,
        }
    }
}

pub type UseTagRowAttrs = (
    UseGridListItemRowAttrs,
    UseKeyboardAttrs,
    SignalAttr<Option<Id>>,
);

impl IntoAttrs for UseTagRowProps {
    type Attrs = UseTagRowAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            self.row.into_attrs(),
            self.keyboard.into_attrs(),
            signal_attr("aria-describedby", self.aria_describedby),
        )
    }
}

/// A tag of a tag group: a grid list row that is a tab stop until a tag gets focus (then only the
/// focused tag is), and can be removed with Delete/Backspace or its remove button.
pub fn use_tag(input: UseTagInput) -> UseTagReturn {
    crate::metrics::subject!();
    let UseTagInput { group, key } = input;
    let TagGroupData { list, on_remove } = group;
    let selection = list.state.selection;
    let row_id = grid_list_row_id(&list.id, &key);
    let button_id = use_id("tag-remove");

    let UseGridListItemReturn {
        row: row_props,
        cell: grid_cell_props,
        is_selected,
        is_focused,
        is_focus_visible,
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
    let (mut row, row_styles) = row_props.into_props().into_inner();

    let key = StoredValue::new(key);
    // A disabled tag can't be removed or focused by Tab, also with `DisabledBehavior::Selection`
    // (react-aria: `disabledKeys` or the item's `isDisabled`).
    let is_disabled: Signal<bool> =
        Memo::new(move |_| key.with_value(|k| selection.is_item_disabled(k))).into();
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
                    .on(Shortcut::new(KeyboardKey::Delete), move |_| remove())
                    .on(Shortcut::new(KeyboardKey::Backspace), move |_| remove()),
            ),
            allow_repeats: true,
            on_key_down: None,
            on_key_up: None,
            allow_composing: false,
        })
        .target
        .into_props()
    } else {
        // Without `on_remove`, the tag has no keyboard handlers of its own (react-aria: none).
        UseKeyboardProps {
            on_keydown: EventHandler::empty(),
            on_keyup: EventHandler::empty(),
        }
    };

    // Every enabled tag is a tab stop until a tag gets focus, then only the focused one (react-aria:
    // Tab enters the group at its first tag, Shift+Tab at its last). The first focus changes every
    // tag's `tabindex`; later focus moves change two (`is_focused_key` tracks only this key).
    row.item.tabindex = Signal::derive(move || {
        let is_this = key.with_value(|k| selection.is_focused_key(k));
        Some(
            if !is_disabled.get() && (is_this || !selection.has_focused_key()) {
                0
            } else {
                -1
            },
        )
    });

    // Keyboard and screen reader users learn how to remove tags. On touch devices, a virtual
    // modality comes from a screen reader's touch gestures, which can't press Delete: they count
    // as pointer interaction (react-aria).
    let modality = use_interaction_modality();
    let has_remove = on_remove.is_some();
    let strings = use_localized_strings::<TagStrings>();
    let aria_describedby = use_description(Signal::derive(move || {
        let modality = match modality.get() {
            Some(Modality::Virtual) if has_touch_events() => Some(Modality::Pointer),
            modality => modality,
        };
        (has_remove && matches!(modality, Some(Modality::Keyboard | Modality::Virtual)))
            .then(|| strings.read().remove_description())
    }));

    let remove_button = on_remove.map(|on_remove| UseButtonInput {
        id: Some(button_id.clone()),
        aria_label: Signal::derive(move || Some(strings.read().remove_button_label())).into(),
        aria_labelledby: Signal::stored(crate::utils::id::join_ids([
            Some(button_id.clone()),
            Some(row_id.clone()),
        ])),
        is_disabled,
        on_press: Some(Callback::new(move |_: PressEvent| {
            on_remove.run(HashSet::from([key.get_value()]));
        })),
        element_type: crate::hooks::button::ButtonElementType::Button,
        button_type: crate::hooks::button::ButtonType::Button,
        is_pending: Signal::stored(false),
        allow_focus_when_disabled: Signal::stored(false),
        exclude_from_tab_order: Signal::stored(false),
        auto_focus: false,
        prevent_focus_on_press: Signal::stored(false),
        href: Signal::stored(None),
        target: Signal::stored(crate::hooks::link::LinkTarget::Same),
        rel: Vec::new(),
        form: crate::hooks::button::ButtonFormAttributes::default(),
        aria_haspopup: Signal::stored(None),
        aria_expanded: Signal::stored(None),
        aria_controls: Signal::stored(None),
        aria_describedby: Signal::stored(None),
        aria_pressed: Signal::stored(None),
        aria_checked: Signal::stored(None),
        role: None,
        aria_current: Signal::stored(None),
        aria_disabled: Signal::stored(None),
        on_press_start: None,
        on_press_end: None,
        on_press_up: None,
        on_press_change: None,
        long_press: None,
        on_hover_start: None,
        on_hover_end: None,
        on_hover_change: None,
        on_focus: None,
        on_blur: None,
        on_focus_change: None,
        on_key_down: None,
        on_key_up: None,
        shortcuts: None,
        on_context_menu: None,
    });

    UseTagReturn {
        row: Part::stored(PropsWithStyles::new(
            UseTagRowProps {
                row,
                keyboard,
                aria_describedby,
            },
            row_styles,
        )),
        cell: grid_cell_props,
        remove_button,
        is_selected,
        is_focused,
        is_focus_visible,
        is_disabled,
        is_pressed,
        allows_selection,
        allows_removing: on_remove.is_some(),
    }
}
