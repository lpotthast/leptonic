//! Contexts scoped to a component's view.

#[cfg(any(feature = "atoms", test))]
use std::marker::PhantomData;

use leptos::{prelude::*, tachys::reactive_graph::OwnedView};

/// Builds `view` in a child owner in which `provide` provides contexts, so that they reach only
/// the view's descendants (as Leptos' `<Provider>`, which wraps its children instead).
///
/// A component body has no owner of its own: `provide_context` there also reaches the
/// component's siblings, so of two sibling fields, the second one's context would win for both.
/// The returned view forwards attributes set on the component to its root element.
pub(crate) fn scoped_view<V>(provide: impl FnOnce(), view: impl FnOnce() -> V) -> OwnedView<V> {
    let owner = Owner::current()
        .expect("a component renders inside a reactive owner")
        .child();
    let view = owner.with(|| {
        provide();
        view()
    });
    OwnedView::new_with_owner(view, owner)
}

/// Hides the context `T` from the current owner's descendants: there, [`use_clearable_context`]
/// finds no `T`, unless a descendant provides one again (react-aria-components'
/// `clearContexts`, which provides `null` for a context). E.g. a combo box's popover hides the
/// combo box's label and input contexts, so that a `Label` or `Input` inside the popover doesn't
/// render as the combo box's.
///
/// Leptos can't remove a context, and providing a `T` again needs a value. So the clearing
/// remembers which provided value it hides: values in an owner's context map are boxed and stay
/// in place while their owner lives, which outlives every descendant of the clearing owner.
#[cfg(any(feature = "atoms", test))]
pub(crate) fn clear_context<T: 'static>() {
    // Values of zero-sized types share one address: a clearing would hide them all.
    const {
        assert!(
            size_of::<T>() > 0,
            "clear_context needs a context type with a size"
        );
    }
    provide_context(Cleared::<T> {
        hidden: provided_value_address::<T>(),
        marker: PhantomData,
    });
}

/// The context `T`, unless [`clear_context`] hid it from this part of the tree.
#[cfg(any(feature = "atoms", test))]
pub(crate) fn use_clearable_context<T: Clone + 'static>() -> Option<T> {
    let hidden = use_context::<Cleared<T>>().and_then(|cleared| cleared.hidden);
    if hidden.is_some() && provided_value_address::<T>() == hidden {
        return None;
    }
    use_context::<T>()
}

/// Contexts an overlay clears for its content (see [`clear_context`]), e.g.
/// `ClearContexts(&[clear_context::<LabelContext>, clear_context::<InputContext>])`.
#[cfg(feature = "atoms")]
#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct ClearContexts(pub &'static [fn()]);

#[cfg(feature = "atoms")]
impl ClearContexts {
    /// Clears the contexts on the current owner.
    pub(crate) fn clear(self) {
        for clear in self.0 {
            clear();
        }
    }
}

/// A [`clear_context`] marker: the address of the `T` it hides (`None`: there was none).
#[cfg(any(feature = "atoms", test))]
struct Cleared<T> {
    hidden: Option<usize>,
    marker: PhantomData<fn() -> T>,
}

#[cfg(any(feature = "atoms", test))]
impl<T> Clone for Cleared<T> {
    fn clone(&self) -> Self {
        *self
    }
}

#[cfg(any(feature = "atoms", test))]
impl<T> Copy for Cleared<T> {}

/// Where the `T` [`use_context`] finds is stored.
#[cfg(any(feature = "atoms", test))]
fn provided_value_address<T: 'static>() -> Option<usize> {
    with_context::<T, _>(|value| std::ptr::from_ref(value).addr())
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;
    use crate::testing::with_owner;

    #[derive(Debug, Clone, PartialEq)]
    struct Label(&'static str);

    #[test]
    fn a_cleared_context_is_hidden_until_provided_again() {
        with_owner(|| {
            provide_context(Label("outer"));
            let cleared = Owner::current().expect("owner").child();
            cleared.with(|| {
                clear_context::<Label>();
                assert_that!(use_clearable_context::<Label>()).is_none();
                // Contexts of other types stay.
                assert_that!(use_clearable_context::<u8>()).is_none();
                let below = Owner::current().expect("owner").child();
                below.with(|| {
                    assert_that!(use_clearable_context::<Label>()).is_none();
                    provide_context(Label("inner"));
                    assert_that!(use_clearable_context::<Label>())
                        .is_equal_to(Some(Label("inner")));
                });
            });
            assert_that!(use_clearable_context::<Label>()).is_equal_to(Some(Label("outer")));
        });
    }

    #[test]
    fn clearing_without_a_context_hides_nothing_provided_later() {
        with_owner(|| {
            clear_context::<Label>();
            let below = Owner::current().expect("owner").child();
            below.with(|| {
                provide_context(Label("inner"));
                assert_that!(use_clearable_context::<Label>()).is_equal_to(Some(Label("inner")));
            });
        });
    }
}
