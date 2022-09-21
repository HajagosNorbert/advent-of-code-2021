use std::env;

pub mod days;

fn main() {
    let default_challenge_id = String::from("4_1");

    let challenge_id  = env::args().nth(1);
    let challenge_id = match challenge_id {
        Some(id) => id,
        None => default_challenge_id
    };

    let result = days::solve_challenge(&challenge_id);

    match result {
        Ok(result) =>  println!("{result}"),
        Err(msg) => eprintln!("{msg}")
    };
}
