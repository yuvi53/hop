use std::error::Error;
use clap::{Arg, Command, ArgAction};
use hopv2::{add_path, search_path};

fn main() -> Result<(), Box<dyn Error>> {
    let matches = Command::new("whatever")
        .arg(Arg::new("add")
            .long("add")
            .action(ArgAction::Set))
        .arg(Arg::new("dir")
            .long("dir")
            .action(ArgAction::Set))
        .get_matches();
    if let Some(dir) = matches.get_one::<String>("dir") {
        println!("{}", search_path(dir.clone())?);
    }
    if let Some(path) = matches.get_one::<String>("add") {
        add_path(path.clone(), None)?;
    }
    Ok(())
}
