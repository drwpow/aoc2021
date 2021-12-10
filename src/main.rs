#![allow(confusable_idents)]
#![allow(mixed_script_confusables)]

use std::env;

mod dec1;
mod dec2;
mod dec3;
mod dec4;
mod dec5;
mod dec6;
mod dec7;
mod dec8;
mod dec9;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();
    let challenge = args.get(1);
    match challenge {
        Some(challenge) => match challenge.as_ref() {
            "1" => dec1::run(),
            "2" => dec2::run(),
            "3" => dec3::run(),
            "4" => dec4::run(),
            "5" => dec5::run(),
            "6" => dec6::run(),
            "7" => dec7::run(),
            "8" => dec8::run(),
            "9" => dec9::run(),
            _ => println!(
                "{}",
                format!("Challenge for day {} not implemented yet", challenge)
            ),
        },
        None => println!("No challenge specified, expected `cargo run -- 1`"),
    }
}
