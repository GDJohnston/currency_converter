use std::{fs, io::Write, path::Path};

const API_KEY_HINT: &'static str = "Replace this text with an api key from https://www.exchangerate-api.com/";

pub(crate) fn from_file(key_file: &Path) -> String {
    let exists = fs::exists(key_file).expect("Could not confirm the existance of a file, check folder permissions and try again");
    if exists == false {
        let mut file = fs::File::create(key_file).expect("Failed to create a neccessary file, check permissions and try again");
        file.write_all(API_KEY_HINT.as_bytes()).expect("Failed to write hint ot api key file, check permissions and try again");
        panic!("Api key missing, please see {:?}", key_file);
    }
    let string = fs::read_to_string(key_file).unwrap();
    if string.chars().all(|c| c.is_ascii_hexdigit()) {
        string
    }
    else {
        panic!("Contents of the \"{:?}\" file don't look like an api key", key_file)
    }
}
