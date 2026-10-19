// No upstream: React re-renders props instead of binding attributes to signals.
//! [`SignalAttr`]: an attribute bound to a signal, with one render effect only if the signal can
//! change, and one implementation for every value type.
//!
//! tachys renders every `Attr(attr::X, signal)` through a `RenderEffect` (~10 allocations and a
//! copy of the attribute name at mount), also when the signal is a constant (`Signal::stored`, a
//! derive of constants), and instantiates its attribute code per value type
//! (`documentation/performance.md`, R2 and R13). A `SignalAttr` evaluates its signal once under a
//! probe ([`probe_sources`]): without reactive sources it writes the attribute and keeps no
//! effect (exactly what an effect that tracked nothing would do forever); with sources it keeps
//! one effect that writes the attribute when its text changes. The value types only convert to
//! [`AttrText`]; rendering is shared.

use std::borrow::Cow;

use leptos::{
    attr::{Attribute, AttributeValue, NamedAttributeKey, NextAttribute},
    oco::Oco,
    prelude::*,
    reactive::effect::RenderEffect,
    tachys::renderer::types::Element,
};

use super::{probe::probe_sources, spread::AttrKeys};

/// What an attribute renders: absent, present without a value (a boolean attribute), or a text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AttrText {
    /// No attribute.
    Absent,
    /// The attribute without a value (`disabled`), as `true` renders.
    Present,
    /// The attribute with this value.
    Text(Oco<'static, str>),
    /// The attribute with this number as its value (formatted when written).
    Int(i64),
}

/// A value an attribute can show: converted to its [`AttrText`] each time the attribute is
/// rendered.
pub trait ToAttrText {
    /// The attribute's text for this value.
    fn to_attr_text(&self) -> AttrText;

    /// The attribute's text, taking the value (no copy of an owned text).
    fn into_attr_text(self) -> AttrText
    where
        Self: Sized,
    {
        self.to_attr_text()
    }
}

impl ToAttrText for bool {
    fn to_attr_text(&self) -> AttrText {
        if *self {
            AttrText::Present
        } else {
            AttrText::Absent
        }
    }
}

impl ToAttrText for i32 {
    fn to_attr_text(&self) -> AttrText {
        AttrText::Int(i64::from(*self))
    }
}

impl ToAttrText for f64 {
    /// Whole numbers as integers (no allocation), others in their shortest decimal form.
    #[allow(clippy::cast_possible_truncation)]
    fn to_attr_text(&self) -> AttrText {
        if self.fract() == 0.0 && self.abs() < 9.0e15 {
            AttrText::Int(*self as i64)
        } else {
            AttrText::Text(Oco::Owned(self.to_string()))
        }
    }
}

impl ToAttrText for usize {
    #[allow(clippy::cast_possible_wrap)]
    fn to_attr_text(&self) -> AttrText {
        AttrText::Int(*self as i64)
    }
}

impl ToAttrText for i64 {
    fn to_attr_text(&self) -> AttrText {
        AttrText::Int(*self)
    }
}

impl ToAttrText for u32 {
    fn to_attr_text(&self) -> AttrText {
        AttrText::Int(i64::from(*self))
    }
}

impl ToAttrText for &'static str {
    fn to_attr_text(&self) -> AttrText {
        AttrText::Text(Oco::Borrowed(self))
    }
}

impl ToAttrText for String {
    fn to_attr_text(&self) -> AttrText {
        AttrText::Text(Oco::Owned(self.clone()))
    }

    fn into_attr_text(self) -> AttrText {
        AttrText::Text(Oco::Owned(self))
    }
}

impl ToAttrText for Oco<'static, str> {
    fn to_attr_text(&self) -> AttrText {
        AttrText::Text(self.clone())
    }
}

impl<T: ToAttrText> ToAttrText for Option<T> {
    fn to_attr_text(&self) -> AttrText {
        self.as_ref()
            .map_or(AttrText::Absent, ToAttrText::to_attr_text)
    }

    fn into_attr_text(self) -> AttrText {
        self.map_or(AttrText::Absent, ToAttrText::into_attr_text)
    }
}

/// The attribute `key` with a value that doesn't change (`fixed_attr("form", form)`): written
/// once, and an absent value costs nothing (tachys' `Attr` allocates its key list either way).
pub fn fixed_attr(key: &'static str, value: impl ToAttrText) -> FixedAttr {
    FixedAttr {
        key,
        text: value.into_attr_text(),
    }
}

/// The attribute of [`fixed_attr`]: one type for every value type.
#[derive(Debug, Clone)]
pub struct FixedAttr {
    key: &'static str,
    text: AttrText,
}

/// The rendered state of a [`FixedAttr`].
pub struct FixedAttrState {
    el: Element,
    key: &'static str,
    written: AttrText,
}

impl Attribute for FixedAttr {
    const MIN_LENGTH: usize = 0;

    type State = FixedAttrState;
    type AsyncOutput = Self;
    type Cloneable = Self;
    type CloneableOwned = Self;

    fn html_len(&self) -> usize {
        0
    }

    fn to_html(self, buf: &mut String, _class: &mut String, _style: &mut String, _: &mut String) {
        attr_html(self.key, self.text, buf);
    }

    fn hydrate<const FROM_SERVER: bool>(self, el: &Element) -> Self::State {
        if !FROM_SERVER {
            update_attr(el, self.key, Some(&AttrText::Absent), &self.text);
        }
        FixedAttrState {
            el: el.clone(),
            key: self.key,
            written: self.text,
        }
    }

    fn build(self, el: &Element) -> Self::State {
        self.hydrate::<false>(el)
    }

    fn rebuild(self, state: &mut Self::State) {
        if self.key == state.key {
            update_attr(&state.el, self.key, Some(&state.written), &self.text);
        } else {
            write_attr(&state.el, state.key, &AttrText::Absent);
            update_attr(&state.el, self.key, Some(&AttrText::Absent), &self.text);
        }
        state.key = self.key;
        state.written = self.text;
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

    fn keys(&self) -> Vec<NamedAttributeKey> {
        let mut keys = Vec::new();
        self.push_keys(&mut keys);
        keys
    }
}

impl AttrKeys for FixedAttr {
    fn key_count(&self) -> usize {
        // Absent, it never renders: nothing to remove.
        usize::from(self.text != AttrText::Absent)
    }

    fn push_keys(&self, keys: &mut Vec<NamedAttributeKey>) {
        if self.key_count() > 0 {
            keys.push(NamedAttributeKey::Attribute(Cow::Borrowed(self.key)));
        }
    }
}

impl NextAttribute for FixedAttr {
    type Output<NewAttr: Attribute> = (Self, NewAttr);

    fn add_any_attr<NewAttr: Attribute>(self, new_attr: NewAttr) -> Self::Output<NewAttr> {
        (self, new_attr)
    }
}

/// The attribute `key` bound to `value` (`signal_attr("aria-expanded", expanded)`): written once
/// when `value` can't change, else updated by one render effect.
pub fn signal_attr<V>(key: &'static str, value: Signal<V>) -> SignalAttr<V>
where
    V: ToAttrText + Send + Sync + 'static,
{
    SignalAttr {
        key,
        value: Some(value),
    }
}

/// The attribute `key` bound to an optional prop: nothing at all while the prop is absent.
pub fn maybe_attr<T>(key: &'static str, value: MaybeProp<T>) -> SignalAttr<Option<T>>
where
    T: ToAttrText + Send + Sync + 'static,
{
    SignalAttr {
        key,
        value: Option::<Signal<Option<T>>>::from(value),
    }
}

/// The attribute of [`signal_attr`] and [`maybe_attr`].
#[derive(Debug)]
pub struct SignalAttr<V: Send + Sync + 'static> {
    key: &'static str,
    /// `None`: an absent prop, rendering nothing.
    value: Option<Signal<V>>,
}

impl<V: Send + Sync + 'static> Clone for SignalAttr<V> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<V: Send + Sync + 'static> Copy for SignalAttr<V> {}

/// The rendered state of a [`SignalAttr`].
pub struct SignalAttrState {
    el: Element,
    key: &'static str,
    /// The text written last (while there is no effect).
    written: AttrText,
    /// Updates the attribute: only for a signal that can change.
    effect: Option<RenderEffect<AttrText>>,
}

impl<V> SignalAttr<V>
where
    V: ToAttrText + Send + Sync + 'static,
{
    /// The text now, and whether it can change.
    fn probe(&self) -> (AttrText, bool) {
        match self.value {
            Some(value) => probe_sources(|| value.with(ToAttrText::to_attr_text)),
            None => (AttrText::Absent, false),
        }
    }

    /// Renders onto `el`, whose attribute shows `written`; writes only if `write`.
    fn render(self, el: &Element, written: &AttrText, write: bool) -> SignalAttrState {
        let (text, is_reactive) = self.probe();
        if write && text != *written {
            write_attr(el, self.key, &text);
        }
        let effect = match self.value {
            Some(value) if is_reactive => {
                Some(attr_effect(el.clone(), self.key, value, text.clone()))
            }
            _ => None,
        };
        SignalAttrState {
            el: el.clone(),
            key: self.key,
            written: text,
            effect,
        }
    }
}

/// The effect updating the attribute `key` of `el` from `value`, whose text is `written` now.
fn attr_effect<V>(
    el: Element,
    key: &'static str,
    value: Signal<V>,
    written: AttrText,
) -> RenderEffect<AttrText>
where
    V: ToAttrText + Send + Sync + 'static,
{
    RenderEffect::new_with_value(
        move |written: Option<AttrText>| {
            let text = value.with(ToAttrText::to_attr_text);
            update_attr(&el, key, written.as_ref(), &text);
            text
        },
        Some(written),
    )
}

/// Writes `text` unless it is what `written` shows.
fn update_attr(el: &Element, key: &str, written: Option<&AttrText>, text: &AttrText) {
    if written != Some(text) {
        write_attr(el, key, text);
    }
}

/// Writes the attribute `key` of `el` as `text` shows it.
fn write_attr(el: &Element, key: &str, text: &AttrText) {
    let _ = match text {
        AttrText::Absent => el.remove_attribute(key),
        AttrText::Present => el.set_attribute(key, ""),
        AttrText::Text(text) => el.set_attribute(key, text),
        AttrText::Int(number) => {
            let mut buf = [0_u8; 20];
            el.set_attribute(key, format_int(*number, &mut buf))
        }
    };
}

/// `number` in decimal, written into `buf` (no allocation).
fn format_int(number: i64, buf: &mut [u8; 20]) -> &str {
    let mut rest = number.unsigned_abs();
    let mut start = buf.len();
    loop {
        start -= 1;
        // A digit: always below 10.
        buf[start] = b'0' + u8::try_from(rest % 10).unwrap_or(0);
        rest /= 10;
        if rest == 0 {
            break;
        }
    }
    if number < 0 {
        start -= 1;
        buf[start] = b'-';
    }
    std::str::from_utf8(&buf[start..]).unwrap_or_default()
}

/// The server's HTML of the attribute.
fn attr_html(key: &str, text: AttrText, buf: &mut String) {
    match text {
        AttrText::Absent => {}
        AttrText::Present => AttributeValue::to_html(true, key, buf),
        AttrText::Text(text) => AttributeValue::to_html(text.as_str(), key, buf),
        AttrText::Int(number) => {
            let mut digits = [0_u8; 20];
            AttributeValue::to_html(format_int(number, &mut digits), key, buf);
        }
    }
}

impl<V> Attribute for SignalAttr<V>
where
    V: ToAttrText + Send + Sync + 'static,
{
    const MIN_LENGTH: usize = 0;

    type State = SignalAttrState;
    type AsyncOutput = Self;
    type Cloneable = Self;
    type CloneableOwned = Self;

    fn html_len(&self) -> usize {
        0
    }

    fn to_html(self, buf: &mut String, _class: &mut String, _style: &mut String, _: &mut String) {
        let text = match self.value {
            Some(value) => value.with_untracked(ToAttrText::to_attr_text),
            None => AttrText::Absent,
        };
        attr_html(self.key, text, buf);
    }

    fn hydrate<const FROM_SERVER: bool>(self, el: &Element) -> Self::State {
        // The server rendered the attribute already (as tachys trusts the server's attributes
        // when hydrating); a template clone has none.
        if FROM_SERVER {
            self.render(el, &AttrText::Absent, false)
        } else {
            self.render(el, &AttrText::Absent, true)
        }
    }

    fn build(self, el: &Element) -> Self::State {
        self.render(el, &AttrText::Absent, true)
    }

    fn rebuild(self, state: &mut Self::State) {
        let written = state
            .effect
            .take()
            .and_then(|effect| effect.take_value())
            .unwrap_or_else(|| std::mem::replace(&mut state.written, AttrText::Absent));
        let el = state.el.clone();
        if self.key == state.key {
            *state = self.render(&el, &written, true);
        } else {
            // Another attribute: remove the previous one.
            write_attr(&el, state.key, &AttrText::Absent);
            *state = self.render(&el, &AttrText::Absent, true);
        }
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

    fn keys(&self) -> Vec<NamedAttributeKey> {
        let mut keys = Vec::new();
        self.push_keys(&mut keys);
        keys
    }
}

impl<V: Send + Sync + 'static> AttrKeys for SignalAttr<V> {
    fn key_count(&self) -> usize {
        // An absent prop never renders: nothing to remove.
        usize::from(self.value.is_some())
    }

    fn push_keys(&self, keys: &mut Vec<NamedAttributeKey>) {
        if self.value.is_some() {
            keys.push(NamedAttributeKey::Attribute(Cow::Borrowed(self.key)));
        }
    }
}

impl<V> NextAttribute for SignalAttr<V>
where
    V: ToAttrText + Send + Sync + 'static,
{
    type Output<NewAttr: Attribute> = (Self, NewAttr);

    fn add_any_attr<NewAttr: Attribute>(self, new_attr: NewAttr) -> Self::Output<NewAttr> {
        (self, new_attr)
    }
}

/// The boolean DOM property `key` bound to `value` (`signal_prop("checked", is_selected)`): set
/// once when `value` can't change, else on every run of one render effect (as tachys'
/// `prop(key, signal)` sets it, also to an unchanged value: the user may have changed the
/// element's property since). Properties don't render on the server; hydration sets them.
pub fn signal_prop(key: &'static str, value: Signal<bool>) -> SignalProp {
    SignalProp { key, value }
}

/// The DOM property of [`signal_prop`].
#[derive(Debug, Clone, Copy)]
pub struct SignalProp {
    key: &'static str,
    value: Signal<bool>,
}

/// The rendered state of a [`SignalProp`]: the effect setting the property, if it can change.
pub struct SignalPropState {
    el: Element,
    effect: Option<RenderEffect<()>>,
}

/// Sets the boolean property `key` of `el`.
fn set_bool_prop(el: &Element, key: &str, value: bool) {
    let _ = js_sys::Reflect::set(
        el,
        &wasm_bindgen::JsValue::from_str(key),
        &wasm_bindgen::JsValue::from_bool(value),
    );
}

impl SignalProp {
    /// Sets the property on `el`, with an effect for a value that can change.
    fn render(self, el: &Element) -> SignalPropState {
        let (value, is_reactive) = probe_sources(|| self.value.get());
        let effect = if is_reactive {
            let (el, key, signal) = (el.clone(), self.key, self.value);
            // Its first run (now) sets the property.
            Some(RenderEffect::new(move |_| {
                set_bool_prop(&el, key, signal.get());
            }))
        } else {
            set_bool_prop(el, self.key, value);
            None
        };
        SignalPropState {
            el: el.clone(),
            effect,
        }
    }
}

impl Attribute for SignalProp {
    const MIN_LENGTH: usize = 0;

    type State = SignalPropState;
    type AsyncOutput = Self;
    type Cloneable = Self;
    type CloneableOwned = Self;

    fn html_len(&self) -> usize {
        0
    }

    fn to_html(self, _buf: &mut String, _class: &mut String, _style: &mut String, _: &mut String) {}

    fn hydrate<const FROM_SERVER: bool>(self, el: &Element) -> Self::State {
        self.render(el)
    }

    fn build(self, el: &Element) -> Self::State {
        self.render(el)
    }

    fn rebuild(self, state: &mut Self::State) {
        // The previous key keeps its value, as with tachys' properties.
        drop(state.effect.take());
        let el = state.el.clone();
        *state = self.render(&el);
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

    fn keys(&self) -> Vec<NamedAttributeKey> {
        vec![NamedAttributeKey::Property(Cow::Borrowed(self.key))]
    }
}

impl NextAttribute for SignalProp {
    type Output<NewAttr: Attribute> = (Self, NewAttr);

    fn add_any_attr<NewAttr: Attribute>(self, new_attr: NewAttr) -> Self::Output<NewAttr> {
        (self, new_attr)
    }
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;
    use crate::testing::with_owner;

    fn html<V: ToAttrText + Send + Sync + 'static>(attr: SignalAttr<V>) -> String {
        let mut buf = String::new();
        attr.to_html(
            &mut buf,
            &mut String::new(),
            &mut String::new(),
            &mut String::new(),
        );
        buf
    }

    #[test]
    fn values_render_as_tachys_renders_them() {
        with_owner(|| {
            assert_that!(html(signal_attr("disabled", Signal::stored(true))))
                .is_equal_to(" disabled".to_owned());
            assert_that!(html(signal_attr("disabled", Signal::stored(false))))
                .is_equal_to(String::new());
            assert_that!(html(signal_attr("tabindex", Signal::stored(Some(-1)))))
                .is_equal_to(r#" tabindex="-1""#.to_owned());
            assert_that!(html(signal_attr("tabindex", Signal::stored(None::<i32>))))
                .is_equal_to(String::new());
            assert_that!(html(signal_attr(
                "title",
                Signal::stored(Some(r#"a "b""#.to_owned()))
            )))
            .is_equal_to(r#" title="a &quot;b&quot;""#.to_owned());
            assert_that!(html(maybe_attr::<String>(
                "aria-label",
                MaybeProp::default()
            )))
            .is_equal_to(String::new());
        });
    }

    #[test]
    fn fixed_values_render_and_absent_ones_have_no_keys() {
        let html = |attr: FixedAttr| {
            let mut buf = String::new();
            attr.to_html(
                &mut buf,
                &mut String::new(),
                &mut String::new(),
                &mut String::new(),
            );
            buf
        };
        assert_that!(html(fixed_attr("id", Some("a".to_owned()))))
            .is_equal_to(r#" id="a""#.to_owned());
        assert_that!(html(fixed_attr("formnovalidate", true)))
            .is_equal_to(" formnovalidate".to_owned());
        assert_that!(fixed_attr("form", None::<String>).keys().len()).is_equal_to(0);
        assert_that!(fixed_attr("id", "a").keys().len()).is_equal_to(1);
    }

    #[test]
    fn numbers_format_without_allocating() {
        let mut buf = [0_u8; 20];
        assert_that!(format_int(0, &mut buf).to_owned()).is_equal_to("0".to_owned());
        assert_that!(format_int(-1, &mut buf).to_owned()).is_equal_to("-1".to_owned());
        assert_that!(format_int(1234, &mut buf).to_owned()).is_equal_to("1234".to_owned());
        assert_that!(format_int(i64::MIN, &mut buf).to_owned()).is_equal_to(i64::MIN.to_string());
    }

    #[test]
    fn an_absent_prop_has_no_keys() {
        with_owner(|| {
            assert_that!(
                maybe_attr::<String>("aria-label", MaybeProp::default())
                    .keys()
                    .len()
            )
            .is_equal_to(0);
            assert_that!(
                signal_attr("aria-label", Signal::stored(None::<String>))
                    .keys()
                    .len()
            )
            .is_equal_to(1);
        });
    }
}
