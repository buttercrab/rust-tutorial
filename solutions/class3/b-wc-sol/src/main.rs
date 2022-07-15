use std::{env, fs};

fn count_by_line(path: &str) -> usize {
    let s = fs::read_to_string(path).expect("error opening file");
    s.split('\n').count()
}

fn count_by_word(path: &str) -> usize {
    let s = fs::read_to_string(path).expect("error opening file");
    s.split(&[' ', '\t']).count()
}

fn count_character(path: &str) -> usize {
    let s = fs::read_to_string(path).expect("error opening file");
    s.chars().count()
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let res = match &*args[1] {
        "-l" => count_by_line(&*args[2]),
        "-w" => count_by_word(&*args[2]),
        "-c" => count_character(&*args[2]),
        _ => panic!("invalid option"),
    };
    println!("{res}");
}
