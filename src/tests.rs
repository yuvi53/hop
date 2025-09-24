use std::path::PathBuf;
use std::error::Error;
use crate::*; 

#[test]
fn test_set_defaults() -> Result<(), Box<dyn Error>> {
    let config = Config {
        data_path: PathBuf::from("/home/yuvi/.local/share/hop/hop.txt"),
        backup_path: PathBuf::from("/home/yuvi/.local/share/hop/hop.txt.bak"),
    };
    assert_eq!(set_defaults()?, config);
    Ok(())
}

#[test]
fn test_search_path() -> Result<(), Box<dyn Error>> {
    let path = search_path(&String::from("lrust"))?;
    assert_eq!(path, String::from("/home/yuvi/lcode/lrust"));
    Ok(())
}
