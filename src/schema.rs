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
    // WIP: support additional properties
}

#[derive(Debug, Deserialize)]
pub struct PackageSchema {
    pub lsp: String,
}
