use array_tool::vec::*;
use itertools::Itertools;
use structopt::StructOpt;

use permscan::Opt;

fn main() {
    let opt = Opt::from_args();
    let files = permscan::run_command(String::from("ls"), String::from("-la"));
    let files_owner_check = files.clone();
    let files_user_check = files_owner_check.clone();
    let files_group_check = files_owner_check.clone();
    let files_other_check = files_owner_check.clone();
    let mut all_lines: Vec<Vec<String>> = Vec::new();
    let mut temp_lines: Vec<String> = Vec::new();

    if opt.owner.is_none()
        && opt.user.is_none()
        && opt.group.is_none()
        && opt.other.is_none()
    {
        let lines = permscan::get_all_files(files, opt.revert);
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
                    files_owner_check,
                    owner,
                    opt.revert,
                )
                .iter()
                .cloned(),
            );
        } else {
            let owner_lines = permscan::get_based_on_owner(
                files_owner_check,
                owner,
                opt.revert,
            );
            all_lines.push(owner_lines);
        }
    }

    if opt.user.is_some() {
        let user = match opt.user {
            None => String::from(""),
            Some(user) => permscan::rem_first(&user),
        };
        if opt.merge {
            temp_lines.extend(
                permscan::get_based_on_user(files_user_check, user, opt.revert)
                    .iter()
                    .cloned(),
            );
        } else {
            let user_lines =
                permscan::get_based_on_user(files_user_check, user, opt.revert);
            all_lines.push(user_lines);
        }
    }

    if opt.group.is_some() {
        let group = match opt.group {
            None => String::from(""),
            Some(group) => permscan::rem_first(&group),
        };
        if opt.merge {
            temp_lines.extend(
                permscan::get_based_on_group(
                    files_group_check,
                    group,
                    opt.revert,
                )
                .iter()
                .cloned(),
            );
        } else {
            let user_lines = permscan::get_based_on_group(
                files_group_check,
                group,
                opt.revert,
            );
            all_lines.push(user_lines);
        }
    }

    if opt.other.is_some() {
        let other = match opt.other {
            None => String::from(""),
            Some(other) => permscan::rem_first(&other),
        };
        if opt.merge {
            temp_lines.extend(
                permscan::get_based_on_other(
                    files_other_check,
                    other,
                    opt.revert,
                )
                .iter()
                .cloned(),
            );
        } else {
            let user_lines = permscan::get_based_on_other(
                files_other_check,
                other,
                opt.revert,
            );
            all_lines.push(user_lines);
        }
    }

    if opt.merge {
        let temp_lines: Vec<String> = temp_lines.into_iter().unique().collect();
        for line in temp_lines {
            println!("{}", line);
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
            println!("{}", line);
        }
    }
}
