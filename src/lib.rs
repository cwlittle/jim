use std::fs;
use toml::Value;

pub fn add_profile() {
    println!("add_profile");
}

pub fn list_profiles() {
    println!("list_profiles");
}

pub fn run_default() {
    let config = read_profiles();
    let default_path: String = match config.get("default") {
        Some(path) => toml::to_string(path).unwrap(),
        None => panic!("No default profile specified"),
    };
    println!("{}", default_path.replace("\"", ""));
    println!("Hello World");
}

fn read_profiles() -> Value {
    let toml_string = fs::read_to_string("svim.toml").unwrap();
    let config = &toml_string.parse::<Value>().unwrap();
    match config.get("profiles") {
        Some(profiles) => profiles.to_owned(),
        None => panic!("What the fuck have you done to the toml?"),
    }
}

