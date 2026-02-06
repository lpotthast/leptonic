//! Traits for merging hook Props types.
//!
//! When composing multiple hooks (e.g., `use_press` + `use_hover` + `use_focus_ring`),
//! their Props need to be merged. The [`MergeWith`] trait provides a type-safe way
//! to combine Props from different hooks into a single merged type.
//!
//! # Merge Semantics
//!
//! Different field types are merged differently:
//!
//! | Field Type | Merge Behavior |
//! |------------|----------------|
//! | `EventHandler<E>` | Chained (both run in sequence) |
//! | `ElementCaptureAttr` | Both kept (both capture element refs) |
//! | Other attributes | "Last wins" (second overrides first) |
//! | Distinct fields | Included in output as-is |
//!
//! # Example
//!
//! ```ignore
//! use leptonic::utils::MergeWith;
//! use leptonic::hooks::{use_press, use_hover, UsePressInput, UseHoverInput};
//!
//! let press = use_press(press_input);
//! let hover = use_hover(hover_input);
//!
//! // Merge press and hover props
//! let combined = press.props.merge_with(hover.props);
//!
//! view! {
//!     <button {..combined.into_attrs()}>
//!         "Hover and click me"
//!     </button>
//! }
//! ```
//!
//! # Chained Merging
//!
//! Merged types can also implement `MergeWith` to enable chaining:
//!
//! ```ignore
//! let combined = press.props
//!     .merge_with(hover.props)
//!     .merge_with(focus_ring.props);
//! ```

/// Trait for merging two Props types into a combined type.
///
/// This trait enables type-safe composition of hook Props. Each implementation
/// defines exactly which output type results from merging two input types.
///
/// # Type Parameters
///
/// - `Other`: The Props type to merge with `Self`.
///
/// # Associated Types
///
/// - `Output`: The resulting merged Props type.
///
/// # Merge Semantics
///
/// Implementations should follow these merge semantics:
///
/// - **`EventHandler<E>` fields**: Chain handlers using `EventHandler::chain()`.
///   Both handlers will run in sequence when the event fires.
///
/// - **`ElementCaptureAttr` fields**: Keep both capture attributes.
///   Both will capture the element reference.
///
/// - **Other attribute fields**: "Last wins" - the `Other` value takes precedence.
///
/// - **Distinct fields**: Include both fields in the output type unchanged.
///
/// # Example Implementation
///
/// ```ignore
/// impl MergeWith<UseHoverProps> for UsePressProps {
///     type Output = MergedPressHoverProps;
///
///     fn merge_with(self, other: UseHoverProps) -> Self::Output {
///         MergedPressHoverProps {
///             // From press (distinct fields)
///             on_keydown: self.on_keydown,
///             on_click: self.on_click,
///             on_pointerdown: self.on_pointerdown,
///             // From hover (distinct fields)
///             on_pointerenter: other.on_pointerenter,
///             on_pointerleave: other.on_pointerleave,
///         }
///     }
/// }
/// ```
pub trait MergeWith<Other>: Sized {
    /// The resulting merged Props type.
    type Output;

    /// Merge self with other, producing the combined Props.
    ///
    /// This consumes both Props types and produces a new merged type
    /// that contains fields from both, with appropriate merge semantics
    /// applied to overlapping fields.
    fn merge_with(self, other: Other) -> Self::Output;
}

/// Extension trait for convenient chaining syntax when merging multiple Props.
///
/// This trait provides a `.and()` method as an alias for `.merge_with()`,
/// which can make chained merges more readable:
///
/// ```ignore
/// let combined = press.props
///     .and(hover.props)
///     .and(focus_ring.props);
/// ```
pub trait MergeWithExt<Other>: MergeWith<Other> {
    /// Alias for [`MergeWith::merge_with`] for more readable chaining.
    fn and(self, other: Other) -> Self::Output {
        self.merge_with(other)
    }
}

impl<T, Other> MergeWithExt<Other> for T where T: MergeWith<Other> {}

#[cfg(test)]
mod tests {
    use super::*;

    struct PropsA {
        a: i32,
    }

    struct PropsB {
        b: String,
    }

    struct MergedAB {
        a: i32,
        b: String,
    }

    impl MergeWith<PropsB> for PropsA {
        type Output = MergedAB;

        fn merge_with(self, other: PropsB) -> Self::Output {
            MergedAB {
                a: self.a,
                b: other.b,
            }
        }
    }

    #[test]
    fn merge_with_combines_distinct_fields() {
        let a = PropsA { a: 42 };
        let b = PropsB {
            b: "hello".to_string(),
        };

        let merged = a.merge_with(b);

        assert_eq!(merged.a, 42);
        assert_eq!(merged.b, "hello");
    }

    #[test]
    fn and_is_alias_for_merge_with() {
        let a = PropsA { a: 42 };
        let b = PropsB {
            b: "hello".to_string(),
        };

        let merged = a.and(b);

        assert_eq!(merged.a, 42);
        assert_eq!(merged.b, "hello");
    }
}
