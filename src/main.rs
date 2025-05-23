use concat::add_list;
use std::path::Path;

mod concat;
mod ffmpeg_exec;

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
