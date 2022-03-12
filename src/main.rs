use regex::Regex;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Permission scanner",
    about = "Scan a directory for files that match permission criterias"
)]
struct Opt {
    #[structopt(short, help = "Specify the owner of the file in the format user:group")]
    owner: Option<String>,
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

fn main() {
    let opt = Opt::from_args();
    let files = run_command(String::from("ls"), String::from("-la"));
    let lines = files.split("\n");
    if opt.owner.is_some() {
        let owner = match opt.owner {
            None => String::from(""),
            Some(owner) => owner.replace(":", " "),
        };
        let retext = String::from(r"[drwxt\-]{10}[ 0-9]* ") + &*owner + r" (.|\n)*";
        let re = Regex::new(&retext).unwrap();
        for line in lines.skip(1) {
            let line = String::from(line);
            if re.is_match(&line) {
                println!("{}", line);
            }
        }
    }
}
