use crate::*; 

#[test]
fn test_set_defaults() -> Result<(), Box<dyn Error>> {
    let config = Config {
        data_path: PathBuf::from("/home/yuvi/projects/hop/hop/hop.txt"),
        backup_path: PathBuf::from("/home/yuvi/projects/hop/hop/hop.txt.bak"),
    };
    assert_eq!(set_defaults()?, config);
    Ok(())
}

#[test]
fn test_add_path() -> Result<(), Box<dyn Error>> {
    let string_foo = String::from("/home/yuvi/foo_dir");  
    if !exist_in_database(&string_foo)? {
        add_path(string_foo.clone(), get_data()?, None)?;
    }
    assert!(exist_in_database(&string_foo)?);
    Ok(())
}

#[test]
fn test_exist_in_database() -> Result<(), Box<dyn Error>> {
    let string_foo = String::from("/home/yuvi/foo_dir");  
    add_path(string_foo.clone(), &get_data()?, None);
    assert!(exist_in_database(&string_foo)?);
    Ok(())
}

#[test]
fn test_get_data() -> Result<(), Box<dyn Error>>{
    let results: Vec<Data> = vec![];
    if get_data()? == results {
        let string_foo = String::from("/home/yuvi/foo_dir");
        add_path(string_foo.clone(), &get_data()?, None);
    }
    let path_foo = PathBuf::from(&string_foo);
    let results = get_data()?;
    assert_eq!(results[0].path, path_foo);
    Ok(())
}

#[test]
fn test_find_matches() -> Result<(), Box<dyn Error>> {
    let string_bar = String::from("/home/yuvi/bar_dir");
    let string_foo = String::from("/home/yuvi/foo_dir");
    let path_bar = PathBuf::from(&string_bar);
    let path_foo = PathBuf::from(&string_foo);
    if !exist_in_database(&string_bar)? {
        add_path(string_bar.clone(), &get_data()?, None);
    }
    if !exist_in_database(&string_foo)? {
        add_path(string_foo.clone(), &get_data()?, None);
    }
    assert!(exist_in_database(&string_bar)?);
    assert!(exist_in_database(&string_foo)?);
    //testing for consecutive
    let results = match_consecutive(String::from("bar_dir"), get_data()?);  
    assert_eq!(results[0].path, path_bar);
    //testing for fuzzy
    let results = match_consecutive(String::from("foo_"), get_data()?);  
    assert_eq!(results[0].path, path_foo);
    Ok(())
}

#[test]
fn test_ifilter() -> Result<(), Box<dyn Error>>{
    let empty_vec: Vec<Data> = vec[]!;
    let closure = |&entry| false;
    assert_eq!(ifilter(closure, get_data()?), empty_vec);
    if get_data()? == empty_vec {
        let string_foo = String::from("/home/yuvi/foo_dir");
        add_path(string_bar.clone(), &get_data()?, None);
    }
    let closure = |&entry| true;
    assert!(ifilter(closure, get_data()?), empty_vec);
    Ok(())
}

#[test]
fn test_match_consecutive() -> Result<(), Box<dyn Error>> {
    let string_path = String::from("/home/yuvi/foo_dir");
    let expected_path = PathBuf::from(&string_path);
    if !exist_in_database(&string_path) {
        add_path(&get_data()?, string_path.clone(), None);
    }
    assert!(exist_in_database(&string_path));
    let results = match_consecutive(String::from("foo_dir"), get_data()?);  
    assert_eq!(results[0].path, expected_path);
    Ok(())
}

#[test]
fn test_match_fuzzy() -> Result<(), Box<dyn Error>> {
    let string_path = String::from("/home/yuvi/foo_dir");
    let expected_path = PathBuf::from(&string_path);
    if !exist_in_database(&string_path) {
        add_path(&get_data()?, string_path.clone(), None);
    }
    assert!(exist_in_database(&string_path));
    let results = match_fuzzy(String::from("foo_"), get_data()?, None);  
    assert_eq!(results[0].path, expected_path);
    Ok(())
}

#[test]
fn test_match_percent() {
    let s1 = "someone";
    let s2 = "some";
    assert_eq!(match_percent(s1, s2), 0.7272727272727273);
}
