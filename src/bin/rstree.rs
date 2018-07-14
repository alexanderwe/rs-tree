#[macro_use]
extern crate structopt;

extern crate rstree;

use std::path::PathBuf;
use structopt::StructOpt;

use std::process;

/// A tree clone written in Rust
#[derive(StructOpt, Debug)]
#[structopt(name = "rstree")]
pub struct Opt {
    /// Colorize output
    #[structopt(short = "c")]
    colorize: bool,

    /// Print all files, including hidden
    #[structopt(short = "a", conflicts_with = "d")]
    show_all: bool,

    /// show only dirs
    #[structopt(short = "d")]
    show_dirs_only: bool,

    /// Set the depth of the iteraton
    #[structopt(short = "L", default_value = "0")]
    level: usize,

    /// Directory to start with
    #[structopt(name = "DIRECTORY", default_value = ".", parse(from_os_str))]
    directory: PathBuf,
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);

    if let Err(e) = rstree::run(
        opt.show_all,
        opt.show_dirs_only,
        opt.colorize,
        opt.level,
        &opt.directory,
    ) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
