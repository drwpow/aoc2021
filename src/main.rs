use std::env;

mod dec1;
mod dec2;

fn main() {
    let args: Vec<String> = env::args().collect();
    let challenge = args.get(1);
    match challenge {
        Some(challenge) => match challenge.as_ref() {
            "1" => dec1::run(),
            "2" => dec2::run(),
            _ => println!(
                "{}",
                format!("Challenge for day {} not implemented yet", challenge)
            ),
        },
        None => println!("No challenge specified, expected `cargo run -- 1`"),
    }
}
