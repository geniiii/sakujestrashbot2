extern crate egg_mode;
extern crate rand;

extern crate serde;
extern crate serde_json;
extern crate tokio_core;

use rand::Rng;
use std::io::Error;
use std::fs::File;

#[derive(Deserialize)]
struct Keys {
    consumer_key: String,
    consumer_secret: String,
    access_key: String,
    access_secret: String,
}

#[derive(Deserialize)]
struct WordsStart {
    words_start: serde_json::value::Value,
}

#[derive(Deserialize)]
struct WordsEnd {
    words_end: serde_json::value::Value,
}

#[derive(Deserialize)]
struct Time {
    time: u64,
}

pub fn load() -> egg_mode::Token {
    let keys = keys().unwrap();

    let con_token = egg_mode::KeyPair::new(keys.consumer_key, keys.consumer_secret);
    let access_token = egg_mode::KeyPair::new(keys.access_key, keys.access_secret);

    let token = egg_mode::Token::Access {
        consumer: con_token,
        access: access_token,
    };

    token
}

// what a mess
pub fn random_string_start() -> Result<String, Error> {
    let config_file = File::open("config.sakujes")?;

    let config_json: WordsStart = serde_json::from_reader(config_file)?;
    let a = config_json.words_start.as_array().unwrap();

    let random_number = rand::thread_rng().gen_range(1, a.len());
    let line = a.iter().nth(random_number).unwrap();

    Ok(line.as_str().unwrap().to_string())
}

// what a mess
pub fn random_string_end() -> Result<String, Error> {
    let config_file = File::open("config.sakujes")?;

    let config_json: WordsEnd = serde_json::from_reader(config_file)?;
    let a = config_json.words_end.as_array().unwrap();

    let random_number = rand::thread_rng().gen_range(1, a.len());
    let line = a.iter().nth(random_number).unwrap();

    Ok(line.as_str().unwrap().to_string())
}

pub fn err_dm_desc(err: &Error) -> String {
    let err_error = err.get_ref().unwrap();

    let mut idk = String::from(err_error.description());
    idk.insert_str(0, "error description: ");

    idk
}

// lotsa time
pub fn time() -> Result<u64, Error> {
    let time = File::open("config.sakujes")?;
    let time: Time = serde_json::from_reader(time)?;

    Ok(time.time)
}

// lotsa keys
fn keys() -> Result<Keys, Error> {
    let keys = File::open("config.sakujes")?;
    let keys: Keys = serde_json::from_reader(keys)?;

    Ok(keys)
}
