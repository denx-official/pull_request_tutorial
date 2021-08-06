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
    let mut when: Vec<String> = Vec::new();
    let mut r#where: Vec<String> = Vec::new();
    let mut who: Vec<String> = Vec::new();
    let mut what: Vec<String> = Vec::new();

    let files = get_files();
    for file in files {
        let reader = BufReader::new(file);
        let story: Story = serde_json::from_reader(reader).expect("deserialization failed");

        when.push(story.when);
        r#where.push(story.r#where);
        who.push(story.who);
        what.push(story.what);
    }

    let mut rng = rand::thread_rng();

    println!("{}", when[rng.gen::<usize>() % when.len()]);
    wait();
    println!("{}", r#where[rng.gen::<usize>() % r#where.len()]);
    wait();
    println!("{}", who[rng.gen::<usize>() % who.len()]);
    wait();
    println!("{}", what[rng.gen::<usize>() % what.len()]);
}

fn get_files() -> Vec<File> {
    let mut file_vec: Vec<File> = Vec::new();
    let paths = read_dir("./json").expect("json directory not found");

    for path in paths {
        let path_string = path.unwrap().path().display().to_string();
        file_vec.push(
            File::open(path_string).expect("file not found")
        );
    }

    file_vec
}

fn wait() {
    let mut x = String::new();
    stdin().read_line(&mut x).expect("Failed to read line.");
}
