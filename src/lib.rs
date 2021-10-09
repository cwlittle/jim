use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::process::{Command, Stdio};
use toml::Value;

pub fn init(default_path: &str) {
    let current_exe_path = env::current_exe().unwrap();
    let resource_path = current_exe_path.parent().unwrap().join("resources");
    //probably should check if this directory exists first or else it will nuke everthing or panic
    println!("{:?}", resource_path);
    fs::create_dir(resource_path.clone()).unwrap();
    let toml_path = resource_path.join("jim.toml");
    let mut out_file = File::create(toml_path).unwrap();
    let default = format!("[profiles]\ndefault = \"{}\"", default_path);
    write!(out_file, "{}", default).unwrap();
}

pub fn add_profile(name: &str, path: &str) {
    let current_exe_path = env::current_exe().unwrap();
    let resource_path = current_exe_path.parent().unwrap().join("resources");
    let toml_path = resource_path.join("jim.toml");
    let toml_string = fs::read_to_string(toml_path.clone()).unwrap();
    let profile_string = format!("\n{} = \"{}\"", name, path);
    let new_toml = toml_string + &profile_string;
    let mut out_file = File::create(toml_path).unwrap();
    write!(out_file, "{}", &new_toml).unwrap();
}

pub fn list_profiles() {
    let profiles = read_profiles();
    for item in profiles.as_table().iter() {
        for (k, _) in item.iter() {
            println!("\t{}", k);
        }
    }
}

pub fn run_profile(profile: &str) {
    let config = read_profiles();
    let default_path: String = match config.get(profile) {
        Some(path) => toml::to_string(path).unwrap(),
        None => panic!("Profile does not exist"),
    };
    run_vim(default_path.replace("\"", ""))
}

fn read_profiles() -> Value {
    let current_exe_path = env::current_exe().unwrap();
    let resource_path = current_exe_path.parent().unwrap().join("resources");
    let toml_path = resource_path.join("jim.toml");
    let toml_string = fs::read_to_string(toml_path).unwrap();
    let config = &toml_string.parse::<Value>().unwrap();
    match config.get("profiles") {
        Some(profiles) => profiles.to_owned(),
        None => panic!("What the fuck have you done to the toml?"),
    }
}

fn run_vim(profile_path: String) {
    Command::new("nvim")
        .arg("-u")
        .arg(profile_path)
        .stdout(Stdio::inherit())
        .spawn()
        .expect("Failed to spawn vim instance")
        .wait()
        .expect("Error closing vim instance");
}
