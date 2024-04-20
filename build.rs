use std::fs;
use std::path::Path;
use std::path::PathBuf;

fn main(){
    if !cfg!(target_os = "windows"){
        return;
    }

    println!("build.rs script loaded.");

    let source_path = "./src";

    let target_paths = ["./target/debug", "./target/release"];

    let files = ["install.ps1", "uninstall.ps1"]; 

    for target_path in target_paths {
        if Path::new(target_path).exists() {
            for file in files {
                let mut src_path_buf = PathBuf::from(source_path);
                src_path_buf.push(file);
                let src_file = src_path_buf.as_os_str();

                if let Ok(meta) = fs::metadata(&src_path_buf) {
                    if !meta.is_file(){
                        println!("file '{:?}' not found.", src_file);
                        continue;
                    }
                }
                let mut dst_path_buf = PathBuf::from(target_path);
                dst_path_buf.push(file);
                let dst_file = dst_path_buf.as_mut_os_str();

                fs::copy(&src_file, &dst_file).expect("failed to copy file.");
            }
        }
    }

}