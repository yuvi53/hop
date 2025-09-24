use std::error::Error;
use clap::{Arg, Command, ArgAction};
use hopv2::{add_path, search_path};

fn main() -> Result<(), Box<dyn Error>> {
    let matches = Command::new("whatever")
        .arg(Arg::new("add")
            .long("add")
            .action(ArgAction::Set))
        .arg(Arg::new("query_dir")
            .value_name("dir_name")
            .required(true))
        .get_matches();
    if let Some(dir_name) = matches.get_one::<String>("query_dir") {
        println!("{}", search_path(dir_name)?);
    }
    if let Some(path) = matches.get_one::<String>("add") {
        add_path(path)?;
    }
    Ok(())
}

