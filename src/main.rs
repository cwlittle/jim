use clap::{crate_version, App, Arg, SubCommand};
// use std::process::exit;
use jim::*;

fn main() {
    let args = App::new("jim")
        .author("cwlittle, <cwlittle@utexas.edu>")
        .about("Named configuration management for vim")
        .version(concat!("version: ", crate_version!()))
        .subcommand(
            SubCommand::with_name("add")
                .arg(Arg::with_name("name").required(true))
                .arg(Arg::with_name("path").required(true)),
        )
        .subcommand(SubCommand::with_name("list"))
        .arg(Arg::with_name("PROFILE").index(1).required(false))
        .get_matches();

    if let Some(_) = args.subcommand_matches("add") {
        add_profile();
    } else if let Some(_) = args.subcommand_matches("list") {
        list_profiles();
    } else {
        run_default();
    }
}

