use clap::{crate_version, App, Arg, SubCommand};
// use std::process::exit;
use jim::*;

fn main() {
    let args = App::new("jim")
        .author("cwlittle, <cwlittle@utexas.edu>")
        .about("Named configuration management for vim")
        .version(concat!("version: ", crate_version!()))
        .subcommand(
            SubCommand::with_name("init").arg(Arg::with_name("default_path").required(true)),
        )
        .subcommand(
            SubCommand::with_name("add")
                .arg(Arg::with_name("name").required(true))
                .arg(Arg::with_name("path").required(true)),
        )
        .subcommand(SubCommand::with_name("list"))
        .arg(Arg::with_name("PROFILE").index(1).required(false))
        .get_matches();

    if std::env::args().len() == 1 {
        run_profile("default");
    } else if let Some(profile_path) = args.value_of("PROFILE") {
        run_profile(profile_path);
    } else if let Some(arg) = args.subcommand_matches("add") {
        add_profile(arg.value_of("name").unwrap(), arg.value_of("path").unwrap());
    } else if let Some(_) = args.subcommand_matches("list") {
        list_profiles();
    } else if let Some(arg) = args.subcommand_matches("init") {
        init(arg.value_of("default_path").unwrap());
    } else {
        //show help
        println!("argument invalid. show help");
    }
}
