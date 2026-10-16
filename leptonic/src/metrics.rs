//! Which hooks and atoms ran (feature `metrics`), for leptonic's performance tests: every public
//! hook and atom starts with [`subject!`], which notes that it ran, under the name of the
//! function it is placed in. The tests' measurements report what ran ([`ran`]: each a
//! [`Subject`], a hook or an atom), so nobody has to know (or list) what a measured thing uses
//! internally.
//!
//! Without the feature, [`subject!`] expands to nothing.

/// Notes that the enclosing hook or atom runs (feature `metrics`; nothing without it). The first
/// statement of every public hook (`pub fn use_*`) and atom (`#[component] pub fn`).
///
/// The subject is named after the function the macro is placed in, so the name can't be wrong:
/// `use_press`, or the snake-cased name of an atom (`#[component]` puts an atom's body into a
/// function `__component_<name in snake case>`: `button`, `virtual_list`).
macro_rules! subject {
    () => {
        #[cfg(feature = "metrics")]
        {
            fn here() {}
            $crate::metrics::record(::std::any::type_name_of_val(&here));
        }
    };
}
pub(crate) use subject;

/// What a [`Subject`] is.
#[cfg(feature = "metrics")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SubjectKind {
    /// A hook (`leptonic::hooks`).
    Hook,
    /// An atom (`leptonic::atoms`).
    Atom,
}

#[cfg(feature = "metrics")]
impl SubjectKind {
    /// The kind's directory in the performance tests (`atoms`, `hooks`), and the first part of
    /// its subjects' ids.
    #[must_use]
    pub const fn dir(self) -> &'static str {
        match self {
            Self::Hook => "hooks",
            Self::Atom => "atoms",
        }
    }
}

/// A hook or atom that ran.
#[cfg(feature = "metrics")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Subject {
    pub kind: SubjectKind,
    /// The hook's name (`use_press`), or the atom's in snake case (`button`, `virtual_list`).
    pub name: &'static str,
}

#[cfg(feature = "metrics")]
impl Subject {
    /// The subject's id: `<kind dir>/<name>` (`hooks/use_press`, `atoms/button`).
    #[must_use]
    pub fn id(self) -> String {
        format!("{}/{}", self.kind.dir(), self.name)
    }
}

#[cfg(feature = "metrics")]
mod registry {
    use std::cell::RefCell;

    use super::{Subject, SubjectKind};

    /// How many subjects can be noted: more than leptonic has hooks and atoms.
    const CAPACITY: usize = 1024;

    thread_local! {
        /// The paths of the functions [`subject!`](super::subject) ran in, each once. A fixed
        /// array: noting a subject allocates nothing, so that it doesn't count in measurements.
        static RAN: RefCell<[Option<&'static str>; CAPACITY]> =
            const { RefCell::new([None; CAPACITY]) };
    }

    /// Notes that the function `path::here` (a [`subject!`](super::subject)) ran.
    pub fn record(path: &'static str) {
        RAN.with_borrow_mut(|ran| {
            for slot in ran.iter_mut() {
                match slot {
                    Some(noted) if std::ptr::eq(*noted, path) || *noted == path => return,
                    Some(_) => {}
                    None => {
                        *slot = Some(path);
                        return;
                    }
                }
            }
            panic!("more than {CAPACITY} subjects ran: raise `metrics::registry::CAPACITY`");
        });
    }

    /// The subjects that ran on this thread, in the order they first ran.
    pub fn ran() -> Vec<Subject> {
        RAN.with_borrow(|ran| ran.iter().map_while(|slot| *slot).map(subject).collect())
    }

    /// The subject from the path of the function [`subject!`](super::subject) defines:
    /// `leptonic::hooks::interactions::press::use_press::here` is the hook `use_press`,
    /// `leptonic::atoms::button::__component_button::here` the atom `button`.
    fn subject(path: &'static str) -> Subject {
        let mut segments = path.rsplit("::");
        let _here = segments.next();
        let function = segments.next().unwrap_or(path);
        match function.strip_prefix("__component_") {
            Some(name) => Subject {
                kind: SubjectKind::Atom,
                name,
            },
            None => Subject {
                kind: SubjectKind::Hook,
                name: function,
            },
        }
    }

    #[cfg(test)]
    mod tests {
        use assertr::prelude::*;

        use super::*;

        #[test]
        fn subjects_are_named_after_their_function() {
            assert_that!(subject(
                "leptonic::hooks::interactions::press::use_press::here"
            ))
            .is_equal_to(Subject {
                kind: SubjectKind::Hook,
                name: "use_press",
            });
            assert_that!(subject("leptonic::atoms::button::__component_button::here")).is_equal_to(
                Subject {
                    kind: SubjectKind::Atom,
                    name: "button",
                },
            );
        }
    }
}

#[cfg(feature = "metrics")]
pub use registry::ran;
#[cfg(feature = "metrics")]
#[doc(hidden)]
pub use registry::record;
