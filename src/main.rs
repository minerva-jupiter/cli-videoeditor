use crate::concat::add_list;
use std::path::Path;

mod concat;

//for test of concat::add_list 
//test Vector

fn main() {
    // Check if ffmpeg is installed
    let is_exist_ffmpeg :bool = exist_check();
    if is_exist_ffmpeg {
        println!("FFmpeg is installed");
    } else {
        eprintln!("FFmpeg is not installed");
        return;
    }

    println!("May I help you? I am a cli video editor.");

    add_list([
        "file1.mp4".to_string(),
        "file2.mp4".to_string(),
        "file3.mp4".to_string(),
    ].to_vec());

}

pub fn exec_ffmpeg(input_files:Vec<String>, output_file:String, args:Vec<String>) {
    let output = std::process::Command::new("ffmpeg")
        .arg("-i")
        .arg(input_files)
        .arg(args)
        .arg(output_file)
        .output()
        .expect("Failed to execute ffmpeg");

    if output.status.success() {
        println!("FFmpeg executed successfully");
    } else {
        eprintln!("FFmpeg execution failed");
    }
}

pub fn ask_inputs() -> Vec<String> {
    let mut isEOF :bool = false;
    let mut input_files :Vec<String>;
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
        
        input_files.append(input);
    }
    return input_files;
}

// list up files in the directory
// ref https://qiita.com/benki/items/70ad2ee44cff9efde778
use std::io;
use std::fs;
fn ls_dir<P: AsRef<Path>>(path: P) -> io::Result<Vec<String>> {
    Ok(fs::read_dir(path)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            if entry.file_type().ok()?.is_file() {
                Some(entry.file_name().to_string_lossy().into_owned())
            } else {
                None
            }
        })
        .collect())
}

fn exist_check() -> bool {
    let output = std::process::Command::new("ffmpeg")
        .arg("-version")
        .output()
        .expect("Failed to execute ffmpeg");

    if output.status.success() {
        return true;
    } else {
        return false;
    }
}
