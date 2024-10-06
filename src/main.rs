use std::env;
use std::fs;
use std::path::PathBuf;

use zip_encryptor::get_confirmed_password;
use zip_encryptor::print_exit;
use zip_encryptor::take_input;
use zip_encryptor::zip_file;

fn main() {
    let src_path: PathBuf = get_src_path_from_args();

    let file_prefix: String = take_input("Any prefix for the the zip file name?");

    let mut dst_path = src_path.clone();
    dst_path.set_extension("zip");
    dst_path.set_file_name(format!(
        "{}_{}",
        file_prefix,
        dst_path.file_name().unwrap().to_str().unwrap()
    ));

    let password = get_confirmed_password().unwrap_or_else(|e: std::io::Error| {
        print_exit!(e);
    });

    zip_file(src_path, dst_path, &password).unwrap_or_else(|e| {
        print_exit!(e);
    });
}

fn get_src_path_from_args() -> PathBuf {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_exit!("Please provide folder or file path to me!");
    }

    let src_path = PathBuf::from(&args[1]);

    let metadata = fs::metadata(&src_path).unwrap_or_else(|e| {
        print_exit!(e);
    });

    if metadata.is_dir() {
        print_exit!("Path is a folder, please provide a file");
    }

    return src_path;
}
