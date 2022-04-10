use std::process::Command;

#[path = "./colors.rs"]
mod colors;

use crate::types::Result;

// run ls to get files
pub fn run_exa(
    path: &str,
    all: &bool,
    recursive: &bool,
    ignore: &Option<Vec<String>>,
) -> Result<String> {
    let args = get_exa_options(all, recursive, ignore);

    let output = Command::new("exa").args(args).arg(path).output();

    match output {
        Ok(content) => output_to_str(content),

        Err(_) => {
            eprintln!(
                "{}permscan: exa: failed to get files. is exa installed ?{}",
                colors::RED,
                colors::RESET
            );
            Err("".into())
        }
    }
}

// determine exa options based on permscan options.
fn get_exa_options<'a>(
    all: &bool,
    recursive: &bool,
    ignore: &'a Option<Vec<String>>,
) -> Vec<&'a str> {
    let mut ls_options = vec!["-lh"];
    if *all {
        ls_options.push("-a")
    }
    if *recursive {
        println!(
            "{}Please be patient, a recursive search can take time...{}",
            colors::BLUE,
            colors::RESET
        );
        ls_options.push("-R");
    }
    if let Some(patterns) = ignore {
        for i in patterns {
            ls_options.push("-I");
            ls_options.push(i);
        }
    }

    ls_options
}

// convert a command output to a String
fn output_to_str(output: std::process::Output) -> Result<String> {
    let stdout = String::from_utf8(output.stdout);
    match stdout {
        Ok(string) => Ok(string),
        Err(_) => {
            eprintln!("permscan: exa: failed to parse ls output");
            Err("".into())
        }
    }
}
