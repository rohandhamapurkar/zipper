use rpassword::read_password;
use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::path::Path;
use std::process;
use zip::write::FileOptions;

pub fn get_src_file_path() -> String {
    let args: Vec<String> = env::args().collect();
    if args.len() < 1 {
        eprintln!("Please provide folder or file path to me!");
        process::exit(1);
    }

    let metadata = fs::metadata(args[1].clone()).unwrap_or_else(|e| {
        eprintln!("error with file path: {}", e);
        process::exit(1);
    });

    if metadata.is_dir() {
        eprintln!("path is a folder, please provide a file");
        process::exit(1);
    }

    return args[1].clone();
}

pub fn take_input(question: &str) -> String {
    print!("{}: ", question);
    io::stdout().flush().unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        process::exit(1);
    });

    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        process::exit(1);
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
    src_path: &str,
    dst_path: &str,
    password: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let src_path = Path::new(src_path);
    let dst_path = Path::new(dst_path);

    // Open the source file
    let mut src_file = File::open(src_path)?;
    let mut src_contents = Vec::new();
    src_file.read_to_end(&mut src_contents)?;

    // Create the zip file
    let dst_file = File::create(dst_path)?;
    let mut zip = zip::ZipWriter::new(dst_file);

    let options: FileOptions<'_, _> = FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflate64)
        .unix_permissions(0o755)
        .with_aes_encryption(zip::AesMode::Aes256, password);

    // Add the file to the zip archive
    zip.start_file(src_path.file_name().unwrap().to_str().unwrap(), options)?;
    zip.write_all(&src_contents)?;

    // Finish writing the zip file
    zip.finish()?;

    println!("File successfully zipped to {}", dst_path.display());
    Ok(())
}
