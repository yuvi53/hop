use std::error::Error;
use std::io::prelude::*;
use std::path::PathBuf;
use std::fs::{self, File};
use std::time::SystemTime;

use crate::{Config, Data};
use crate::BACKUP_THRESHOLD;

pub fn load(config: Config) -> Result<Vec<Data>, Box<dyn Error>> {
    let mut data_path = config.data_path;
    if !data_path.exists() {
        match config.backup_path.exists() {
            true => data_path = config.backup_path,
            false => return Ok(Vec::new()),
        }
    }
    let file = fs::read_to_string(&data_path)?;
    let results: Vec<Data> = file.lines().map(|line| {
        let v: Vec<&str> = line.split("\t").collect();
        let weight = v[0]
            .parse::<f64>()
            .expect("couldn't convert &str to f64(while parsing)");
        let path = v[1].trim();
        Data {weight, path: PathBuf::from(path)}
    }).collect();
    Ok(results)
}

pub fn save(config: Config, data: Vec<Data>) -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();
    for Data {weight, path} in data {
        let path = path.to_str().unwrap();
        buffer.push_str(& format!("{}\t{}\n", weight, path));
    }
    let data_path = config.data_path;
    if !data_path.parent().unwrap().exists() {
        fs::create_dir_all(
            data_path.parent().unwrap()
        )?;
    }
    let mut file = File::create(&data_path)?;
    write!(file, "{}", buffer)?;
    let backup_path = config.backup_path;
    if !backup_path.exists() {
        let mut file = File::create(&backup_path)?;
        write!(file, "{}", buffer)?;
    }
    let time = SystemTime::now()
        .duration_since(file.metadata()?.modified()?)?
        .as_secs();
    if time > BACKUP_THRESHOLD {
        let mut file = File::create(&backup_path)?;
        write!(file, "{}", buffer)?;
    }
    Ok(())
}

pub fn exist_in_database(queried_path: PathBuf, data: Vec<Data>) -> bool {
    let mut exist = false;
    for Data {weight: _, path} in data {
        if path == queried_path {
            exist = true;
        }
    }
    exist
}
