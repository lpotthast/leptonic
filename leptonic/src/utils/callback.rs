use leptos::callback::Callable;
use leptos::prelude::{AnyView, Callback, IntoAny};
use std::fmt::{Debug, Formatter};

/// A callback which returns a `leptos::View` without requiring any input.
/// Use `ViewProducer` when you would otherwise write `Callback<(), leptos::View>`.
#[derive(Clone, Copy)]
pub struct ViewProducer(Callback<(), AnyView>);

impl ViewProducer {
    pub fn new<C: Into<Callback<(), AnyView>>>(callback: C) -> Self {
        Self(callback.into())
    }

    pub fn produce(&self) -> AnyView {
        self.0.run(())
    }
}

impl std::ops::Deref for ViewProducer {
    type Target = Callback<(), AnyView>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// TODO (new): Is this impl still necessary?
impl Callable<(), AnyView> for ViewProducer {
    fn run(&self, _input: ()) -> AnyView {
        self.produce()
    }
}

impl Debug for ViewProducer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ViewProducer").finish()
    }
}

impl<V, F> From<F> for ViewProducer
where
    V: leptos::IntoView + 'static,
    <V as leptos::prelude::Render>::State: 'static,
    F: Fn() -> V + Send + Sync + 'static,
{
    fn from(fun: F) -> Self {
        Self::new(move || fun().into_any())
    }
}

/// A callback returning `AnyView`.
/// Use `ViewCallback<In>` when you would otherwise write `Callback<In, AnyView>`.
pub struct ViewCallback<In>(Callback<In, AnyView>)
where
    In: 'static;

impl<In: 'static> ViewCallback<In> {
    pub fn new(callback: impl Into<Callback<In, AnyView>>) -> Self
    {
        Self(callback.into())
    }

    pub fn render(&self, input: In) -> AnyView {
        self.0.run(input)
    }
}

impl<In: 'static> std::ops::Deref for ViewCallback<In> {
    type Target = Callback<In, AnyView>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<In: 'static> Copy for ViewCallback<In> {}

impl<In: 'static> Clone for ViewCallback<In> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<In: 'static> Callable<In, AnyView> for ViewCallback<In> {
    fn run(&self, input: In) -> AnyView {
        self.render(input)
    }
}

impl<In: 'static> Debug for ViewCallback<In> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ViewCallback").finish()
    }
}

impl<In, V, F> From<F> for ViewCallback<In>
where
    In: Send + Sync + 'static,
    V: leptos::IntoView + 'static,
    <V as leptos::prelude::Render>::State: 'static,
    F: Fn(In) -> V + Send + Sync + 'static,
{
    fn from(fun: F) -> Self {
        let c: Callback<In, AnyView> = Callback::new(move |input| fun(input).into_any());
        Self::new(c)
    }
}
