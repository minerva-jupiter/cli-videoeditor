use std::fs::File;
use std::io::Write;

use crate::ffmpeg_exec;

const FILE_LIST: &str = "file_list.txt";

// concat mode master
pub fn concat_mode() {
    
}

//create and add videos for textfile
pub fn add_list(file_path: Vec<String>) {
    let mut file = File::create(FILE_LIST).expect("Unable to create file");

    file.write(file_path.join("\n").as_bytes()).expect("Unable to write file");
}

//execute concat
pub fn exec_concat() {
    //add -f concat option
    let mut args = Vec::new();
    args.push("-f concat".to_string());

    //execute ffmpeg with some options
    ffmpeg_exec::exec_ffmpeg(ffmpeg_exec::ask_inputs(),ffmpeg_exec::ask_output(),args);
}
