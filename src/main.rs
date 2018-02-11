#[macro_use] extern crate serde_derive;
extern crate egg_mode;
extern crate rand;
extern crate tokio_core;
extern crate serde;
extern crate serde_json;

mod stuff;

use std::io::{BufRead, BufReader, Error};
use std::fs::File;
use tokio_core::reactor;
use rand::Rng;
use egg_mode::tweet::DraftTweet;

fn main() {
    println!("starting...");

    let time = std::time::Duration::from_millis(stuff::time().unwrap());

    let mut core = reactor::Core::new().unwrap();
    let handle = core.handle();

    let token = stuff::load();

    if let Err(err) = core.run(egg_mode::verify_tokens(&token, &handle)) {
        panic!("invalid or expired tokens: {:?}", err);
    }

    loop {
        let number = get_number().unwrap();

        // this is pretty bad
        let mut line: String = match get_line(number) {
            Ok(n) => n,
            Err(err) => {
                println!("failed to get line: {:?}\nfallbacking...", err);
                if core.run(egg_mode::direct::send(egg_mode::user::UserID::ScreenName("geniiii_"), &stuff::err_dm_desc(&err), &token, &handle)).is_err() {
                    println!("failed to send DM (most likely missing permissions to send DMs)\ncontinuing anyways...");
                }
                String::from("something broke! @geniiii_ welp, here's a random number: ") + &number.to_string()
            },
        };

        line.insert_str(0, stuff::random_string_start().unwrap().as_str());
        line.push_str(stuff::random_string_end().unwrap().as_str());

        println!("sending message: {}", &line);

        // more bad code
        let send = core.run(DraftTweet::new(line.clone()).send(&token, &handle));
        if let Err(err) = send {
            println!("failed to send tweet: {:?}\ncontinuing anyways...", err);
        } else {
            println!("sent message: {}", &line);
            std::thread::sleep(time);
        }
    }
}

// any better way to do this?
fn get_number() -> Result<usize, Error> {
    let f = File::open("stuff.sakujes")?;
    let count = BufReader::new(f).lines().count();

    if count == 0 {
        panic!("file is empty");
    }

    let random_number = rand::thread_rng().gen_range(1, count);

    Ok(random_number)
}

// any better way to do this?
fn get_line(random_number: usize) -> Result<String, Error> {
    let f = File::open("stuff.sakujes")?;
    let line = BufReader::new(f).lines().nth(random_number).unwrap().unwrap();

    Ok(line)
}

