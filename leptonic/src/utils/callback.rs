use leptos::View;
use leptos_reactive::{Callable, Callback};
use std::fmt::{Debug, Formatter};

/// A callback.
/// Use `Consumer<In, Out>` when you would otherwise write `Callback<In, Out>`.
pub struct Consumer<In: 'static = (), Out: 'static = ()>(
    leptos::StoredValue<Box<dyn Fn(In) -> Out>>,
);

impl<In: 'static, Out: 'static> Consumer<In, Out> {
    pub fn new<F: Fn(In) -> Out + 'static>(fun: F) -> Self {
        Self(leptos::store_value(Box::new(fun)))
    }

    pub fn consume(&self, arg: In) -> Out {
        self.0.with_value(|cb| cb(arg))
    }
}

impl<In: 'static, Out: 'static> std::ops::Deref for Consumer<In, Out> {
    type Target = leptos::StoredValue<Box<dyn Fn(In) -> Out>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<In: 'static, Out: 'static> Copy for Consumer<In, Out> {}

impl<In: 'static, Out: 'static> Clone for Consumer<In, Out> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<In: 'static, Out: 'static> Callable<In, Out> for Consumer<In, Out> {
    fn call(&self, arg: In) -> Out {
        self.consume(arg)
    }
}

impl<In: 'static, Out: 'static> Debug for Consumer<In, Out> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Consumer").finish()
    }
}

pub fn consumer<In: 'static, Out: 'static, F: Fn(In) -> Out + 'static>(
    fun: F,
) -> Consumer<In, Out> {
    Consumer::new(fun)
}

impl<In: 'static, Out: 'static, F: Fn(In) -> Out + 'static> From<F> for Consumer<In, Out> {
    fn from(fun: F) -> Self {
        Self::new(fun)
    }
}

#[cfg(not(feature = "nightly"))]
impl<In: 'static, Out: 'static> From<Callback<In, Out>> for Consumer<In, Out> {
    fn from(cb: Callback<In, Out>) -> Self {
        Self::new(move |arg| Callable::call(&cb, arg))
    }
}

/// A callback which returns something without requiring any input.
/// Use `Producer<Out>` when you would otherwise write `Callback<(), Out>`.
pub struct Producer<Out: 'static = ()>(leptos::StoredValue<Box<dyn Fn() -> Out>>);

impl<Out: 'static> Producer<Out> {
    pub fn new<F: Fn() -> Out + 'static>(fun: F) -> Self {
        Self(leptos::store_value(Box::new(fun)))
    }

    pub fn produce(&self) -> Out {
        self.0.with_value(|cb| cb())
    }
}

impl<Out: 'static> std::ops::Deref for Producer<Out> {
    type Target = leptos::StoredValue<Box<dyn Fn() -> Out>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<Out: 'static> Copy for Producer<Out> {}

impl<Out: 'static> Clone for Producer<Out> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<Out: 'static> Callable<(), Out> for Producer<Out> {
    fn call(&self, _arg: ()) -> Out {
        self.produce()
    }
}

impl<Out: 'static> Debug for Producer<Out> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Producer").finish()
    }
}

pub fn producer<Out: 'static, F: Fn() -> Out + 'static>(fun: F) -> Producer<Out> {
    Producer::new(fun)
}

impl<Out: 'static, F: Fn() -> Out + 'static> From<F> for Producer<Out> {
    fn from(fun: F) -> Self {
        Self::new(fun)
    }
}

/// A callback which returns a `leptos::View` without requiring any input.
/// Use `ViewProducer` when you would otherwise write `Callback<(), leptos::View>`.
#[derive(Clone, Copy)]
pub struct ViewProducer(leptos::StoredValue<Box<dyn Fn() -> View>>);

impl ViewProducer {
    pub fn new<F: Fn() -> View + 'static>(fun: F) -> Self {
        Self(leptos::store_value(Box::new(fun)))
    }

    pub fn produce(&self) -> View {
        self.0.with_value(|cb| cb())
    }
}

impl std::ops::Deref for ViewProducer {
    type Target = leptos::StoredValue<Box<dyn Fn() -> View>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Callable<(), View> for ViewProducer {
    fn call(&self, _arg: ()) -> View {
        self.produce()
    }
}

impl Debug for ViewProducer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ViewProducer").finish()
    }
}

pub fn view_producer<V: leptos::IntoView, F: Fn() -> V + 'static>(fun: F) -> ViewProducer {
    ViewProducer::new(move || fun().into_view())
}

impl<V: leptos::IntoView, F: Fn() -> V + 'static> From<F> for ViewProducer {
    fn from(fun: F) -> Self {
        Self::new(move || fun().into_view())
    }
}

/// A callback which returns a `leptos::View` without.
/// Use `ViewCallback<In>` when you would otherwise write `Callback<In, leptos::View>`.
pub struct ViewCallback<In: 'static>(leptos::StoredValue<Box<dyn Fn(In) -> View>>);

impl<In: 'static> ViewCallback<In> {
    pub fn new<F: Fn(In) -> View + 'static>(fun: F) -> Self {
        Self(leptos::store_value(Box::new(fun)))
    }

    pub fn render(&self, arg: In) -> View {
        self.0.with_value(|cb| cb(arg))
    }
}

impl<In: 'static> std::ops::Deref for ViewCallback<In> {
    type Target = leptos::StoredValue<Box<dyn Fn(In) -> View>>;

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

impl<In: 'static> Callable<In, View> for ViewCallback<In> {
    fn call(&self, arg: In) -> View {
        self.render(arg)
    }
}

impl<In: 'static> Debug for ViewCallback<In> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ViewProducer").finish()
    }
}

pub fn view_callback<In: 'static, V: leptos::IntoView, F: Fn(In) -> V + 'static>(
    fun: F,
) -> ViewCallback<In> {
    ViewCallback::new(move |t| fun(t).into_view())
}

impl<In: 'static, V: leptos::IntoView, F: Fn(In) -> V + 'static> From<F> for ViewCallback<In> {
    fn from(fun: F) -> Self {
        Self::new(move |t| fun(t).into_view())
    }
}
