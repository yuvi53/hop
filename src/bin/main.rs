use std::error::Error;
use clap::{Arg, Command, ArgAction};
use hopv2::{add_path};

fn main() -> Result<(), Box<dyn Error>> {
    let matches = Command::new("whatever")
        .arg(Arg::new("add")
            .long("add")
            .action(ArgAction::Set))
        .get_matches();
    if let Some(path) = matches.get_one::<String>("add") {
        add_path(path.clone())?;
    }
    Ok(())
}

