use std::fs::File;
use std::io::Write;

const FILE_LIST: &str = "file_list.txt";



pub fn add_list(file_path: Vec<String>) {
    let mut file = File::create(FILE_LIST).expect("Unable to create file");

    file.write(file_path.join("\n").as_bytes()).expect("Unable to write file");
}