use std::fs;
use toml::Value;
use std::process::{Command, Stdio};
use std::fs::File;
use std::io::Write;

pub fn init() {
    println!("init");
} 

pub fn add_profile(name: &str, path: &str) {
    println!("hit here");
    let toml_string = fs::read_to_string("jim.toml").unwrap();
    let profile_string = format!("{} = \"{}\"", name, path);
    let new_toml = toml_string + &profile_string;
    let mut out_file = File::create("jim.toml").unwrap();
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

pub fn run_default() {
    let config = read_profiles();
    let default_path: String = match config.get("default") {
        Some(path) => toml::to_string(path).unwrap(),
        None => panic!("No default profile specified"),
    };
    run_vim(default_path.replace("\"", ""))
}

fn read_profiles() -> Value {
    let toml_string = fs::read_to_string("jim.toml").unwrap();
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
