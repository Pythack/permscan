use regex::Regex;

#[path = "./misc.rs"]
mod misc;
#[path = "./types.rs"]
mod types;

use crate::Opt;
use crate::PermscanOutput;

// Call get_results_nomerge() or get_results_merge() based on opt.merge.
// As these to functions have two different return type we store the result
// in the PermscanOutput enum
pub fn get_results<'a>(opt: &Opt, files: &'a str) -> PermscanOutput<'a> {
    match &opt.merge {
        false => PermscanOutput::NoMerge(get_results_nomerge(opt, files)),
        true => PermscanOutput::Merge(get_results_merge(opt, files)),
    }
}

// Get files matching criteria. Called when opt.merge is false
fn get_results_nomerge<'a>(opt: &Opt, files: &'a str) -> Vec<Vec<&'a str>> {
    let mut lines: Vec<Vec<&str>> = Vec::new();

    if opt.owner.is_none()
        && opt.user.is_none()
        && opt.group.is_none()
        && opt.other.is_none()
        && opt.item_type.is_none()
    {
        lines.push(get_all_files(files, &opt.invert))
    }

    if let Some(owner_list) = &opt.owner {
        for owner in owner_list {
            let owner = owner.replace(':', " *");

            lines.push(get_based_on_owner(
                files,
                &owner,
                &opt.invert,
                &opt.recursive,
            ));
        }
    }

    if let Some(user_list) = &opt.user {
        for user in user_list {
            let user = misc::rem_first(user).replace('?', r"[rwx\-]");

            lines.push(get_based_on_user(
                files,
                &user,
                &opt.invert,
                &opt.recursive,
            ));
        }
    }

    if let Some(group_list) = &opt.group {
        for group in group_list {
            let group = misc::rem_first(group).replace('?', r"[rwx\-]");

            lines.push(get_based_on_group(
                files,
                &group,
                &opt.invert,
                &opt.recursive,
            ));
        }
    }

    if let Some(other_list) = &opt.other {
        for other in other_list {
            let other = misc::rem_first(other).replace('?', r"[rwx\-]");

            lines.push(get_based_on_other(
                files,
                &other,
                &opt.invert,
                &opt.recursive,
            ));
        }
    }

    if let Some(item_type_list) = &opt.item_type {
        for item_type in item_type_list {
            let item_type = item_type.replace('?', r"[rwx\-]");

            lines.push(get_based_on_type(
                files,
                &item_type,
                &opt.invert,
                &opt.recursive,
            ));
        }
    }
    lines
}

// Get files matching criteria. Called when opt.merge is true
fn get_results_merge<'a>(opt: &Opt, files: &'a str) -> Vec<&'a str> {
    let mut lines: Vec<&str> = Vec::new();

    if opt.owner.is_none()
        && opt.user.is_none()
        && opt.group.is_none()
        && opt.other.is_none()
        && opt.item_type.is_none()
    {
        lines.extend(get_all_files(files, &opt.invert))
    }

    if let Some(owner_list) = &opt.owner {
        for owner in owner_list {
            let owner = owner.replace(':', " *");

            lines.extend(
                get_based_on_owner(files, &owner, &opt.invert, &opt.recursive)
                    .iter()
                    .copied(),
            );
        }
    }

    if let Some(user_list) = &opt.user {
        for user in user_list {
            let user = misc::rem_first(user).replace('?', r"[rwx\-]");

            lines.extend(
                get_based_on_user(files, &user, &opt.invert, &opt.recursive)
                    .iter()
                    .copied(),
            );
        }
    }

    if let Some(group_list) = &opt.group {
        for group in group_list {
            let group = misc::rem_first(group).replace('?', r"[rwx\-]");

            lines.extend(
                get_based_on_group(files, &group, &opt.invert, &opt.recursive)
                    .iter()
                    .copied(),
            );
        }
    }

    if let Some(other_list) = &opt.other {
        for other in other_list {
            let other = misc::rem_first(other).replace('?', r"[rwx\-]");

            lines.extend(
                get_based_on_other(files, &other, &opt.invert, &opt.recursive)
                    .iter()
                    .copied(),
            );
        }
    }

    if let Some(item_type_list) = &opt.item_type {
        for item_type in item_type_list {
            let item_type = item_type.replace('?', r"[rwx\-]");

            lines.extend(
                get_based_on_type(
                    files,
                    &item_type,
                    &opt.invert,
                    &opt.recursive,
                )
                .iter()
                .copied(),
            );
        }
    }
    lines
}

fn get_based_on_owner<'a>(
    files: &'a str,
    owner: &str,
    invert: &bool,
    recursive: &bool,
) -> Vec<&'a str> {
    let lines = files.split('\n');
    let mut temp_lines: Vec<&str> = Vec::new();
    let re = Regex::new(
        &(String::from(r"^[dlcbps\-][rwx\-]{9}[ 0-9]* *")
            + owner
            + r" (.|\n)*$"),
    )
    .unwrap();
    let sub_dir = Regex::new(&String::from(r"^(.+)/*([^/]+)*:$")).unwrap();
    for line in lines.skip(1) {
        if (!invert && re.is_match(line))
            || (*invert && !re.is_match(line))
            || (*recursive && sub_dir.is_match(line))
        {
            temp_lines.push(line);
        }
    }
    temp_lines
}

fn get_based_on_user<'a>(
    files: &'a str,
    user: &str,
    invert: &bool,
    recursive: &bool,
) -> Vec<&'a str> {
    let lines = files.split('\n');
    let mut temp_lines: Vec<&str> = Vec::new();
    let re = Regex::new(
        &(String::from(r"^[dlcbps\-]") + user + r"[rwx\-]{6}(.|\n)*$"),
    )
    .unwrap();
    let sub_dir = Regex::new(&String::from(r"^(.+)/*([^/]+)*:$")).unwrap();
    for line in lines.skip(1) {
        if (!invert && re.is_match(line))
            || (*invert && !re.is_match(line))
            || (*recursive && sub_dir.is_match(line))
        {
            temp_lines.push(line);
        }
    }
    temp_lines
}

fn get_based_on_group<'a>(
    files: &'a str,
    group: &str,
    invert: &bool,
    recursive: &bool,
) -> Vec<&'a str> {
    let lines = files.split('\n');
    let mut temp_lines: Vec<&str> = Vec::new();
    let re = Regex::new(
        &(String::from(r"^[dlcbps\-][rwx\-]{3}")
            + group
            + r"[rwx\-]{3}(.|\n)*$"),
    )
    .unwrap();
    let sub_dir = Regex::new(&String::from(r"^(.+)/*([^/]+)*:$")).unwrap();
    for line in lines.skip(1) {
        if (!invert && re.is_match(line))
            || (*invert && !re.is_match(line))
            || (*recursive && sub_dir.is_match(line))
        {
            temp_lines.push(line);
        }
    }
    temp_lines
}

fn get_based_on_other<'a>(
    files: &'a str,
    other: &str,
    invert: &bool,
    recursive: &bool,
) -> Vec<&'a str> {
    let lines = files.split('\n');
    let mut temp_lines: Vec<&str> = Vec::new();

    let re = Regex::new(
        &(String::from(r"^[dlcbps\-][rwx\-]{6}") + other + r"(.|\n)*$"),
    )
    .unwrap();
    let sub_dir = Regex::new(&String::from(r"^(.+)/*([^/]+)*:$")).unwrap();
    for line in lines.skip(1) {
        if (!invert && re.is_match(line))
            || (*invert && !re.is_match(line))
            || (*recursive && sub_dir.is_match(line))
        {
            temp_lines.push(line);
        }
    }
    temp_lines
}

fn get_based_on_type<'a>(
    files: &'a str,
    file_type: &str,
    invert: &bool,
    recursive: &bool,
) -> Vec<&'a str> {
    let lines = files.split('\n');
    let mut temp_lines: Vec<&str> = Vec::new();
    let re =
        Regex::new(&(String::from(r"^") + file_type + r"[rwx\-]{9}(.|\n)*$"))
            .unwrap();
    let sub_dir = Regex::new(&String::from(r"^(.+)/*([^/]+)*:$")).unwrap();
    for line in lines.skip(1) {
        if (!invert && re.is_match(line))
            || (*invert && !re.is_match(line))
            || (*recursive && sub_dir.is_match(line))
        {
            temp_lines.push(line);
        }
    }
    temp_lines
}

fn get_all_files<'a>(files: &'a str, invert: &bool) -> Vec<&'a str> {
    let lines = files.split('\n');
    let mut temp_lines: Vec<&str> = Vec::new();
    if !invert {
        for line in lines.skip(1) {
            temp_lines.push(line)
        }
    }
    temp_lines
}
