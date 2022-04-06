use std::error::Error;
use std::path::Path;
use std::process::Command;

// remove first character from string
pub fn rem_first(string: &str) -> &str {
    let mut chars = string.chars();
    chars.next();
    chars.as_str()
}

// check if the path entered by the user exists and return
// an error if it doesn't
pub fn check_path_exists(path: &str) -> Result<(), Box<dyn Error>> {
    let path_exists = Path::new(&path).exists();
    if !path_exists {
        eprintln!(
            "\x1b[91mpermscan: {}: No such file or directory\x1b[0m",
            &path
        );
        return Err("".into());
    }
    Ok(())
}

// determine ls options based on permscan options.
pub fn get_ls_options(all: &bool, recursive: &bool) -> String {
    let ls_options = String::from("-l")
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

// run ls to get files
pub fn run_ls(args: String, path: &str) -> Result<String, Box<dyn Error>> {
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
