// No upstream: the values of leptonic's state `data-*` attributes (react-aria-components writes
// `value || undefined` at each site).
//! Values of the state `data-*` attributes atoms render (`data-selected`, `data-selection-mode`).

use leptos::prelude::*;

#[cfg(feature = "atoms")]
use super::signal_attr::{AttrText, SignalAttr, ToAttrText, signal_attr};
#[cfg(feature = "atoms")]
use crate::hooks::collections::SelectionMode;

/// The value of a boolean `data-*` attribute: `"true"` while `signal` is `true`, absent otherwise
/// (as react-aria-components renders its state attributes). Style with `[data-selected]`.
///
/// ```ignore
/// view! { <div data-selected=flag(is_selected)></div> }
/// ```
pub fn flag(signal: Signal<bool>) -> impl Fn() -> Option<&'static str> + Clone + Send + Sync {
    move || signal.get().then_some("true")
}

/// The `data-selection-mode` attribute: `single` or `multiple`, absent without selection (as
/// react-aria-components renders it on collection items). Written once when `mode` can't change
/// (no render effect), else updated by one: `{..selection_mode(selection.selection_mode_signal())}`.
#[cfg(feature = "atoms")]
pub fn selection_mode(mode: Signal<SelectionMode>) -> SignalAttr<SelectionMode> {
    signal_attr("data-selection-mode", mode)
}

/// As `data-selection-mode` shows it ([`selection_mode`]).
#[cfg(feature = "atoms")]
impl ToAttrText for SelectionMode {
    fn to_attr_text(&self) -> AttrText {
        match self {
            Self::None => AttrText::Absent,
            Self::Single => AttrText::Text(Oco::Borrowed("single")),
            Self::Multiple => AttrText::Text(Oco::Borrowed("multiple")),
        }
    }
}

/// Boolean `data-*` attributes of one element, all set in one render effect (or none when every
/// flag is constant): `{..data_flags([("data-pressed", is_pressed), ("data-hovered", is_hovered)])}`.
/// Each flag renders as [`flag`] does (`"true"` while its signal is `true`, absent otherwise).
///
/// `data-x=flag(signal)` costs a render effect per attribute (~10 allocations, plus the boxing of
/// type-erased attributes); this costs one for all of an element's flags, and a change writes only
/// the flags that changed (`documentation/performance.md`, R4). Flags whose signals have no
/// reactive source are written once, like [`SignalAttr`](super::signal_attr::SignalAttr)s.
#[cfg(feature = "atoms")]
pub fn data_flags<const N: usize>(flags: [(&'static str, Signal<bool>); N]) -> DataFlags<N> {
    DataFlags(flags)
}

/// The attribute of [`data_flags`].
#[derive(Debug, Clone, Copy)]
#[cfg(feature = "atoms")]
pub struct DataFlags<const N: usize>([(&'static str, Signal<bool>); N]);

/// The rendered state of [`DataFlags`]: the values written, and the effect updating them (none if
/// every flag is constant).
#[cfg(feature = "atoms")]
pub struct DataFlagsState<const N: usize> {
    el: leptos::tachys::renderer::types::Element,
    written: [bool; N],
    effect: Option<leptos::reactive::effect::RenderEffect<[bool; N]>>,
}

#[cfg(feature = "atoms")]
impl<const N: usize> DataFlags<N> {
    fn read(&self) -> [bool; N] {
        self.0.map(|(_, flag)| flag.get())
    }

    fn read_untracked(&self) -> [bool; N] {
        self.0.map(|(_, flag)| flag.get_untracked())
    }

    /// Renders the flags onto `el`, whose flags are `written` (all absent on a new element).
    fn render(
        self,
        el: &leptos::tachys::renderer::types::Element,
        written: [bool; N],
        write: bool,
    ) -> DataFlagsState<N> {
        let (values, is_reactive) = super::probe::probe_sources(|| self.read());
        if write {
            write_flags(el, &self.names(), written, values);
        }
        let effect = is_reactive.then(|| {
            let el = el.clone();
            let names = self.names();
            leptos::reactive::effect::RenderEffect::new_with_value(
                move |written: Option<[bool; N]>| {
                    let values = self.read();
                    if let Some(written) = written {
                        write_flags(&el, &names, written, values);
                    }
                    values
                },
                Some(values),
            )
        });
        DataFlagsState {
            el: el.clone(),
            written: values,
            effect,
        }
    }

    fn names(&self) -> [&'static str; N] {
        self.0.map(|(name, _)| name)
    }
}

/// Sets the flags that became `true` and removes those that became `false`.
#[cfg(feature = "atoms")]
fn write_flags<const N: usize>(
    el: &leptos::tachys::renderer::types::Element,
    names: &[&'static str; N],
    written: [bool; N],
    values: [bool; N],
) {
    for ((name, was), is) in names.iter().zip(written).zip(values) {
        match (was, is) {
            (false, true) => {
                let _ = el.set_attribute(name, "true");
            }
            (true, false) => {
                let _ = el.remove_attribute(name);
            }
            _ => {}
        }
    }
}

#[cfg(feature = "atoms")]
impl<const N: usize> leptos::attr::Attribute for DataFlags<N> {
    const MIN_LENGTH: usize = 0;

    type State = DataFlagsState<N>;
    type AsyncOutput = Self;
    type Cloneable = Self;
    type CloneableOwned = Self;

    fn html_len(&self) -> usize {
        self.0
            .iter()
            .filter(|(_, flag)| flag.get_untracked())
            .map(|(name, _)| name.len() + r#" ="true""#.len())
            .sum()
    }

    fn to_html(self, buf: &mut String, _class: &mut String, _style: &mut String, _: &mut String) {
        for ((name, _), value) in self.0.iter().zip(self.read_untracked()) {
            if value {
                buf.push(' ');
                buf.push_str(name);
                buf.push_str(r#"="true""#);
            }
        }
    }

    fn hydrate<const FROM_SERVER: bool>(
        self,
        el: &leptos::tachys::renderer::types::Element,
    ) -> Self::State {
        // From the server, the attributes are rendered already (as tachys trusts the server's
        // attributes when hydrating).
        if FROM_SERVER {
            let written = self.read_untracked();
            self.render(el, written, false)
        } else {
            self.render(el, [false; N], true)
        }
    }

    fn build(self, el: &leptos::tachys::renderer::types::Element) -> Self::State {
        self.render(el, [false; N], true)
    }

    fn rebuild(self, state: &mut Self::State) {
        let written = state
            .effect
            .take()
            .and_then(|effect| effect.take_value())
            .unwrap_or(state.written);
        *state = self.render(&state.el, written, true);
    }

    fn into_cloneable(self) -> Self::Cloneable {
        self
    }

    fn into_cloneable_owned(self) -> Self::CloneableOwned {
        self
    }

    fn dry_resolve(&mut self) {}

    fn resolve(self) -> impl Future<Output = Self::AsyncOutput> + Send {
        std::future::ready(self)
    }

    fn keys(&self) -> Vec<leptos::attr::NamedAttributeKey> {
        self.0
            .iter()
            .map(|(name, _)| leptos::attr::NamedAttributeKey::Attribute((*name).into()))
            .collect()
    }
}

#[cfg(feature = "atoms")]
impl<const N: usize> leptos::attr::NextAttribute for DataFlags<N> {
    type Output<NewAttr: leptos::attr::Attribute> = (Self, NewAttr);

    fn add_any_attr<NewAttr: leptos::attr::Attribute>(
        self,
        new_attr: NewAttr,
    ) -> Self::Output<NewAttr> {
        (self, new_attr)
    }
}

#[cfg(all(test, feature = "atoms"))]
mod tests {
    use assertr::prelude::*;
    use leptos::attr::Attribute;

    use super::*;
    use crate::testing::with_owner;

    #[test]
    fn flags_render_true_ones_and_list_every_key() {
        with_owner(|| {
            let flags = data_flags([
                ("data-pressed", Signal::stored(true)),
                ("data-hovered", Signal::stored(false)),
            ]);
            assert_that!(flags.keys().len()).is_equal_to(2);
            let mut buf = String::new();
            flags.to_html(
                &mut buf,
                &mut String::new(),
                &mut String::new(),
                &mut String::new(),
            );
            assert_that!(buf).is_equal_to(r#" data-pressed="true""#.to_owned());
        });
    }
}
