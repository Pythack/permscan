use array_tool::vec::*;
use itertools::Itertools;
use regex::Regex;
use std::error::Error;
use std::io::Write;
use std::path::Path;
use structopt::StructOpt;

mod get_files;
mod misc;
mod opt;
mod updates;

use opt::Opt;

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
            if exit_code != 0 {
                println!("\x1b[91mpermscan: process exited with exit code {}. to know more about error codes, visit https://github.com/Pythack/permscan/wiki/Error-codes\x1b[0m", exit_code)
            } else {
                eprintln!(
                    "\x1b[92mpermscan: process successfully exited with exit code 0\x1b[0m"
                )
            }
        }
    }
    std::process::exit(exit_code)
}

// The true main function. Returns an exit code
fn permscan(opt: Opt) -> i32 {
    // Run the checks_for_newer_version function if the update flag is raised.
    // Returns exit code when done
    if opt.check_update {
        if let Err(e) = updates::check_for_newer_version(opt.build) {
            match &*e.to_string() {
                "update" => return exit::UPDATE_ERR,
                "connection" => return exit::CONNECTION_ERR,
                _ => return exit::UNKNOWN_ERR,
            };
        }
        return exit::SUCCESS;
    }

    // Check if the path entered by the user exists
    let path_exists = Path::new(&opt.path).exists();
    if !path_exists {
        eprintln!(
            "\x1b[91mpermscan: {}: No such file or directory\x1b[0m",
            &opt.path
        );
        return exit::PATH_ERR;
    }

    // We are going to run ls to get all the files before filtering them
    // by permissions. We use different flags for ls based on the permscan
    // flags
    let ls_options = String::from("-l")
        + match opt.all {
            true => "a",
            false => "",
        };
    let ls_options = ls_options
        + match opt.recursive {
            true => {
                println!("\x1b[94mPlease be patient, a recursive search can take time... \x1b[0m");
                "R"
            }
            false => "",
        };
    // Get all files using ls
    let files = misc::run_ls(ls_options, &opt.path);

    match files {
        Ok(content) => {
            print_matching_files(opt, &content) // print files matching permscan options and flags and return exit code
        }
        Err(_e) => {
            eprintln!("\x1b[91mpermscan: ls: failed to get files. is ls installed ?\x1b[0m");
            exit::LS_ERR
        }
    }
}

// Get files matching criteria and call the print_result function
// that prints them
fn print_matching_files(opt: Opt, files: &str) -> i32 {
    let mut all_lines: Vec<Vec<String>> = Vec::new();
    let mut temp_lines: Vec<String> = Vec::new();

    // if no filtering option is passed to permscan we just print all files
    if opt.owner.is_none()
        && opt.user.is_none()
        && opt.group.is_none()
        && opt.other.is_none()
    {
        let lines = get_files::get_all_files(files, opt.invert);
        all_lines.push(lines)
    }

    if opt.owner.is_some() {
        let owner = match opt.owner {
            None => String::from(""),
            Some(owner) => owner.replace(':', " *"),
        };
        if opt.merge {
            temp_lines.extend(
                get_files::get_based_on_owner(
                    files,
                    owner,
                    opt.invert,
                    opt.recursive,
                )
                .iter()
                .cloned(),
            );
        } else {
            let owner_lines = get_files::get_based_on_owner(
                files,
                owner,
                opt.invert,
                opt.recursive,
            );
            all_lines.push(owner_lines);
        }
    }

    if opt.user.is_some() {
        let user = match opt.user {
            None => String::from(""),
            Some(user) => misc::rem_first(&user).replace('?', r"[rwx\-]"),
        };
        if opt.merge {
            temp_lines.extend(
                get_files::get_based_on_user(
                    files,
                    user,
                    opt.invert,
                    opt.recursive,
                )
                .iter()
                .cloned(),
            );
        } else {
            let user_lines = get_files::get_based_on_user(
                files,
                user,
                opt.invert,
                opt.recursive,
            );
            all_lines.push(user_lines);
        }
    }

    if opt.group.is_some() {
        let group = match opt.group {
            None => String::from(""),
            Some(group) => misc::rem_first(&group).replace('?', r"[rwx\-]"),
        };
        if opt.merge {
            temp_lines.extend(
                get_files::get_based_on_group(
                    files,
                    group,
                    opt.invert,
                    opt.recursive,
                )
                .iter()
                .cloned(),
            );
        } else {
            let user_lines = get_files::get_based_on_group(
                files,
                group,
                opt.invert,
                opt.recursive,
            );
            all_lines.push(user_lines);
        }
    }

    if opt.other.is_some() {
        let other = match opt.other {
            None => String::from(""),
            Some(other) => misc::rem_first(&other).replace('?', r"[rwx\-]"),
        };
        if opt.merge {
            temp_lines.extend(
                get_files::get_based_on_other(
                    files,
                    other,
                    opt.invert,
                    opt.recursive,
                )
                .iter()
                .cloned(),
            );
        } else {
            let user_lines = get_files::get_based_on_other(
                files,
                other,
                opt.invert,
                opt.recursive,
            );
            all_lines.push(user_lines);
        }
    }

    if opt.file_type.is_some() {
        let file_type = match opt.file_type {
            None => String::from(""),
            Some(file_type) => {
                misc::rem_first(&file_type).replace('?', r"[rwx\-]")
            }
        };
        if opt.merge {
            temp_lines.extend(
                get_files::get_based_on_type(
                    files,
                    file_type,
                    opt.invert,
                    opt.recursive,
                )
                .iter()
                .cloned(),
            );
        } else {
            let user_lines = get_files::get_based_on_type(
                files,
                file_type,
                opt.invert,
                opt.recursive,
            );
            all_lines.push(user_lines);
        }
    }
    let sub_dir = Regex::new(&String::from(r"^(.+)/*([^/]+)*:$")).unwrap();

    match print_results(
        sub_dir,
        temp_lines,
        all_lines,
        opt.recursive,
        opt.merge,
    ) {
        Ok(()) => exit::SUCCESS,
        Err(_e) => {
            eprintln!(
                "\x1b[91mpermscan: stdout: failed to print results\x1b[0m"
            );
            exit::IO_ERR
        }
    }
}

fn print_results(
    sub_dir: Regex,
    temp_lines: Vec<String>,
    mut all_lines: Vec<Vec<String>>,
    recursive: bool,
    merge: bool,
) -> Result<(), Box<dyn Error>> {
    // lock stdout manually for better performances since we are going to print
    // to it a lot
    let stdout = std::io::stdout();
    let mut lock = stdout.lock();

    if merge {
        let temp_lines: Vec<String> = temp_lines.into_iter().unique().collect();
        for line in temp_lines {
            if recursive && sub_dir.is_match(&line) {
                writeln!(lock, "\x1b[92m{}\x1b[0m", line)?;
            } else {
                writeln!(lock, "{}", line)?;
            }
        }
    } else if !all_lines.is_empty() {
        let reference_lines = all_lines[0].clone();
        let mut final_lines: Vec<Vec<String>> = vec![reference_lines];
        all_lines.remove(0);
        for lines_set in &all_lines {
            let final_lines_len = final_lines.len();
            final_lines.push(
                final_lines[final_lines_len - 1].intersect(lines_set.to_vec()),
            );
        }
        let final_lines_len = final_lines.len();
        for line in &final_lines[final_lines_len - 1] {
            if recursive && sub_dir.is_match(line) {
                writeln!(lock, "\x1b[92m{}\x1b[0m", line)?;
            } else {
                writeln!(lock, "{}", line)?;
            }
        }
    }
    Ok(())
}
