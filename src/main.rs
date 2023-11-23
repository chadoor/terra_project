use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Write;
use tera::{Context, Tera};
use std::{env, path::Path};

mod model;
mod unit_builder;
mod weapon;

use crate::model::Model;
// use crate::unit_builder::*;
use crate::weapon::*;

include!(concat!(env!("OUT_DIR"), "/units.rs"));


#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    occupation: String,
}

fn read_json_file(file_path: &str) -> HashMap<String, Vec<Person>> {
    let data = fs::read_to_string(file_path).expect("Unable to read file");
    serde_json::from_str(&data).expect("JSON was not well-formatted")
}

fn read_json_model_file(file_path: &str) -> HashMap<String, Vec<Model>> {
    let data = fs::read_to_string(file_path).expect("Unable to read file");
    serde_json::from_str(&data).expect("JSON was not well-formatted")
}




fn main() {
    // let model_map: HashMap<String, Vec<Model>> = read_json_model_file("models.json");

    // let tera = match Tera::new("templates/**/*") {
    //     Ok(t) => t,
    //     Err(e) => {
    //         println!("Parsing error(s): {}", e);
    //         ::std::process::exit(1);
    //     }
    // };
    // let mut file = File::create("src/unit_builder.rs").expect("Failed to create file");

    // let import_rendered = tera
    //     .render("imports_template.tera", &Context::new())
    //     .expect("Failed to render import template");
    // writeln!(file, "{}", import_rendered).expect("Failed to write imports to file");

    // for (key, models) in model_map {
    //     let key_lower = key.to_lowercase();

    //     let mut context = Context::new();
    //     context.insert("key", &key_lower);

    //     let rendered = tera
    //         .render("unit_builder_function_template.tera", &context)
    //         .expect("Failed to render template");

    //     writeln!(file, "{}", rendered).expect("Failed to write to file");
    // }

    // for model in a {
    //     println!("{:#?}", model);
    // }
    // let b = intercessor_squad_builder();
    //let a = intercessor_squad_builder();
    // println!("cargo:warning=The OUT_DIR is located at: {}", env::var("OUT_DIR").unwrap());

    // let a = kaballite_warriors_builder();
    // for model in a {
    //     println!("{:#?}", model);
    // }

}
