use leptos_reactive::{MaybeSignal, Signal, SignalGet};

pub trait MaybeSignalExt<T: 'static> {
    fn map<U: 'static, F: Fn(T) -> U + 'static>(self, mapper: F) -> MaybeSignal<U>;
}

impl<T: Clone + 'static> MaybeSignalExt<T> for MaybeSignal<T> {
    fn map<U: 'static, F: Fn(T) -> U + 'static>(self, mapper: F) -> MaybeSignal<U> {
        match self {
            Self::Static(v) => MaybeSignal::Static(mapper(v)),
            Self::Dynamic(sig) => MaybeSignal::Dynamic(Signal::derive(move || mapper(sig.get()))),
        }
    }
}

pub trait SignalExt<T: 'static> {
    fn map<U: 'static, F: Fn(T) -> U + 'static>(self, mapper: F) -> Signal<U>;
}

impl<T: Clone + 'static> SignalExt<T> for Signal<T> {
    fn map<U: 'static, F: Fn(T) -> U + 'static>(self, mapper: F) -> Signal<U> {
        Signal::derive(move || mapper(self.get()))
    }
}
