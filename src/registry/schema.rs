use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Registry(Vec<Package>);

#[derive(Debug, Deserialize)]
pub struct Package {
    pub name: String,
    pub description: String,
    pub homepage: String,
    pub licenses: Vec<String>,
    pub categories: Vec<Category>,
    pub source: PackageSource,
    pub bin: Option<HashMap<String, String>>,
    pub schemas: Option<PackageSchema>,
    pub share: Option<HashMap<String, String>>,
    pub opt: Option<HashMap<String, String>>,
    pub deprecation: Option<PackageDeprecation>,
}

#[derive(Debug, Deserialize)]
pub enum Category {
    Compiler,
    DAP,
    Formatter,
    LSP,
    Linter,
    Runtime,
}

#[derive(Debug, Deserialize)]
pub struct PackageSource {
    pub id: String,
    pub asset: Option<PackageAsset>,
    pub build: Option<PackageBuild>,
    pub download: Option<PackageDownload>,
    pub extra_packages: Option<Vec<String>>,
    pub version_overrides: Option<Vec<PackageVersionOverride>>,
    pub supported_platforms: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum PackageAsset {
    SingleFile { file: String },
    PlatformDependant(Vec<PlatformAsset>),
}

#[derive(Debug, Deserialize)]
pub struct PlatformAsset {
    pub target: PlatformTarget,
    pub file: PackageFile,
    pub bin: Option<PackageBin>,
}

#[derive(Debug, Deserialize)]
pub struct PackageSchema {
    pub lsp: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum PlatformTarget {
    Single(String),
    Multiple(Vec<String>),
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum PackageFile {
    Single(String),
    Multiple(Vec<String>),
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum PackageBin {
    Single(String),
    Multiple(HashMap<String, String>),
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum PackageBuild {
    Run { run: String },
    PlatformDependant(Vec<PlatformBuild>),
}

#[derive(Debug, Deserialize)]
pub struct PlatformBuild {
    pub target: PlatformTarget,
    pub run: String,
    pub staged: Option<bool>,
    pub env: Option<HashMap<String, String>>,
    pub bin: Option<PackageBin>,
    #[serde(flatten)]
    pub extra: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum PackageDownload {
    Single { file: String },
    Multiple { files: HashMap<String, String> },
    PlatformDependant(Vec<PlatformDownload>),
}

#[derive(Debug, Deserialize)]
pub struct PlatformDownload {
    pub target: PlatformTarget,
    pub files: HashMap<String, String>,
    pub config: Option<String>,
    pub bin: Option<PackageBin>,
    pub man: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PackageVersionOverride {
    pub constraint: String,
    pub id: String,
    pub bin: Option<String>,
    pub build: Option<PackageBuild>,
    pub asset: Option<PackageAsset>,
    pub extra_packages: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct PackageDeprecation {
    pub since: String,
    pub message: String,
}
