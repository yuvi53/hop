use clap::{Arg, ArgAction, Command};
use hop::{
    add_path,
    data::{load, save},
    find_matches, set_defaults,
};
use std::error::Error;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = Command::new("whatever")
        .arg(Arg::new("add").long("add").action(ArgAction::Set))
        .arg(Arg::new("dir").long("dir").action(ArgAction::Set))
        .get_matches();
    let data_path = set_defaults()?;
    let data = load(data_path.clone())?;
    if let Some(dir) = matches.get_one::<String>("dir") {
        let matches = find_matches(dir.clone(), data.clone());
        for (i, m) in matches.into_iter().enumerate() {
            if i == 0 {
                println!("{}", m.path.display());
            }
        }
    }
    if let Some(path) = matches.get_one::<String>("add") {
        let entries = add_path(PathBuf::from(&path), data.clone(), None);
        save(data_path, entries)?;
    }
    Ok(())
}
