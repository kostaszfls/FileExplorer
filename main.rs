use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

fn main() {
    // Prompt the user to enter the disk drive letter
    let drive_letter = get_drive_letter();

    // Form the path to the selected disk
    let disk_path = PathBuf::from(format!("{}:", drive_letter));

    // List all files and directories in the selected disk
    let entries = fs::read_dir(disk_path).expect("Failed to read directory");

    // Get the search term from the user
    let search_term = get_search_term();

    // Iterate over the directory entries
    for entry in entries {
        if let Ok(entry) = entry {
            let file_name = entry.file_name();
            let file_type = entry.file_type().expect("Failed to determine file type");

            // Check if the entry matches the search term
            if file_name.to_string_lossy().to_lowercase().contains(&search_term) {
                // Print the file/directory name
                println!("{}", file_name.to_string_lossy());

                // Check if it's a directory and print its contents recursively
                if file_type.is_dir() {
                    explore_directory(entry.path(), &search_term);
                }
            }
        }
    }
}

fn explore_directory(dir_path: PathBuf, search_term: &str) {
    // Indent the output for directories
    println!("  [{}]", dir_path.display());

    // List all files and directories in the given directory
    let entries = fs::read_dir(dir_path).expect("Failed to read directory");

    // Iterate over the directory entries
    for entry in entries {
        if let Ok(entry) = entry {
            let file_name = entry.file_name();
            let file_type = entry.file_type().expect("Failed to determine file type");

            // Check if the entry matches the search term
            if file_name.to_string_lossy().to_lowercase().contains(&search_term) {
                // Indent the output for files
                print!("  ");

                // Print the file/directory name
                println!("{}", file_name.to_string_lossy());

                // Check if it's a directory and print its contents recursively
                if file_type.is_dir() {
                    explore_directory(entry.path(), &search_term);
                }
            }
        }
    }
}

fn get_drive_letter() -> char {
    print!("Enter the drive letter: ");
    io::stdout().flush().unwrap();

    let mut drive_letter = String::new();
    io::stdin()
        .read_line(&mut drive_letter)
        .expect("Failed to read drive letter");

    drive_letter
        .trim()
        .chars()
        .next()
        .expect("Failed to read drive letter")
        .to_ascii_lowercase()
}

fn get_search_term() -> String {
    print!("Enter search term: ");
    io::stdout().flush().unwrap();

    let mut search_term = String::new();
    io::stdin().read_line(&mut search_term).expect("Failed to read search term");

    search_term.trim().to_lowercase()
}
