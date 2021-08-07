use serde::{Deserialize, Serialize};
use rand::Rng;

use std::fs::{File, read_dir};
use std::io::BufReader;

#[derive(Serialize, Deserialize, Debug)]
pub struct Story {
    pub when: String,
    pub r#where: String,
    pub who: String,
    pub what: String,
}

pub struct JsonStory {
    path: String,
}

impl JsonStory {
    pub fn new(path: String) -> JsonStory {
        JsonStory { path: path }
    }

    pub fn json_to_stories(self, num: i32) -> Vec<Story> {
        self.get_files(num)
            .iter()
            .map(|file|file_to_story(file))
            .collect()
    }

    fn get_file_names(self) -> Vec<String> {
        let paths = read_dir(self.path).expect("directory not found");
        paths.map(|path|path.unwrap().path().display().to_string()).collect()
    }

    fn get_files(self, num: i32) -> Vec<File> {
        let file_names:Vec<String> = self.get_file_names();
        let mut rng = rand::thread_rng();
        (0..=num)
            .map(|_|File::open(&file_names[rng.gen::<usize>() % file_names.len()]).expect("file not found"))
            .collect()
    }
}

fn file_to_story(file: &File) -> Story {
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).expect("deserialization failed")
}
