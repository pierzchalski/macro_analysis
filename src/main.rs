#[macro_use]
extern crate quicli;
extern crate reqwest;
extern crate serde_json;
extern crate tar;
extern crate libflate;

use quicli::prelude::*;
use std::path::PathBuf;
use std::io::Read;
use serde_json::Value as JsonValue;
use libflate::gzip;

#[derive(StructOpt, Debug)]
struct Get {
    /// Count of top crates to download.
    top: usize,
    #[structopt(short = "o", long = "out-dir", parse(from_os_str),
            default_value = "downloads")]
    /// Download directory for crate source.
    dl_dir: PathBuf,
}

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

fn round_up(x: usize, n: usize) -> usize {
    let rem = x % n;
    if rem == 0 {
        x
    } else {
        (x / n + 1) * n
    }
}

struct CratesIoClient {
    client: reqwest::Client,
}

#[derive(Debug)]
struct Crate {
    id: String,
    max_version: String,
}

impl CratesIoClient {
    fn new() -> Self {
        CratesIoClient {
            client: reqwest::Client::new(),
        }
    }

    fn get_top(&self, count: usize) -> Result<Vec<Crate>> {
        let max_page = round_up(count, 100) / 100;
        let mut crates = Vec::new();
        for page in 1..(max_page + 1) {
            let mut req = self.client.get("https://crates.io/api/v1/crates?per_page=100&sort=recent-downloads")
                .query(&[("page", page)])
                .send()?;
            let json: JsonValue = req.json()?;
            for krate in json["crates"].as_array().unwrap().iter() {
                crates.push(Crate {
                   id: krate["id"].as_str().unwrap().into(), 
                   max_version: krate["max_version"].as_str().unwrap().into(), 
                });
                if crates.len() == count {
                    break;
                }
            }
        }
        Ok(crates)
    }

    fn get_src(&self, krate: &Crate) -> Result<tar::Archive<Box<Read>>> {
        let url = format!("https://crates.io/api/v1/crates/{}/{}/download", krate.id, krate.max_version);
        let req = self.client.get(&url).send()?;
        let decoder: Box<Read> = Box::new(gzip::Decoder::new(req)?);
        let tar = tar::Archive::new(decoder);
        Ok(tar)
    }
}

#[test]
fn test_round_up() {
    assert_eq!(round_up(300, 100), 300);
    assert_eq!(round_up(301, 100), 400);
}

fn get_cmd(opts: &Get) -> Result<()> {
    let client = CratesIoClient::new();
    let top_crates = client.get_top(opts.top)?;
    for krate in top_crates.iter() {
        let mut tarball = client.get_src(krate)?;
        tarball.unpack(&opts.dl_dir)?;
    }
    Ok(())
}

main!(|opts: Options, log_level: verbosity| {
    match opts.cmd {
        Command::Get(ref get) => get_cmd(get)?,
    };
});
