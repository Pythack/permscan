#![allow(dead_code)]

use structopt::StructOpt;

#[path = "./opt.rs"]
mod arguments;
#[path = "./get_results.rs"]
mod get_results;
#[path = "./ls.rs"]
mod ls;
#[path = "./misc.rs"]
mod misc;
#[path = "./print_results.rs"]
mod print_results;
#[path = "./types.rs"]
mod types;
#[path = "./updates.rs"]
mod updates;

use crate::arguments::Opt;
use types::PermscanOutput;

mod exit {

    // exit code when permscan runs without problems
    pub const SUCCESS: i32 = 0;

    // exit code when update failed
    pub const UPDATE_ERR: i32 = 1;

    // exit code when the path entered by the user doesn't exist
    pub const PATH_ERR: i32 = 2;

    // exit code when failing to run ls to get files
    pub const LS_ERR: i32 = 3;

    // exit code when failing to parse github api response
    pub const PARSING_ERR: i32 = 4;

    // exit code when an IO error occurs
    pub const IO_ERR: i32 = 5;

    // exit code when connection to the github api failed
    // (while checking for updates)
    pub const CONNECTION_ERR: i32 = 60;

    // exit code for unknown error
    pub const UNKNOWN_ERR: i32 = -1;
}

fn main() {
    let exit_code;
    // this scope ensures all destructors are ran
    // before using std::process::exit
    {
        let opt = Opt::from_args();
        let exit_info: bool = opt.exit_info;
        exit_code = permscan(opt);
        if exit_info {
            print_exit_info(exit_code)
        }
    }
    std::process::exit(exit_code)
}

fn print_exit_info(exit_code: i32) {
    if exit_code != 0 {
        println!("\x1b[91mpermscan: process exited with exit code {}. to know more about error codes, visit https://github.com/Pythack/permscan/wiki/Error-codes\x1b[0m", exit_code)
    } else {
        eprintln!(
            "\x1b[92mpermscan: process successfully exited with exit code 0\x1b[0m"
        )
    }
}

// The true main function. Returns an exit code
fn permscan(opt: Opt) -> i32 {
    // Run the checks_for_newer_version function if the update flag is raised.
    // Returns exit code when done
    if opt.check_update {
        if let Err(e) = updates::check_for_newer_version(opt.build) {
            match &*e.to_string() {
                "updateErr" => return exit::UPDATE_ERR,
                "connectionErr" => return exit::CONNECTION_ERR,
                "parsingErr" => return exit::PARSING_ERR,
                _ => return exit::UNKNOWN_ERR,
            };
        }
        return exit::SUCCESS;
    }

    // Check if the path entered by the user exists
    if misc::check_path_exists(&opt.path).is_err() {
        return exit::PATH_ERR;
    }
    let files = ls::run_ls(&opt.path, &opt.all, &opt.recursive);

    // exit if we got an error while running ls
    if files.is_err() {
        return exit::LS_ERR;
    }

    let files_unwrapped = files.unwrap();
    let results = get_results::get_results(&opt, &files_unwrapped);

    match print_results::print_results(results, opt.recursive) {
        Ok(()) => exit::SUCCESS,
        Err(_) => {
            eprintln!(
                "\x1b[91mpermscan: stdout: failed to print results\x1b[0m"
            );
            exit::IO_ERR
        }
    }
}
