use inquire::Text;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::render::Renderable;

use super::common;

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

#[derive(Deserialize, Serialize, Debug)]
pub struct Kustomization {
    namespace: String,
    files: Vec<String>,
}

impl Kustomization {
    pub fn new() -> Self {
        let namespace = Text::new("namespace").prompt().unwrap();
        let files = common::list("kubernetes/base");

        return Self { namespace, files };
    }
}

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
