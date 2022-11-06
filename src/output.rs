use array_tool::vec::*;
use regex::Regex;
use std::io::Write;

#[path = "./types.rs"]
mod types;

use types::Result;

use crate::PermscanOutput;

pub fn print_results(lines: &mut PermscanOutput, recursive: bool) -> Result<()> {
    match lines {
        PermscanOutput::NoMerge(lines) => print_results_default(lines, recursive),
        PermscanOutput::Merge(lines) => print_results_merge(lines, recursive),
    }
}

fn print_results_default(lines: &mut Vec<Vec<&str>>, recursive: bool) -> Result<()> {
    if !lines.is_empty() {
        let reference_lines = lines[0].clone();
        let mut final_lines: Vec<Vec<&str>> = vec![reference_lines];
        lines.remove(0);
        for lines_set in lines {
            final_lines.push(final_lines[final_lines.len() - 1].intersect(lines_set.to_vec()));
        }
        print_lines(&final_lines[final_lines.len() - 1], recursive)?;
    }
    Ok(())
}

fn print_results_merge(lines: &Vec<&str>, recursive: bool) -> Result<()> {
    // Remove items that appears multiple times.
    let lines: Vec<&str> = lines.unique();

    print_lines(&lines, recursive)?;

    Ok(())
}

fn print_lines(lines: &Vec<&str>, recursive: bool) -> Result<()> {
    // When using the recursive option, we have lines that tells us what
    // folder we are into. This regex match those lines so we can print them in color.
    let sub_dir_text = Regex::new(&String::from(r"^(.+)/*([^/]+)*:$")).unwrap();

    // Avoid flushing each time we print a line.
    let stdout = std::io::stdout();
    let mut lock = stdout.lock();

    const GREEN: &str = "\x1b[92m";
    const RESET: &str = "\x1b[0m";

    for line in lines {
        if recursive && sub_dir_text.is_match(line) {
            writeln!(lock, "{}{}{}", GREEN, line, RESET)?;
        } else {
            writeln!(lock, "{}", line)?;
        }
    }

    Ok(())
}
