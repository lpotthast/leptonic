//! Timings of the browser tests, kept per machine (`documentation/testing.md`, "Timing runs").
//!
//! Timings depend on the machine and its load, so they are stored and compared per machine only:
//! `tests/timings/<machine>/` holds the timed run's test durations (`tests.json`, written by
//! [`TimingRecorder`] after `just browser-test-timed`) and the performance suite's scenario reports
//! (`scenarios/`, `crate::perf::report`). The machine is identified by its hardware ([`Machine`]),
//! so every developer's machine has its own files, and a new CPU starts a new history.

use std::{
    collections::BTreeMap,
    fmt::Write as _,
    path::PathBuf,
    process::Command,
    sync::OnceLock,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use browser_test::{BrowserTestRunReport, RunReportConsumer, TestOutcome};
use rootcause::{Report, prelude::ResultExt};
use serde::{Deserialize, Serialize};

/// `BROWSER_TEST_TIMINGS`: `1` records the durations of the run's tests ([`TimingRecorder`]),
/// unset or `0` (the default) doesn't.
pub fn enabled() -> Result<bool, Report> {
    match std::env::var("BROWSER_TEST_TIMINGS").as_deref() {
        Err(_) | Ok("" | "0") => Ok(false),
        Ok("1") => Ok(true),
        Ok(other) => Err(rootcause::report!(
            "BROWSER_TEST_TIMINGS is {other:?}, expected `1` or `0`"
        )
        .into_dynamic()),
    }
}

/// The hardware the tests run on: what timings depend on besides the code. Its [`id`](Self::id)
/// names the directory of its timings.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Machine {
    /// Derived from the other fields (or `BROWSER_TEST_MACHINE`), e.g.
    /// `linux-x86-64-amd-ryzen-9-7950x-16-core-processor-32-cpus-64-gib`.
    pub id: String,
    pub os: String,
    pub arch: String,
    pub cpu: String,
    pub logical_cpus: usize,
    pub memory_gib: u64,
}

/// This machine, read once. `BROWSER_TEST_MACHINE` overrides the derived id (e.g. on a system
/// whose hardware can't be read).
pub fn machine() -> Result<&'static Machine, Report> {
    static MACHINE: OnceLock<Machine> = OnceLock::new();
    if let Some(machine) = MACHINE.get() {
        return Ok(machine);
    }
    let machine = read_machine()?;
    Ok(MACHINE.get_or_init(|| machine))
}

/// The directory of this machine's timings.
pub fn machine_dir() -> Result<PathBuf, Report> {
    Ok(PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/timings")).join(&machine()?.id))
}

fn read_machine() -> Result<Machine, Report> {
    let os = std::env::consts::OS.to_owned();
    let arch = std::env::consts::ARCH.to_owned();
    let (cpu, logical_cpus, memory_bytes) = match os.as_str() {
        "linux" => {
            let cpuinfo =
                std::fs::read_to_string("/proc/cpuinfo").context("failed to read /proc/cpuinfo")?;
            let cpu = cpuinfo
                .lines()
                .find_map(|line| line.strip_prefix("model name")?.split_once(':'))
                .map(|(_, name)| name.trim().to_owned())
                .unwrap_or_default();
            let logical_cpus = cpuinfo
                .lines()
                .filter(|line| line.starts_with("processor"))
                .count();
            let meminfo =
                std::fs::read_to_string("/proc/meminfo").context("failed to read /proc/meminfo")?;
            let memory_kib: u64 = meminfo
                .lines()
                .find_map(|line| line.strip_prefix("MemTotal:"))
                .and_then(|value| value.trim().trim_end_matches("kB").trim().parse().ok())
                .unwrap_or_default();
            (cpu, logical_cpus, memory_kib * 1024)
        }
        "macos" => (
            sysctl("machdep.cpu.brand_string")?,
            sysctl("hw.logicalcpu")?.parse().unwrap_or_default(),
            sysctl("hw.memsize")?.parse().unwrap_or_default(),
        ),
        _ => (String::new(), 0, 0),
    };
    // Rounded: the kernel reserves a little, differently from boot to boot.
    let memory_gib = (memory_bytes + (1 << 29)) >> 30;
    let id = match std::env::var("BROWSER_TEST_MACHINE") {
        Ok(id) if !id.is_empty() => slug(&id),
        _ if cpu.is_empty() || logical_cpus == 0 => {
            return Err(rootcause::report!(
                "could not read this machine's CPU on {os}: set BROWSER_TEST_MACHINE to name it"
            )
            .into_dynamic());
        }
        _ => slug(&format!(
            "{os} {arch} {cpu} {logical_cpus} cpus {memory_gib} gib"
        )),
    };
    Ok(Machine {
        id,
        os,
        arch,
        cpu,
        logical_cpus,
        memory_gib,
    })
}

fn sysctl(name: &str) -> Result<String, Report> {
    let output = Command::new("sysctl")
        .args(["-n", name])
        .output()
        .context_with(|| format!("failed to run `sysctl -n {name}`"))?;
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

/// Lowercase ASCII letters and digits, other runs of characters as one `-`.
fn slug(text: &str) -> String {
    let mut slug = String::with_capacity(text.len());
    for c in text.chars() {
        if c.is_ascii_alphanumeric() {
            slug.push(c.to_ascii_lowercase());
        } else if !slug.is_empty() && !slug.ends_with('-') {
            slug.push('-');
        }
    }
    slug.trim_end_matches('-').to_owned()
}

/// The timings of one machine (`tests.json`).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct Timings {
    machine: Option<Machine>,
    /// The latest timed run.
    last_run: Option<RunSummary>,
    /// Every test's duration from the latest timed run it passed in: a run of some tests (a
    /// filter) replaces only theirs.
    tests: BTreeMap<String, TestTiming>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RunSummary {
    /// The code: `git describe --always --dirty` of the checkout.
    revision: String,
    /// When the run ended (UTC).
    date: String,
    /// `BROWSER_TEST_FILTER` and `BROWSER_TEST_GROUP`, if set.
    selection: Option<String>,
    passed: usize,
    failed: usize,
    /// Wall time of the whole run, browser startup included.
    total_ms: f64,
    /// Every step kind (`page_load`, `measure`, ...) of the passed tests: count and summed time.
    steps: BTreeMap<String, StepTiming>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
struct StepTiming {
    count: u32,
    total_ms: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TestTiming {
    /// The test's body (its session ready).
    body_ms: f64,
    revision: String,
}

/// Records the durations of a sequential run's passed tests in this machine's `tests.json`,
/// after logging how they compare with the previous timed run.
pub struct TimingRecorder;

impl RunReportConsumer for TimingRecorder {
    fn consume(&self, report: &BrowserTestRunReport) {
        if let Err(error) = record(report) {
            tracing::error!("Failed to record the test timings: {error:?}");
        }
    }
}

fn record(report: &BrowserTestRunReport) -> Result<(), Report> {
    let machine = machine()?;
    let path = machine_dir()?.join("tests.json");
    let previous: Timings = match std::fs::read_to_string(&path) {
        Ok(json) => serde_json::from_str(&json)
            .context_with(|| format!("failed to parse {}", path.display()))?,
        Err(_) => Timings::default(),
    };
    let revision = revision();
    let passed: Vec<_> = report
        .tests
        .iter()
        .filter(|test| test.outcome == TestOutcome::Passed)
        .filter_map(|test| Some((test.name.as_str(), test.body?, &test.steps)))
        .collect();
    let mut steps: BTreeMap<String, StepTiming> = BTreeMap::new();
    for (_, _, test_steps) in &passed {
        for (kind, stats) in *test_steps {
            let step = steps.entry(kind.clone()).or_insert(StepTiming {
                count: 0,
                total_ms: 0.0,
            });
            step.count += stats.count;
            step.total_ms += millis(stats.total);
        }
    }
    let run = RunSummary {
        revision: revision.clone(),
        date: utc_now(),
        selection: selection(),
        passed: passed.len(),
        failed: report
            .tests
            .iter()
            .filter(|t| t.outcome.is_failed())
            .count(),
        total_ms: millis(report.total),
        steps,
    };
    let current: BTreeMap<&str, f64> = passed
        .iter()
        .map(|(name, body, _)| (*name, millis(*body)))
        .collect();
    tracing::info!(
        "{}",
        comparison(machine, &previous, &run, &current, Thresholds::DEFAULT)
    );

    let mut timings = previous;
    timings.machine = Some(machine.clone());
    for (name, body_ms) in &current {
        timings.tests.insert(
            (*name).to_owned(),
            TestTiming {
                body_ms: round(*body_ms),
                revision: revision.clone(),
            },
        );
    }
    timings.last_run = Some(run);
    std::fs::create_dir_all(machine_dir()?).context("failed to create the timings dir")?;
    std::fs::write(&path, serde_json::to_string_pretty(&timings)? + "\n")
        .context_with(|| format!("failed to write {}", path.display()))?;
    tracing::info!(
        "Recorded the timings of {} tests in {}.",
        current.len(),
        path.display()
    );
    Ok(())
}

/// Changes smaller than both are noise in a sequential run on a quiet machine.
#[derive(Debug, Clone, Copy)]
struct Thresholds {
    min_ms: f64,
    min_ratio: f64,
}

impl Thresholds {
    const DEFAULT: Self = Self {
        min_ms: 20.0,
        min_ratio: 0.10,
    };
}

/// How this run's tests compare with the previous timings of the same tests: the sum, each step
/// kind's total (for runs of the same selection), and the tests that changed beyond `thresholds`.
fn comparison(
    machine: &Machine,
    previous: &Timings,
    run: &RunSummary,
    current: &BTreeMap<&str, f64>,
    thresholds: Thresholds,
) -> String {
    let mut text = format!(
        "Test timings on {} ({}, {} CPUs, {} GiB), revision {}: {} passed, {} failed, {} in all.",
        machine.id,
        machine.cpu,
        machine.logical_cpus,
        machine.memory_gib,
        run.revision,
        run.passed,
        run.failed,
        duration_text(run.total_ms),
    );
    let pairs: Vec<(&str, f64, f64)> = current
        .iter()
        .filter_map(|(name, now)| Some((*name, previous.tests.get(*name)?.body_ms, *now)))
        .collect();
    if pairs.is_empty() {
        text.push_str(
            "\n  No earlier timings of these tests on this machine: this run is the baseline.",
        );
        return text;
    }
    let before: f64 = pairs.iter().map(|(_, before, _)| before).sum();
    let now: f64 = pairs.iter().map(|(_, _, now)| now).sum();
    let _ = write!(
        text,
        "\n  {} tests timed before: their bodies {} -> {} ({})",
        pairs.len(),
        duration_text(before),
        duration_text(now),
        change_text(before, now),
    );
    if let Some(last) = previous
        .last_run
        .as_ref()
        .filter(|last| last.selection == run.selection)
    {
        let _ = write!(
            text,
            "\n  vs. the previous run ({}, {}): the whole run {} -> {} ({})",
            last.revision,
            last.date,
            duration_text(last.total_ms),
            duration_text(run.total_ms),
            change_text(last.total_ms, run.total_ms),
        );
        for (kind, step) in &run.steps {
            if let Some(before) = last.steps.get(kind) {
                let _ = write!(
                    text,
                    "\n    {kind:<24} {:>6}x {:>10} -> {:>10} ({})",
                    step.count,
                    duration_text(before.total_ms),
                    duration_text(step.total_ms),
                    change_text(before.total_ms, step.total_ms),
                );
            }
        }
    }
    let mut changed: Vec<&(&str, f64, f64)> = pairs
        .iter()
        .filter(|(_, before, now)| {
            let change = (now - before).abs();
            change >= thresholds.min_ms && change >= before * thresholds.min_ratio
        })
        .collect();
    changed.sort_by(|a, b| (b.2 - b.1).abs().total_cmp(&(a.2 - a.1).abs()));
    for (title, slower) in [("slower", true), ("faster", false)] {
        let tests: Vec<_> = changed
            .iter()
            .filter(|(_, before, now)| (now > before) == slower)
            .take(15)
            .collect();
        if tests.is_empty() {
            continue;
        }
        let count = changed
            .iter()
            .filter(|(_, before, now)| (now > before) == slower)
            .count();
        let _ = write!(
            text,
            "\n  {count} {title} by at least {} ms and {:.0}% (the {} largest changes):",
            thresholds.min_ms,
            thresholds.min_ratio * 100.0,
            tests.len()
        );
        for (name, before, now) in tests {
            let _ = write!(
                text,
                "\n    {:>9} -> {:>9} ({})  {name}",
                duration_text(*before),
                duration_text(*now),
                change_text(*before, *now),
            );
        }
    }
    text
}

/// The filter variables of the run, as `BROWSER_TEST_FILTER=... BROWSER_TEST_GROUP=...`.
fn selection() -> Option<String> {
    let parts: Vec<String> = ["BROWSER_TEST_FILTER", "BROWSER_TEST_GROUP"]
        .into_iter()
        .filter_map(|name| {
            let value = std::env::var(name).ok().filter(|value| !value.is_empty())?;
            Some(format!("{name}={value}"))
        })
        .collect();
    (!parts.is_empty()).then(|| parts.join(" "))
}

fn revision() -> String {
    Command::new("git")
        .args(["describe", "--always", "--dirty"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .ok()
        .filter(|output| output.status.success())
        .map_or_else(
            || "unknown".to_owned(),
            |output| String::from_utf8_lossy(&output.stdout).trim().to_owned(),
        )
}

/// The current UTC date and time, `YYYY-MM-DD HH:MM`.
fn utc_now() -> String {
    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |since| since.as_secs());
    let (days, time) = (seconds / 86_400, seconds % 86_400);
    let (year, month, day) = civil_date(i64::try_from(days).unwrap_or_default());
    format!(
        "{year:04}-{month:02}-{day:02} {:02}:{:02}",
        time / 3600,
        time % 3600 / 60
    )
}

/// The proleptic Gregorian date `days` after 1970-01-01 (Howard Hinnant's `civil_from_days`).
fn civil_date(days: i64) -> (i64, u32, u32) {
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let day_of_era = z.rem_euclid(146_097);
    let year_of_era =
        (day_of_era - day_of_era / 1460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let shifted_month = (5 * day_of_year + 2) / 153;
    let day = u32::try_from(day_of_year - (153 * shifted_month + 2) / 5 + 1).unwrap_or(1);
    let month = u32::try_from(if shifted_month < 10 {
        shifted_month + 3
    } else {
        shifted_month - 9
    })
    .unwrap_or(1);
    let year = year_of_era + era * 400 + i64::from(month <= 2);
    (year, month, day)
}

fn millis(duration: Duration) -> f64 {
    duration.as_secs_f64() * 1000.0
}

/// To a tenth of a millisecond: finer digits are noise.
fn round(ms: f64) -> f64 {
    (ms * 10.0).round() / 10.0
}

fn duration_text(ms: f64) -> String {
    if ms >= 60_000.0 {
        let minutes = (ms / 60_000.0).floor();
        format!("{minutes:.0}m {:04.1}s", (ms - minutes * 60_000.0) / 1000.0)
    } else if ms >= 1000.0 {
        format!("{:.2}s", ms / 1000.0)
    } else {
        format!("{ms:.1}ms")
    }
}

fn change_text(before: f64, now: f64) -> String {
    if before > 0.0 {
        format!("{:+.1}%", (now / before - 1.0) * 100.0)
    } else {
        format!("{:+.1}ms", now - before)
    }
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;

    #[test]
    fn machine_ids_are_slugs() {
        assert_that!(slug(
            "linux x86_64 AMD Ryzen 9 7950X 16-Core Processor 32 cpus 64 gib"
        ))
        .is_equal_to("linux-x86-64-amd-ryzen-9-7950x-16-core-processor-32-cpus-64-gib");
        assert_that!(slug("  Apple M2 (Pro)  ")).is_equal_to("apple-m2-pro");
    }

    #[test]
    fn dates_are_civil_utc_dates() {
        assert_that!(civil_date(0)).is_equal_to((1970, 1, 1));
        assert_that!(civil_date(20_379)).is_equal_to((2025, 10, 18));
        assert_that!(civil_date(11_016)).is_equal_to((2000, 2, 29));
    }

    fn machine() -> Machine {
        Machine {
            id: "test-machine".to_owned(),
            os: "linux".to_owned(),
            arch: "x86_64".to_owned(),
            cpu: "CPU".to_owned(),
            logical_cpus: 4,
            memory_gib: 8,
        }
    }

    fn run(total_ms: f64) -> RunSummary {
        RunSummary {
            revision: "abc".to_owned(),
            date: "2026-10-18 12:00".to_owned(),
            selection: None,
            passed: 3,
            failed: 0,
            total_ms,
            steps: BTreeMap::new(),
        }
    }

    fn timing(body_ms: f64) -> TestTiming {
        TestTiming {
            body_ms,
            revision: "abc".to_owned(),
        }
    }

    #[test]
    fn a_first_run_is_the_baseline() {
        let current = BTreeMap::from([("a", 100.0)]);
        let text = comparison(
            &machine(),
            &Timings::default(),
            &run(1000.0),
            &current,
            Thresholds::DEFAULT,
        );
        assert_that!(text).contains("this run is the baseline");
    }

    #[test]
    fn only_changes_beyond_both_thresholds_are_listed() {
        let previous = Timings {
            machine: None,
            last_run: Some(run(2000.0)),
            tests: BTreeMap::from([
                ("slower".to_owned(), timing(100.0)),
                ("noise".to_owned(), timing(1000.0)),
                ("faster".to_owned(), timing(500.0)),
            ]),
        };
        // +50% (+50 ms), +1.5% (+15 ms), -40% (-200 ms).
        let current = BTreeMap::from([("slower", 150.0), ("noise", 1015.0), ("faster", 300.0)]);
        let text = comparison(
            &machine(),
            &previous,
            &run(1900.0),
            &current,
            Thresholds::DEFAULT,
        );
        assert_that!(text.as_str()).contains("1 slower");
        assert_that!(text.as_str()).contains("1 faster");
        assert_that!(text.as_str()).does_not_contain("noise");
        assert_that!(text.as_str()).contains("2.00s -> 1.90s (-5.0%)");
    }
}
