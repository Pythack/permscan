#![allow(dead_code)]

use array_tool::vec::*;
use itertools::Itertools;
use regex::Regex;
use std::error::Error;
use std::io::Write;

use structopt::StructOpt;

#[path = "./get_files.rs"]
mod get_files;
#[path = "./misc.rs"]
mod misc;
#[path = "./opt.rs"]
mod opt;

#[path = "./updates.rs"]
mod updates;

use crate::opt::Opt;

mod exit {

    // exit code when permscan runs without problems
    pub const SUCCESS: i32 = 0;

    // exit code for unknown error
    pub const UNKNOWN_ERR: i32 = -1;

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
                "input" => return exit::IO_ERR,
                "parsing" => return exit::PARSING_ERR,
                _ => return exit::UNKNOWN_ERR,
            };
        }
        return exit::SUCCESS;
    }

    // Check if the path entered by the user exists
    if let Err(_e) = misc::check_path_exists(&opt.path) {
        return exit::PATH_ERR;
    }

    // We are going to use ls to get all the files before filtering them
    // by permissions. Here we determine what flags we are going to run ls with
    let ls_options = misc::get_ls_options(&opt.all, &opt.recursive);

    // print the matching files if we run ls successfully
    if let Ok(files) = misc::run_ls(ls_options, &opt.path) {
        match print_matching_files(&opt, &files) {
            Ok(()) => exit::SUCCESS,
            _ => {
                eprintln!(
                    "\x1b[91mpermscan: stdout: failed to print results\x1b[0m"
                );
                exit::IO_ERR
            }
        };
    }
    exit::LS_ERR
}

// Get files matching criteria and call the print_result_nomerge function or
// the print_result_merge function that prints those files
fn print_matching_files(opt: &Opt, files: &str) -> Result<(), Box<dyn Error>> {
    if opt.merge {
        let mut lines: Vec<String> = Vec::new();

        if opt.owner.is_none()
            && opt.user.is_none()
            && opt.group.is_none()
            && opt.other.is_none()
            && opt.file_type.is_none()
        {
            lines.extend(get_files::get_all_files(files, opt.invert))
        }

        if let Some(owner) = &opt.owner {
            let owner = owner.replace(':', " *");

            lines.extend(
                get_files::get_based_on_owner(
                    files,
                    owner,
                    opt.invert,
                    opt.recursive,
                )
                .iter()
                .cloned(),
            );
        }

        if let Some(user) = &opt.user {
            let user = misc::rem_first(user).replace('?', r"[rwx\-]");

            lines.extend(
                get_files::get_based_on_user(
                    files,
                    user,
                    opt.invert,
                    opt.recursive,
                )
                .iter()
                .cloned(),
            );
        }

        if let Some(group) = &opt.group {
            let group = misc::rem_first(group).replace('?', r"[rwx\-]");

            lines.extend(
                get_files::get_based_on_group(
                    files,
                    group,
                    opt.invert,
                    opt.recursive,
                )
                .iter()
                .cloned(),
            );
        }

        if let Some(other) = &opt.other {
            let other = misc::rem_first(other).replace('?', r"[rwx\-]");

            lines.extend(
                get_files::get_based_on_other(
                    files,
                    other,
                    opt.invert,
                    opt.recursive,
                )
                .iter()
                .cloned(),
            );
        }

        if let Some(file_type) = &opt.file_type {
            let file_type = file_type.replace('?', r"[rwx\-]");

            lines.extend(
                get_files::get_based_on_type(
                    files,
                    file_type,
                    opt.invert,
                    opt.recursive,
                )
                .iter()
                .cloned(),
            );
        }
        print_results_merge(lines, opt.recursive)?
    } else {
        let mut lines: Vec<Vec<String>> = Vec::new();

        if opt.owner.is_none()
            && opt.user.is_none()
            && opt.group.is_none()
            && opt.other.is_none()
            && opt.file_type.is_none()
        {
            lines.push(get_files::get_all_files(files, opt.invert))
        }

        if let Some(owner) = &opt.owner {
            let owner = owner.replace(':', " *");

            lines.push(get_files::get_based_on_owner(
                files,
                owner,
                opt.invert,
                opt.recursive,
            ));
        }

        if let Some(user) = &opt.user {
            let user = misc::rem_first(user).replace('?', r"[rwx\-]");

            lines.push(get_files::get_based_on_user(
                files,
                user,
                opt.invert,
                opt.recursive,
            ));
        }

        if let Some(group) = &opt.group {
            let group = misc::rem_first(group).replace('?', r"[rwx\-]");

            lines.push(get_files::get_based_on_group(
                files,
                group,
                opt.invert,
                opt.recursive,
            ));
        }

        if let Some(other) = &opt.other {
            let other = misc::rem_first(other).replace('?', r"[rwx\-]");

            lines.push(get_files::get_based_on_other(
                files,
                other,
                opt.invert,
                opt.recursive,
            ));
        }

        if let Some(file_type) = &opt.file_type {
            let file_type = file_type.replace('?', r"[rwx\-]");

            lines.push(get_files::get_based_on_type(
                files,
                file_type,
                opt.invert,
                opt.recursive,
            ));
        }
        print_results_nomerge(lines, opt.recursive)?
    }

    Ok(())
}

// print results. Called when opt.merge is false
fn print_results_nomerge(
    mut lines: Vec<Vec<String>>,
    recursive: bool,
) -> Result<(), Box<dyn Error>> {
    let sub_dir_text = Regex::new(&String::from(r"^(.+)/*([^/]+)*:$")).unwrap();

    // lock stdout manually for better performances since we are going to print
    // to it a lot
    let stdout = std::io::stdout();
    let mut lock = stdout.lock();

    if !lines.is_empty() {
        let reference_lines = lines[0].clone();
        let mut final_lines: Vec<Vec<String>> = vec![reference_lines];
        lines.remove(0);
        for lines_set in &lines {
            let final_lines_len = final_lines.len();
            final_lines.push(
                final_lines[final_lines_len - 1].intersect(lines_set.to_vec()),
            );
        }
        let final_lines_len = final_lines.len();
        for line in &final_lines[final_lines_len - 1] {
            if recursive && sub_dir_text.is_match(line) {
                writeln!(lock, "\x1b[92m{}\x1b[0m", line)?;
            } else {
                writeln!(lock, "{}", line)?;
            }
        }
    }
    Ok(())
}

// print results. Called when opt.merge is true
fn print_results_merge(
    lines: Vec<String>,
    recursive: bool,
) -> Result<(), Box<dyn Error>> {
    let sub_dir_text = Regex::new(&String::from(r"^(.+)/*([^/]+)*:$")).unwrap();

    // lock stdout manually for better performances since we are going to print
    // to it a lot
    let stdout = std::io::stdout();
    let mut lock = stdout.lock();

    let lines: Vec<String> = lines.into_iter().unique().collect();

    for line in lines {
        if recursive && sub_dir_text.is_match(&line) {
            writeln!(lock, "\x1b[92m{}\x1b[0m", line)?;
        } else {
            writeln!(lock, "{}", line)?;
        }
    }
    Ok(())
}
