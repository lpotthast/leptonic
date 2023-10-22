use leptos::View;
use std::fmt::{Debug, Formatter};

/// A callback which returns nothing.
/// Use `Consumer<In>` when you would otherwise write `Callback<In, ()>`.
pub struct Consumer<In: 'static = ()>(leptos::StoredValue<Box<dyn Fn(In) -> ()>>);

impl<In: 'static> Consumer<In> {
    pub fn new<F: Fn(In) -> () + 'static>(fun: F) -> Self {
        Self(leptos::store_value(Box::new(fun)))
    }

    pub fn consume(&self, arg: In) {
        self.0.with_value(|cb| cb(arg))
    }
}

impl<In: 'static> std::ops::Deref for Consumer<In> {
    type Target = leptos::StoredValue<Box<dyn Fn(In) -> ()>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<In: 'static> Copy for Consumer<In> {}

impl<In: 'static> Clone for Consumer<In> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<In: 'static> leptos::Callable<In, ()> for Consumer<In> {
    fn call(&self, arg: In) -> () {
        self.consume(arg)
    }
}

impl<In: 'static> Debug for Consumer<In> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Consumer").finish()
    }
}

pub fn consumer<In: 'static, F: Fn(In) -> () + 'static>(fun: F) -> Consumer<In> {
    Consumer::new(fun)
}

impl<In: 'static, F: Fn(In) -> () + 'static> From<F> for Consumer<In> {
    fn from(fun: F) -> Self {
        Consumer::new(fun)
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

impl<Out: 'static> leptos::Callable<(), Out> for Producer<Out> {
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
        Producer::new(fun)
    }
}

/// A callback which returns a `leptos::View` without requiring any input.
/// Use `ViewProducer` when you would otherwise write `Callback<(), leptos::View>`.
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

impl Copy for ViewProducer {}

impl Clone for ViewProducer {
    fn clone(&self) -> Self {
        *self
    }
}

impl leptos::Callable<(), View> for ViewProducer {
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
        ViewProducer::new(move || fun().into_view())
    }
}

/// A callback which returns a `leptos::View` without.
/// Use `ViewCallback<In>` when you would otherwise write `Callback<In, leptos::View>`.
pub struct ViewCallback<In: 'static>(leptos::StoredValue<Box<dyn Fn(In) -> View>>);

impl<In: 'static> ViewCallback<In> {
    pub fn new<F: Fn(In) -> View + 'static>(fun: F) -> Self {
        Self(leptos::store_value(Box::new(fun)))
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

impl<In: 'static> leptos::Callable<In, View> for ViewCallback<In> {
    fn call(&self, arg: In) -> View {
        self.0.with_value(|cb| cb(arg))
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
        ViewCallback::new(move |t| fun(t).into_view())
    }
}
