use quicli::prelude::*;
use std::result::Result as StdResult;
use std::fmt::Debug;
use std::path::PathBuf;
use glob::glob;

type Paths<'i> = Box<Iterator<Item = PathBuf> + 'i>;

#[derive(StructOpt, Debug)]
pub struct Extract {
    #[structopt(short = "i", long = "in-dir", parse(from_os_str), default_value = "downloads")]
    /// Directory to look for downloaded crate source code.
    /// Expects crates to be layed out as <in-dir>/crate_name.
    src_dir: PathBuf,

    #[structopt(short = "o", long = "out-dir", parse(from_os_str), default_value = "extracts")]
    /// Directory to send `macro_rules!` extracts.
    /// The files in <in-dir>/crate_name/* will have their results
    /// placed in <out-dir>/crate_name_extracts.rs.
    extracts_dir: PathBuf,
}

fn filter_warn<'t, T, I, E>(iter: T) -> Box<Iterator<Item = I> + 't>
where
    T: Iterator<Item = StdResult<I, E>> + 't,
    E: Debug,
{
    Box::new(iter.filter_map(|item| match item {
        Ok(item) => Some(item),
        Err(err) => {
            warn!("{:#?}", err);
            None
        }
    }))
}

impl Extract {
    pub fn run(&self) -> Result<()> {
        for src_dir in filter_warn(self.src_dir.read_dir()?) {
            println!("{}", src_dir.path().display());
        }
        Ok(())
    }
}
