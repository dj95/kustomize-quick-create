use std::fs;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::render::Renderable;

#[derive(Serialize)]
pub struct Kustomization {
    pub files: Vec<String>,
}

impl Kustomization {
    pub fn new(app_name: &str) -> Self {
        let mut files = Vec::new();

        for path in fs::read_dir(format!("kubernetes/base/{}/", app_name)).expect("") {
            let filename = path.unwrap().file_name().into_string().unwrap();

            if filename == "kustomization.yaml" {
                continue;
            }

            files.push(filename);
        }

        return Self { files };
    }
}

impl Renderable for Kustomization {
    fn get_file_name(&self, app_name: &str) -> String {
        return format!("kubernetes/base/{}/kustomization.yaml", app_name);
    }

    fn get_template(&self) -> String {
        return include_str!("../../templates/kustomization.yaml").to_string();
    }
}

#[derive(JsonSchema, Deserialize, Serialize, Debug)]
pub struct Ingress {
    name: String,
    domain: String,
    tls: bool,
}

impl Renderable for Ingress {
    fn get_file_name(&self, env_name: &str) -> String {
        return format!("kubernetes/base/{}/ingress.yaml", env_name);
    }

    fn get_template(&self) -> String {
        return include_str!("../../templates/base/ingress.yaml").to_string();
    }
}

#[derive(JsonSchema, Deserialize, Serialize, Debug)]
pub struct Deployment {
    name: String,
    container_image: String,
    port: i32,
}

impl Renderable for Deployment {
    fn get_file_name(&self, env_name: &str) -> String {
        return format!("kubernetes/base/{}/deployment.yaml", env_name);
    }

    fn get_template(&self) -> String {
        return include_str!("../../templates/base/deployment.yaml").to_string();
    }
}

#[derive(JsonSchema, Deserialize, Serialize, Debug)]
pub struct Service {
    name: String,
    port: i32,
}

impl Renderable for Service {
    fn get_file_name(&self, env_name: &str) -> String {
        return format!("kubernetes/base/{}/service.yaml", env_name);
    }

    fn get_template(&self) -> String {
        return include_str!("../../templates/base/service.yaml").to_string();
    }
}

#[derive(JsonSchema, Deserialize, Serialize, Debug)]
pub struct StatefulSet {
    name: String,
    container_image: String,
    port: i32,
}

impl Renderable for StatefulSet {
    fn get_file_name(&self, env_name: &str) -> String {
        return format!("kubernetes/base/{}/statefulset.yaml", env_name);
    }

    fn get_template(&self) -> String {
        return include_str!("../../templates/base/statefulset.yaml").to_string();
    }
}
