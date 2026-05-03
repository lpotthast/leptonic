use std::{
    collections::VecDeque,
    net::TcpListener,
    path::PathBuf,
    sync::{Arc, Mutex},
    time::Duration,
};

use anyhow::{Context, Result, bail};
use tokio::process::Command;
use tokio_process_tools::{
    Inspector, LineParsingOptions, Next, Process, TerminateOnDrop, WaitForLineResult,
    broadcast::BroadcastOutputStream,
};

pub struct Frontend {
    #[expect(unused)]
    cargo_leptos_process: TerminateOnDrop<BroadcastOutputStream>,
    #[expect(unused)]
    stdout_replay: Inspector,
    #[expect(unused)]
    stderr_replay: Inspector,
    pub base_url: String,
}

const STARTUP_LOG_TAIL_LINES: usize = 200;

fn push_log_line(logs: &Arc<Mutex<VecDeque<String>>>, line: &str) {
    let mut logs = logs.lock().expect("startup log buffer mutex poisoned");
    if logs.len() == STARTUP_LOG_TAIL_LINES {
        logs.pop_front();
    }
    logs.push_back(line.to_string());
}

fn render_log_lines(logs: &Arc<Mutex<VecDeque<String>>>) -> String {
    let logs = logs.lock().expect("startup log buffer mutex poisoned");
    if logs.is_empty() {
        "<no output captured>".to_string()
    } else {
        logs.iter().cloned().collect::<Vec<_>>().join("\n")
    }
}

/// Bind a `TcpListener` to port 0 and return the OS-assigned port.
fn find_free_port() -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").expect("failed to bind to a free port");
    listener
        .local_addr()
        .expect("failed to get local address")
        .port()
}

fn frontend_dir() -> Result<PathBuf> {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../testing/test-app")
        .canonicalize()
        .context("failed to resolve leptonic test app directory")
}

pub async fn start_frontend() -> Result<Frontend> {
    let fe_dir = frontend_dir()?;

    let site_port = find_free_port();
    let reload_port = find_free_port();
    let site_addr = format!("127.0.0.1:{site_port}");
    let base_url = format!("http://{site_addr}");

    tracing::info!("Starting frontend in {fe_dir:?} on {site_addr} (reload port {reload_port})");
    let mut cmd = Command::new("cargo");
    cmd.arg("leptos")
        .arg("serve")
        .env("RUST_BACKTRACE", "1")
        .env("LEPTOS_SITE_ADDR", &site_addr)
        .env("LEPTOS_RELOAD_PORT", reload_port.to_string())
        .current_dir(fe_dir);

    let fe_process = Process::new(cmd)
        .spawn_broadcast()
        .context("failed to spawn `cargo leptos serve` for leptonic test app")?
        .terminate_on_drop(Duration::from_secs(3), Duration::from_secs(8));

    let stdout_lines = Arc::new(Mutex::new(VecDeque::with_capacity(STARTUP_LOG_TAIL_LINES)));
    let stderr_lines = Arc::new(Mutex::new(VecDeque::with_capacity(STARTUP_LOG_TAIL_LINES)));

    let stdout_lines_for_replay = Arc::clone(&stdout_lines);
    let stdout_replay = fe_process.stdout().inspect_lines(
        move |line| {
            push_log_line(&stdout_lines_for_replay, &line);
            // Only visible when tests are run with the `--nocapture` flag.
            println!("{line}");
            Next::Continue
        },
        LineParsingOptions::default(),
    );
    let stderr_lines_for_replay = Arc::clone(&stderr_lines);
    let stderr_replay = fe_process.stderr().inspect_lines(
        move |line| {
            push_log_line(&stderr_lines_for_replay, &line);
            // Only visible when tests are run with the `--nocapture` flag.
            eprintln!("{line}");
            Next::Continue
        },
        LineParsingOptions::default(),
    );

    let expected_msg = format!("listening on http://{site_addr}");
    let fe_start_timeout = Duration::from_secs(60 * 10);
    tracing::info!("Waiting {fe_start_timeout:?} for frontend to start...");
    let expected_msg_for_wait = expected_msg.clone();
    match fe_process
        .stdout()
        .wait_for_line_with_timeout(
            move |line| line.contains(&expected_msg_for_wait),
            LineParsingOptions::default(),
            fe_start_timeout,
        )
        .await
    {
        WaitForLineResult::Matched => {}
        WaitForLineResult::StreamClosed => {
            bail!(
                "leptonic test app stdout closed before startup completed. Expected stdout to contain {expected_msg:?}.\n\nRecent stdout:\n{}\n\nRecent stderr:\n{}",
                render_log_lines(&stdout_lines),
                render_log_lines(&stderr_lines),
            );
        }
        WaitForLineResult::Timeout => {
            bail!(
                "leptonic test app did not start within {fe_start_timeout:?}; expected stdout to contain {expected_msg:?}.\n\nRecent stdout:\n{}\n\nRecent stderr:\n{}",
                render_log_lines(&stdout_lines),
                render_log_lines(&stderr_lines),
            );
        }
    }

    tracing::info!("Frontend started at {base_url}!");
    Ok(Frontend {
        cargo_leptos_process: fe_process,
        stdout_replay,
        stderr_replay,
        base_url,
    })
}
