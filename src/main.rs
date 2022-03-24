use array_tool::vec::*;
use itertools::Itertools;
use regex::Regex;
use structopt::StructOpt;

use permscan::Opt;

fn main() {
    let opt = Opt::from_args();
    if opt.recursive {
        println!("\x1b[94mPlease be patient, a recursive search can take time... \x1b[0m");
    }
    let ls_options = String::from("-l")
        + match opt.all {
            true => "a",
            false => "",
        };
    let ls_options = ls_options
        + match opt.recursive {
            true => "R",
            false => "",
        };
    let files = permscan::run_command(String::from("ls"), ls_options, opt.path);
    let mut all_lines: Vec<Vec<String>> = Vec::new();
    let mut temp_lines: Vec<String> = Vec::new();

    if opt.owner.is_none()
        && opt.user.is_none()
        && opt.group.is_none()
        && opt.other.is_none()
    {
        let lines = permscan::get_all_files(files.as_str(), opt.invert);
        all_lines.push(lines)
    }

    if opt.owner.is_some() {
        let owner = match opt.owner {
            None => String::from(""),
            Some(owner) => owner.replace(':', " *"),
        };
        if opt.merge {
            temp_lines.extend(
                permscan::get_based_on_owner(
                    files.as_str(),
                    owner,
                    opt.invert,
                    opt.recursive,
                )
                .iter()
                .cloned(),
            );
        } else {
            let owner_lines = permscan::get_based_on_owner(
                files.as_str(),
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
            Some(user) => permscan::rem_first(&user).replace('?', r"[rwx\-]"),
        };
        if opt.merge {
            temp_lines.extend(
                permscan::get_based_on_user(
                    &files,
                    user,
                    opt.invert,
                    opt.recursive,
                )
                .iter()
                .cloned(),
            );
        } else {
            let user_lines = permscan::get_based_on_user(
                files.as_str(),
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
            Some(group) => permscan::rem_first(&group).replace('?', r"[rwx\-]"),
        };
        if opt.merge {
            temp_lines.extend(
                permscan::get_based_on_group(
                    files.as_str(),
                    group,
                    opt.invert,
                    opt.recursive,
                )
                .iter()
                .cloned(),
            );
        } else {
            let user_lines = permscan::get_based_on_group(
                &files,
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
            Some(other) => permscan::rem_first(&other).replace('?', r"[rwx\-]"),
        };
        if opt.merge {
            temp_lines.extend(
                permscan::get_based_on_other(
                    files.as_str(),
                    other,
                    opt.invert,
                    opt.recursive,
                )
                .iter()
                .cloned(),
            );
        } else {
            let user_lines = permscan::get_based_on_other(
                &files,
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
                permscan::rem_first(&file_type).replace('?', r"[rwx\-]")
            }
        };
        if opt.merge {
            temp_lines.extend(
                permscan::get_based_on_type(
                    &files,
                    file_type,
                    opt.invert,
                    opt.recursive,
                )
                .iter()
                .cloned(),
            );
        } else {
            let user_lines = permscan::get_based_on_type(
                &files,
                file_type,
                opt.invert,
                opt.recursive,
            );
            all_lines.push(user_lines);
        }
    }
    let sub_dir = Regex::new(&String::from(r"^(.+)/*([^/]+)*:$")).unwrap();
    if opt.merge {
        let temp_lines: Vec<String> = temp_lines.into_iter().unique().collect();
        for line in temp_lines {
            if opt.recursive && sub_dir.is_match(&line) {
                println!("\x1b[92m{}\x1b[0m", line);
            } else {
                println!("{}", line);
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
            if opt.recursive && sub_dir.is_match(line) {
                println!("\x1b[92m{}\x1b[0m", line);
            } else {
                println!("{}", line);
            }
        }
    }
}
