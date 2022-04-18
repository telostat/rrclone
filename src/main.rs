use std::env;

mod config;
mod rclone;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();
    let dryrun = args.len() == 3 && &args[2] == "--dry-run";

    match config::read_config(&args[1]) {
        Ok(config) => run(&config, dryrun),
        Err(err) => utils::log(format!(
            "Something went wrong while reading configuration file. Error was: {}",
            err
        )),
    }
}

fn run(config: &config::Config, dryrun: bool) -> () {
    for task in &config.tasks {
        rclone::run_task(&task, dryrun)
    }
}
