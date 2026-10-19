// No upstream: react-aria's hooks each have a `useEffect` for these resets.
//! Reactions of hooks to a flag becoming `true` (`is_disabled`: end the press, the hover, the
//! focus), sharing one effect when a hook composes several hooks, and created only once there is
//! something to react for (`documentation/performance.md`, R6).
//!
//! An `Effect` costs ~18 allocations at mount (~8 to create, ~10 in its first run) and ~4 per
//! run; a `Button` composing `use_press`, `use_hover` and `use_focus_ring` had one per hook, all
//! reacting to its disabled state. [`when_true`] registers a reaction: for a constant flag nothing
//! (an effect that tracked a constant would never rerun), else an effect; inside
//! [`sharing_one_effect`], the reactions of every hook called share one effect. Reactions are
//! resets ("if disabled, end the hover"), so running all of them when one flag changes is the
//! same as running the one.
//!
//! A reset that only acts on state the hook holds between events (a press in progress, a hover, a
//! focus) registers with [`when_true_once_armed`] instead: its effect is created when the hook
//! first arms it ([`Arm::arm`], when the state begins), under the hook's owner, and then stays as
//! [`when_true`]'s would. Until then the effect's runs would all have been no-ops: there was no
//! state to reset. Equivalent to [`when_true`] as long as
//! - the reset does nothing (not even notifying a signal) while the hook has no state, and
//! - the state never begins while the flag is `true` (the hooks' handlers check it): the effect's
//!   first run, a tick after arming, then resets exactly when the flag became `true` since the
//!   state began, as the earlier effect would have on that change.
//!
//! An item of a list that is never pressed, hovered or focused creates no effect.

use std::cell::RefCell;

use leptos::{prelude::*, reactive::owner::WeakOwner};

use super::probe::probe_sources;

/// A flag and what to do while it is `true`.
type Reaction = (Signal<bool>, Box<dyn Fn() + Send + Sync>);

/// Reactions waiting to be armed, and the owner their effect will belong to (weakly: the record
/// is one of its arena items, `leptos-and-dom.md`, "Ownership and Cleanup").
#[cfg_attr(
    feature = "ssr",
    allow(dead_code, reason = "nothing arms on the server")
)]
struct Pending {
    reactions: Vec<Reaction>,
    owner: Option<WeakOwner>,
}

/// Where armed reactions wait for their effect: `None` once it exists (or before the shared
/// effect's group ended, during which nothing arms).
type Record = StoredValue<Option<Pending>>;

/// The reactions collected by the outermost [`sharing_one_effect`] running.
struct Group {
    reactions: Vec<Reaction>,
    /// Whether a reaction needs the effect from the start ([`when_true`]).
    eager: bool,
    /// The record handed to the hooks registering with [`when_true_once_armed`] (created by the
    /// first of them).
    record: Option<Record>,
}

thread_local! {
    /// The group of the outermost [`sharing_one_effect`] running, if any.
    static SHARED: RefCell<Option<Group>> = const { RefCell::new(None) };
}

/// Creates the effect running `reactions` whenever one of their flags is or becomes `true`.
fn react_in_effect(reactions: Vec<Reaction>) {
    Effect::new(move |_| {
        for (flag, react) in &reactions {
            if flag.get() {
                react();
            }
        }
    });
}

/// Runs `react` after rendering (in an effect) whenever `flag` is or becomes `true`, as
/// `Effect::new(move || if flag.get() { react() })` would: nothing at all for a flag that is
/// constantly `false`. Inside [`sharing_one_effect`], it joins that effect.
pub(crate) fn when_true(flag: Signal<bool>, react: impl Fn() + Send + Sync + 'static) {
    if let (false, false) = probe_sources(|| flag.get()) {
        return;
    }
    let reaction: Reaction = (flag, Box::new(react));
    let unshared = SHARED.with_borrow_mut(|shared| match shared {
        Some(group) => {
            group.eager = true;
            group.reactions.push(reaction);
            None
        }
        None => Some(reaction),
    });
    if let Some((flag, react)) = unshared {
        // Unshared, the effect of its own (no list).
        Effect::new(move |_| {
            if flag.get() {
                react();
            }
        });
    }
}

/// Makes sure the effect of a hook's [`when_true_once_armed`] reactions exists. `Copy`, for the
/// hook's handlers; arming again does nothing.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(
    feature = "ssr",
    allow(dead_code, reason = "nothing arms on the server")
)]
pub(crate) struct Arm(Option<Record>);

#[cfg_attr(
    feature = "ssr",
    allow(dead_code, reason = "nothing arms on the server")
)]
impl Arm {
    /// Creates the effect, unless it exists, under the owner of the hook (or of the hooks
    /// sharing it). Call it when the state the reactions reset begins, while the flag is `false`.
    pub(crate) fn arm(self) {
        let Some(record) = self.0 else { return };
        let Some(Pending { reactions, owner }) = record.try_update_value(Option::take).flatten()
        else {
            return;
        };
        match owner {
            // Not if the owner is gone (its record would be too).
            Some(owner) => {
                if let Some(owner) = owner.upgrade() {
                    owner.with(|| react_in_effect(reactions));
                }
            }
            // Hooks called outside any owner: an effect of its own, as `Effect::new` there.
            None => react_in_effect(reactions),
        }
    }
}

/// Where a [`when_true_once_armed`] reaction went.
#[cfg(not(feature = "ssr"))]
enum Joined {
    /// No group: a record of its own.
    Unshared(Reaction),
    /// A group whose effect is created when it ends (nothing to arm), or which has a
    /// record already.
    Group(Arm),
    /// A group without a record yet.
    GroupWithoutRecord,
}

/// [`when_true`] for a reset that only matters while the hook holds state between events (a
/// press in progress, a hover, a focus): its effect is created on the first [`Arm::arm`], which
/// the hook calls when that state begins (see the module's documentation for the conditions).
/// Nothing on the server, where no state begins.
pub(crate) fn when_true_once_armed(
    flag: Signal<bool>,
    react: impl Fn() + Send + Sync + 'static,
) -> Arm {
    #[cfg(feature = "ssr")]
    {
        let _ = (flag, react);
        Arm(None)
    }
    #[cfg(not(feature = "ssr"))]
    {
        // A constant flag: `false` never resets, `true` lets no state begin.
        if !probe_sources(|| flag.get()).1 {
            return Arm(None);
        }
        let reaction: Reaction = (flag, Box::new(react));
        let joined = SHARED.with_borrow_mut(|shared| match shared {
            Some(group) => {
                group.reactions.push(reaction);
                match (group.eager, group.record) {
                    (true, _) => Joined::Group(Arm(None)),
                    (false, Some(record)) => Joined::Group(Arm(Some(record))),
                    (false, None) => Joined::GroupWithoutRecord,
                }
            }
            None => Joined::Unshared(reaction),
        });
        match joined {
            Joined::Unshared(reaction) => Arm(Some(StoredValue::new(Some(Pending {
                reactions: vec![reaction],
                owner: Owner::current().map(|owner| owner.downgrade()),
            })))),
            Joined::Group(arm) => arm,
            // Created outside the borrow (the arena may run code).
            Joined::GroupWithoutRecord => {
                let record: Record = StoredValue::new(None);
                SHARED.with_borrow_mut(|shared| {
                    if let Some(group) = shared {
                        group.record = Some(record);
                    }
                });
                Arm(Some(record))
            }
        }
    }
}

/// Calls `f` and lets the hooks it calls share one effect for their resets: several leptonic
/// hooks end their state when their element becomes disabled (`use_press` ends the press,
/// `use_hover` the hover, `use_focus_ring` the focus), each in an effect of its own. Called in
/// `sharing_one_effect`, they register these resets instead, and one effect runs them when a
/// disabled flag changes (~10 allocations at mount and ~4 per run saved per hook after the
/// first). The effect is created once a hook has state to reset (the first press, hover or
/// focus), or after `f` returned if a reset may apply from the start. A reset whose flag never
/// changes costs nothing either way. Nested calls join the outermost one. Leptonic's composing
/// hooks (`use_button`, `use_selectable_item`, ...) share theirs already; use it where you compose
/// several hooks for one element, as leptonic's atoms do (`GridListItem`):
///
/// ```ignore
/// let (item, hover) =
///     sharing_one_effect(|| (use_grid_list_item(item_input), use_hover(hover_input)));
/// ```
pub fn sharing_one_effect<T>(f: impl FnOnce() -> T) -> T {
    /// Ends the collection when dropped, also if `f` panics.
    struct End;
    impl Drop for End {
        fn drop(&mut self) {
            SHARED.with_borrow_mut(|shared| *shared = None);
        }
    }

    if SHARED.with_borrow(Option::is_some) {
        return f();
    }
    SHARED.with_borrow_mut(|shared| {
        *shared = Some(Group {
            reactions: Vec::new(),
            eager: false,
            record: None,
        });
    });
    let end = End;
    let value = f();
    let group = SHARED.with_borrow_mut(Option::take);
    drop(end);
    if let Some(Group {
        reactions,
        eager,
        record,
    }) = group
        && !reactions.is_empty()
    {
        match record {
            // Every reaction waits for its hook's state: the effect comes with the first.
            Some(record) if !eager => record.set_value(Some(Pending {
                reactions,
                owner: Owner::current().map(|owner| owner.downgrade()),
            })),
            // Some reaction applies from the start: the effect now (armed reactions find no
            // pending record).
            _ => react_in_effect(reactions),
        }
    }
    value
}

#[cfg(test)]
mod tests {
    use std::sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    };

    use assertr::prelude::*;

    use super::*;
    use crate::testing::{count_allocations, flush_effects, with_owner};

    fn counter() -> (Arc<AtomicUsize>, impl Fn() + Send + Sync + 'static) {
        let count = Arc::new(AtomicUsize::new(0));
        let counted = Arc::clone(&count);
        (count, move || {
            counted.fetch_add(1, Ordering::Relaxed);
        })
    }

    #[test]
    fn reacts_while_the_flag_is_true() {
        with_owner(|| {
            let flag = RwSignal::new(false);
            let (count, react) = counter();
            when_true(flag.into(), react);
            flush_effects();
            assert_that!(count.load(Ordering::Relaxed)).is_equal_to(0);
            flag.set(true);
            flush_effects();
            assert_that!(count.load(Ordering::Relaxed)).is_equal_to(1);
        });
    }

    #[test]
    fn shared_reactions_run_and_nested_ones_join_the_outer_effect() {
        with_owner(|| {
            let first = RwSignal::new(false);
            let second = RwSignal::new(false);
            let (first_count, react_first) = counter();
            let (second_count, react_second) = counter();
            let (constant_count, react_constant) = counter();
            sharing_one_effect(|| {
                when_true(first.into(), react_first);
                when_true(Signal::stored(false), react_constant);
                sharing_one_effect(|| when_true(second.into(), react_second));
            });
            flush_effects();
            second.set(true);
            flush_effects();
            assert_that!(first_count.load(Ordering::Relaxed)).is_equal_to(0);
            assert_that!(second_count.load(Ordering::Relaxed)).is_equal_to(1);
            first.set(true);
            flush_effects();
            assert_that!(first_count.load(Ordering::Relaxed)).is_equal_to(1);
            // One effect: the nested reaction runs again with the outer one.
            assert_that!(second_count.load(Ordering::Relaxed)).is_equal_to(2);
            assert_that!(constant_count.load(Ordering::Relaxed)).is_equal_to(0);
        });
    }

    /// A hook panicking inside `sharing_one_effect` (and caught) leaves no collection behind, in
    /// which later reactions would wait for an effect that is never created.
    #[test]
    fn a_panic_ends_the_shared_collection() {
        fn failing_hook() {
            when_true(RwSignal::new(false).into(), || {});
            panic!("a hook");
        }
        with_owner(|| {
            let panicked = std::panic::catch_unwind(|| sharing_one_effect(failing_hook));
            assert_that!(panicked.is_err()).is_true();
            assert_that!(SHARED.with_borrow(Option::is_none)).is_true();
        });
    }

    /// No effect until armed: changes of the flag before do nothing (there is no state to
    /// reset), and a flag that became `true` after arming (in the same tick) reacts in the
    /// effect's first run, as an effect created earlier would on that change.
    #[test]
    fn armed_reactions_react_from_the_arming_on() {
        with_owner(|| {
            let flag = RwSignal::new(false);
            let (count, react) = counter();
            let arm = when_true_once_armed(flag.into(), react);
            flush_effects();
            flag.set(true);
            flush_effects();
            flag.set(false);
            flush_effects();
            assert_that!(count.load(Ordering::Relaxed)).is_equal_to(0);

            arm.arm();
            flag.set(true);
            flush_effects();
            assert_that!(count.load(Ordering::Relaxed)).is_equal_to(1);
            // Armed again: still one effect.
            arm.arm();
            flag.set(false);
            flush_effects();
            flag.set(true);
            flush_effects();
            assert_that!(count.load(Ordering::Relaxed)).is_equal_to(2);
        });
    }

    /// A constant flag needs no effect: `false` never resets, and `true` lets no state begin
    /// (the hooks don't arm while it holds).
    #[test]
    fn a_constant_flag_arms_nothing() {
        with_owner(|| {
            let (count, react) = counter();
            let flag = Signal::stored(false);
            // The thread's first probe and flush set themselves up.
            let _ = probe_sources(|| flag.get());
            flush_effects();
            let ((), allocations) = count_allocations(|| {
                when_true_once_armed(flag, react).arm();
                flush_effects();
            });
            assert_that!(count.load(Ordering::Relaxed)).is_equal_to(0);
            assert_that!(allocations).is_equal_to(0);
        });
    }

    /// Arming one hook of a group creates the group's one effect, which runs every reaction.
    #[test]
    fn arming_a_shared_group_creates_its_one_effect() {
        with_owner(|| {
            let first = RwSignal::new(false);
            let second = RwSignal::new(false);
            let (first_count, react_first) = counter();
            let (second_count, react_second) = counter();
            let (first_arm, second_arm) = sharing_one_effect(|| {
                (
                    when_true_once_armed(first.into(), react_first),
                    sharing_one_effect(|| when_true_once_armed(second.into(), react_second)),
                )
            });
            second.set(true);
            flush_effects();
            assert_that!(second_count.load(Ordering::Relaxed)).is_equal_to(0);

            first_arm.arm();
            flush_effects();
            // The effect's first run: the second flag is `true`.
            assert_that!(second_count.load(Ordering::Relaxed)).is_equal_to(1);
            second_arm.arm();
            first.set(true);
            flush_effects();
            assert_that!(first_count.load(Ordering::Relaxed)).is_equal_to(1);
            assert_that!(second_count.load(Ordering::Relaxed)).is_equal_to(2);
        });
    }

    /// With a reaction that applies from the start (`when_true`), the group's effect exists after
    /// the group, and arming adds none.
    #[test]
    fn an_eager_reaction_creates_the_shared_effect_at_once() {
        with_owner(|| {
            let eager = RwSignal::new(false);
            let armed = RwSignal::new(false);
            let (eager_count, react_eager) = counter();
            let (armed_count, react_armed) = counter();
            let arm = sharing_one_effect(|| {
                let arm = when_true_once_armed(armed.into(), react_armed);
                when_true(eager.into(), react_eager);
                arm
            });
            armed.set(true);
            flush_effects();
            assert_that!(armed_count.load(Ordering::Relaxed)).is_equal_to(1);
            arm.arm();
            eager.set(true);
            flush_effects();
            assert_that!(eager_count.load(Ordering::Relaxed)).is_equal_to(1);
            // One effect: both ran once more.
            assert_that!(armed_count.load(Ordering::Relaxed)).is_equal_to(2);
        });
    }

    /// Armed from elsewhere (an event handler runs under another owner), the effect belongs to
    /// the hook's owner and ends with it; arming after the owner is gone does nothing.
    #[test]
    fn the_effect_belongs_to_the_hooks_owner() {
        with_owner(|| {
            let flag = RwSignal::new(false);
            let (count, react) = counter();
            let item = Owner::current().expect("owner").child();
            let arm = item.with(|| sharing_one_effect(|| when_true_once_armed(flag.into(), react)));
            arm.arm();
            flush_effects();
            item.cleanup();
            flag.set(true);
            flush_effects();
            assert_that!(count.load(Ordering::Relaxed)).is_equal_to(0);

            let (later_count, react_later) = counter();
            let gone = Owner::current().expect("owner").child();
            let late_arm = gone.with(|| when_true_once_armed(flag.into(), react_later));
            gone.cleanup();
            late_arm.arm();
            flush_effects();
            assert_that!(later_count.load(Ordering::Relaxed)).is_equal_to(0);
        });
    }

    /// Until armed, a group costs its record instead of an effect (`performance.md` R6).
    #[test]
    fn an_unarmed_group_costs_no_effect() {
        with_owner(|| {
            let flag = RwSignal::new(false);
            let ((), eager) = count_allocations(|| {
                sharing_one_effect(|| {
                    when_true(flag.into(), || {});
                    when_true(flag.into(), || {});
                });
                flush_effects();
            });
            let (arm, armed) = count_allocations(|| {
                let arm = sharing_one_effect(|| {
                    when_true_once_armed(flag.into(), || {});
                    when_true_once_armed(flag.into(), || {})
                });
                flush_effects();
                arm
            });
            let ((), arming) = count_allocations(|| {
                arm.arm();
                flush_effects();
            });
            // Measured: 24 (the flags' signals 4, the list 1-2, the effect ~18), 8, 10.
            assert_that!(armed + 14).is_less_than(eager);
            assert_that!(arming).is_less_than(eager);
        });
    }
}
