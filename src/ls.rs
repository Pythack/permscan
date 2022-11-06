use std::process;
use std::process::Command;

#[path = "./types.rs"]
mod types;

use types::Result;

pub fn run_ls(path: &str, all: bool, recursive: bool) -> Result<String> {
    let args = get_ls_options(all, recursive);

    let output = Command::new("ls").arg(args).arg(path).output();

    match output {
        Ok(content) => output_to_str(content),

        Err(e) => Err(format!("ls: failed to get files: {}. is ls installed ?", e).into()),
    }
}

fn get_ls_options(all: bool, recursive: bool) -> String {
    let ls_options = String::from("-lh")
        + match all {
            true => "a",
            false => "",
        };
    let ls_options = ls_options
        + match recursive {
            true => "R",
            false => "",
        };
    ls_options
}

fn output_to_str(output: process::Output) -> Result<String> {
    let stdout = String::from_utf8(output.stdout);
    match stdout {
        Ok(string) => Ok(string),
        Err(e) => return Err(format!("ls: failed to parse ls output: {}", e).into()),
    }
}
