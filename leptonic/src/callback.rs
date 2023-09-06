use std::{
    fmt::{Debug, Formatter},
    rc::Rc,
    sync::Arc,
};

pub trait Callable<A, R = ()> {
    fn call(&self, arg: A) -> R;
}

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

impl<T: 'static> Callable<T, ()> for Consumer<T> {
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

impl<R: 'static> Callable<(), R> for Producer<R> {
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

/// A callback backed by a `leptos::StoredValue` where the stored function...
/// - ❌ is Clone
/// - ✅ is Copy
///
/// ⚠️ requires a leptos context
pub struct Callback<T: 'static, R: 'static = ()>(leptos::StoredValue<Box<dyn Fn(T) -> R>>);

impl<T: 'static, R: 'static> Callback<T, R> {
    pub fn new<F: Fn(T) -> R + 'static>(fun: F) -> Self {
        Self(leptos::store_value(Box::new(fun)))
    }
}

impl<T: 'static, R: 'static> std::ops::Deref for Callback<T, R> {
    type Target = leptos::StoredValue<Box<dyn Fn(T) -> R>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: 'static, R: 'static> Copy for Callback<T, R> {}

impl<T: 'static, R: 'static> Clone for Callback<T, R> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: 'static, R: 'static> Callable<T, R> for Callback<T, R> {
    fn call(&self, arg: T) -> R {
        self.0.with_value(|cb| cb(arg))
    }
}

impl<T: 'static, R: 'static> Debug for Callback<T, R> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Callback").finish()
    }
}

pub fn callback<T: 'static, R: 'static, F: Fn(T) -> R + 'static>(fun: F) -> Callback<T, R> {
    Callback::new(fun)
}

impl<T: 'static, R: 'static, F: Fn(T) -> R + 'static> From<F> for Callback<T, R> {
    fn from(fun: F) -> Self {
        Callback::new(fun)
    }
}

/// A callback backed by a `leptos::StoredValue` where the stored function...
/// - ✅ is Clone
/// - ✅ is Copy
///
/// ⚠️ requires a leptos context
pub struct CallbackRc<T: 'static, R: 'static = ()>(leptos::StoredValue<Rc<dyn Fn(T) -> R>>);

impl<T: 'static, R: 'static> CallbackRc<T, R> {
    pub fn new<F: Fn(T) -> R + 'static>(fun: F) -> Self {
        Self(leptos::store_value(Rc::new(fun)))
    }
}

impl<T: 'static, R: 'static> std::ops::Deref for CallbackRc<T, R> {
    type Target = leptos::StoredValue<Rc<dyn Fn(T) -> R>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: 'static, R: 'static> Copy for CallbackRc<T, R> {}

impl<T: 'static, R: 'static> Clone for CallbackRc<T, R> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: 'static, R: 'static> Callable<T, R> for CallbackRc<T, R> {
    fn call(&self, arg: T) -> R {
        self.0.with_value(|cb| cb(arg))
    }
}

impl<T: 'static, R: 'static> Debug for CallbackRc<T, R> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("CallbackRc").finish()
    }
}

pub fn callback_rc<T: 'static, R: 'static, F: Fn(T) -> R + 'static>(
    fun: F,
) -> CallbackRc<T, R> {
    CallbackRc::new(fun)
}

/// A callback backed by a `leptos::StoredValue` where the stored function...
/// - ✅ is Clone
/// - ✅ is Copy
/// - ✅ is Send and Sync if T is
///
/// ⚠️ requires a leptos context
pub struct CallbackArc<T: 'static, R: 'static = ()>(leptos::StoredValue<Arc<dyn Fn(T) -> R>>);

impl<T: 'static, R: 'static> CallbackArc<T, R> {
    pub fn new<F: Fn(T) -> R + 'static>(fun: F) -> Self {
        Self(leptos::store_value(Arc::new(fun)))
    }
}

impl<T: 'static, R: 'static> std::ops::Deref for CallbackArc<T, R> {
    type Target = leptos::StoredValue<Arc<dyn Fn(T) -> R>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: 'static, R: 'static> Copy for CallbackArc<T, R> {}

impl<T: 'static, R: 'static> Clone for CallbackArc<T, R> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: 'static, R: 'static> Callable<T, R> for CallbackArc<T, R> {
    fn call(&self, arg: T) -> R {
        self.0.with_value(|cb| cb(arg))
    }
}

impl<T: 'static, R: 'static> Debug for CallbackArc<T, R> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("CallbackArc").finish()
    }
}

pub fn callback_arc<T: 'static, R: 'static, F: Fn(T) -> R + 'static>(
    fun: F,
) -> CallbackArc<T, R> {
    CallbackArc::new(fun)
}

/// A callback not backed by leptos which...
/// - ✅ is Clone
/// - ❌ but is not Copy!
/// You may use this instead of a standard `Callback` if you do not want to use a generic callback
/// but do not require the callback to be copy (you only need it in one place).s
pub struct SimpleCallback<T, R = ()>(Rc<dyn Fn(T) -> R>);

impl<T, R> SimpleCallback<T, R> {
    pub fn new<F: Fn(T) -> R + 'static>(fun: F) -> Self {
        Self(Rc::new(fun))
    }
}

impl<T, R> std::ops::Deref for SimpleCallback<T, R> {
    type Target = Rc<dyn Fn(T) -> R>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, R> Clone for SimpleCallback<T, R> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T, R> Callable<T, R> for SimpleCallback<T, R> {
    fn call(&self, arg: T) -> R {
        self.0(arg)
    }
}

impl<T, R> Debug for SimpleCallback<T, R> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("SimpleCallback").finish()
    }
}

pub fn simple_callback<T: 'static, R: 'static, F: Fn(T) -> R + 'static>(
    fun: F,
) -> SimpleCallback<T, R> {
    SimpleCallback::new(fun)
}

impl<T: 'static, R: 'static, F: Fn(T) -> R + 'static> From<F> for SimpleCallback<T, R> {
    fn from(value: F) -> Self {
        SimpleCallback::new(value)
    }
}
