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
fn test_add_path() -> Result<(), Box<dyn Error>> {
    let new_path = String::from("/home/yuvi/test_dir/test_dir2/");  
    if let false = if_exist(&new_path)? {
        add_path(new_path.clone())?;
    }
    assert!(if_exist(&new_path)?);
    Ok(())
}

#[test]
fn test_if_exist() -> Result<(), Box<dyn Error>> {
    let new_path = String::from("/home/yuvi/test_dir/test_dir2/");  
    add_path(new_path.clone())?;
    assert!(if_exist(&new_path)?);
    Ok(())
}

#[test]
fn test_search_path() -> Result<(), Box<dyn Error>> {
    let query_path = String::from("test_dir2");
    let expected_path = String::from("/home/yuvi/test_dir/test_dir2/"); 
    if let false = if_exist(&expected_path)? {
        add_path(expected_path.clone())?;
    }
    assert_eq!(search_path(query_path)?, expected_path);
    Ok(())
}
