use std::{fs, path::Path};

use anyhow::bail;
use inquire::{Text, Select};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::render::Renderable;

pub fn select_or_create() -> anyhow::Result<String> {
    let mut env_names = list();
    env_names.push("Create new".to_string());

    let mut ans = match Select::new("Which env should be used?", env_names).prompt() {
        Ok(ans) => ans,
        Err(e) => bail!(e),
    };

    if ans == "Create new" {
        ans = create().unwrap();
    }

    return Ok(ans);
}

fn create() -> anyhow::Result<String> {
    let name = match Text::new("Env name").prompt() {
        Ok(name) => name,
        Err(e) => bail!(e),
    };

    fs::create_dir_all(format!("kubernetes/envs/{}", name)).unwrap();

    return Ok(name);
}

fn list() -> Vec<String> {
    if !Path::new("kubernetes/envs").exists() {
        return Vec::new();
    }

    let mut output = Vec::new();

    for path in fs::read_dir("kubernetes/envs").expect("") {
        output.push(path.unwrap().file_name().into_string().unwrap());
    }

    return output;
}

#[derive(JsonSchema, Deserialize, Serialize, Debug)]
pub struct Ingress {
    name: String,
    tls: bool,
}

impl Renderable for Ingress {
    fn get_file_name(&self, env_name: &str) -> String {
        return format!("kubernetes/envs/{}/ingress.yaml", env_name);
    }

    fn get_template(&self) -> String {
        return include_str!("../../templates/envs/ingress.yaml").to_string();
    }
}

#[derive(JsonSchema, Deserialize, Serialize, Debug)]
pub struct Kustomization {
    namespace: String,
}

// TODO: generate with resources from disk
impl Renderable for Kustomization {
    fn get_file_name(&self, env_name: &str) -> String {
        return format!("kubernetes/envs/{}/kustomization.yaml", env_name);
    }

    fn get_template(&self) -> String {
        return include_str!("../../templates/envs/kustomization.yaml").to_string();
    }
}

#[derive(JsonSchema, Deserialize, Serialize, Debug)]
pub struct Secret {
    name: String,
}

impl Renderable for Secret {
    fn get_file_name(&self, env_name: &str) -> String {
        return format!("kubernetes/envs/{}/secrets.yaml", env_name);
    }

    fn get_template(&self) -> String {
        return include_str!("../../templates/envs/secrets.yaml").to_string();
    }
}
