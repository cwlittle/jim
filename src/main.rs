use clap::{crate_version, App, Arg, SubCommand};
use jim::*;
use std::str::from_utf8;

fn main() {
    let mut app = App::new("jim")
        .author("cwlittle, <cwlittle@utexas.edu>")
        .about("Named configuration management for vim")
        .version(concat!("version: ", crate_version!()))
        .subcommand(
            SubCommand::with_name("init")
                .about("Initializes jim and adds default profile")
                .arg(Arg::with_name("default_path").required(true)),
        )
        .subcommand(
            SubCommand::with_name("add")
                .about("Adds a configuration profile to jim")
                .arg(Arg::with_name("name").required(true))
                .arg(Arg::with_name("path").required(true)),
        )
        .subcommand(SubCommand::with_name("list").about("Lists all available profiles for use"))
        .arg(
            Arg::with_name("PROFILE")
                .index(1)
                .conflicts_with_all(&["init", "add", "list", "help"])
                .required(false),
        );

    let mut help = Vec::new();
    app.write_long_help(&mut help).unwrap();

    let args = app.get_matches();

    if std::env::args().len() == 1 {
        match run_profile("default") {
            Err(e) => {
                println!("\tFailed to run default profile\n\tError: {}", e);
            }
            _ => {}
        };
    } else if let Some(profile) = args.value_of("PROFILE") {
        match run_profile(profile) {
            Err(e) => {
                println!("\tFailed to run profile {}\n\tError: {}", profile, e);
            }
            _ => {}
        };
    } else if let Some(arg) = args.subcommand_matches("add") {
        let name = arg.value_of("name").unwrap();
        let path = arg.value_of("path").unwrap();
        match add_profile(arg.value_of("name").unwrap(), arg.value_of("path").unwrap()) {
            Err(e) => {
                println!("\tFailed to add profile {}: {}\n\tError: {}", name, path, e);
            }
            _ => {}
        };
    } else if let Some(_) = args.subcommand_matches("list") {
        match list_profiles() {
            Err(e) => {
                println!("\tFailed to list profiles\n\tError: {}", e);
            }
            _ => {}
        };
    } else if let Some(arg) = args.subcommand_matches("init") {
        match init(arg.value_of("default_path").unwrap()) {
            Err(e) => {
                println!("\tFailed to initialize jim\n\tError: {}", e);
            }
            _ => {}
        };
    } else {
        println!("{}", from_utf8(&help).unwrap());
    }
}
