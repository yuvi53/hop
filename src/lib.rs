use std::error::Error;
use std::path::PathBuf;
use std::{env};
use dotenvy::dotenv;

#[cfg(test)]
mod tests;

pub mod data;
pub mod search;

#[derive(Debug, PartialEq, Clone)]
pub struct Config {
    pub data_path: PathBuf,
    pub backup_path: PathBuf,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Data {
    pub weight: f64,
    pub path: PathBuf,
}

pub const BACKUP_THRESHOLD: u64 = 24 * 60 * 60;
pub const FUZZY_MATCH_THRESHOLD: f64 = 0.6;

pub fn set_defaults() -> Result<Config, Box<dyn Error>> {
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

pub fn add_path(path: PathBuf, mut data: Vec<Data>, weight: Option<f64>) -> Vec<Data> {
    let weight = match weight {
        Some(num) => num,
        None => 10.0,
    };

    if path == PathBuf::from(env::var("HOME").unwrap()) {
        return data;
    }
    match data::exist_in_database(path.clone(), data.clone()) {
        false => {
            data.push(Data {weight, path});
        },
        true => {
            for Data {weight: lweight, path: lpath} in data.iter_mut() {
                if path == *lpath {
                    *lweight = ((*lweight * *lweight) + (weight * weight)).sqrt();
                } 
            }
        },
    }
    data
}

pub fn find_matches(needle: String, entries: Vec<Data>) -> Vec<Data> {
    let is_cwd = |entry: &Data| {
        let pwd = std::env::current_dir()
            .expect("couldn't get the working directory");
        let pwd = pwd.to_str().expect("couldn't convert pwd to &str");
        let entry_path = entry.path.to_str().unwrap();
        pwd == entry_path
    };
    let meets_threshold = |entry: &Data| {
        let entry = entry.path
            .file_name()
            .expect("couldn't get the dir name")
            .to_str()
            .expect("couldn't convert OsStr into &str");
        search::match_percent(&entry, &needle) >= FUZZY_MATCH_THRESHOLD
    };
    let match_anywhere = |entry: &Data| {
        let mut exist = false;
        for component in entry.path.iter() {
            if needle == component.to_str().unwrap() {
                exist = true;
            }
        }
        exist
    };
    let entries: Vec<Data> = entries
        .into_iter()
        .filter(|entry| !is_cwd(entry) && entry.path.exists())
        .collect();

    entries.clone()
        .into_iter()
        .filter(|entry| entry.path.ends_with(&needle))
        .chain(entries.clone()
            .into_iter()
            .filter(meets_threshold))
        .chain(entries.clone()
            .into_iter()
            .filter(match_anywhere))
        .collect::<Vec<Data>>()
}
