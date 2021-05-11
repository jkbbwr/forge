#![allow(dead_code)]

use crate::commands::Forge;
use structopt::StructOpt;

mod build;
mod forge;
mod ninja;
mod commands;

use commands::Command;
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

    match app {
        Forge::New(new) => new.execute()?,
        Forge::Deps(_) => {}
        Forge::Build(_) => {}
        Forge::Doctor(_) => {}
    }

    Ok(())
}
