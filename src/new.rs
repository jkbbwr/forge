use std::{fs, path::PathBuf, process::Command};

use crate::forge::{ForgeFile, Project, Target, TargetType};

const GITIGNORE: &str = include_str!("templates/forge.gitignore");
const MAIN_C: &str = include_str!("templates/main.c");
const LIB_C: &str = include_str!("templates/lib.c");

pub fn new(project_name: &str) -> anyhow::Result<()> {
    let project_path = PathBuf::from(project_name);

    let forge_file = ForgeFile {
        project: Project {
            name: project_name.into(),
            c_flags: String::new(),
            release_flags: String::new(),
            build_type: Default::default(),
        },
        targets: vec![Target {
            name: project_name.into(),
            include_dirs: vec!["include/".into()],
            src: vec!["src/main.c".into()],
            r#type: TargetType::Exe,
        }],
    };

    fs::create_dir_all(project_path.join("src"))?;
    fs::create_dir_all(project_path.join("tests"))?;
    fs::create_dir_all(project_path.join(format!("include/{}", project_name)))?;

    // Initialize git repo
    let _output = Command::new("git")
        .arg("init")
        .arg(&project_path)
        .output()?;

    // Create files
    fs::write(project_path.join(".gitignore"), GITIGNORE)?;
    let toml = toml::to_string(&forge_file)?;
    fs::write(project_path.join("Forge.toml"), &toml)?;
    fs::write(project_path.join("src/main.c"), MAIN_C)?;
    Ok(())
}
