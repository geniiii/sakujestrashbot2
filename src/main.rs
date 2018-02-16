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
use egg_mode::{tweet::DraftTweet, direct::send, user};

fn main() {
    println!("starting...");

    // get sleep time from config
    let time_conf = stuff::time().unwrap();
    let time = std::time::Duration::from_millis(time_conf);

    let mut core = reactor::Core::new().unwrap();
    let handle = core.handle();

    let token = stuff::load();

    // panic if tokens are invalid or expired
    if let Err(err) = core.run(egg_mode::verify_tokens(&token, &handle)) {
        panic!("invalid or expired tokens: {:?}", err);
    }

    // tweet loop
    loop {
        let number = get_number().unwrap();

        // this is pretty bad
        let mut line: String = match get_line(number) {
            Ok(n) => n,
            Err(err) => {
                println!("failed to get line: {:?}\nfallbacking...", err);
                // if sending a dm failed
                if core.run(send(user::UserID::ScreenName("geniiii_"), &stuff::err_dm_desc(&err), &token, &handle)).is_err() {
                    println!("failed to send DM (most likely missing permissions to send DMs)\ncontinuing anyways...");
                }
                String::from("something broke! @geniiii_ welp, here's a random number: ") + &number.to_string()
            },
        };

        // add random member from config to start and end of string
        line.insert_str(0, stuff::random_string_start().unwrap().as_str());
        line.push_str(stuff::random_string_end().unwrap().as_str());

        println!("sending tweet: {}", &line);

        // tries to send tweet
        let send = core.run(DraftTweet::new(line.clone()).send(&token, &handle));
        if let Err(err) = send {
            println!("failed to send tweet: {:?}\ncontinuing anyways...", err);
        } else {
            println!("sent tweet: {}", &line);
            println!("sleeping for {}ms...", &time_conf);
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

