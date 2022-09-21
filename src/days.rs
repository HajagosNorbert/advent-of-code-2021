use std::{env, fs, path};

pub mod day1_1;
pub mod day1_2;
pub mod day2_1;
pub mod day2_2;
pub mod day3_1;
pub mod day3_2;
pub mod day4_1;

pub fn solve_challenge(challenge_id: &str) -> Result<String, &'static str>{
    let day_id = challenge_id.split(['-', '_']).next().expect("wrong challenge format");

    let input = read_input_file(&format!("{day_id}.txt"));
    match challenge_id {
        "1_1" => Ok(day1_1::solve(&input)),
        "1_2" => Ok(day1_2::solve(&input)),
        "2_1" => Ok(day2_1::solve(&input)),
        "2_2" => Ok(day2_2::solve(&input)),
        "3_1" => Ok(day3_1::solve(&input)),
        "3_2" => Ok(day3_2::solve(&input)),
        "4_1" => Ok(day4_1::solve(&input)),
        _ => Err("challenge not implemented")
    }


}


pub fn read_input_file(filename: &str) -> String {
    let arg1 = env::args().next().unwrap();
    let file_path = path::Path::new(&arg1);
    let file_path = file_path
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join(format!("input/{}", filename));
    fs::read_to_string(file_path).expect("File open failed")
}
