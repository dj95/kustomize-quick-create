use std::{fs, path::Path};

use anyhow::bail;
use inquire::{Select, Text};

pub fn select_or_create_dir(base_dir: &str) -> anyhow::Result<String> {
    let mut env_names = list(base_dir);
    env_names.push("Create new".to_string());

    let mut ans = match Select::new("Which env should be used?", env_names).prompt() {
        Ok(ans) => ans,
        Err(e) => bail!(e),
    };

    if ans == "Create new" {
        ans = create(base_dir).unwrap();
    }

    return Ok(ans);
}

fn create(base_dir: &str) -> anyhow::Result<String> {
    let name = match Text::new("Env name").prompt() {
        Ok(name) => name,
        Err(e) => bail!(e),
    };

    fs::create_dir_all(format!("{}/{}", base_dir, name)).unwrap();

    return Ok(name);
}

pub fn list(base_dir: &str) -> Vec<String> {
    if !Path::new(base_dir).exists() {
        return Vec::new();
    }

    let mut output = Vec::new();

    for path in fs::read_dir(base_dir).expect("") {
        output.push(path.unwrap().file_name().into_string().unwrap());
    }

    return output;
}
