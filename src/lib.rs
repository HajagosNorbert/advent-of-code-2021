use std::{env, fs, path};
pub fn read_input() -> String {
    let arg1 = env::args().next().unwrap();
    let file_path = path::Path::new(&arg1);
    let challange_number = file_path.file_name().unwrap().to_str().unwrap();
    let file_path = file_path
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join(format!("input/{}.txt", challange_number));
    fs::read_to_string(file_path).expect("File open failed")
}
