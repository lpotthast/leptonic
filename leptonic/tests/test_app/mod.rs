use std::{env, net::TcpListener, time::Duration};

use tokio::process::Command;
use tokio_process_tools::{
    broadcast::BroadcastOutputStream, Inspector, LineParsingOptions, Next, Process, TerminateOnDrop,
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

/// Bind a `TcpListener` to port 0 and return the OS-assigned port.
fn find_free_port() -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").expect("failed to bind to a free port");
    listener
        .local_addr()
        .expect("failed to get local address")
        .port()
}

pub async fn start_frontend() -> Frontend {
    let fe_dir = env::current_dir()
        .unwrap()
        .join("../testing/test-app")
        .canonicalize()
        .unwrap();

    let site_port = find_free_port();
    let reload_port = find_free_port();
    let site_addr = format!("127.0.0.1:{site_port}");
    let base_url = format!("http://{site_addr}");

    tracing::info!("Starting frontend in {fe_dir:?} on {site_addr} (reload port {reload_port})");
    let mut cmd = Command::new("cargo");
    cmd.arg("leptos")
        .arg("watch")
        .env("RUST_BACKTRACE", "1")
        .env("LEPTOS_SITE_ADDR", &site_addr)
        .env("LEPTOS_RELOAD_PORT", reload_port.to_string())
        .current_dir(fe_dir);

    let fe_process = Process::new(cmd)
        .spawn_broadcast()
        .unwrap()
        .terminate_on_drop(Duration::from_secs(3), Duration::from_secs(8));

    let stdout_replay = fe_process.stdout().inspect_lines(
        |line| {
            // Only visible when tests are run with the `--nocapture` flag.
            println!("{line}");
            Next::Continue
        },
        LineParsingOptions::default(),
    );
    let stderr_replay = fe_process.stderr().inspect_lines(
        |line| {
            // Only visible when tests are run with the `--nocapture` flag.
            eprintln!("{line}");
            Next::Continue
        },
        LineParsingOptions::default(),
    );

    let expected_msg = format!("listening on http://{site_addr}");
    let fe_start_timeout = Duration::from_secs(60 * 10);
    tracing::info!("Waiting {fe_start_timeout:?} for frontend to start...");
    match fe_process
        .stdout()
        .wait_for_line_with_timeout(
            move |line| line.contains(&expected_msg),
            LineParsingOptions::default(),
            fe_start_timeout,
        )
        .await
    {
        Ok(_wait_for) => {}
        Err(_elapsed) => {
            tracing::error!(
                "Frontend failed to start in {fe_start_timeout:?}. Expected to see 'listening on http://{site_addr}' on stdout. Compilation might not be ready yet. A restart might work as it will pick up the previously done compilation work."
            );
        }
    }

    tracing::info!("Frontend started at {base_url}!");
    Frontend {
        cargo_leptos_process: fe_process,
        stdout_replay,
        stderr_replay,
        base_url,
    }
}
