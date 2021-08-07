mod json;
mod story;

use json::JsonDir;
use story::json_to_stories;

use std::io::stdin;

fn main() {
    let json_dir = JsonDir::new(format!("./json"));
    let files = json_dir.get_files(4);
    let stories = json_to_stories(files);

    println!("{}", stories[0].when);
    wait();
    println!("{}", stories[1].r#where);
    wait();
    println!("{}", stories[2].who);
    wait();
    println!("{}", stories[3].what);
    wait();
}

fn wait() {
    let mut x = String::new();
    stdin().read_line(&mut x).expect("Failed to read line.");
}
