#![cfg_attr(feature = "cargo-clippy", deny(warnings))]

#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

mod util;

use std::process;
use util::{init_conf, lock_file, print_run_status, FileConf, Result};

fn run(_file_conf: &FileConf) -> Result<()> {
    // code to run here
    Ok(())
}

fn main() {
    let file_conf = init_conf();

    if let Err(ref e) = file_conf {
        eprintln!(
            "ERROR: {}\n > BACKTRACE: {}",
            e.cause(),
            e.backtrace()
        );
    }

    let res = file_conf
        .and_then(|file_conf| {
            debug!("```\n{:#?}```", file_conf);
            lock_file(&file_conf.lock_file_path).map(|flock| (file_conf, flock))
        })
        .and_then(|(file_conf, _)| run(&file_conf));

    print_run_status(&res);

    if res.is_err() {
        process::exit(1);
    }
}
