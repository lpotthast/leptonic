#![recursion_limit = "256"]
// Workspace-level clippy allows. These are specified in workspace Cargo.toml lints but
// must also be set here because CLI `-D clippy::pedantic` takes precedence over Cargo.toml lints.
#![allow(
    clippy::option_if_let_else,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::wildcard_imports,
    // `ignored_unit_patterns` fires on Leptos `view!` macro expansions and `#[component]` artifacts.
    clippy::ignored_unit_patterns,
    // `type_complexity` fires on `#[component]` macro-generated prop types that cannot be annotated individually.
    clippy::type_complexity
)]

use leptos::prelude::*;
use leptos_use::use_window;

pub mod atoms;
pub mod components;
pub mod hooks;
pub mod utils;

// Let's make some types of our public API more easily accessible.
pub use crate::utils::scroll_behavior::ScrollBehavior;

pub mod prelude {
    // Reexport
    pub use icondata;
    #[cfg(feature = "tiptap")]
    pub use leptos_tiptap::*;

    pub use super::{
        FontWeight, Height, Margin, Mount, OptionDeref, Out, Padding, Width,
        utils::{
            aria::{AriaExpanded, AriaHasPopup},
            callback::{ViewCallback, ViewProducer},
        },
    };
    //pub use crate::atoms::prelude::*;
    //pub use crate::components::prelude::*;
    //pub use crate::hooks::prelude::*;
    pub use crate::hooks::IntoAttrs;
    pub use crate::signal_ls;
}

#[derive(Debug, Clone, Copy)]
pub enum Language {
    En,
}

/// The `Out` type represents any outgoing / emittable value. Use it in components that should
/// return (propagate) a value upwards using a function-like property.
/// Out can be anything that can be written to:
/// - a `WriteSignal`,
/// - a combined `RwSignal` or
/// - a `Callback` which only consumes an input and returns `()`.
///
/// This helps you to define props where the user can choose to use a signal directly
/// or use a closure (which will be converted to a `Callback`).
#[derive(Debug)]
pub enum Out<O: 'static, S = SyncStorage> {
    Fn(fn(O) -> ()),
    Callback(Callback<O, ()>),
    WriteSignal(WriteSignal<O, S>),
    RwSignal(RwSignal<O, S>),
    StoredValue(StoredValue<O, S>),
}

impl<O: 'static, S> Copy for Out<O, S> {}

impl<O: 'static, S> Clone for Out<O, S> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<O: 'static, S> Default for Out<O, S> {
    fn default() -> Self {
        Self::new_fn(|_| {
            // intentional noop
        })
    }
}

impl<O: 'static, S> Out<O, S> {
    /// Creates a new `Out` from the given function pointer.
    pub fn new_fn(f: fn(O) -> ()) -> Self {
        Self::Fn(f)
    }

    /// Creates a new `Out` from the given function.
    pub fn new_callback(f: impl Fn(O) + Send + Sync + 'static) -> Self {
        Self::Callback(Callback::new(f))
    }
}

impl<O: 'static> Out<O, LocalStorage> {
    pub fn set(&self, new_value: O) {
        match self {
            Self::Fn(f) => f(new_value),
            Self::Callback(callback) => Callable::run(callback, new_value),
            Self::WriteSignal(write_signal) => write_signal.set(new_value),
            Self::RwSignal(rw_signal) => rw_signal.set(new_value),
            Self::StoredValue(stored_value) => stored_value.set_value(new_value),
        }
    }
}

impl<O: Send + Sync + 'static> Out<O, SyncStorage> {
    pub fn set(&self, new_value: O) {
        match self {
            Self::Fn(f) => f(new_value),
            Self::Callback(callback) => Callable::run(callback, new_value),
            Self::WriteSignal(write_signal) => write_signal.set(new_value),
            Self::RwSignal(rw_signal) => rw_signal.set(new_value),
            Self::StoredValue(stored_value) => stored_value.set_value(new_value),
        }
    }
}

impl<T, F, S> From<F> for Out<T, S>
where
    T: 'static,
    F: Fn(T) + Send + Sync + 'static,
{
    fn from(fun: F) -> Self {
        Self::new_callback(fun)
    }
}

impl<O: 'static> From<Callback<O, ()>> for Out<O, SyncStorage> {
    fn from(callback: Callback<O, ()>) -> Self {
        Self::Callback(callback)
    }
}

#[cfg(not(feature = "nightly"))]
impl<O: 'static, S> From<WriteSignal<O, S>> for Out<O, S> {
    fn from(write_signal: WriteSignal<O, S>) -> Self {
        Self::WriteSignal(write_signal)
    }
}

#[cfg(not(feature = "nightly"))]
impl<O: 'static, S> From<RwSignal<O, S>> for Out<O, S> {
    fn from(rw_signal: RwSignal<O, S>) -> Self {
        Self::RwSignal(rw_signal)
    }
}

impl<O: 'static, S> From<StoredValue<O, S>> for Out<O, S> {
    fn from(stored_value: StoredValue<O, S>) -> Self {
        Self::StoredValue(stored_value)
    }
}

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub enum Mount {
    /// Mount the child view once. Then keep it mounted as long as the parent lives.
    #[default]
    Once,

    /// Mount the child view once. May defer mounting to the point where the view is first needed. Then keep it mounted as long as the parent lives.
    // OnceShown, // TODO: Implement this variant in tabs.
    /// Always re-mount the child view when it is needed.
    WhenShown,
}

/// Create a read-write signal pair that automatically syncs the stored value in the browsers
/// `LocalStorage`. When called, the value is read back from storage.
/// When the value is not found, `initial` is set.
pub fn signal_ls<
    T: Send + Sync + Clone + serde::Serialize + serde::de::DeserializeOwned + 'static,
>(
    key: &'static str,
    initial: T,
) -> (ReadSignal<T>, WriteSignal<T>) {
    let (signal, set_signal) = signal(read_from_local_storage::<T>(key).unwrap_or(initial));

    track_in_local_storage(key, signal);

    (signal, set_signal)
}

#[must_use]
pub fn read_from_local_storage<T: serde::de::DeserializeOwned>(key: &'static str) -> Option<T> {
    use_window().as_ref().and_then(|window| {
        let storage = window.local_storage().ok()??;
        let stored = storage.get(key).ok()??;
        match serde_json::from_str(&stored) {
            Ok(des) => Some(des),
            Err(err) => {
                tracing::error!(
                    "Could not deserialize local-storage value at key '{key}'. Received '{stored}'. Tried to convert to '{ty}'. App may continue using a default value. Err: {err}",
                    ty = std::any::type_name::<T>()
                );
                None
            }
        }
    })
}

pub fn track_in_local_storage<T: Send + Sync + serde::Serialize + Clone + 'static>(
    key: &'static str,
    signal: ReadSignal<T>,
) {
    Effect::new(move |_old| {
        if let Some(window) = &*use_window() {
            let storage = window.local_storage().ok()??;
            let val = signal.get(); // TODO (new): Can we use read() instead?
            storage
                .set(key, serde_json::to_string(&val).ok()?.as_ref())
                .ok()
        } else {
            Some(())
        }
    });
}

pub trait OptionDeref<T: std::ops::Deref> {
    fn deref(&self) -> Option<&T::Target>;
    fn deref_or<'a>(&'a self, default: &'a T::Target) -> &'a T::Target;
    fn deref_or_else<'a, F: Fn() -> &'a T::Target>(&'a self, default: F) -> &'a T::Target;
}

impl<T: std::ops::Deref> OptionDeref<T> for Option<T> {
    fn deref(&self) -> Option<&T::Target> {
        self.as_ref().map(std::ops::Deref::deref)
    }

    fn deref_or<'a>(&'a self, default: &'a T::Target) -> &'a T::Target {
        self.as_ref().map_or(default, std::ops::Deref::deref)
    }

    fn deref_or_else<'a, F: Fn() -> &'a T::Target>(&'a self, default: F) -> &'a T::Target {
        self.as_ref().map_or_else(default, std::ops::Deref::deref)
    }
}

pub type Width = utils::css::CssDimension;
pub type Height = utils::css::CssDimension;

// Re-export CSS shorthand value types from leptos-styles.
pub use utils::css::{FontWeight, Margin, Padding};
