#[macro_use]
extern crate quicli;
extern crate reqwest;
extern crate serde_json;
extern crate tar;
extern crate libflate;

use quicli::prelude::*;

mod commands;
use commands::Get;

#[derive(StructOpt, Debug)]
enum Command {
    #[structopt(name = "get")]
    /// Download top crates from crates.io
    Get(Get),
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
    };
});
