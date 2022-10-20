//! functions to get all files (before filtering them) using ls

use std::process::Command;

#[path = "./colors.rs"]
mod colors;
#[path = "./types.rs"]
mod types;

use types::Result;

// Runs ls to get files
pub fn run_ls(path: &str, all: &bool, recursive: &bool) -> Result<String> {
    let args = get_ls_options(all, recursive);

    let output = Command::new("ls").arg(args).arg(path).output();

    match output {
        Ok(content) => output_to_str(content),

        Err(_) => {
            eprintln!(
                "{} permscan: ls: failed to get files. is ls installed ?{}",
                colors::RED,
                colors::RESET
            );
            Err("".into())
        }
    }
}

// Determines ls options based on permscan options.
fn get_ls_options(all: &bool, recursive: &bool) -> String {
    let ls_options = String::from("-lh")
        + match all {
            true => "a",
            false => "",
        };
    let ls_options = ls_options
        + match recursive {
            true => {
                println!("{}Please be patient, a recursive search can take time... {}", colors::BLUE, colors::RESET);
                "R"
            }
            false => "",
        };
    ls_options
}

// Converts a command output to a String
fn output_to_str(output: std::process::Output) -> Result<String> {
    let stdout = String::from_utf8(output.stdout);
    match stdout {
        Ok(string) => Ok(string),
        Err(_) => {
            eprintln!("permscan: ls: failed to parse ls output");
            Err("".into())
        }
    }
}
