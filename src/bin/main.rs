use std::error::Error;
use clap::{Arg, Command, ArgAction};
use hop::{
    add_path,
    find_matches,
    get_data
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
        let matches = find_matches(dir.clone(), get_data()?);
        println!("{}", matches[0].path.display());
    }
    if let Some(path) = matches.get_one::<String>("add") {
        add_path(&data, path.clone(), None)?;
    }
    Ok(())
}
