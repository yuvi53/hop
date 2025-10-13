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
    if let false = if_exist(&get_data()?, &new_path)? {
        add_path(&get_data()?, new_path.clone(), None)?;
    }
    assert!(if_exist(&get_data()?, &new_path)?);
    Ok(())
}

#[test]
fn test_if_exist() -> Result<(), Box<dyn Error>> {
    let new_path = String::from("/home/yuvi/test_dir/test_dir2/");  
    add_path(&get_data()?, new_path.clone(), None)?;
    assert!(if_exist(&get_data()?, &new_path)?);
    Ok(())
}

#[test]
fn test_search_for_path() -> Result<(), Box<dyn Error>> {
    let query_path = String::from("test_dir2");
    let expected_path = String::from("/home/yuvi/test_dir/test_dir2/"); 
    if let false = if_exist(&get_data()?, &expected_path)? {
        add_path(&get_data()?, expected_path.clone(), None)?;
    }
    let data = search_for_path(&get_data()?, query_path);
    assert_eq!(data[0].path, expected_path);
    Ok(())
}


#[test]
fn test_match_path() -> Result<(), Box<dyn Error>> {
    let query_path = String::from("test_dir2");
    let expected_path = String::from("/home/yuvi/test_dir/test_dir2/"); 
    if let false = if_exist(&get_data()?, &expected_path)? {
        add_path(&get_data()?, expected_path.clone(), None)?;
    }
    assert_eq!(match_path(&search_for_path(&get_data()?, query_path))?, expected_path);
    Ok(())
}



