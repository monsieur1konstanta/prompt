// Importing necessary modules from the standard library
use std::{collections::HashMap, env, fs::{File, OpenOptions}, io::{self, Read, Write}, path::PathBuf};

// Main function
fn main() {
    // Getting the default path from the environment variable
    let default_path = option_env!("PROMPT_DIR_DEFAULT");

    // Collecting command line arguments
    let args: Vec<String> = env::args().collect();

    // Mapping the default path to a PathBuf
    let default_path = default_path.map(|path| {
        let mut path_buf = PathBuf::new();
        path_buf.push(path);
        path_buf
    });

    // Loading storage from file
    let mut storage = load_storage(
        default_path.clone()
    );

    // If there is only one argument
    if args.len() == 2 {
        // If the argument is "-l"
        if args[1].as_str().eq("-l") {
            // Print header for key-value list
            println!("{:-^47}", "Key Value(first 40 charts) List");
            // Collect and sort key-value pairs from storage
            let mut values = storage.iter().map(|(k, v)| (k.clone(), v.clone())).collect::<Vec<_>>();
            values.sort();

            // Print key-value pairs
            for (key, value) in values {
                println!("KEY:   {}\nVALUE: {:.40}\n", key, value)
            }
            // Exit
            return;
        }
    }

    // If there are more than two arguments
    if args.len() > 2 {
        match args[1].as_str() {
            // If the first argument is "-c"
            "-c" => {
                // Get the key from the third argument
                let key = &args[3];

                // Read value from standard input
                let mut value = String::new();
                io::stdin().read_to_string(&mut value).unwrap();
                // Insert key-value pair into storage
                storage.insert(key.trim().to_string(), value);
                // Save storage to file
                save_storage(&storage, default_path);
            }
            // If the first argument is "-r"
            "-r" => {
                // Get the key from the third argument
                let key = &args[3];
                // Remove key-value pair from storage and save to file if something was removed
                if storage.remove(key).is_some() {
                    save_storage(&storage, default_path);
                }
            },
            // Otherwise
            _ => {
                // Get the key from the second argument
                let key = &args[2];
                // If the key exists in storage, print its value. Otherwise print an error message.
                if let Some(value) = storage.get(key) {
                    println!("{}", value);
                } else {
                    eprintln!("Error: key not found");
                }
            }
        }
    }
}

// Function to convert default path to a PathBuf with "storage.json" appended to it.
fn default_into_path(default: Option<PathBuf>) -> PathBuf {
    if let Some(mut default) = default {
        default.push("storage.json");
        default
    } else {
        let mut path_buf = PathBuf::new();
        path_buf.push("./storage.json");
        path_buf
    }
}

// Function to load storage from file as a HashMap<String, String>
fn load_storage(default: Option<PathBuf>) -> HashMap<String, String> {
    let path = default_into_path(default);
    if path.exists() {
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        serde_json::from_str(&contents).unwrap()
    } else {
        HashMap::new()
    }
}

// Function to save storage to file as JSON.
fn save_storage(storage: &HashMap<String, String>, default: Option<PathBuf>) {
    let path = default_into_path(default);

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
        .unwrap();
    let contents = serde_json::to_string(storage).unwrap();
    file.write_all(contents.as_bytes()).unwrap();
}