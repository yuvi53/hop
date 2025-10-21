use std::error::Error;
use std::io::prelude::*;
use std::path::PathBuf;
use std::fs::{self, File, OpenOptions};

use crate::{Config, Data};

pub fn load(config: Config) -> Result<Vec<Data>, Box<dyn Error>> {
    let data_path = config.data_path;
    if !data_path.exists() {
        return Ok(Vec::new());
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
    let data_path = config.data_path;
    if !data_path.exists() {
        fs::create_dir_all(
            data_path.parent().unwrap()
        )?;
        File::create(&data_path)?;
    }
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&data_path)?;
    let mut buffer = String::new();
    for Data {weight, path} in data {
        let path = path.to_str().unwrap();
        buffer.push_str(& format!("{}\t{}\n", weight, path));
    }
    write!(file, "{}", buffer)?;
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
