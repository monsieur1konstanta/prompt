use std::{collections::HashMap, env, fs::{File, OpenOptions}, io::{self, Read, Write}, path::PathBuf};

fn main() {
    // define default storage
    let default_path = option_env!("PROMPT_DIR_DEFAULT");

    // collect args
    let args: Vec<String> = env::args().collect();

    // load storage
    let default_path = default_path.map(|path| {
        let mut path_buf = PathBuf::new();
        path_buf.push(path);
        path_buf
    });

    let mut storage = load_storage(
        default_path.clone()
    );

    // handle args
    if args.len() > 2 {
        match args[1].as_str() {
            "-c" => {
                let key = &args[3];

                let mut value = String::new();
                io::stdin().read_to_string(&mut value).unwrap();
                storage.insert(key.trim().to_string(), value);
                save_storage(&storage, default_path.clone());
            }
            "-r" => {
                let key = &args[3];
                // save the storage only if something new have been added
                if storage.remove(key).is_some() {
                    save_storage(&storage, default_path.clone());
                }
            }
            _ => {
                let key = &args[2];
                if let Some(value) = storage.get(key) {
                    println!("{}", value);
                } else {
                    eprintln!("Error: key not found");
                }
            }
        }
    }
}

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
