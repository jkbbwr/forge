#![allow(dead_code)]

use crate::build::setup;
use crate::forge::ForgeFile;
use crate::ninja::NinjaWriter;
use anyhow::Error;
use globwalk::{GlobError, WalkError};
use std::path::{Path, PathBuf};
use structopt::StructOpt;

mod build;
mod forge;
mod ninja;

#[derive(StructOpt, Debug)]
enum Forge {
    #[structopt(about = "Create a new project.")]
    New,
    #[structopt(about = "Setup a new build directory.")]
    Init,
    #[structopt(about = "Manage dependencies.")]
    Deps(Deps),
    #[structopt(about = "Repair an existing build directory.")]
    Repair,
    #[structopt(about = "Ensure the project is in a good state.")]
    Doctor
}

#[derive(StructOpt, Debug)]
enum Deps {
    #[structopt(about = "Sync the deps.")]
    Sync
}

/*
   println!("{:#?}", forge);
   let mut ninja = NinjaWriter::new();
   ninja.rule("cc", "gcc $cflags -MD -MF $out.d -c $in -o $out", Some("Compile with C compiler."), Some("$out.d"), Some("gcc"), None);
   ninja.rule("link", "gcc $in -o $out", Some("Link with C linker."), None, None, None);
   ninja.build(&["_build/hello-world/main.o"], "cc", &["src/main.c"], &[]);
   ninja.build(&["_build/hello-world"], "link", &["_build/hello-world/main.o"], &[]);
    let forge = Forge::from_path("./Forge.toml".as_ref())?;
    println!("{:#?}", forge);
   setup(&forge)?;
*/
fn main() -> anyhow::Result<()> {
    let app = Forge::from_args();
    println!("{:#?}", app);
    Ok(())
}
