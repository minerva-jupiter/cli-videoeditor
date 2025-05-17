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

}


fn exec_ls() {
    let output = std::process::Command::new("ls")
        .arg("-l")
        .arg("-a")
        .output()
        .expect("Failed to execute ls");

    println!("{:?}", output);

    if output.status.success() {
        println!("ls executed successfully");
    } else {
        eprintln!("ls execution failed");
    }
}
fn exec_ffmpeg() {
    let output = std::process::Command::new("ffmpeg")
        .arg("-i")
        .arg("input.mp4")
        .arg("-c:v")
        .arg("libx264")
        .arg("-preset")
        .arg("fast")
        .arg("-crf")
        .arg("22")
        .arg("output.mp4")
        .output()
        .expect("Failed to execute ffmpeg");

    if output.status.success() {
        println!("FFmpeg executed successfully");
    } else {
        eprintln!("FFmpeg execution failed");
    }
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
