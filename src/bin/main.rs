use std::error::Error;
use clap::{Arg, Command, ArgAction};
use hop::{
    add_path,
    search_for_path,
    get_data,
};

fn main() -> Result<(), Box<dyn Error>> {
    let matches = Command::new("whatever")
        .arg(Arg::new("add")
            .long("add")
            .action(ArgAction::Set))
        .arg(Arg::new("dir")
            .long("dir")
            .action(ArgAction::Set))
        .get_matches();
    let data = get_data()?;
    if let Some(dir) = matches.get_one::<String>("dir") {
        println!("{}", search_for_path(&data, dir.clone())?);
    }
    if let Some(path) = matches.get_one::<String>("add") {
        add_path(&data, path.clone(), None)?;
    }
    Ok(())
}
