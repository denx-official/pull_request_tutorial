use rand::Rng;

use std::fs::{File, read_dir};

pub struct JsonDir {
    path: String,
}

impl JsonDir {
    pub fn new(path: String) -> JsonDir {
        JsonDir{ path: path }
    }

    pub fn get_files(self, num: i32) -> Vec<File> {
        let file_names:Vec<String> = self.get_file_names();
        let mut rng = rand::thread_rng();
        (0..=num)
            .map(|_| {
                let name = &file_names[rng.gen::<usize>() % file_names.len()];
                File::open(name).expect("file not found")
            })
            .collect()
    }

    pub fn get_all_files(self) -> Vec<File> {
        self.get_file_names()
            .iter()
            .map(|name|File::open(name).expect("file not found"))
            .collect()
    }

    fn get_file_names(self) -> Vec<String> {
        let paths = read_dir(self.path).expect("directory not found");
        paths.map(|path|path.unwrap().path().display().to_string()).collect()
    }
}
