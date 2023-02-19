use reqwest;
use serde_json;
use std::fs::File;
use std::io::prelude::*;
use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
struct Credentials {
    secret: String,
    key: String,
    url: String
}


fn main() {
    let creds = load_credentials("creds.json".to_string());
    println!("loaded credentials with key: {} | secret: {}", creds.key, creds.secret);
}


fn load_credentials(path:String) -> Credentials {

    let mut file = File::open(path).expect("File not found");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");

    let credentials: Credentials = serde_json::from_str(&contents)
        .expect("Error deserializing JSON");

    return credentials;
}