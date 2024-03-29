use confy::ConfyError;
use ctbox::network::entity::User;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Root {
    pub network: Network,
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Network {
    pub users: HashMap<String, User>,
    pub default: String,
}

pub fn read() -> Result<Root, ConfyError> {
    confy::load("ctbox", "data")
}

pub fn write(root: Root) -> Result<(), ConfyError> {
    confy::store("ctbox", "data", root)
}

pub fn path() -> PathBuf {
    confy::get_configuration_file_path("ctbox", "data").unwrap()
}
