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
        assert!(exist_in_database(&string_foo)?);
    }
    Ok(())
}

#[test]
fn test_exist_in_database() -> Result<(), Box<dyn Error>> {
    let string_foo = String::from("/home/yuvi/foo_dir");  
    add_path(string_foo.clone(), get_data()?, None)?;
    assert!(exist_in_database(&string_foo)?);
    Ok(())
}

#[test]
fn test_get_data() -> Result<(), Box<dyn Error>>{
    let string_foo = String::from("/home/yuvi/foo_dir");
    if get_data()?.len() == 0 {
        add_path(string_foo.clone(), get_data()?, None)?;
        assert!(get_data()?.len() > 0);
    }
    Ok(())
}

#[test]
fn test_find_matches() -> Result<(), Box<dyn Error>> {
    let data_path = set_defaults()?.data_path;
    let mut path_foo = data_path.clone();
    path_foo.push("/test_data/foo_dir");
    fs::create_dir_all(&path_foo)?;
    let str_foo = path_foo.to_str().unwrap();
    if !exist_in_database(str_foo)? {
        add_path(String::from(str_foo), get_data()?, None)?;
        //testing for consecutive
        let results = find_matches(String::from("foo_dir"), get_data()?);  
        assert_eq!(results[0].path, path_foo);
    }
    let mut path_bar = data_path.clone();
    path_bar.push("/test_data/bar_dir");
    fs::create_dir_all(&path_bar)?;
    let str_bar = path_bar.to_str().unwrap();
    if !exist_in_database(str_bar)? {
        add_path(String::from(str_bar), get_data()?, None)?;
        //testing for fuzzy
        let results = find_matches(String::from("bar_"), get_data()?);  
        assert_eq!(results[0].path, path_bar);
    }
    Ok(())
}

#[test]
fn test_ifilter() -> Result<(), Box<dyn Error>> {
    let empty_vec: Vec<Data> = vec![];
    let closure = |_| false;
    assert_eq!(ifilter(closure, get_data()?), empty_vec);
    if get_data()?.len() == 0 {
        let string_foo = String::from("/home/yuvi/foo_dir");
        add_path(string_foo.clone(), get_data()?, None);
    }
    let closure = |_| true;
    assert!(ifilter(closure, get_data()?).len() >= 1);
    Ok(())
}

#[test]
fn test_match_consecutive() -> Result<(), Box<dyn Error>> {
    let string_path = String::from("/home/yuvi/foo_dir");
    let expected_path = PathBuf::from(&string_path);
    if !exist_in_database(&string_path)? {
        add_path(string_path.clone(), get_data()?, None)?;
        let results = match_consecutive(String::from("foo_dir"), get_data()?);  
        assert_eq!(results[0].path, expected_path);
    }
    Ok(())
}

#[test]
fn test_match_fuzzy() -> Result<(), Box<dyn Error>> {
    let string_path = String::from("/home/yuvi/foo_dir");
    let expected_path = PathBuf::from(&string_path);
    if !exist_in_database(&string_path)? {
        add_path(string_path.clone(), get_data()?, None)?;
        let results = match_fuzzy(String::from("foo_"), get_data()?, None);  
        assert_eq!(results[0].path, expected_path);
    }
    Ok(())
}

#[test]
fn test_match_percent() {
    let s1 = "someone";
    let s2 = "some";
    assert_eq!(match_percent(s1, s2), 0.7272727272727273);
}
