use std::fs::File;
use std::io::prelude::*;
use toml::Value;

pub fn get_file_contents(file_name: &str) -> String {
    let mut file = File::open(file_name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

pub fn get_toml_values_from_toml_file(file_name: &str) -> Value {
    let contents = get_file_contents(file_name);
    contents.parse::<Value>().unwrap()
}

pub fn get_xrdb_value_from_toml_value(v: &Value) -> String {
    let xrdb_v: String;

    if v.is_str() {
        xrdb_v = format!("{}", v.as_str().unwrap());
    } else if v.is_bool() {
        xrdb_v = format!("{}", v);
    } else if v.is_integer() {
        xrdb_v = format!("{}", v);
    } else {
        xrdb_v = String::from("UNSUPPORTED_DATA_TYPE");
        // println!("! {}.{} value is an unsupported data type", mod_name, k);
    }
    xrdb_v
}
