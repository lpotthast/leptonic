//! Budgets of the measurements (`Page::measure`) of the performance tests: every hook and atom
//! has one, `perf_tests/<kind>/<subject>.rs` (`perf_tests/atoms/button.rs`,
//! `perf_tests/hooks/use_press.rs`; the subject's id is `<kind>/<subject>`, see [`inventory`]),
//! with a test per thing it offers. What each test's measurements counted is checked in at
//! `tests/budgets/<kind>/<subject>.json` per test and measurement: the hooks and atoms that ran
//! (leptonic reports them), how often it was measured, and every budgeted metric's total (see
//! [`Metric::is_budgeted`]).
//!
//! Counts are exact, so a test's measurements must equal their budget: a count above it is a
//! regression, one below it an improvement to lock in, and both fail the test until the budget is
//! recorded again (`BROWSER_TEST_PERF_BUDGETS=update`). The coverage check
//! ([`PerformanceCoverage`]) runs after every regular run: every hook and atom has a performance
//! test with budgets, or is listed in `tests/budgets/unmeasured.txt` (which update runs keep
//! current).

use std::{
    borrow::Cow,
    collections::{BTreeMap, BTreeSet},
    fmt::Write as _,
    path::{Path, PathBuf},
    sync::Mutex,
};

use browser_test::{BrowserTest, async_trait, thirtyfour::WebDriver};
use leptonic_perf_metrics::Metric;
use rootcause::{Report, bail, prelude::ResultExt};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use super::inventory;
use crate::pages::Page;

/// One measurement of a test: its name, the value of every metric and the hooks and atoms that
/// ran on the page by its end (sorted).
#[derive(Debug, Clone)]
pub struct Measured {
    pub name: String,
    pub values: Vec<(Metric, i64)>,
    pub ran: Vec<String>,
}

/// The budget of one measurement (all measurements of its name in a test, summed).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Entry {
    /// The hooks and atoms that ran on the page by the measurements' end.
    ran: Vec<String>,
    /// How often the test measured it.
    windows: u64,
    /// The budgeted metrics' totals, by name.
    totals: Map<String, Value>,
}

/// A test's budgets, by measurement.
type TestBudgets = BTreeMap<String, Entry>;
/// A test module's budgets, by test.
type ModuleBudgets = BTreeMap<String, TestBudgets>;

/// What the performance tests' names start with; `<kind>::<subject>::<test>` follows.
const PERF_TESTS: &str = "browser_test::perf_tests::";

/// The kinds of subjects: their directories in `perf_tests/` and `tests/budgets/`.
const KINDS: [&str; 2] = ["atoms", "hooks"];

/// The file listing the hooks and atoms without measurements yet.
const UNMEASURED: &str = "unmeasured.txt";

/// Serializes the updates of budget files: tests run in parallel, and tests of one module share a
/// file.
static FILES: Mutex<()> = Mutex::new(());

fn budgets_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/budgets")
}

/// The budgets of the subject `subject` (`atoms/button`).
fn subject_file(subject: &str) -> PathBuf {
    budgets_dir().join(format!("{subject}.json"))
}

/// `BROWSER_TEST_PERF_BUDGETS`: `check` (the default) checks measurements against their budgets,
/// `update` records them as the budgets.
fn updating_all() -> Result<bool, Report> {
    match std::env::var("BROWSER_TEST_PERF_BUDGETS").as_deref() {
        Err(_) | Ok("" | "check") => Ok(false),
        Ok("update") => Ok(true),
        Ok(other) => bail!("BROWSER_TEST_PERF_BUDGETS is {other:?}, expected `check` or `update`"),
    }
}

/// `BROWSER_TEST_PERF_BUDGETS_UPDATE`: the tests whose budgets to record, by name filters
/// (comma-separated, as `BROWSER_TEST_FILTER`): one run can record some tests' budgets and check
/// the others'.
fn update_filters() -> Vec<String> {
    std::env::var("BROWSER_TEST_PERF_BUDGETS_UPDATE")
        .unwrap_or_default()
        .split(',')
        .map(str::trim)
        .filter(|filter| !filter.is_empty())
        .map(str::to_owned)
        .collect()
}

/// Whether to record the budgets of the test named `test` (instead of checking them).
fn updating(test: &str) -> Result<bool, Report> {
    Ok(updating_all()?
        || update_filters()
            .iter()
            .any(|filter| test.contains(filter.as_str())))
}

/// Whether this run records any budgets (the coverage check then updates `unmeasured.txt` and
/// removes the budgets of tests that no longer exist).
fn updating_any() -> Result<bool, Report> {
    Ok(updating_all()? || !update_filters().is_empty())
}

/// The subject (`atoms/button`) and function of the performance test named `test`
/// (`browser_test::perf_tests::atoms::button::mount_and_unmount`); `None` for other tests.
fn perf_test(test: &str) -> Option<(String, &str)> {
    let mut parts = test.strip_prefix(PERF_TESTS)?.splitn(3, "::");
    let (kind, name, function) = (parts.next()?, parts.next()?, parts.next()?);
    Some((format!("{kind}/{name}"), function))
}

/// The filter that selects the performance test of `subject` named `function`.
fn filter(subject: &str, function: &str) -> String {
    format!("perf_tests::{}::{function}", subject.replace('/', "::"))
}

fn read(subject: &str) -> Result<ModuleBudgets, Report> {
    let path = subject_file(subject);
    match std::fs::read_to_string(&path) {
        Ok(json) => Ok(serde_json::from_str(&json)
            .context_with(|| format!("failed to read the budgets {}", path.display()))?),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(ModuleBudgets::new()),
        Err(error) => {
            Err(rootcause::report!("failed to read {}: {error}", path.display()).into_dynamic())
        }
    }
}

fn write(subject: &str, budgets: &ModuleBudgets) -> Result<(), Report> {
    let path = subject_file(subject);
    if budgets.is_empty() {
        if path.exists() {
            std::fs::remove_file(&path)
                .context_with(|| format!("failed to remove {}", path.display()))?;
        }
        return Ok(());
    }
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir).context("failed to create the budgets dir")?;
    }
    std::fs::write(&path, serde_json::to_string_pretty(budgets)? + "\n")
        .context_with(|| format!("failed to write {}", path.display()))?;
    Ok(())
}

/// The measurements of one name in a test, summed.
struct Sum {
    ran: BTreeSet<String>,
    windows: u64,
    values: BTreeMap<Metric, i64>,
}

/// The budgets that `measured` amounts to.
fn aggregate(measured: &[Measured]) -> TestBudgets {
    let mut sums: BTreeMap<&str, Sum> = BTreeMap::new();
    for measurement in measured {
        let sum = sums.entry(&measurement.name).or_insert(Sum {
            ran: BTreeSet::new(),
            windows: 0,
            values: BTreeMap::new(),
        });
        sum.ran.extend(measurement.ran.iter().cloned());
        sum.windows += 1;
        for &(metric, value) in &measurement.values {
            if metric.is_budgeted() {
                *sum.values.entry(metric).or_default() += value;
            }
        }
    }
    sums.into_iter()
        .map(|(name, sum)| {
            let totals = Metric::ALL
                .into_iter()
                .filter_map(|metric| {
                    Some((metric.name().to_owned(), (*sum.values.get(&metric)?).into()))
                })
                .collect();
            (
                name.to_owned(),
                Entry {
                    ran: sum.ran.into_iter().collect(),
                    windows: sum.windows,
                    totals,
                },
            )
        })
        .collect()
}

/// How `recorded` differs from `budget`, one line per difference.
fn differences(budget: &TestBudgets, recorded: &TestBudgets) -> Vec<String> {
    let mut differences = Vec::new();
    let names: BTreeSet<&String> = budget.keys().chain(recorded.keys()).collect();
    for name in names {
        let (budget, recorded) = match (budget.get(name), recorded.get(name)) {
            (Some(budget), Some(recorded)) => (budget, recorded),
            (None, _) => {
                differences.push(format!("{name}: measured, without a budget"));
                continue;
            }
            (_, None) => {
                differences.push(format!("{name}: has a budget, but wasn't measured"));
                continue;
            }
        };
        if budget.ran != recorded.ran {
            let added: Vec<&String> = recorded
                .ran
                .iter()
                .filter(|s| !budget.ran.contains(s))
                .collect();
            let gone: Vec<&String> = budget
                .ran
                .iter()
                .filter(|s| !recorded.ran.contains(s))
                .collect();
            differences.push(format!(
                "{name}: other hooks and atoms ran (now also {added:?}, no longer {gone:?})"
            ));
        }
        if budget.windows != recorded.windows {
            differences.push(format!(
                "{name}: measured {} times, its budget {} times",
                recorded.windows, budget.windows
            ));
        }
        for metric in Metric::ALL {
            let key = metric.name();
            match (
                budget.totals.get(key).and_then(Value::as_i64),
                recorded.totals.get(key).and_then(Value::as_i64),
            ) {
                (Some(budget), Some(total)) if total > budget => differences.push(format!(
                    "{name}.{key}: {total}, {} above its budget {budget}",
                    total - budget
                )),
                (Some(budget), Some(total)) if total < budget => differences.push(format!(
                    "{name}.{key}: {total}, {} below its budget {budget} (an improvement: lock it in)",
                    budget - total
                )),
                (Some(_), None) => {
                    differences.push(format!("{name}.{key}: has a budget, but wasn't counted"));
                }
                (None, Some(total)) => {
                    differences.push(format!("{name}.{key}: {total}, without a budget"));
                }
                _ => {}
            }
        }
    }
    differences
}

/// Checks the measurements of the test named `test` against its budgets (or records them, with
/// `BROWSER_TEST_PERF_BUDGETS=update`). Only performance tests measure, and each measurement must
/// have run their subject.
pub fn check(test: &str, measured: &[Measured]) -> Result<(), Report> {
    let perf = perf_test(test);
    if !measured.is_empty() {
        let Some((subject, _)) = &perf else {
            bail!(
                "{test} measures, but only performance tests do: the performance test of a hook or \
                 atom is `perf_tests/<kind>/<subject>.rs`"
            );
        };
        if !inventory::subjects().contains(subject) {
            bail!(
                "{test} is the test of {subject:?}, which is no hook or atom (`perf::inventory`)"
            );
        }
        for measurement in measured {
            if !measurement.ran.contains(subject) {
                bail!(
                    "{test}: {subject} didn't run in the measurement {:?} (ran: {:?}); does the \
                     fixture render it, and does it start with `metrics::subject!()`?",
                    measurement.name,
                    measurement.ran
                );
            }
        }
    }
    let Some((subject, function)) = perf else {
        return Ok(());
    };
    let recorded = aggregate(measured);
    if updating(test)? {
        let _files = FILES
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let mut budgets = read(&subject)?;
        if recorded.is_empty() {
            budgets.remove(function);
        } else {
            budgets.insert(function.to_owned(), recorded);
        }
        return write(&subject, &budgets);
    }
    let budgets = read(&subject)?;
    let path = subject_file(&subject).display().to_string();
    let filter = filter(&subject, function);
    let Some(budget) = budgets.get(function) else {
        if recorded.is_empty() {
            return Ok(());
        }
        bail!(
            "{test} has no budget for its measurements ({path}): record it with \
             BROWSER_TEST_PERF_BUDGETS=update BROWSER_TEST_FILTER={filter}{}",
            summary(&recorded)
        );
    };
    let differences = differences(budget, &recorded);
    if !differences.is_empty() {
        bail!(
            "{test}'s measurements differ from their budget ({path}); if intended, record it again \
             with BROWSER_TEST_PERF_BUDGETS=update BROWSER_TEST_FILTER={filter}:\n  {}",
            differences.join("\n  ")
        );
    }
    Ok(())
}

/// The totals of `recorded`, one line per measurement.
fn summary(recorded: &TestBudgets) -> String {
    let mut summary = String::new();
    for (name, entry) in recorded {
        let totals: Vec<String> = entry
            .totals
            .iter()
            .map(|(metric, total)| format!("{metric} {total}"))
            .collect();
        let _ = write!(
            summary,
            "\n  {name} ({}x): {}",
            entry.windows,
            totals.join(", ")
        );
    }
    summary
}

/// The tests registered for this run (`harness::fixture_test`), to find budgets of tests that no
/// longer exist.
static REGISTERED: Mutex<BTreeSet<String>> = Mutex::new(BTreeSet::new());

/// Notes that the test `name` is registered.
pub fn register(name: &str) {
    if let Some((subject, function)) = perf_test(name) {
        REGISTERED
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .insert(format!("{subject}::{function}"));
    }
}

/// Runs last in every regular run, also in filtered ones: checks that every hook and atom has a
/// performance test with budgets or is listed in `tests/budgets/unmeasured.txt`, that every
/// budget belongs to a registered performance test of a hook or atom, and that every hook and
/// atom that ran is one. With `BROWSER_TEST_PERF_BUDGETS=update`, removes the budgets of tests
/// that no longer exist and writes `unmeasured.txt` instead. Problems fail only unfiltered runs
/// (a filtered run logs them): while performance tests are being written in parallel, other
/// runs' budgets change the coverage under a filtered one.
pub struct PerformanceCoverage;

#[async_trait]
impl<'page> BrowserTest<Page<'page>> for PerformanceCoverage {
    fn name(&self) -> Cow<'_, str> {
        "performance_coverage".into()
    }

    async fn run(&self, _driver: &WebDriver, _page: &Page<'page>) -> Result<(), Report> {
        coverage()
    }
}

fn coverage() -> Result<(), Report> {
    let updating = updating_any()?;
    let registered = REGISTERED
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .clone();
    let mut problems = Vec::new();
    let mut covered = BTreeSet::new();
    let mut ran = BTreeSet::new();
    let _files = FILES
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let subjects = inventory::subjects();
    for kind in KINDS {
        let dir = budgets_dir().join(kind);
        std::fs::create_dir_all(&dir).context("failed to create the budgets dir")?;
        let entries = std::fs::read_dir(&dir).context("failed to read the budgets dir")?;
        for entry in entries.flatten() {
            let path = entry.path();
            let Some(name) = path
                .file_name()
                .and_then(|name| name.to_str())
                .and_then(|name| name.strip_suffix(".json"))
            else {
                problems.push(format!("{}: no budgets file", path.display()));
                continue;
            };
            let subject = format!("{kind}/{name}");
            if !subjects.contains(&subject) {
                problems.push(format!("{subject}.json: budgets of no hook or atom"));
            }
            let mut budgets = read(&subject)?;
            let stale: Vec<String> = budgets
                .keys()
                .filter(|function| !registered.contains(&format!("{subject}::{function}")))
                .cloned()
                .collect();
            for function in &stale {
                if updating {
                    budgets.remove(function);
                } else {
                    problems.push(format!(
                        "{subject}::{function} has a budget, but no such test is registered"
                    ));
                }
            }
            if updating && !stale.is_empty() {
                write(&subject, &budgets)?;
            }
            if !budgets.is_empty() {
                covered.insert(subject.clone());
            }
            for test in budgets.values() {
                for entry in test.values() {
                    ran.extend(entry.ran.iter().cloned());
                }
            }
        }
    }
    for unknown in ran.difference(subjects) {
        problems.push(format!(
            "{unknown} reported that it ran (`metrics::subject!()`), but is no hook or atom"
        ));
    }
    let unmeasured: BTreeSet<String> = subjects.difference(&covered).cloned().collect();
    let path = budgets_dir().join(UNMEASURED);
    if updating {
        let mut text = "# The hooks and atoms without a performance test (`perf_tests/<kind>/<subject>.rs`\n\
                        # with budgets) yet, written by `BROWSER_TEST_PERF_BUDGETS=update` runs. Add\n\
                        # them: this list only shrinks.\n"
            .to_owned();
        for subject in &unmeasured {
            text.push_str(subject);
            text.push('\n');
        }
        std::fs::write(&path, text)
            .context_with(|| format!("failed to write {}", path.display()))?;
    } else {
        let listed: BTreeSet<String> = std::fs::read_to_string(&path)
            .context_with(|| format!("failed to read {}", path.display()))?
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty() && !line.starts_with('#'))
            .map(str::to_owned)
            .collect();
        for subject in unmeasured.difference(&listed) {
            problems.push(format!(
                "{subject} has no performance test: write `perf_tests/{subject}.rs`"
            ));
        }
        for subject in listed.difference(&unmeasured) {
            problems.push(if subjects.contains(subject) {
                format!("{subject} has a performance test now: remove it from {UNMEASURED}")
            } else {
                format!("{subject} is no hook or atom (any more): remove it from {UNMEASURED}")
            });
        }
    }
    tracing::info!(
        "Performance coverage: {} of {} hooks and atoms have performance tests; {} not yet ({}).",
        subjects.len() - unmeasured.len(),
        subjects.len(),
        unmeasured.len(),
        path.display()
    );
    if problems.is_empty() {
        return Ok(());
    }
    let problems = format!(
        "the performance budgets don't cover what leptonic provides (record them again with \
         BROWSER_TEST_PERF_BUDGETS=update where intended):\n  {}",
        problems.join("\n  ")
    );
    let filtered = ["BROWSER_TEST_FILTER", "BROWSER_TEST_GROUP"]
        .iter()
        .any(|variable| std::env::var(variable).is_ok_and(|value| !value.trim().is_empty()));
    if filtered {
        tracing::warn!("{problems}\n(a filtered run: not failing)");
        return Ok(());
    }
    bail!("{problems}")
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;
    use serde_json::json;

    use super::*;

    fn measured(name: &str, values: &[(Metric, i64)]) -> Measured {
        Measured {
            name: name.to_owned(),
            values: values.to_vec(),
            ran: vec!["atoms/button".to_owned(), "hooks/use_press".to_owned()],
        }
    }

    #[test]
    fn measurements_of_one_name_add_up_without_unbudgeted_metrics() {
        let budgets = aggregate(&[
            measured("press", &[(Metric::Allocations, 3), (Metric::Frames, 2)]),
            measured("press", &[(Metric::Allocations, 4), (Metric::Frames, 3)]),
        ]);
        assert_that!(budgets.get("press").cloned()).is_equal_to(Some(Entry {
            ran: vec!["atoms/button".to_owned(), "hooks/use_press".to_owned()],
            windows: 2,
            totals: serde_json::from_value(json!({"allocations": 7})).expect("totals"),
        }));
    }

    #[test]
    fn measurements_must_equal_their_budget() {
        let budget = aggregate(&[measured(
            "press",
            &[(Metric::Allocations, 100), (Metric::Layouts, 2)],
        )]);
        assert_that!(differences(&budget, &budget)).is_empty();
        let more = aggregate(&[measured(
            "press",
            &[(Metric::Allocations, 101), (Metric::Layouts, 1)],
        )]);
        assert_that!(differences(&budget, &more)).is_equal_to(vec![
            "press.allocations: 101, 1 above its budget 100".to_owned(),
            "press.layouts: 1, 1 below its budget 2 (an improvement: lock it in)".to_owned(),
        ]);
        let other = aggregate(&[measured("hover", &[(Metric::Allocations, 100)])]);
        assert_that!(differences(&budget, &other)).is_equal_to(vec![
            "hover: measured, without a budget".to_owned(),
            "press: has a budget, but wasn't measured".to_owned(),
        ]);
    }

    #[test]
    fn performance_tests_are_named_for_their_subject() {
        assert_that!(perf_test(
            "browser_test::perf_tests::atoms::button::mount_and_unmount"
        ))
        .is_equal_to(Some(("atoms/button".to_owned(), "mount_and_unmount")));
        assert_that!(perf_test("browser_test::ui_tests::test_button::pending")).is_none();
    }
}
