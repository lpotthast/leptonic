// Upstream: react-aria/src/grid/useGridRowGroup.ts @ 99e6102368

use crate::{
    IntoAttrs,
    utils::{
        aria::AriaRole,
        part::{Part, Remount},
        signal_attr::{FixedAttr, fixed_attr},
    },
};

// No deviations from react-aria beyond the project-wide API conventions.

/// The return value of [`use_grid_row_group`].
#[derive(Debug)]
pub struct UseGridRowGroupReturn {
    /// The row group element.
    pub row_group: Part<UseGridRowGroupProps>,
}

/// Props for a group of rows (like `<tbody>`).
#[derive(Debug)]
pub struct UseGridRowGroupProps {
    pub role: AriaRole,
}

impl Remount for UseGridRowGroupProps {
    fn remount(&self) -> Self {
        Self { role: self.role }
    }
}

pub type UseGridRowGroupAttrs = (FixedAttr,);

impl IntoAttrs for UseGridRowGroupProps {
    type Attrs = UseGridRowGroupAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (fixed_attr("role", self.role),)
    }
}

/// A group of rows in a grid.
pub fn use_grid_row_group() -> UseGridRowGroupReturn {
    crate::metrics::subject!();
    UseGridRowGroupReturn {
        row_group: Part::stored(UseGridRowGroupProps {
            role: AriaRole::Rowgroup,
        }),
    }
}
