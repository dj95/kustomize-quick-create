use std::fs;

use handlebars::Handlebars;
use serde::Serialize;

pub trait Renderable {
    fn get_file_name(&self, env_name: &str) -> String;
    fn get_template(&self) -> String;
}


pub trait Render: Serialize {
    fn render(&self, name: &str) -> anyhow::Result<()>;
}

impl<T> Render for T
where
    T: Renderable + serde::Serialize,
{
    fn render(&self, name: &str) -> anyhow::Result<()> {
        let reg = Handlebars::new();

        let res = reg.render_template(&self.get_template(), self)?;

        fs::write(self.get_file_name(name), res).unwrap();

        return Ok(());
    }
}
