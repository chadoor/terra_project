use std::collections::HashMap;
use crate::weapon::*;
use crate::model::Model;
use serde_json;
use std::fs::{self, File};

// any other common imports


fn read_json_model_file(file_path: &str) -> HashMap<String, Vec<Model>> {
    let data = fs::read_to_string(file_path).expect("Unable to read file");
    serde_json::from_str(&data).expect("JSON was not well-formatted")
}

pub fn intercessor_squad_builder() -> Vec<Model> {

    let model_map: HashMap<String, Vec<Model>> = read_json_model_file("models.json");
    
    
    let mut unit: Vec<Model> = vec![];

    if let Some(models) = model_map.get("intercessor_squad") {
        for model in models {
            unit.push(model.clone());
        }
    }
    unit
}

pub fn kaballite_warriors_builder() -> Vec<Model> {

    let model_map: HashMap<String, Vec<Model>> = read_json_model_file("models.json");
    
    
    let mut unit: Vec<Model> = vec![];

    if let Some(models) = model_map.get("kaballite_warriors") {
        for model in models {
            unit.push(model.clone());
        }
    }
    unit
}
