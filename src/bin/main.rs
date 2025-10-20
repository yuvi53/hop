use std::error::Error;
use clap::{Arg, Command, ArgAction};
use std::path::PathBuf;
use hop::{
    add_path,
    find_matches,
    load,
    set_defaults,
    save,
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
    let data_path = set_defaults()?;
    let data = load(data_path.clone())?;
    if let Some(dir) = matches.get_one::<String>("dir") {
        let matches = find_matches(dir.clone(), data.clone(), None);
        println!("{}", matches[0].path.display());
    }
    if let Some(path) = matches.get_one::<String>("add") {
        let entries = add_path(PathBuf::from(&path), data.clone(), None);
        save(data_path, entries)?;
    }
    Ok(())
}
