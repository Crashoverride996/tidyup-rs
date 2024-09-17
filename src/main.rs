use std::fs;
use std::env;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

fn print_usage() {
    println!(r#"
        Usage: tidyup -d directory
        This program groups and displays desktop items based on their type.

        Options:
          -h, --help                    Display this help message and exit.
          -d, --directory   DIRECTORY   Specify the input directory containing items.
          -e, --extensions  extensions  Specify extensions to consider
          -i, --ignore      extensions  List of extensions to ignore
          -v, --verbose                 Enable verbose mode to show additional details during processing.

        Description:
          The program groups a list of directory items from an input folder, groups them by their type (e.g., images, extensions, shortcuts), and displays or saves the grouped items based on the specified options.

          Each item in the input file should be in the format:
            NAME.EXTENSION
          where NAME is the name of the item and TYPE is one of the following:
            PNG, JPG, JPEG, CPP, PY

        Examples:
          tidyup -d DESKTOP
            Reads items from 'DESKTOP', groups them, and writes the results to appropriate folders.

          tidyup --verbose
            Runs the program in verbose mode, providing additional processing details.

        For more information, please refer to the documentation or visit the project website.)
    "#);
}

fn tidyup() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut relevant_extensions: Vec<String> = Vec::new();
    let mut ignore_extensions: Vec<String> = Vec::new();
    let mut dir_name = ".".to_string();

    let mut verbose = true;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-d" | "--directory" => {
                if i + 1 < args.len() {
                    dir_name = args[i + 1].clone();
                    i += 1;
                } else {
                    eprintln!("Error: Missing directory after -d or --directory");
                    print_usage();
                    return Ok(()); // or Err(...)
                }
            }
            "-e" | "--extensions" => {
                i += 1;
                while i < args.len() && !args[i].starts_with('-') {
                    relevant_extensions.push(args[i].clone());
                    i += 1;
                }
                i -= 1; // Adjust for the upcoming increment
            }
            "-i" | "--ignore" => {
                i += 1;
                while i < args.len() && !args[i].starts_with('-') {
                    ignore_extensions.push(args[i].clone());
                    i += 1;
                }
                i -= 1; // Adjust for the upcoming increment
            }
            "-v" | "--verbose" => {
                verbose = true;
            }
            "-h" | "--help" => {
                print_usage();
                return Ok(());
            }
            _ => {
                eprintln!("Error: Unknown argument {}", args[i]);
                print_usage();
                return Ok(()); // or Err(...)
            }
        }
        i += 1;
    }

    println!("Cleaning {}", dir_name);
    println!("Verbose {}", verbose);
    let path = Path::new(&dir_name);

    let mut extension_mapping: HashMap<String, PathBuf> = HashMap::new();
    extension_mapping.insert("png".to_string(), path.join("images"));
    extension_mapping.insert("jpg".to_string(), path.join("images"));
    extension_mapping.insert("jpeg".to_string(), path.join("images"));
    extension_mapping.insert("py".to_string(), path.join("python"));
    extension_mapping.insert("cpp".to_string(), path.join("c++"));

    for ext_dir in extension_mapping.values() {
        if !ext_dir.exists() {
            fs::create_dir(ext_dir)?;
        }
    }

    let entries = fs::read_dir(path)?;
    for entry in entries {
        let entry = entry?;
        let metadata = entry.metadata()?;

        if metadata.is_file() {
            let file_name = entry.file_name();
            println!("{}", file_name.to_string_lossy());

            let file_path = entry.path();
            let extension = file_path.extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or_default()
                .to_ascii_lowercase();

            if !extension.is_empty() && ((relevant_extensions.contains(&extension) || relevant_extensions.is_empty()) && (ignore_extensions.is_empty() || !ignore_extensions.contains(&extension))) {
                if let Some(target_dir) = extension_mapping.get(&extension) {
                    let new_path = target_dir.join(file_name);
                    fs::rename(&file_path, &new_path)?;
                }
            }
        }
    }

    Ok(())
}

fn main() {
    let status = tidyup();
    match status {
        Ok(_) => println!("Tidyup finished."),
        Err(e) => eprintln!("Failed to tidy directory: {}.", e)
    }
}
