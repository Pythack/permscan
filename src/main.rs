use array_tool::vec::*;
use itertools::Itertools;
use regex::Regex;
use std::process::Command;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Permission scanner",
    about = "Scan a directory for files that match permission criteria. \n Visit https://github.com/Pythack/permscan for more information. "
)]
struct Opt {
    #[structopt(
        long,
        help = "Specify permissions that the user who owns the file or directory needs to have on the item in the format ?rwx"
    )]
    user: Option<String>,

    #[structopt(
        long,
        help = "Specify permissions that the group who owns the file or directory needs to have on the item in the format ?rwx"
    )]
    group: Option<String>,

    #[structopt(
        long,
        help = "Specify permissions that users who does not own the file or directory needs to have on the item in the format ?rwx"
    )]
    other: Option<String>,

    #[structopt(long, help = "Specify the owner of the file in the format user:group")]
    owner: Option<String>,

    #[structopt(
        short,
        help = "If present, will return the list of files that match at least one criteria, else return the list of files that match all criteria"
    )]
    merge: bool,
}

fn run_command(command: String, args: String) -> String {
    let output = Command::new(command).arg(args).output().expect("");
    let stdout = String::from_utf8(output.stdout);

    match stdout {
        Err(_e) => String::from(""),
        Ok(out) => out,
    }
}

fn rem_first(value: &str) -> String {
    let mut chars = value.chars();
    //println!("{}", chars.nth(0).unwrap());
    let first_value = match chars.nth(0) {
        None => String::from(""),
        Some(value) => String::from(value),
    };
    if first_value == String::from('?') {
        return String::from(chars.as_str());
    } else {
        return String::from(value);
    }
}

fn get_based_on_owner(files: String, owner: String) -> Vec<String> {
    let lines = files.split('\n');
    let mut temp_lines: Vec<String> = Vec::new();
    let retext = String::from(r"^[drwxt\-]{10}[ 0-9]* *") + &*owner + r" (.|\n)*$";
    let re = Regex::new(&retext).unwrap();
    for line in lines.skip(1) {
        let line = String::from(line);
        if re.is_match(&line) {
            temp_lines.push(line);
        }
    }
    temp_lines
}

fn get_based_on_user(files: String, user: String) -> Vec<String> {
    let lines = files.split('\n');
    let mut temp_lines: Vec<String> = Vec::new();
    let retext = String::from(r"^[drwxt\-]") + &user + r"(.|\n)*$";
    let re = Regex::new(&retext).unwrap();
    for line in lines.skip(1) {
        let line = String::from(line);
        if re.is_match(&line) {
            temp_lines.push(line);
        }
    }
    temp_lines
}

fn get_based_on_group(files: String, user: String) -> Vec<String> {
    let lines = files.split('\n');
    let mut temp_lines: Vec<String> = Vec::new();
    let retext = String::from(r"^[drwxt\-]{4}") + &user + r"(.|\n)*$";
    let re = Regex::new(&retext).unwrap();
    for line in lines.skip(1) {
        let line = String::from(line);
        if re.is_match(&line) {
            temp_lines.push(line);
        }
    }
    temp_lines
}

fn get_based_on_other(files: String, user: String) -> Vec<String> {
    let lines = files.split('\n');
    let mut temp_lines: Vec<String> = Vec::new();
    let retext = String::from(r"^[drwxt\-]{7}") + &user + r"(.|\n)*$";
    let re = Regex::new(&retext).unwrap();
    for line in lines.skip(1) {
        let line = String::from(line);
        if re.is_match(&line) {
            temp_lines.push(line);
        }
    }
    temp_lines
}

fn main() {
    let opt = Opt::from_args();
    let files = run_command(String::from("ls"), String::from("-la"));
    let files_owner_check = files;
    let files_user_check = files_owner_check.clone();
    let files_group_check = files_owner_check.clone();
    let files_other_check = files_owner_check.clone();
    let mut all_lines: Vec<Vec<String>> = Vec::new();
    let mut temp_lines: Vec<String> = Vec::new();
    if opt.owner.is_some() {
        let owner = match opt.owner {
            None => String::from(""),
            Some(owner) => owner.replace(':', " *"),
        };
        if opt.merge {
            temp_lines.extend(get_based_on_owner(files_owner_check, owner).iter().cloned());
        } else {
            let owner_lines = get_based_on_owner(files_owner_check, owner);
            all_lines.push(owner_lines);
        }
    }
    if opt.user.is_some() {
        let user = match opt.user {
            None => String::from(""),
            Some(user) => rem_first(&user),
        };
        if opt.merge {
            temp_lines.extend(get_based_on_user(files_user_check, user).iter().cloned());
        } else {
            let user_lines = get_based_on_user(files_user_check, user);
            all_lines.push(user_lines);
        }
    }
    if opt.group.is_some() {
        let group = match opt.group {
            None => String::from(""),
            Some(group) => rem_first(&group),
        };
        if opt.merge {
            temp_lines.extend(get_based_on_group(files_group_check, group).iter().cloned());
        } else {
            let user_lines = get_based_on_group(files_group_check, group);
            all_lines.push(user_lines);
        }
    }
    if opt.other.is_some() {
        let other = match opt.other {
            None => String::from(""),
            Some(other) => rem_first(&other),
        };
        if opt.merge {
            temp_lines.extend(get_based_on_other(files_other_check, other).iter().cloned());
        } else {
            let user_lines = get_based_on_other(files_other_check, other);
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
            final_lines.push(final_lines[final_lines_len - 1].intersect(lines_set.to_vec()));
        }
        let final_lines_len = final_lines.len();
        for line in &final_lines[final_lines_len - 1] {
            println!("{}", line);
        }
    }
}
