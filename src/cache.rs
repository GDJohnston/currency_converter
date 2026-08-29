use std::{
    fs::{self, File},
    io::{Read, Write},
    time::{Duration, SystemTime}
};
use crate::api::api_response::Rates;

const CACHE_FOLDER: &'static str = "./data";
const LATEST_FOLDER: &'static str = "/latest/";

pub(crate) fn read_from_cache(basecode: &str) -> Option<Rates> {
    let cache_file = CACHE_FOLDER.to_owned() + LATEST_FOLDER + basecode;
    let mut file = match File::open(cache_file) {
        Ok(f) => f,
        Err(_) => return None,
    };

    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    println!("read from cache");

    let rates_body = Rates::new(&data);
    let next_update =
        SystemTime::UNIX_EPOCH + Duration::from_secs(rates_body.time_next_update_unix as u64);
    let now = SystemTime::now();
    if now >= next_update {
        println!("cache expired");
        return None; // Return None so we can refresh the cache
    }

    Some(rates_body)
}

pub(crate) fn add_to_cache(basecode: &str, data: &str) {
    let cache_file = "/latest/".to_owned() + basecode;
    fs::create_dir_all("./data/latest/").unwrap();
    let path = "./data".to_owned() + &cache_file;
    let mut file = File::create(path).unwrap();
    file.write_all(data.as_bytes()).unwrap();
}
