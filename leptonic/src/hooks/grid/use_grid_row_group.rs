// Upstream: react-aria/src/grid/useGridRowGroup.ts @ 99e6102368
use leptos::attr::{self, Attr};

use crate::{hooks::IntoAttrs, utils::aria::AriaRole};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// No intentional deviations from the react-aria implementation.
//
// =============================================================================

/// Return value of [`use_grid_row_group`].
#[derive(Debug)]
pub struct UseGridRowGroupReturn {
    pub row_group_props: UseGridRowGroupProps,
}

/// Props for a group of rows (like `<tbody>`).
#[derive(Debug)]
pub struct UseGridRowGroupProps {
    pub role: AriaRole,
}

pub type UseGridRowGroupAttrs = (Attr<attr::Role, AriaRole>,);

impl IntoAttrs for UseGridRowGroupProps {
    type Attrs = UseGridRowGroupAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (Attr(attr::Role, self.role),)
    }
}

/// A group of rows in a grid.
pub fn use_grid_row_group() -> UseGridRowGroupReturn {
    UseGridRowGroupReturn {
        row_group_props: UseGridRowGroupProps {
            role: AriaRole::Rowgroup,
        },
    }
}
