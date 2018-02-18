extern crate glob;
extern crate libflate;
#[macro_use]
extern crate quicli;
extern crate reqwest;
extern crate serde_json;
extern crate tar;

use quicli::prelude::*;

mod commands;
use commands::{Extract, Get};

#[derive(StructOpt, Debug)]
enum Command {
    #[structopt(name = "get")]
    /// Download top crates from crates.io
    Get(Get),

    #[structopt(name = "extract")]
    /// Extract `macro_rules!` definitions
    Extract(Extract),
}

#[derive(StructOpt, Debug)]
#[structopt(name = "macro_analysis")]
struct Options {
    #[structopt(short = "v", parse(from_occurrences))]
    /// Verbosity level. Repeat for higher verbosity.
    verbosity: usize,
    #[structopt(subcommand)]
    cmd: Command,
}

main!(|opts: Options, log_level: verbosity| {
    match opts.cmd {
        Command::Get(ref get) => get.run()?,
        Command::Extract(ref extract) => extract.run()?,
    };
});
