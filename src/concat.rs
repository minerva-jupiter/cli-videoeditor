use std::fs::File;
use std::io::Write;

mod main

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
    args :Vec<String> = [];
    args.append("-f concat");

    //decide input_files output_file
    const input_files :Vec<String> = main::ask_inputs();
    const output_file :&str = main::ask_output();

    //execute ffmpeg with some options
    main::exec_ffmpeg(input_files,output_file,args);
}
