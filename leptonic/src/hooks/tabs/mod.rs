//! Tab hooks: a tab list of tabs, one of which is selected, and the selected tab's panel.
//! Create the state with [`use_tab_list_state`], render the tab list with [`use_tab_list`],
//! its tabs with [`use_tab`] and the panel with [`use_tab_panel`].

/// Tab list navigation.
pub(crate) mod tabs_keyboard_delegate;

/// A tab.
pub(crate) mod use_tab;

/// The tab list element.
pub(crate) mod use_tab_list;

/// Tab list state: tabs, the selected tab, focus.
pub(crate) mod use_tab_list_state;

/// The content of a tab.
pub(crate) mod use_tab_panel;

pub use tabs_keyboard_delegate::*;
pub use use_tab::*;
pub use use_tab_list::*;
pub use use_tab_list_state::*;
pub use use_tab_panel::*;
