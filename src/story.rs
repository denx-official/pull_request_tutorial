use log;

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
            log::error!("file: {:?}", file);
            let reader = BufReader::new(file);
            serde_json::from_reader(reader).expect("deserialization failed")
        })
        .collect()
}

#[cfg(test)]
mod test {
    use env_logger;
    use crate::story::json_to_stories;
    use std::fs::{File, read_dir};

    #[test]
    fn check_all_json_files() {
        let _ = env_logger::builder().is_test(true).try_init();

        let files = get_all_json_files();
        json_to_stories(files);
    }

    fn get_all_json_files() -> Vec<File> {
        read_dir("./json")
            .expect("directory not found")
            .map(|path|path.unwrap().path().display().to_string())
            .map(|name|File::open(name).expect("file not found"))
            .collect()
    }
}
