use std::result::Result as StdResult;
use quicli::prelude::*;
use std::fmt::Debug;
use std::path::{PathBuf, Path};
use std::fs::File;
use std::io::Read;
use glob::glob;
use regex::Regex;
use syn;
use syn::visit::Visit;
use syn::ItemMacro;
use quote::ToTokens;

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

#[derive(Debug)]
struct ItemMacroCollector {
    macros: Vec<ItemMacro>,
}

impl ItemMacroCollector {
    fn new() -> Self {
        ItemMacroCollector {
            macros: Vec::new(),
        }
    }
}

impl<'ast> Visit<'ast> for ItemMacroCollector {
    fn visit_item_macro(&mut self, item: &'ast ItemMacro) {
        let macro_rules: syn::Path = parse_quote!(macro_rules);
        let ref path = item.mac.path;
        if path == &macro_rules {
            println!("{}", item.clone().into_tokens());
        }
    }
}

impl Extract {

    fn macros(&self, src_file: PathBuf) -> Result<()> {
        let mut src_file = File::open(src_file)?;
        let mut src = String::new();
        src_file.read_to_string(&mut src)?;
        let file = syn::parse_file(&src)?;
        let mut collector = ItemMacroCollector::new();
        collector.visit_file(&file);
        for mac in collector.macros.iter() {
            println!("{}", mac.clone().into_tokens());
        }
        Ok(())
    }

    fn process_src_dir(&self, path: PathBuf) -> Result<()> {
        let mut glob_path = path.clone();
        glob_path.push("**");
        glob_path.push("*.rs");
        let src_files = filter_warn(glob(&glob_path.to_string_lossy())?);
        for file in src_files {
            self.macros(file)?;
        }
        Ok(())
    }

    pub fn run(&self) -> Result<()> {
        for src_dir in filter_warn(self.src_dir.read_dir()?) {
            self.process_src_dir(src_dir.path())?;
        }
        Ok(())
    }
}
