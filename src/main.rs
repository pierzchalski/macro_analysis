#[macro_use]
extern crate structopt;

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "ma")]
enum Opt {
    #[structopt(name = "get")]
    Get {
        #[structopt(help = "count of top crates to get")]
        top: u32,
        #[structopt(short = "o", long = "out-dir", parse(from_os_str),
                default_value = ".", help = "directory to download crate source to")]
        dl_dir: PathBuf,
    }
}

fn round_up(x: u32, n: u32) -> u32 {
    let rem = x % n;
    if rem == 0 {
        x
    } else {
        (x / n + 1) * n
    }
}

#[test]
fn test_round_up() {
    assert_eq!(round_up(300, 100), 300);
    assert_eq!(round_up(301, 100), 400);
}

fn main() {
    let opts = Opt::from_args();
    println!("{:#?}", opts);
}
