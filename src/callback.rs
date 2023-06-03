use std::{rc::Rc, sync::Arc};

pub trait Callable<A> {
    fn call(&self, arg: A);
}

// TODO: Derive Debug
/// A callback which...
/// - ✅ is Clone
/// - ❌ is not Copy
/// - ✅ requires no leptos context
pub struct SimpleCallback<T: 'static, R: 'static = ()>(Rc<dyn Fn(T) -> R>);

impl<T: 'static, R: 'static> SimpleCallback<T, R> {
    pub fn new<F: Fn(T) -> R + 'static>(fun: F) -> Self {
        Self(Rc::new(fun))
    }
}

impl<T: 'static, R: 'static> std::ops::Deref for SimpleCallback<T, R> {
    type Target = Rc<dyn Fn(T) -> R>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: 'static, R: 'static> Clone for SimpleCallback<T, R> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: 'static, R: 'static> Callable<T> for SimpleCallback<T, R> {
    fn call(&self, arg: T) {
        self.0(arg);
    }
}

// TODO: Derive Debug
/// A callback which...
/// - ❌ is Clone
/// - ✅ is Copy
/// - ⚠️ requires a leptos context
pub struct Callback<T: 'static, R: 'static = ()>(leptos::StoredValue<Box<dyn Fn(T) -> R>>);

impl<T: 'static, R: 'static> Callback<T, R> {
    pub fn new<F: Fn(T) -> R + 'static>(cx: leptos::Scope, fun: F) -> Self {
        Self(leptos::store_value(cx, Box::new(fun)))
    }
}

impl<T: 'static, R: 'static> std::ops::Deref for Callback<T, R> {
    type Target = leptos::StoredValue<Box<dyn Fn(T) -> R>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: 'static, R: 'static> Clone for Callback<T, R> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: 'static, R: 'static> Copy for Callback<T, R> {}

impl<T: 'static, R: 'static> Callable<T> for Callback<T, R> {
    fn call(&self, arg: T) {
        self.0.with_value(|cb| cb(arg));
    }
}

// TODO: Derive Debug
/// A callback which...
/// - ✅ is Clone
/// - ✅ is Copy
/// - ⚠️ requires a leptos context
pub struct CallbackRc<T: 'static, R: 'static = ()>(leptos::StoredValue<Rc<dyn Fn(T)-> R>>);

impl<T: 'static, R: 'static> CallbackRc<T, R> {
    pub fn new<F: Fn(T) -> R + 'static>(cx: leptos::Scope, fun: F) -> Self {
        Self(leptos::store_value(cx, Rc::new(fun)))
    }
}

impl<T: 'static, R: 'static> std::ops::Deref for CallbackRc<T, R> {
    type Target = leptos::StoredValue<Rc<dyn Fn(T) -> R>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: 'static, R: 'static> Clone for CallbackRc<T, R> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: 'static, R: 'static> Copy for CallbackRc<T, R> {}

impl<T: 'static, R: 'static> Callable<T> for CallbackRc<T, R> {
    fn call(&self, arg: T) {
        self.0.with_value(|cb| cb(arg));
    }
}

// TODO: Derive Debug
/// A callback which...
/// - ✅ is Clone
/// - ✅ is Copy
/// - ✅ is Send and Sync if T is
/// - ⚠️ requires a leptos context
pub struct CallbackArc<T: 'static, R: 'static = ()>(leptos::StoredValue<Arc<dyn Fn(T) -> R>>);

impl<T: 'static, R: 'static> CallbackArc<T, R> {
    pub fn new<F: Fn(T) -> R + 'static>(cx: leptos::Scope, fun: F) -> Self {
        Self(leptos::store_value(cx, Arc::new(fun)))
    }
}

impl<T: 'static, R: 'static> std::ops::Deref for CallbackArc<T, R> {
    type Target = leptos::StoredValue<Arc<dyn Fn(T) -> R>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: 'static, R: 'static> Clone for CallbackArc<T, R> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: 'static, R: 'static> Copy for CallbackArc<T, R> {}

impl<T: 'static, R: 'static> Callable<T> for CallbackArc<T, R> {
    fn call(&self, arg: T) {
        self.0.with_value(|cb| cb(arg));
    }
}
