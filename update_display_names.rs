extern crate rmp_serde as rmps;

use rmps::Serializer;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs};

#[derive(Deserialize)]
struct Item {
    internalname: String,
    displayname: String,
}

fn remove_color_codes(input_str: &str) -> String {
    let mut result = String::new();
    let mut chars = input_str.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '§' {
            // skip the color code
            let _ = chars.next();
            continue;
        }
        result.push(c);
    }
    result
}

fn main() {
    if !std::path::Path::new("./neudata").exists() {
        std::process::Command::new("git")
            .args([
                "clone",
                "https://github.com/NotEnoughUpdates/NotEnoughUpdates-REPO",
                "neudata",
            ])
            .output()
            .unwrap();
    } else {
        std::process::Command::new("git")
            .args(["pull"])
            .current_dir("./neudata")
            .output()
            .unwrap();
    }

    let mut data = HashMap::new();
    for file in fs::read_dir("./neudata/items").unwrap() {
        let file = file.unwrap();
        if !file.file_type().unwrap().is_dir() && file.path().extension().unwrap() == "json" {
            let file = fs::read(file.path()).unwrap();
            let file: Item = serde_json::from_slice(&file).unwrap();
            let displayname = remove_color_codes(&file.displayname);
            data.insert(file.internalname, displayname);
        }
    }
    let mut buf = Vec::new();
    data.serialize(&mut Serializer::new(&mut buf)).unwrap();
    fs::write("./resources/display-names.bin", buf).unwrap();
}
