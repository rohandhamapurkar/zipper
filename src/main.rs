use std::path::Path;
use std::process;

mod helper;
use helper::get_confirmed_password;
use helper::get_src_file_path;
use helper::take_input;
use helper::zip_file;

fn main() {
    let src_path: String = get_src_file_path();

    let zip_file_prefix: String = take_input("Any prefix for the the zip file name?");

    let mut dst_path = Path::new(&src_path).to_path_buf();
    dst_path.set_extension("zip");

    let dst_filename = format!(
        "./{}_{}",
        zip_file_prefix,
        dst_path.file_name().unwrap().to_str().unwrap()
    );

    let password = get_confirmed_password().unwrap_or_else(|e| {
        eprintln!("error with password input: {}", e);
        process::exit(1);
    });

    zip_file(&src_path, &dst_filename, &password).unwrap_or_else(|e| {
        eprintln!("error with file path: {}", e);
        process::exit(1);
    });
}
