use typed_builder::TypedBuilder;

use crate::{
    state::Key,
    utils::{attributes::Attributes, event_handlers::EventHandlers},
};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/%40react-aria/selection/src/useSelectableList.ts

#[derive(Debug, Clone, TypedBuilder)]
pub struct UseSelectableListInput {
    /// The item keys that are disabled. These items cannot be selected, focused, or otherwise interacted with.
    pub(crate) disabled_keys: Vec<Key>,
}

pub struct UseSelectableListReturn {
    pub attrs: Attributes,
    pub handlers: EventHandlers,
}

pub fn use_selectable_list(input: UseSelectableListInput) -> UseSelectableListReturn {
    UseSelectableListReturn {
        attrs: Attributes::new(),
        handlers: EventHandlers::builder().build(),
    }
}
