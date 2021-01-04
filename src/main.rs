use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::process::Command;
use std::process::Stdio;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Task {
    source_remote: String,
    source_path: String,
    target_remote: String,
    target_path: String,
    filters: Vec<String>,
}

fn main() -> Result<(), serde_yaml::Error> {
    let args: Vec<String> = env::args().collect();
    let dryrun = args.len() == 3 && &args[2] == "--dry-run";

    let contents =
        fs::read_to_string(&args[1]).expect("Something went wrong while reading the YAML file");

    let result: Result<Vec<Task>, _> = serde_yaml::from_str(&contents);

    match result {
        Ok(tasks) => run_tasks(&tasks, dryrun),
        Err(x) => log(format!("Something went wrong while decoding the YAML file {:?}:", x)),
    }

    Ok(())
}

fn log(msg: String) -> () {
    eprintln!(
        "RRCLONE>> [{}] {}",
        Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        msg
    );
}

fn run_tasks(ts: &Vec<Task>, dryrun: bool) -> () {
    for t in ts {
        run_task(t, dryrun)
    }
}

fn run_task(t: &Task, dryrun: bool) -> () {
    let tstart = Utc::now();
    let source = format!("{}:{}", t.source_remote, t.source_path);
    let target = format!("{}:{}", t.target_remote, t.target_path);

    log(format!("Syncing from \"{}\" to \"{}\"", &source, &target));

    let mut args = vec![
        "-v",
        "--stats-log-level",
        "NOTICE",
        "--stats",
        "10000m",
        "sync",
        &source,
        &target,
    ];

    if dryrun {
        args.push("--dry-run");
    }

    for f in &t.filters {
        args.push("--filter");
        args.push(f);
    }

    let mut child = Command::new("rclone")
        .args(&args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Failed to execute process");

    let output = child.wait().expect("Failed to read stdout from the task");

    let elapsed = Utc::now() - tstart;

    match output.code() {
        Some(0) => log(format!("Tasks finished successfully in {:?} second(s).", elapsed.num_seconds())),
        Some(code) => log(format!("Task finished with errors in {:?} second(s). Error code: {}", elapsed.num_seconds(), code)),
        None => log(format!("Task terminated by signal in {:?} second(s).", elapsed.num_seconds())),
    }
}
