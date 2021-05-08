use globwalk::{GlobError, WalkError};
use serde::de::{DeserializeOwned, Error};
use serde::{Deserialize, Deserializer};
use std::ffi::OsStr;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::fs::read_to_string;
use std::path::PathBuf;
use toml::from_str;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum TargetType {
    Exe,
    Static,
    Shared,
}

impl Display for TargetType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Ok(write!(
            f,
            "{}",
            match self {
                TargetType::Exe => "exe",
                TargetType::Shared => "shared",
                TargetType::Static => "static",
            }
        )?)
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum BuildType {
    Release,
    Debug,
}

impl Display for BuildType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Ok(write!(
            f,
            "{}",
            match self {
                BuildType::Release => "release",
                BuildType::Debug => "debug",
            }
        )?)
    }
}

impl Default for BuildType {
    fn default() -> Self {
        BuildType::Release
    }
}

#[derive(Deserialize, Debug)]
pub struct ForgeFile {
    pub project: Project,
    #[serde(rename = "target")]
    pub targets: Vec<Target>,
}

fn expand_glob<'de, D>(deserializer: D) -> Result<Vec<PathBuf>, D::Error>
    where
        D: Deserializer<'de>,
{
    let patterns: Vec<String> = Deserialize::deserialize(deserializer)?;
    let mut results = Vec::new();
    for pattern in patterns {
        for walker in globwalk::glob(pattern) {
            for path in walker {
                results.push(path.map_err(|_e| {
                    D::Error::custom("Failed to walk.")
                })?.into_path())
            }
        }
    }

    Ok(results)
}

#[derive(Deserialize, Debug)]
pub struct Target {
    pub name: String,
    pub r#type: TargetType,
    #[serde(default)]
    pub include_dirs: Vec<String>,
    #[serde(deserialize_with = "expand_glob")]
    pub src: Vec<PathBuf>,
}

fn release_flags() -> String {
    "-O3".into()
}

#[derive(Deserialize, Debug)]
pub struct Project {
    pub name: String,
    pub c_flags: String,
    #[serde(default = "release_flags")]
    pub release_flags: String,
    #[serde(default)]
    pub build_type: BuildType,
}

impl ForgeFile {
    pub fn from_path(path: &OsStr) -> anyhow::Result<Self> {
        let data = read_to_string(path)?;
        Ok(from_str(&data)?)
    }
}
