mod json_story;
use json_story::JsonStory;

use std::io::stdin;

fn main() {
    let json = JsonStory::new(format!("./json"));
    let stories = json.json_to_stories(4);

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
