use std::fs::File;
use std::io::Write;

const file_list: &str = "file_list.txt";



pub fn add_list(file_path: Vec<String>) {
    let mut file = File::create(file_list).expect("Unable to create file");

    file.write(file_path.join("\n").as_bytes()).expect("Unable to write file");
}