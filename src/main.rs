use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{
    env,
    fs::OpenOptions,
    io::{BufReader, BufWriter},
    process::exit,
};

#[derive(Deserialize, Serialize, Debug)]
struct Entry {
    title: String,
    user: String,
    pass: String,
}

enum Command {
    Add {
        title: String,
        user: String,
        pass: String,
    },
    Delete {
        title: String,
    },
    List,
}

fn print_help_menu() {
    println!("usage: pw <flag> <args>");
    println!("-a : adds entry to file\t\tex. pw -a <title> <username> <password>");
    println!("-d : deletes entry in file\tex. pw -d <title>");
    println!("-l : lists all entries");
}

// Define a function to add a new password entry to the password manager file
fn add_entry(title: &str, user: &str, pass: &str) {
    // Open the "pwd.json" file with read and write permissions
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("pwd.json")
        .expect("add_entry: Failed to open pwd.json (1)");

    // Read the contents of the file into a string variable
    let reader = BufReader::new(file);

    // Initialize an empty HashMap to hold the password entries
    let mut pwd_map: HashMap<String, Entry> = match serde_json::from_reader(reader) {
        Ok(map) => map,
        Err(_) => HashMap::new(),
    };

    // Check if an entry with the same title already exists in the HashMap
    if pwd_map.contains_key(title) {
        println!("Entry already exists.");
        exit(0);
    }

    // Create a new password entry with the provided title, username, and password
    let new_entry = Entry {
        title: title.to_string(),
        user: user.to_string(),
        pass: pass.to_string(),
    };

    // Insert the new entry into the HashMap
    pwd_map.insert(title.to_string(), new_entry);

    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("pwd.json")
        .expect("add_entry: Faile to open pwd.json (2)");

    let writer = BufWriter::new(file);

    serde_json::to_writer_pretty(writer, &pwd_map).expect("Failed to write to pwd.json");
}

fn delete_entry(title: &str) {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open("pwd.json")
        .expect("delete_entry: Failed to open pwd.json (1)");

    let reader = BufReader::new(file);

    let mut pwd_map: HashMap<String, Entry> = match serde_json::from_reader(reader) {
        Ok(map) => map,
        Err(_) => HashMap::new(),
    };

    if pwd_map.contains_key(title) {
        pwd_map.remove(title);
    } else {
        println!("Entry does not exist.");
        exit(0);
    }

    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("pwd.json")
        .expect("delete_entry: Failed to open pwd.json (2)");

    let writer = BufWriter::new(file);

    serde_json::to_writer_pretty(writer, &pwd_map).expect("Failed to write to pwd.json");
}

fn list_entries() {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("pwd.json")
        .expect("list_entries: Failed to open pwd.json");

    let reader = BufReader::new(file);

    let pwd_map: HashMap<String, Entry> = match serde_json::from_reader(reader) {
        Ok(map) => map,
        Err(_) => HashMap::new(),
    };

    for kv in pwd_map {
        println!("{}", kv.0);
    }

}

fn parse_args(args: &Vec<String>) -> Result<Command, ()> {
    match args.as_slice() {
        [_, flag, title, user, pass] => {
            if flag == "-a" {
                Ok(Command::Add {
                    title: title.to_string(),
                    user: user.to_string(),
                    pass: pass.to_string(),
                })
            } else {
                Err(())
            }
        }

        [_, flag, title] => {
            if flag == "-d" {
                Ok(Command::Delete {
                    title: title.to_string(),
                })
            } else {
                Err(())
            }
        }

        [_, flag] => {
            if flag == "-l" {
                Ok(Command::List)
            } else {
                Err(())
            }
        }
        _ => Err(()),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = parse_args(&args);

    match command {
        Ok(Command::Add { title, user, pass }) => {
            add_entry(title.as_str(), user.as_str(), pass.as_str());
        },
        Ok(Command::Delete { title }) => {
            delete_entry(title.as_str());
        },
        Ok(Command::List) => {
            list_entries();
        },
        Err(_) => {
            print_help_menu();
        },
    }
}
