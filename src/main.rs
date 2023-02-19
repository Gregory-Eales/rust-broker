
use reqwest;
use serde_json;
use std::fs::File;
use std::io::prelude::*;
use serde::{Deserialize, Serialize};
use error_chain::error_chain;
use std::io::Read;


error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}


#[derive(Debug, Deserialize, Serialize)]
struct Credentials {
    secret: String,
    key: String,
    url: String
}


fn main() -> Result<()> {

    let creds = load_credentials("creds.json".to_string());
    println!("loaded credentials with key: {} | secret: {}", creds.key, creds.secret);

    let mut res = reqwest::blocking::get(creds.url)?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;

    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());
    println!("Body:\n{}", body);

    Ok(())
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




