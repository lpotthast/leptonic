// No upstream: React spreads props objects; tachys' type-erased attributes keep key lists.
//! [`Spread`]: a hook's attribute tuple whose key list is allocated once.
//!
//! With `erase_components`, a spread `{..attrs}` becomes one `AnyAttribute`, which asks the tuple
//! for its keys (to remove the attributes if the element is rebuilt with other ones). tachys'
//! tuples collect them by allocating a `Vec` per attribute and growing the first one
//! (`documentation/performance.md`, R11): ~1 allocation per attribute of every spread at mount.
//! `Spread` collects them into one `Vec` of the right size.

use leptos::{
    attr::{Attribute, NamedAttributeKey, NextAttribute},
    tachys::renderer::types::Element,
};

/// The keys of an attribute (tachys' `Attribute::keys`), written into a shared list.
pub trait AttrKeys {
    /// How many keys [`push_keys`](Self::push_keys) pushes.
    fn key_count(&self) -> usize;

    /// Pushes the keys.
    fn push_keys(&self, keys: &mut Vec<NamedAttributeKey>);
}

/// The attributes `attrs` (a tuple of leptonic attributes), with one allocation for their keys.
#[derive(Debug, Clone)]
pub struct Spread<A>(pub A);

/// For attributes that are their own cloneable and resolved forms (leptonic's are).
impl<A> Attribute for Spread<A>
where
    A: Attribute<AsyncOutput = A, Cloneable = A, CloneableOwned = A> + AttrKeys + Clone + 'static,
{
    const MIN_LENGTH: usize = A::MIN_LENGTH;

    type State = A::State;
    type AsyncOutput = Self;
    type Cloneable = Self;
    type CloneableOwned = Self;

    fn html_len(&self) -> usize {
        self.0.html_len()
    }

    fn to_html(
        self,
        buf: &mut String,
        class: &mut String,
        style: &mut String,
        inner_html: &mut String,
    ) {
        self.0.to_html(buf, class, style, inner_html);
    }

    fn hydrate<const FROM_SERVER: bool>(self, el: &Element) -> Self::State {
        self.0.hydrate::<FROM_SERVER>(el)
    }

    fn build(self, el: &Element) -> Self::State {
        self.0.build(el)
    }

    fn rebuild(self, state: &mut Self::State) {
        self.0.rebuild(state);
    }

    fn into_cloneable(self) -> Self::Cloneable {
        self
    }

    fn into_cloneable_owned(self) -> Self::CloneableOwned {
        self
    }

    fn dry_resolve(&mut self) {
        self.0.dry_resolve();
    }

    async fn resolve(self) -> Self::AsyncOutput {
        Self(self.0.resolve().await)
    }

    fn keys(&self) -> Vec<NamedAttributeKey> {
        let mut keys = Vec::with_capacity(self.0.key_count());
        self.0.push_keys(&mut keys);
        keys
    }
}

impl<A> NextAttribute for Spread<A>
where
    A: Attribute<AsyncOutput = A, Cloneable = A, CloneableOwned = A> + AttrKeys + Clone + 'static,
{
    type Output<NewAttr: Attribute> = (Self, NewAttr);

    fn add_any_attr<NewAttr: Attribute>(self, new_attr: NewAttr) -> Self::Output<NewAttr> {
        (self, new_attr)
    }
}

/// Keys of the attributes that have none (listeners, element captures).
macro_rules! no_keys {
    ($($ty:ty),* $(,)?) => {$(
        impl AttrKeys for $ty {
            fn key_count(&self) -> usize {
                0
            }

            fn push_keys(&self, _keys: &mut Vec<NamedAttributeKey>) {}
        }
    )*};
}
no_keys!(
    leptos_element_capture::ElementCaptureAttr,
    crate::hooks::focus::FocusableContextAttr
);

impl<D> AttrKeys for crate::OnEvent<D>
where
    D: leptos::ev::EventDescriptor,
    D::EventType: 'static,
{
    fn key_count(&self) -> usize {
        0
    }

    fn push_keys(&self, _keys: &mut Vec<NamedAttributeKey>) {}
}

/// Tuples of attributes: their elements' keys, in order.
macro_rules! tuple_keys {
    ($($name:ident),+) => {
        impl<$($name: AttrKeys),+> AttrKeys for ($($name,)+) {
            fn key_count(&self) -> usize {
                #[allow(non_snake_case)]
                let ($($name,)+) = self;
                0 $(+ $name.key_count())+
            }

            fn push_keys(&self, keys: &mut Vec<NamedAttributeKey>) {
                #[allow(non_snake_case)]
                let ($($name,)+) = self;
                $($name.push_keys(keys);)+
            }
        }
    };
}
tuple_keys!(A);
tuple_keys!(A, B);
tuple_keys!(A, B, C);
tuple_keys!(A, B, C, D);
tuple_keys!(A, B, C, D, E);
tuple_keys!(A, B, C, D, E, F);
tuple_keys!(A, B, C, D, E, F, G);
tuple_keys!(A, B, C, D, E, F, G, H);
tuple_keys!(A, B, C, D, E, F, G, H, I);
tuple_keys!(A, B, C, D, E, F, G, H, I, J);
tuple_keys!(A, B, C, D, E, F, G, H, I, J, K);
tuple_keys!(A, B, C, D, E, F, G, H, I, J, K, L);
tuple_keys!(A, B, C, D, E, F, G, H, I, J, K, L, M);

#[cfg(test)]
mod tests {
    use assertr::prelude::*;
    use leptos::prelude::Signal;

    use super::*;
    use crate::{
        testing::with_owner,
        utils::signal_attr::{fixed_attr, signal_attr},
    };

    /// The keys of nested tuples, in order, absent attributes left out.
    #[test]
    fn keys_of_nested_tuples_are_collected_in_order() {
        with_owner(|| {
            let spread = Spread((
                fixed_attr("id", "a"),
                (
                    fixed_attr("form", None::<String>),
                    signal_attr("type", Signal::stored("button")),
                ),
            ));
            let keys: Vec<String> = spread
                .keys()
                .into_iter()
                .map(|key| match key {
                    NamedAttributeKey::Attribute(key) => key.into_owned(),
                    NamedAttributeKey::Property(_) | NamedAttributeKey::InnerHtml => String::new(),
                })
                .collect();
            assert_that!(keys).is_equal_to(vec!["id".to_owned(), "type".to_owned()]);
        });
    }
}
