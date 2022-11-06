use regex::Regex;

#[path = "./misc.rs"]
mod misc;
#[path = "./types.rs"]
mod types;

use crate::Opt;
use crate::PermscanOutput;

pub fn get_results<'a>(opt: &Opt, files: &'a str) -> PermscanOutput<'a> {
    match &opt.merge {
        false => PermscanOutput::NoMerge(get_results_nomerge(opt, files)),
        true => PermscanOutput::Merge(get_results_merge(opt, files)),
    }
}

fn get_results_nomerge<'a>(opt: &Opt, files: &'a str) -> Vec<Vec<&'a str>> {
    let mut lines: Vec<Vec<&str>> = Vec::new();

    if opt.owner.is_none()
        && opt.user.is_none()
        && opt.group.is_none()
        && opt.other.is_none()
        && opt.item_type.is_none()
    {
        lines.push(return_all_lines(files, opt.invert))
    }

    if let Some(owner) = &opt.owner {
        let owner = owner.replace(':', " *");

        lines.push(get_based_on_owner(files, &owner, opt.invert, opt.recursive));
    }

    if let Some(user) = &opt.user {
        let user = misc::remove_first_char(user).replace('?', r"[rwx\-]");

        lines.push(get_based_on_user(files, &user, opt.invert, opt.recursive));
    }

    if let Some(group) = &opt.group {
        let group = misc::remove_first_char(group).replace('?', r"[rwx\-]");

        lines.push(get_based_on_group(files, &group, opt.invert, opt.recursive));
    }

    if let Some(other) = &opt.other {
        let other = misc::remove_first_char(other).replace('?', r"[rwx\-]");

        lines.push(get_based_on_other(files, &other, opt.invert, opt.recursive));
    }

    if let Some(file_type) = &opt.item_type {
        let file_type = file_type.replace('?', r"[rwx\-]");

        lines.push(get_based_on_type(files, &file_type, opt.invert, opt.recursive));
    }
    lines
}

fn get_results_merge<'a>(opt: &Opt, files: &'a str) -> Vec<&'a str> {
    let mut lines: Vec<&str> = Vec::new();

    if opt.owner.is_none()
        && opt.user.is_none()
        && opt.group.is_none()
        && opt.other.is_none()
        && opt.item_type.is_none()
    {
        lines.extend(return_all_lines(files, opt.invert))
    }

    if let Some(owner) = &opt.owner {
        let owner = owner.replace(':', " *");

        lines.extend(
            get_based_on_owner(files, &owner, opt.invert, opt.recursive)
                .iter()
                .copied(),
        );
    }

    if let Some(user) = &opt.user {
        let user = misc::remove_first_char(user).replace('?', r"[rwx\-]");

        lines.extend(
            get_based_on_user(files, &user, opt.invert, opt.recursive)
                .iter()
                .copied(),
        );
    }

    if let Some(group) = &opt.group {
        let group = misc::remove_first_char(group).replace('?', r"[rwx\-]");

        lines.extend(
            get_based_on_group(files, &group, opt.invert, opt.recursive)
                .iter()
                .copied(),
        );
    }

    if let Some(other) = &opt.other {
        let other = misc::remove_first_char(other).replace('?', r"[rwx\-]");

        lines.extend(
            get_based_on_other(files, &other, opt.invert, opt.recursive)
                .iter()
                .copied(),
        );
    }

    if let Some(file_type) = &opt.item_type {
        let file_type = file_type.replace('?', r"[rwx\-]");

        lines.extend(
            get_based_on_type(files, &file_type, opt.invert, opt.recursive)
                .iter()
                .copied(),
        );
    }
    lines
}

fn get_based_on_owner<'a>(files: &'a str, owner: &str, invert: bool, recursive: bool) -> Vec<&'a str> {
    let re = Regex::new(&(String::from(r"^[dlcbps\-][rwx\-]{9}[ 0-9]* *") + owner + r" (.|\n)*$")).unwrap();
    return_matching_lines(files, re, invert, recursive)
}

fn get_based_on_user<'a>(files: &'a str, user: &str, invert: bool, recursive: bool) -> Vec<&'a str> {
    let re = Regex::new(&(String::from(r"^[dlcbps\-]") + user + r"[rwx\-]{6}(.|\n)*$")).unwrap();
    return_matching_lines(files, re, invert, recursive)
}

fn get_based_on_group<'a>(files: &'a str, group: &str, invert: bool, recursive: bool) -> Vec<&'a str> {
    let re = Regex::new(&(String::from(r"^[dlcbps\-][rwx\-]{3}") + group + r"[rwx\-]{3}(.|\n)*$")).unwrap();
    return_matching_lines(files, re, invert, recursive)
}

fn get_based_on_other<'a>(files: &'a str, other: &str, invert: bool, recursive: bool) -> Vec<&'a str> {
    let re = Regex::new(&(String::from(r"^[dlcbps\-][rwx\-]{6}") + other + r"(.|\n)*$")).unwrap();
    return_matching_lines(files, re, invert, recursive)
}

fn get_based_on_type<'a>(files: &'a str, file_type: &str, invert: bool, recursive: bool) -> Vec<&'a str> {
    let re = Regex::new(&(String::from(r"^") + file_type + r"[rwx\-]{9}(.|\n)*$")).unwrap();
    return_matching_lines(files, re, invert, recursive)
}

fn return_all_lines(files: &str, invert: bool) -> Vec<&str> {
    let mut temp_lines: Vec<&str> = Vec::new();
    let lines = files.split('\n');
    if !invert {
        for line in lines.skip(1) {
            temp_lines.push(line)
        }
    }
    temp_lines
}

fn return_matching_lines(files: &str, re: Regex, invert: bool, recursive: bool) -> Vec<&str> {
    let lines = files.split('\n');
    let mut temp_lines: Vec<&str> = Vec::new();
    let sub_dir_regex = Regex::new(&String::from(r"^(.+)/*([^/]+)*:$")).unwrap();
    for line in lines.skip(1) {
        if (!invert && re.is_match(line))
            || (invert && !re.is_match(line))
            || (recursive && sub_dir_regex.is_match(line))
        {
            temp_lines.push(line);
        }
    }
    temp_lines
}
