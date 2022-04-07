use array_tool::vec::*;
use regex::Regex;
use std::io::Write;

#[path = "./colors.rs"]
mod colors;
#[path = "./types.rs"]
mod types;

use types::Result;

use crate::PermscanOutput;

// wrapper around print_results_merge() and print_results_nomerge()
// that call one of them based on the type of the output
pub fn print_results(lines: PermscanOutput, recursive: &bool) -> Result<()> {
    match lines {
        PermscanOutput::NoMerge(lines) => {
            print_results_nomerge(lines, recursive)
        }
        PermscanOutput::Merge(lines) => print_results_merge(lines, recursive),
    }
}

// print results. Called when opt.merge is false
fn print_results_nomerge(
    mut lines: Vec<Vec<&str>>,
    recursive: &bool,
) -> Result<()> {
    // when using the recursive option, we have lines that tells us what
    // folder we are into. We want to be able to match these lines to
    // print them in color
    let sub_dir_text = Regex::new(&String::from(r"^(.+)/*([^/]+)*:$")).unwrap();

    // lock stdout manually for better performances since we are going to print
    // to it a lot
    let stdout = std::io::stdout();
    let mut lock = stdout.lock();

    // TODO: comment this
    if !lines.is_empty() {
        let reference_lines = lines[0].clone();
        let mut final_lines: Vec<Vec<&str>> = vec![reference_lines];
        lines.remove(0);
        for lines_set in &lines {
            final_lines.push(
                final_lines[final_lines.len() - 1]
                    .intersect(lines_set.to_vec()),
            );
        }
        for line in &final_lines[final_lines.len() - 1] {
            if *recursive && sub_dir_text.is_match(line) {
                writeln!(lock, "{}{}{}", colors::GREEN, line, colors::RESET)?;
            } else {
                writeln!(lock, "{}", line)?;
            }
        }
    }
    Ok(())
}

// print results. Called when opt.merge is true
fn print_results_merge(lines: Vec<&str>, recursive: &bool) -> Result<()> {
    // when using the recursive option, we have lines that tells us what
    // folder we are into. We want to be able to match these lines to
    // print them in color
    let sub_dir_text = Regex::new(&String::from(r"^(.+)/*([^/]+)*:$")).unwrap();

    // lock stdout manually for better performances since we are going to print
    // to it a lot
    let stdout = std::io::stdout();
    let mut lock = stdout.lock();

    // remove items that appears multiple times
    let lines: Vec<&str> = lines.unique();

    for line in lines {
        if *recursive && sub_dir_text.is_match(line) {
            writeln!(lock, "{}{}{}", colors::GREEN, line, colors::RESET)?;
        } else {
            writeln!(lock, "{}", line)?;
        }
    }
    Ok(())
}
