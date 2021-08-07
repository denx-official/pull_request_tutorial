use serde::{Deserialize, Serialize};
use rand::Rng;

use std::fs::{File, read_dir};
use std::io::{BufReader, stdin};

#[derive(Serialize, Deserialize, Debug)]
struct Story {
    when: String,
    r#where: String,
    who: String,
    what: String,
}

fn main() {
    let file_names = get_file_names("./json");
    let files = get_files(file_names, 4);

    let stories: Vec<Story> = files
        .iter()
        .map(|file|{
            let reader = BufReader::new(file);
            serde_json::from_reader(reader).expect("deserialization failed")
        })
        .collect();

    println!("{}", stories[0].when);
    wait();
    println!("{}", stories[1].r#where);
    wait();
    println!("{}", stories[2].who);
    wait();
    println!("{}", stories[3].what);
    wait();
}

fn get_file_names(path: &str) -> Vec<String> {
    let paths = read_dir(path).expect("directory not found");
    paths.map(|path|path.unwrap().path().display().to_string()).collect()
}

fn get_files(file_names: Vec<String>, num: i32) -> Vec<File> {
    let mut rng = rand::thread_rng();
    (0..=num)
        .map(|_|File::open(&file_names[rng.gen::<usize>() % file_names.len()]).expect("file not found"))
        .collect()
}

fn wait() {
    let mut x = String::new();
    stdin().read_line(&mut x).expect("Failed to read line.");
}
