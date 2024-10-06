use rpassword::read_password;
use std::env;
use std::error;
use std::fs;
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::path::PathBuf;
use zip::write::ExtendedFileOptions;
use zip::write::FileOptions;

#[macro_export]
macro_rules! print_exit {
    ($message:expr) => {
        eprintln!("error: {}", $message);
        std::process::exit(1);
    };
}

pub fn get_src_file_path() -> PathBuf {
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

pub fn take_input(question: &str) -> String {
    print!("{}: ", question);
    io::stdout().flush().unwrap_or_else(|e| {
        print_exit!(e);
    });

    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap_or_else(|e| {
        print_exit!(e);
    });

    input.trim().to_string()
}

pub fn get_confirmed_password() -> io::Result<String> {
    let password = loop {
        print!("Enter password: ");
        io::stdout().flush()?;
        let pass = read_password()?;

        print!("Confirm password: ");
        io::stdout().flush()?;
        let confirm_pass = read_password()?;

        if pass == confirm_pass {
            break pass;
        } else {
            println!("Passwords do not match. Please try again.");
        }
    };

    Ok(password)
}

pub fn zip_file(
    src_path: PathBuf,
    dst_path: PathBuf,
    password: &str,
) -> Result<(), Box<dyn error::Error>> {
    let src_path = src_path.as_path();
    let dst_path = dst_path.as_path();

    // Open the source file
    let mut src_file = File::open(src_path)?;
    let mut src_contents = Vec::new();
    src_file.read_to_end(&mut src_contents)?;
    drop(src_file);

    // Create the zip file
    let dst_file = File::create(dst_path)?;
    let mut zip = zip::ZipWriter::new(dst_file);

    let options: FileOptions<'_, ExtendedFileOptions> = FileOptions::default()
        .unix_permissions(0o755)
        .with_aes_encryption(zip::AesMode::Aes256, password);

    let src_file_name: &str = src_path.file_name().unwrap().to_str().unwrap();

    // Add the file to the zip archive
    zip.start_file(src_file_name, options)?;
    zip.write_all(&src_contents)?;

    // Finish writing the zip file
    zip.finish()?;

    println!("File successfully zipped to {}", dst_path.display());
    Ok(())
}
