pub fn exec_ffmpeg(input_files:Vec<String>, output_file:String, args:Vec<String>) {
    let mut output = std::process::Command::new("ffmpeg");

    output.arg("-i");
    for input in input_files.iter() {
        output.arg(input);
    }

    for arg in args.iter() {
        output.arg(arg);
    }
    
    output.arg(output_file);
    output.output();

    if output.status().expect("Failed to execute ffmpeg").success() {
        println!("FFmpeg executed successfully");
    } else {
        eprintln!("FFmpeg execution failed");
    }
}

//ask what file will user input
pub fn ask_inputs() -> Vec<String> {
    let mut is_EOF :bool = false;
    let mut input_files = Vec::new();
    while is_EOF {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        input = input.trim_end().to_owned();

        if input=="EOF" {
            is_EOF = true;
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