use rpassword::read_password;
use std::error::{self, Error};
use std::fs::File;
use std::io::{self, BufReader};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use zip::write::ExtendedFileOptions;
use zip::write::FileOptions;
use zip::ZipWriter;

#[macro_export]
macro_rules! print_exit {
    ($message:expr) => {
        // print error message
        eprintln!("error: {}", $message);
        // exit the process.
        std::process::exit(1);
    };
}

pub fn take_input(question: &str) -> Result<String, Box<dyn Error>> {
    // ask question
    print!("{}: ", question);
    io::stdout().flush()?;

    let mut input: String = String::new();
    // get user input from terminal
    io::stdin().read_line(&mut input)?;

    return Ok(input.trim().to_string());
}

pub fn get_confirmed_password() -> io::Result<String> {
    let password = loop {
        print!("Enter password: ");
        io::stdout().flush()?;
        let pass = read_password()?;

        print!("Confirm password: ");
        io::stdout().flush()?;
        let confirm_pass = read_password()?;

        // check if confirm password is matching.
        if pass == confirm_pass {
            break pass;
        } else {
            println!("Passwords do not match. Please try again.");
        }
    };

    Ok(password)
}

fn init_zip(
    file_prefix: &str,
    src_file_name: &str,
    src_path: &PathBuf,
) -> Result<(zip::ZipWriter<File>, String), Box<dyn error::Error>> {
    // Create the zip filename
    let mut zip_filename = src_file_name.to_string();
    if !file_prefix.is_empty() {
        zip_filename = format!("{}_{}", file_prefix, &zip_filename);
    }

    // Create zip file path
    let mut zip_path_buf: PathBuf = src_path.to_path_buf();
    zip_path_buf.set_file_name(&zip_filename);
    zip_path_buf.set_extension("zip");

    // Create the zip file
    let zip_file = File::create(&zip_path_buf)?;
    let zip = zip::ZipWriter::new(zip_file);

    Ok((zip, format!("{}.zip", zip_filename)))
}

fn stream_file_to_zip(path: &Path, zip: &mut ZipWriter<File>) -> Result<(), Box<dyn error::Error>> {
    // create a file buffer
    let mut f: BufReader<File> = BufReader::new(File::open(path)?);
    let mut buffer = [0; 10240]; // 10KB buffer
    loop {
        // read into buffer
        let bytes_read = f.read(&mut buffer)?;
        if bytes_read == 0 {
            return Ok(());
        }
        // write to zip
        zip.write_all(&buffer[..bytes_read])?;
    }
}

pub fn secure_zip_single_file(
    src_path: PathBuf,
    file_prefix: &str,
    password: &str,
) -> Result<(), Box<dyn error::Error>> {
    // build source file name
    let src_file_name: &str = src_path.file_name().unwrap().to_str().unwrap();

    // Create the zip file
    let (mut zip, zip_file_name) = init_zip(file_prefix, src_file_name, &src_path)?;

    // create password protected zip file options
    let options: FileOptions<'_, ExtendedFileOptions> = FileOptions::default()
        .unix_permissions(0o755)
        .with_aes_encryption(zip::AesMode::Aes256, password);

    // Add the source file to the zip archive
    zip.start_file(src_file_name, options)?;
    stream_file_to_zip(&src_path, &mut zip)?;

    // Finish writing the zip file
    zip.finish()?;

    println!("File successfully zipped to {}", zip_file_name);

    Ok(())
}

pub fn secure_zip_dir(
    src_path: PathBuf,
    file_prefix: &str,
    password: &str,
) -> Result<(), Box<dyn error::Error>> {
    let src_file_name: &str = src_path.file_name().unwrap().to_str().unwrap();

    // Create the zip file
    let (mut zip, zip_file_name) = init_zip(file_prefix, src_file_name, &src_path)?;

    for entry in WalkDir::new(&src_path) {
        let entry = entry?;
        let path = entry.path();

        // skip the root directory
        if path.eq(&src_path) {
            continue;
        }

        // create password protected zip file options
        let options: FileOptions<'_, ExtendedFileOptions> = FileOptions::default()
            .unix_permissions(0o755)
            .with_aes_encryption(zip::AesMode::Aes256, password);

        // get relative path
        let relative_path = path.strip_prefix(&src_path)?;
        println!("relative: {}", relative_path.to_str().unwrap());

        if path.is_file() {
            // add file to zip with it's contents
            zip.start_file_from_path(relative_path.to_str().unwrap(), options)?;
            stream_file_to_zip(path, &mut zip)?;
        } else if path.is_dir() {
            // Add directory to zip if it exists.
            zip.add_directory_from_path(relative_path, options)?;
        }
    }

    // Finish writing the zip file
    zip.finish()?;

    println!("File successfully zipped to {}", zip_file_name);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;
    use zip::read::ZipArchive;

    #[test]
    fn test_init_zip() {
        let temp_dir = tempdir().unwrap().into_path();

        let (_, zip_filename) = init_zip("prefix", "test_file", &temp_dir).unwrap();

        assert_eq!(zip_filename, "prefix_test_file.zip");
    }

    #[test]
    fn test_secure_zip_single_file() {
        let password = "password";
        let temp_dir = tempdir().unwrap().into_path();
        let file_path = temp_dir.as_path().join("test_file.txt");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "Test content").unwrap();

        secure_zip_single_file(file_path.clone(), "prefix", password).unwrap();

        let zip_path = temp_dir.as_path().join("prefix_test_file.zip");
        assert!(zip_path.exists());

        // Verify zip contents with password
        let zip_file = File::open(zip_path).unwrap();
        let mut archive = ZipArchive::new(zip_file).unwrap();
        assert_eq!(archive.len(), 1);

        let mut zipped_file = archive.by_index_decrypt(0, password.as_bytes()).unwrap();
        assert_eq!(zipped_file.name(), "test_file.txt");
        let mut contents = String::new();
        zipped_file.read_to_string(&mut contents).unwrap();
        assert_eq!(contents.trim(), "Test content");

        drop(temp_dir);
    }

    #[test]
    fn test_secure_zip_dir() {
        let password = "password";
        let temp_dir = tempdir().unwrap().into_path();
        let sub_dir = temp_dir.as_path().join("subdir");
        fs::create_dir(&sub_dir).unwrap();
        let file_path = sub_dir.join("test_file.txt");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "Test content").unwrap();

        secure_zip_dir(temp_dir.clone(), "", password).unwrap();

        let mut zip_path = temp_dir;
        zip_path.set_extension("zip");
        assert!(zip_path.exists());

        // Verify zip contents
        let zip_file = File::open(zip_path).unwrap();
        let mut archive = ZipArchive::new(zip_file).unwrap();
        assert_eq!(archive.len(), 2); // subdir and test_file.txt

        let mut zipped_file = archive
            .by_name_decrypt("subdir/test_file.txt", "password".as_bytes())
            .unwrap();
        let mut contents = String::new();
        zipped_file.read_to_string(&mut contents).unwrap();
        assert_eq!(contents.trim(), "Test content");
    }

    #[test]
    fn test_stream_file_to_zip() {
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("test_file.txt");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "Test content").unwrap();

        let zip_path = temp_dir.path().join("test.zip");
        let zip_file = File::create(&zip_path).unwrap();
        let mut zip = ZipWriter::new(zip_file);

        let options: FileOptions<'_, ExtendedFileOptions> = FileOptions::default();
        zip.start_file("test_file.txt", options).unwrap();
        stream_file_to_zip(&file_path, &mut zip).unwrap();
        zip.finish().unwrap();

        // Verify zip contents
        let zip_file = File::open(zip_path).unwrap();
        let mut archive = ZipArchive::new(zip_file).unwrap();
        let mut zipped_file = archive.by_name("test_file.txt").unwrap();
        let mut contents = String::new();
        zipped_file.read_to_string(&mut contents).unwrap();
        assert_eq!(contents.trim(), "Test content");
    }
}
