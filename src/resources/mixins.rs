use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::render::Renderable;

#[derive(JsonSchema, Deserialize, Serialize, Debug)]
pub struct MariaDB {
    suffix: String,
}

impl Renderable for MariaDB {
    fn get_file_name(&self, env_name: &str) -> String {
        return format!("kubernetes/envs/{}/mariadb.yaml", env_name);
    }

    fn get_template(&self) -> String {
        return include_str!("../../templates/mixins/mariadb.yaml").to_string();
    }
}

#[derive(JsonSchema, Deserialize, Serialize, Debug)]
pub struct Redis {
    suffix: String,
}

impl Renderable for Redis {
    fn get_file_name(&self, env_name: &str) -> String {
        return format!("kubernetes/envs/{}/redis.yaml", env_name);
    }

    fn get_template(&self) -> String {
        return include_str!("../../templates/mixins/redis.yaml").to_string();
    }
}
