use std::env;
use std::fs;
use std::path::PathBuf;

use zipper::get_confirmed_password;
use zipper::print_exit;
use zipper::secure_zip_dir;
use zipper::secure_zip_single_file;
use zipper::take_input;

fn main() {
    let src_path: PathBuf = get_src_path_from_args();

    let file_prefix: String = take_input("Any prefix for the the zip file name?").unwrap_or_else(|e| {
        print_exit!(e);
    });

    let password = get_confirmed_password().unwrap_or_else(|e: std::io::Error| {
        print_exit!(e);
    });

    if src_path.is_file() {
        if let Err(e) = secure_zip_single_file(src_path, &file_prefix, &password) {
            print_exit!(e);
        }
    } else if src_path.is_dir() {
        if let Err(e) = secure_zip_dir(src_path, &file_prefix, &password) {
            print_exit!(e);
        }
    } else {
        print_exit!(format!(
            "Source path is neither a file nor a directory: {}",
            src_path.display()
        ));
    }
}

fn get_src_path_from_args() -> PathBuf {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_exit!("Please provide folder or file path to me!");
    }

    let src_path = PathBuf::from(&args[1]);

    if !fs::exists(&src_path).unwrap_or_else(|e| {
        print_exit!(e);
    }) {
        print_exit!("Path doesn't exist!");
    }

    return src_path;
}
