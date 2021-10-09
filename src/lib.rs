use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::process::{Command, Stdio};
use toml::Value;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Profile not found: {0}")]
    ProfileNotFound(String),
    #[error("Key not found in manifest: {0}")]
    Manifest(String),
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error(transparent)]
    TomlSerialize(#[from] toml::ser::Error),
    #[error(transparent)]
    TomlDeserialize(#[from] toml::de::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

pub fn init(default_path: &str) -> Result<()> {
    let current_exe_path = env::current_exe()?;
    let resource_path = current_exe_path.parent().unwrap().join("resources");
    //probably should check if this directory exists first or else it will nuke everthing or panic
    fs::create_dir(resource_path.clone())?;
    let toml_path = resource_path.join("jim.toml");
    let mut out_file = File::create(toml_path)?;
    let default = format!("[profiles]\ndefault = \"{}\"", default_path);
    write!(out_file, "{}", default)?;
    Ok(())
}

pub fn add_profile(name: &str, path: &str) -> Result<()> {
    let current_exe_path = env::current_exe()?;
    let resource_path = current_exe_path.parent().unwrap().join("resources");
    let toml_path = resource_path.join("jim.toml");
    let toml_string = fs::read_to_string(toml_path.clone())?;
    let profile_string = format!("\n{} = \"{}\"", name, path);
    let new_toml = toml_string + &profile_string;
    let mut out_file = File::create(toml_path)?;
    write!(out_file, "{}", &new_toml)?;
    Ok(())
}

pub fn list_profiles() -> Result<()> {
    let profiles = read_profiles()?;
    for item in profiles.as_table().iter() {
        for (k, _) in item.iter() {
            println!("\t{}", k);
        }
    }
    Ok(())
}

pub fn run_profile(profile: &str) -> Result<()> {
    let config = read_profiles()?;
    let default_path: String = match config.get(profile) {
        Some(path) => toml::to_string(path)?,
        None => return Err(Error::ProfileNotFound(profile.to_string())),
    };
    run_vim(default_path.replace("\"", ""))?;
    Ok(())
}

fn read_profiles() -> Result<Value> {
    let current_exe_path = env::current_exe()?;
    let resource_path = current_exe_path.parent().unwrap().join("resources");
    let toml_path = resource_path.join("jim.toml");
    let toml_string = fs::read_to_string(toml_path).unwrap();
    let config = &toml_string.parse::<Value>()?;
    match config.get("profiles") {
        Some(profiles) => Ok(profiles.to_owned()),
        None => Err(Error::Manifest("profiles".to_string())),
    }
}

fn run_vim(profile_path: String) -> Result<()> {
    let mut child = Command::new("nvim")
        .arg("-u")
        .arg(profile_path)
        .stdout(Stdio::inherit())
        .spawn()?;

    child.wait()?;
    Ok(())
}
