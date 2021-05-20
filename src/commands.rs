use structopt::StructOpt;

use crate::new;

pub trait Command {
    fn execute(self) -> anyhow::Result<()>;
}

#[derive(StructOpt, Debug)]
pub enum Forge {
    #[structopt(about = "Create a new project.")]
    New(New),
    #[structopt(about = "Build the project.")]
    Build(Build),
    #[structopt(about = "Manage dependencies.")]
    Deps(Deps),
    #[structopt(about = "Ensure the project is in a good state.")]
    Doctor(Doctor),
}

#[derive(StructOpt, Debug)]
pub enum Deps {
    #[structopt(about = "Sync the deps.")]
    Sync,
}

#[derive(StructOpt, Debug)]
pub struct New {
    #[structopt(about = "Project name.")]
    name: String,
}

impl Command for New {
    fn execute(self) -> anyhow::Result<()> {
        new::new(&self.name)?;
        Ok(())
    }
}

#[derive(StructOpt, Debug)]
pub struct Build {}

impl Command for Build {
    fn execute(self) -> anyhow::Result<()> {
        Ok(())
    }
}

#[derive(StructOpt, Debug)]
pub struct Doctor {}

impl Command for Doctor {
    fn execute(self) -> anyhow::Result<()> {
        todo!()
    }
}
