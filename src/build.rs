use crate::forge::ForgeFile;
use crate::ninja::NinjaWriter;
use anyhow::Error;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::Path;

pub fn setup(forge: &ForgeFile) -> anyhow::Result<()> {
    let build_dir = Path::new("_build");
    if build_dir.exists() {
        return Err(Error::msg(
            "Build directory already exists. Consider running repair.",
        ));
    }
    let build_path = build_dir
        .join(forge.project.build_type.to_string())
        .join(forge.project.name.clone());
    create_dir_all(&build_path)?;

    let ninja_path = build_path.join("build.ninja");
    let mut ninja_file = File::create(ninja_path)?;
    let mut ninja = NinjaWriter::new();
    ninja.rule(
        "cc",
        "gcc $cflags -MD -MF $out.d -c $in -o $out",
        Some("Compile with C compiler."),
        Some("$out.d"),
        Some("gcc"),
        None,
    );
    ninja.rule(
        "link",
        "gcc $in -o $out",
        Some("Link with C linker."),
        None,
        None,
        None,
    );

    for _target in &forge.targets {}

    write!(ninja_file, "{}", ninja).unwrap();
    Ok(())
}
