use serde::{Deserialize, Serialize};
use serde_json::{self, Value};
use std::collections::HashMap;
use std::fs::{self, File};
use std::{env, path::Path};
use tera::{Context, Tera};
use std::io::Write;

mod lib;

use crate::lib::*;

fn read_json_file(file_path: &str) -> HashMap<String, Vec<Person>> {
    let data = fs::read_to_string(file_path).expect("Unable to read file");
    serde_json::from_str(&data).expect("JSON was not well-formatted")
}

fn read_json_model_file(file_path: &str) -> HashMap<String, Vec<Model>> {
    let data = fs::read_to_string(file_path).expect("Unable to read file");
    serde_json::from_str(&data).expect("JSON was not well-formatted")
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Person {
    name: String,
    occupation: String,
}


fn main(){
    
}

fn main2() {
    let model_map: HashMap<String, Vec<Model>> = read_json_model_file("models.json");

    let mut file = File::create("output.txt").expect("Failed to create file");

    writeln!(file, "Test").expect("Failed to write imports to file");

    let tera = match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };

    // let import_rendered = tera
    //     .render("imports_template.tera", &Context::new())
    //     .expect("Failed to render import template");
    // writeln!(file, "{}",import_rendered).expect("Failed to write imports to file");
     let out_dir = env::var("OUT_DIR").unwrap();
     let dest_path = Path::new(&out_dir).join("units.rs");

    // fs::write(&dest_path, import_rendered).expect("Failed to write generated code");

    for (key, models) in model_map {
        let key_lower = key.to_lowercase();

        writeln!(file, "{}",key_lower).expect("Failed to write imports to file");

        let mut context = Context::new();
        context.insert("key", &key_lower);

        let rendered = tera
            .render("unit_builder_function_template.tera", &context)
            .expect("Failed to render template");

        writeln!(file, "{}",rendered).expect("Failed to write imports to file");

        fs::write(&dest_path, rendered).expect("Failed to write generated code");
    }
    println!("Ran");
}

fn main1() {
    let people_map = read_json_file("data.json");

    let tera = match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };
    let mut file = File::create("src/Person.rs").expect("Failed to create file");

    for (key, people) in people_map {
        let key_lower = key.to_lowercase();
        for person in people {
            let mut context = Context::new();
            context.insert("key", &key_lower);
            context.insert("name", &person.name);
            context.insert("occupation", &person.occupation);

            let rendered = tera
                .render("template.tera", &context)
                .expect("Failed to render template");

            let out_dir = env::var("OUT_DIR").unwrap();
            let dest_path = Path::new(&out_dir).join("generated.rs");
            fs::write(&dest_path, rendered).expect("Failed to write generated code");
        }
    }
    writeln!(file,"cargo:warning=The OUT_DIR is located at: {}", env::var("OUT_DIR").unwrap()).expect("err");

}
