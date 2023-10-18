use std::fmt::{Debug, Formatter};

/// A callback backed by a `leptos::StoredValue` where the stored function...
/// - ❌ is Clone
/// - ✅ is Copy
///
/// ⚠️ requires a leptos context
pub struct Consumer<T: 'static = ()>(leptos::StoredValue<Box<dyn Fn(T) -> ()>>);

impl<T: 'static> Consumer<T> {
    pub fn new<F: Fn(T) -> () + 'static>(fun: F) -> Self {
        Self(leptos::store_value(Box::new(fun)))
    }

    pub fn consume(&self, arg: T) {
        self.0.with_value(|cb| cb(arg))
    }
}

impl<T: 'static> std::ops::Deref for Consumer<T> {
    type Target = leptos::StoredValue<Box<dyn Fn(T) -> ()>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: 'static> Copy for Consumer<T> {}

impl<T: 'static> Clone for Consumer<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: 'static> leptos::Callable<T, ()> for Consumer<T> {
    fn call(&self, arg: T) -> () {
        self.consume(arg)
    }
}

impl<T: 'static> Debug for Consumer<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Consumer").finish()
    }
}

pub fn consumer<T: 'static, F: Fn(T) -> () + 'static>(fun: F) -> Consumer<T> {
    Consumer::new(fun)
}

impl<T: 'static, F: Fn(T) -> () + 'static> From<F> for Consumer<T> {
    fn from(fun: F) -> Self {
        Consumer::new(fun)
    }
}

/// A callback backed by a `leptos::StoredValue` where the stored function...
/// - ❌ is Clone
/// - ✅ is Copy
///
/// ⚠️ requires a leptos context
pub struct Producer<R: 'static = ()>(leptos::StoredValue<Box<dyn Fn() -> R>>);

impl<R: 'static> Producer<R> {
    pub fn new<F: Fn() -> R + 'static>(fun: F) -> Self {
        Self(leptos::store_value(Box::new(fun)))
    }

    pub fn produce(&self) -> R {
        self.0.with_value(|cb| cb())
    }
}

impl<R: 'static> std::ops::Deref for Producer<R> {
    type Target = leptos::StoredValue<Box<dyn Fn() -> R>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<R: 'static> Copy for Producer<R> {}

impl<R: 'static> Clone for Producer<R> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<R: 'static> leptos::Callable<(), R> for Producer<R> {
    fn call(&self, _arg: ()) -> R {
        self.produce()
    }
}

impl<R: 'static> Debug for Producer<R> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Producer").finish()
    }
}

pub fn producer<R: 'static, F: Fn() -> R + 'static>(fun: F) -> Producer<R> {
    Producer::new(fun)
}

impl<R: 'static, F: Fn() -> R + 'static> From<F> for Producer<R> {
    fn from(fun: F) -> Self {
        Producer::new(fun)
    }
}
