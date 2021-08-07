use serde::{Deserialize, Serialize};

use std::fs::File;
use std::io::BufReader;

#[derive(Serialize, Deserialize, Debug)]
pub struct Story {
    pub when: String,
    pub r#where: String,
    pub who: String,
    pub what: String,
}

pub fn json_to_stories(files: Vec<File>) -> Vec<Story> {
    files
        .iter()
        .map(|file|{
            let reader = BufReader::new(file);
            serde_json::from_reader(reader).expect("deserialization failed")
        })
        .collect()
}
