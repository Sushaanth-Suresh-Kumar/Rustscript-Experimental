// mod repl;

use std::env;
use std::fs;
use std::path::Path;

use compiler::compile;

fn main() {
    // repl::start();

    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();

    // Ensure a file path is provided
    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    // The file path is the second argument
    let file_path = &args[1];

    // Convert the string slice to a Path
    let path = Path::new(file_path);

    // Use match to handle the file name, extension, and file reading logic
    match (
        path.file_name(),
        path.extension().and_then(|ext| ext.to_str()),
    ) {
        (Some(file_name), Some("rsc")) => {
            // File has the correct extension; read the file contents
            match fs::read_to_string(path) {
                Ok(contents) => {
                    println!("File name: {}", file_name.to_string_lossy());
                    println!("File content:\n{}", contents);
                    compile(&contents, file_name.to_str().unwrap());
                }
                Err(e) => {
                    eprintln!("Error: Failed to read the file: {}", e);
                    std::process::exit(1);
                }
            }
        }
        (Some(_), _) => {
            eprintln!("Error: Only files with a .rsc extension are allowed.");
            std::process::exit(1);
        }
        _ => {
            eprintln!("Error: No valid file name found in the path.");
            std::process::exit(1);
        }
    }
}
