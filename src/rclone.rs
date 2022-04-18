use chrono::Utc;
use std::process::Command;
use std::process::Stdio;

use crate::config;
use crate::utils;

pub fn run_task(task: &config::Task, dryrun: bool) -> () {
    let tstart = Utc::now();
    utils::log(format!("Running task: {}", &task.name));

    let mut args = task_to_args(&task);

    if dryrun {
        args.push("--dry-run".to_string());
        utils::log(format!("Running rclone with args {}", args.join(" ")))
    }

    let mut child = Command::new("rclone")
        .args(&args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Failed to execute process");

    let output = child.wait().expect("Failed to read stdout from the task");

    let elapsed = (Utc::now() - tstart).num_seconds();

    match output.code() {
        Some(0) => utils::log(format!(
            "Tasks finished successfully in {:?} second(s).",
            elapsed
        )),
        Some(code) => utils::log(format!(
            "Task finished with errors in {:?} second(s). Error code: {}",
            elapsed, code
        )),
        None => utils::log(format!(
            "Task terminated by signal in {:?} second(s).",
            elapsed
        )),
    }
}

pub fn task_to_args(task: &config::Task) -> Vec<String> {
    let mut args = vec![
        "-v".to_string(), // Run in verbose mode.
        "--stats-log-level".to_string(),
        "NOTICE".to_string(),
        "--stats".to_string(),
        "10000m".to_string(),
        "sync".to_string(),
        format!(
            "{}{}",
            backend_to_args(&task.source.backend),
            &task.source.path
        ),
        format!(
            "{}{}",
            backend_to_args(&task.target.backend),
            &task.target.path
        ),
    ];

    for f in &task.source.filters {
        args.push("--filter".to_string());
        args.push(f.to_string());
    }

    args
}

pub fn backend_to_args(backend: &config::Backend) -> String {
    let vars: Vec<String> = match &backend.vars {
        Some(xs) => xs.iter().map(|(x, y)| format!("{}={}", x, y)).collect(),
        None => vec![],
    };

    return format!(
        ":{}{}{}:",
        backend.ctype,
        if vars.len() == 0 { "" } else { "," },
        vars.join(","),
    );
}
