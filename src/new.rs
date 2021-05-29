use std::{fs, path::PathBuf, process::Command};

use crate::forge::{ForgeFile, Project, Target, TargetType};

const GITIGNORE: &str = include_str!("templates/forge.gitignore");
const MAIN_C: &str = include_str!("templates/main.c");
const LIB_C: &str = include_str!("templates/lib.c");

pub fn new(project_name: &str, is_lib: bool, is_verbose: bool) -> anyhow::Result<()> {
    let project_path = PathBuf::from(project_name);

    let target = if is_lib {
        Target {
            name: project_name.into(),
            include_dirs: vec!["include/".into()],
            src: vec!["src/lib.c".into()],
            r#type: TargetType::Static,
        }
    } else {
        Target {
            name: project_name.into(),
            include_dirs: vec!["include/".into()],
            src: vec!["src/main.c".into()],
            r#type: TargetType::Exe,
        }
    };

    let forge_file = ForgeFile {
        project: Project {
            name: project_name.into(),
            c_flags: String::new(),
            release_flags: String::new(),
            build_type: Default::default(),
        },
        targets: vec![target],
    };

    fs::create_dir_all(project_path.join("src"))?;
    fs::create_dir_all(project_path.join("tests"))?;
    fs::create_dir_all(project_path.join(format!("include/{}", project_name)))?;

    // Initialize git repo
    let _output = Command::new("git")
        .arg("init")
        .arg(&project_path)
        .output()?;

    if is_verbose {
        println!("Initialized git repo in {}", project_name)
    };

    // Create files
    fs::write(project_path.join(".gitignore"), GITIGNORE)?;
    let toml = toml::to_string(&forge_file)?;
    fs::write(project_path.join("Forge.toml"), &toml)?;
    if is_verbose {
        println!("Wrote Forge.toml in {}", project_name)
    };
    if is_lib {
        fs::write(project_path.join("src/lib.c"), LIB_C)?;
    } else {
        fs::write(project_path.join("src/main.c"), MAIN_C)?;
    }

    if is_verbose {
        println!(
            "Wrote {}.c in {}",
            if is_lib { "lib" } else { "main" },
            project_path.join("src").to_str().unwrap()
        )
    };

    println!("Created new project at {}", project_name);
    Ok(())
}
