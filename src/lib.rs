use regex::Regex;
use std::process::Command;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Permission scanner",
    about = "Scan a directory for files that match permission criteria. \nVisit https://github.com/Pythack/permscan#readme for more information. "
)]
pub struct Opt {
    #[structopt(
        long,
        help = "Specify permissions that the user who owns the file or directory needs to have on the item in the format @rwx"
    )]
    pub user: Option<String>,

    #[structopt(
        long,
        help = "Specify permissions that the group who owns the file or directory needs to have on the item in the format @rwx"
    )]
    pub group: Option<String>,

    #[structopt(
        long,
        help = "Specify permissions that users who does not own the file or directory needs to have on the item in the format @rwx"
    )]
    pub other: Option<String>,

    #[structopt(
        long,
        help = "Specify the owner of the file in the format user:group"
    )]
    pub owner: Option<String>,

    #[structopt(
        short,
        help = "If present, will recursively traverse the folder"
    )]
    pub recursive: bool,

    #[structopt(
        short,
        help = "If present, will return the list of files that match at least one criteria, else return the list of files that match all criteria"
    )]
    pub merge: bool,

    #[structopt(
        short,
        help = "If present, will return the list of files that don't match with the criteria"
    )]
    pub invert: bool,

    #[structopt(
        short,
        help = "If present, permscan will parse hidden files as well"
    )]
    pub all: bool,

    #[structopt(
        default_value = "./",
        help = "The path of the directory your want to look into."
    )]
    pub path: String,
}
pub fn run_command(command: String, args: String, path: String) -> String {
    let output = Command::new(command)
        .arg(args)
        .arg(path)
        .output()
        .expect("");
    let stdout = String::from_utf8(output.stdout);

    match stdout {
        Err(_e) => String::from(""),
        Ok(out) => out,
    }
}
pub fn rem_first(value: &str) -> String {
    let mut chars = value.chars();
    let first_value = match chars.next() {
        None => String::from(""),
        Some(value) => String::from(value),
    };
    if first_value == String::from('@') {
        return String::from(chars.as_str());
    } else {
        String::from(value)
    }
}

pub fn get_based_on_owner(
    files: String,
    owner: String,
    invert: bool,
    recursive: bool,
) -> Vec<String> {
    let lines = files.split('\n');
    let mut temp_lines: Vec<String> = Vec::new();
    let retext = String::from(r"^[dlcbps\-][rwx\-]{9}[ 0-9]* *")
        + &*owner
        + r" (.|\n)*$";
    let re = Regex::new(&retext).unwrap();
    let sub_dir_text = String::from(r"^(.+)/*([^/]+)*:$");
    let sub_dir = Regex::new(&sub_dir_text).unwrap();
    for line in lines.skip(1) {
        let line = String::from(line);
        if (!invert && re.is_match(&line))
            || (invert && !re.is_match(&line))
            || (recursive && sub_dir.is_match(&line))
        {
            temp_lines.push(line);
        }
    }
    temp_lines
}

pub fn get_based_on_user(
    files: String,
    user: String,
    invert: bool,
    recursive: bool,
) -> Vec<String> {
    let lines = files.split('\n');
    let mut temp_lines: Vec<String> = Vec::new();
    let retext = String::from(r"^[dlcbps\-]") + &user + r"[rwx\-]{6}(.|\n)*$";
    let re = Regex::new(&retext).unwrap();
    let sub_dir_text = String::from(r"^(.+)/*([^/]+)*:$");
    let sub_dir = Regex::new(&sub_dir_text).unwrap();
    for line in lines.skip(1) {
        let line = String::from(line);
        if (!invert && re.is_match(&line))
            || (invert && !re.is_match(&line))
            || (recursive && sub_dir.is_match(&line))
        {
            temp_lines.push(line);
        }
    }
    temp_lines
}

pub fn get_based_on_group(
    files: String,
    user: String,
    invert: bool,
    recursive: bool,
) -> Vec<String> {
    let lines = files.split('\n');
    let mut temp_lines: Vec<String> = Vec::new();
    let retext =
        String::from(r"^[dlcbps\-][rwx\-]{3}") + &user + r"[rwx\-]{3}(.|\n)*$";
    let re = Regex::new(&retext).unwrap();
    let sub_dir_text = String::from(r"^(.+)/*([^/]+)*:$");
    let sub_dir = Regex::new(&sub_dir_text).unwrap();
    for line in lines.skip(1) {
        let line = String::from(line);
        if (!invert && re.is_match(&line))
            || (invert && !re.is_match(&line))
            || (recursive && sub_dir.is_match(&line))
        {
            temp_lines.push(line);
        }
    }
    temp_lines
}

pub fn get_based_on_other(
    files: String,
    user: String,
    invert: bool,
    recursive: bool,
) -> Vec<String> {
    let lines = files.split('\n');
    let mut temp_lines: Vec<String> = Vec::new();
    let retext = String::from(r"^[dlcbps\-][rwx\-]{6}") + &user + r"(.|\n)*$";
    let re = Regex::new(&retext).unwrap();
    let sub_dir_text = String::from(r"^(.+)/*([^/]+)*:$");
    let sub_dir = Regex::new(&sub_dir_text).unwrap();
    for line in lines.skip(1) {
        let line = String::from(line);
        if (!invert && re.is_match(&line))
            || (invert && !re.is_match(&line))
            || (recursive && sub_dir.is_match(&line))
        {
            temp_lines.push(line);
        }
    }
    temp_lines
}

pub fn get_all_files(files: String, invert: bool) -> Vec<String> {
    let lines = files.split('\n');
    let mut temp_lines: Vec<String> = Vec::new();
    if !invert {
        for line in lines.skip(1) {
            let line = String::from(line);
            temp_lines.push(line)
        }
    }
    temp_lines
}
