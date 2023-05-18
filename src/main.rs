use inquire::Select;
use render::Render;
use resources::{base, common, envs};

mod render;
mod resources;

fn menu_config() {
    let env_name = common::select_or_create_dir("kubernetes/envs").unwrap();

    loop {
        let choices: Vec<&str> = vec!["kustomization.yaml", "Secret", "Ingress", "MariaDB", "Exit"];

        let choice = Select::new("Which resource should be created?", choices)
            .prompt()
            .unwrap();

        if choice == "Exit" {
            break;
        }

        add_config_by_type(&env_name, choice).expect("cannot add config");
    }
}

fn add_config_by_type(env_name: &str, typ: &str) -> anyhow::Result<()> {
    use interactive_parse::InteractiveParseObj;

    match typ {
        "Ingress" => envs::Ingress::parse_to_obj()
            .unwrap()
            .render(env_name)
            .expect("cannot create ingress"),
        "kustomization.yaml" => envs::Kustomization::new()
            .render(env_name)
            .expect("cannot create kustomization"),
        "MariaDB" => envs::MariaDB::parse_to_obj()
            .unwrap()
            .render(env_name)
            .expect("cannot create mariadb"),
        "Secret" => envs::Secret::parse_to_obj()
            .unwrap()
            .render(env_name)
            .expect("cannot create secret"),
        &_ => println!("wrong type selected"),
    }

    return Ok(());
}

fn menu_base() {
    let app_name = common::select_or_create_dir("kubernetes/base").unwrap();

    loop {
        let choices: Vec<&str> = vec!["Deployment", "Ingress", "Service", "StatefulSet", "Exit"];

        let choice = Select::new("Which resource should be created?", choices)
            .prompt()
            .unwrap();

        if choice == "Exit" {
            break;
        }

        add_resource_by_type(&app_name, choice).expect("cannot add base resource");
    }
}

fn add_resource_by_type(app_name: &str, typ: &str) -> anyhow::Result<()> {
    use interactive_parse::InteractiveParseObj;

    match typ {
        "Deployment" => base::Deployment::parse_to_obj()
            .unwrap()
            .render(app_name)
            .expect("cannot create deployment"),
        "Ingress" => base::Ingress::parse_to_obj()
            .unwrap()
            .render(app_name)
            .expect("cannot create ingress"),
        "Service" => base::Service::parse_to_obj()
            .unwrap()
            .render(app_name)
            .expect("cannot create service"),
        "StatefulSet" => base::StatefulSet::parse_to_obj()
            .unwrap()
            .render(app_name)
            .expect("cannot create service"),
        &_ => println!("wrong type selected"),
    }

    base::Kustomization::new(app_name)
        .render(app_name)
        .expect("cannot create kustomization.yaml");

    return Ok(());
}

fn main() {
    loop {
        let choices: Vec<&str> = vec!["Add resources", "Add configuration", "Exit"];

        let choice = Select::new("What do you want to do?", choices)
            .prompt()
            .expect("no selection");

        match choice {
            "Add resources" => menu_base(),
            "Add configuration" => menu_config(),
            "Exit" => std::process::exit(0),
            &_ => println!("panic"),
        }
    }
}
