use std::process::Command;

#[path = "./types.rs"]
mod types;

use types::Result;

// run ls to get files
pub fn run_ls(path: &str, all: &bool, recursive: &bool) -> Result<String> {
    let args = get_ls_options(all, recursive);

    let output = Command::new("ls").arg(args).arg(path).output();

    match output {
        Ok(content) => {
            let stdout = String::from_utf8(content.stdout);
            match stdout {
                Ok(out) => Ok(out),
                Err(_) => Ok(String::from("")),
            }
        }

        Err(_) => {
            eprintln!("\x1b[91mpermscan: ls: failed to get files. is ls installed ?\x1b[0m");
            Err("".into())
        }
    }
}

// determine ls options based on permscan options.
fn get_ls_options(all: &bool, recursive: &bool) -> String {
    let ls_options = String::from("-lh")
        + match all {
            true => "a",
            false => "",
        };
    let ls_options = ls_options
        + match recursive {
            true => {
                println!("\x1b[94mPlease be patient, a recursive search can take time... \x1b[0m");
                "R"
            }
            false => "",
        };
    ls_options
}
