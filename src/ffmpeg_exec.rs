pub fn exec_ffmpeg(input_files:Vec<String>, output_file:String, args:Vec<String>) {
    let output = std::process::Command::new("ffmpeg")
        .arg("-i")
        //.arg(input_files)
        //.arg(args)
        .arg(output_file)
        .output()
        .expect("Failed to execute ffmpeg");

    if output.status.success() {
        println!("FFmpeg executed successfully");
    } else {
        eprintln!("FFmpeg execution failed");
    }
}

//ask what file will user input
pub fn ask_inputs() -> Vec<String> {
    let mut isEOF :bool = false;
    let mut input_files = Vec::new();
    while isEOF {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        input = input.trim_end().to_owned();

        if input=="EOF" {
            isEOF = true;
            break;
        }
        
        //have to check file exists
        
        input_files.push(input);
    }
    return input_files;
}

// ask what file will user output
pub fn ask_output() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input = input.trim_end().to_owned();
    
    //have to check file exists
    
    return input;
}