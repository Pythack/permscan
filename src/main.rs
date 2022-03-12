use array_tool::vec::*;
use itertools::Itertools;
use regex::Regex;
use std::process::Command;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Permission scanner",
    about = "Scan a directory for files that match permission criterias"
)]
struct Opt {
    #[structopt(
        long,
        help = "Specify permissions that the user who owns the file or directory needs to have on the item in the format rwx"
    )]
    user: Option<String>,

    #[structopt(long, help = "Specify the owner of the file in the format user:group")]
    owner: Option<String>,

    #[structopt(
        short,
        help = "If present, will return the list of files that match at least one criteria, else return the list of files that match all criterias"
    )]
    merge: bool,
}

fn run_command(command: String, args: String) -> String {
    let output = Command::new(command).arg(args).output().expect("");
    let stdout = String::from_utf8(output.stdout);
    let stdout = match stdout {
        Err(_e) => String::from(""),
        Ok(out) => out,
    };
    return stdout;
}

fn get_based_on_owner(files: String, owner: String) -> Vec<String> {
    let lines = files.split("\n");
    let mut temp_lines: Vec<String> = Vec::new();
    let retext = String::from(r"^[drwxt\-]{10}[ 0-9]* *") + &*owner + r" (.|\n)*$";
    let re = Regex::new(&retext).unwrap();
    for line in lines.skip(1) {
        let line = String::from(line);
        if re.is_match(&line) {
            temp_lines.push(line);
        }
    }
    return temp_lines;
}

fn get_based_on_user(files: String, user: String) -> Vec<String> {
    let lines = files.split("\n");
    let mut temp_lines: Vec<String> = Vec::new();
    let retext = String::from(r"^[drwxt\-]") + &user + r"(.|\n)*$";
    let re = Regex::new(&retext).unwrap();
    for line in lines.skip(1) {
        let line = String::from(line);
        if re.is_match(&line) {
            temp_lines.push(line);
        }
    }
    return temp_lines;
}

fn main() {
    let opt = Opt::from_args();
    let files = run_command(String::from("ls"), String::from("-la"));
    let files_owner_check = files.clone();
    let files_user_check = files.clone();
    let mut all_lines: Vec<Vec<String>> = Vec::new();
    let mut temp_lines: Vec<String> = Vec::new();
    if opt.owner.is_some() {
        let owner = match opt.owner {
            None => String::from(""),
            Some(owner) => owner.replace(":", " *"),
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
            Some(user) => user,
        };
        if opt.merge {
            temp_lines.extend(get_based_on_user(files_user_check, user).iter().cloned());
        } else {
            let user_lines = get_based_on_user(files_user_check, user);
            all_lines.push(user_lines);
        }
    }
    if opt.merge {
        let temp_lines: Vec<String> = temp_lines.into_iter().unique().collect();
        for line in temp_lines {
            println!("{}", line);
        }
    } else {
        if all_lines.len() > 0 {
            let reference_lines = all_lines[0].clone();
            let mut final_lines: Vec<Vec<String>> = Vec::new();
            final_lines.push(reference_lines);
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
}
