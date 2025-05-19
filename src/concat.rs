use std::fs::File;
use std::io::Write;

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
pub fn exec_concat(input_files: Vec<String>,output_file: String,args: Vec<String>) {
    let output = std::process::Command::new("ffmpeg")
        .arg("-i")
        .arg(input_files)
        .arg(output_file)
        .arg(&args)
        .output()
        .expect("Faild to execute ffmpeg");
    if output.status.success() {
        println!("FFmpeg Concat executed successfully");
    } else {
        eprintln!("FFmpeg Concat execution failed");
    }
}
