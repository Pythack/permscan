#![allow(dead_code)]

use structopt::StructOpt;
use std::path::Path;
use types::Result;

#[path = "./colors.rs"]
mod colors;
#[path = "./get_results.rs"]
mod get_results;
#[path = "./ls.rs"]
mod ls;
#[path = "./misc.rs"]
mod misc;
#[path = "./opt.rs"]
mod opt;
#[path = "./output.rs"]
mod output;
#[path = "./types.rs"]
mod types;
#[path = "./updates.rs"]
mod updates;

use crate::opt::Opt;
use types::PermscanOutput;

/// Exit codes
mod exit {
    pub const SUCCESS: i32 = 0;
    pub const ERR: i32 = 1;
}

fn main() {
    let opt = Opt::from_args();
    let exit_code = permscan(opt);
    std::process::exit(exit_code)
}

// The true main function. Returns an exit code
fn permscan(opt: Opt) -> i32 {
    // Run the checks_for_newer_version function if the update flag is raised.
    // Returns exit code when done
    if opt.check_update {
        if updates::check_for_newer_version(&opt.build).is_err() {
            return exit::ERR
        }
        return exit::SUCCESS;
    }

    // Check if the path entered by the user exists
    if check_path_exists(&opt.path).is_err() {
        return exit::ERR;
    }
    let files = ls::run_ls(&opt.path, &opt.all, &opt.recursive);

    // if the item_type argument is present, check
    // wether or not it is a valid type
    if opt.item_type != None
        && misc::verify_type_argument(opt.item_type.as_ref().unwrap()).is_err()
    {
        return exit::ERR;
    }

    // exit if we got an error while running ls
    if files.is_err() {
        return exit::ERR;
    }

    let files_unwrapped = files.unwrap();
    let results = get_results::get_results(&opt, &files_unwrapped);

    match output::print_results(results, &opt.recursive) {
        Ok(()) => exit::SUCCESS,
        Err(_) => {
            eprintln!(
                "{}permscan: stdout: failed to print results{}",
                colors::RED,
                colors::RESET
            );
            exit::ERR
        }
    }
}

// Checks if the path entered by the user exists and return
// an error if it doesn't
pub fn check_path_exists(path: &str) -> Result<()> {
    let path_exists = Path::new(&path).exists();
    if !path_exists {
        eprintln!(
            "{}permscan: {}: No such file or directory\x1b[0m{}",
            colors::RED,
            &path,
            colors::RESET
        );
        return Err("".into());
    }
    Ok(())
}
