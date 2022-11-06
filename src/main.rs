use std::process::exit;
use structopt::StructOpt;
use types::Result;

#[macro_use]
extern crate log;

#[path = "./get_results.rs"]
mod get_results;
#[path = "./logger.rs"]
mod logger;
#[path = "./ls.rs"]
mod ls;
#[path = "./misc.rs"]
mod misc;
#[path = "./opt.rs"]
mod opt;
#[path = "./output.rs"]
mod output;
#[path = "./types.rs"]
mod types;
#[path = "./updates.rs"]
mod updates;

use crate::opt::Opt;
use types::PermscanOutput;

fn main() {
    logger::new_logger();
    let opt = Opt::from_args();
    if let Err(e) = permscan(opt) {
        error!("permscan: {}", e);
        exit(1);
    }
    exit(0);
}

fn permscan(opt: Opt) -> Result<()> {
    if opt.check_update {
        updates::check_for_newer_version(opt.build)?;
        return Ok(());
    }

    misc::check_path_exists(&opt.path)?;

    let files = ls::run_ls(&opt.path, opt.all, opt.recursive);

    if opt.item_type != None {
        misc::verify_type_argument(opt.item_type.as_ref().unwrap())?
    }

    if let Err(e) = files {
        return Err(e);
    }

    let binding = files.unwrap();
    let mut results = get_results::get_results(&opt, &binding);

    if let Err(e) = output::print_results(&mut results, opt.recursive) {
        return Err(format!("stdout: failed to print results: {}", e).into());
    }
    return Ok(());
}
