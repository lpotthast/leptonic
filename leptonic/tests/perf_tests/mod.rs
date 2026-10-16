//! The performance tests of leptonic's hooks and atoms (`documentation/testing.md`, "Performance
//! measurements"): one module per subject, by kind, [`atoms`] (`atoms/<atom in snake case>.rs`:
//! `atoms/button.rs`) and [`hooks`] (`hooks/<hook>.rs`: `hooks/use_press.rs`). Each measures
//! (`Page::measure`) everything its subject offers, on the subject's fixture
//! (`/perf/<kind>/<subject>`, `testing/test-app/src/pages/perf/`), and the harness checks the
//! measurements against the subject's budget (`tests/budgets/<kind>/<subject>.json`,
//! `perf::budgets`).
//!
//! Unlike the regular tests (`ui_tests/mod.rs`), each subject's module registers its own tests
//! (`register`): with a module for each of hundreds of subjects, its list stays next to its tests.
//! The kind modules list every subject's module.

pub mod atoms;
pub mod hooks;

use browser_test::TestGroup;

/// The atoms' performance tests.
pub fn atom_tests() -> TestGroup<str> {
    atoms::register(TestGroup::new("perf_atoms"))
}

/// The hooks' performance tests.
pub fn hook_tests() -> TestGroup<str> {
    hooks::register(TestGroup::new("perf_hooks"))
}

/// The cases the performance suite times (scenarios, `PerfScenarios`): the regular suite counts
/// them, as every performance test.
pub fn timed_tests() -> TestGroup<str> {
    atoms::virtual_list::register(TestGroup::new("performance"))
}
