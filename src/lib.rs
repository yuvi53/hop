use std::io::prelude::*;
use std::fs::{self, File, OpenOptions};
use std::path::PathBuf;
use std::env;
use std::error::Error;
use dotenvy::dotenv;

#[cfg(test)]
mod tests;

#[derive(PartialEq)]
#[derive(Debug)]
pub struct Config {
    pub data_path: PathBuf,
    pub backup_path: PathBuf,
}

fn set_defaults() -> Result<Config, Box<dyn Error>> {
    dotenv().ok();
    let data_home: PathBuf = match env::var("XDG_DATA_HOME") {
        Ok(path) => PathBuf::from(&path),
        Err(_) => {
            [
                &env::var("HOME").unwrap(),
                ".local",
                "share",
            ].iter().collect()
        },
    };
    let mut data_path = data_home.clone();
        data_path.push("hop/hop.txt");
    let mut backup_path = data_home.clone();
        backup_path.push("hop/hop.txt.bak");
    Ok(Config {data_path, backup_path})
}

fn create_data_dir() -> Result<(), Box<dyn Error>> {
    let data_path = set_defaults()?.data_path;
    fs::create_dir_all(data_path.parent().unwrap())?;
    Ok(())
}

pub fn add_path(path: String) -> Result<(), Box<dyn Error>>{
    let data_path = set_defaults()?.data_path;
    if !data_path.exists() {
        create_data_dir()?;
    }
    let mut file = OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open(data_path)?;
    write!(file, "{}\n", &path)?;
    Ok(())
}

pub fn search_path(query: String) -> Result<String, Box<dyn Error>> {
    Ok(String::new())
} 

