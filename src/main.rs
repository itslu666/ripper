use clap::{Arg, Command};
use std::fs;
use std::env;
use std::path::PathBuf;

fn dig() {
    println!("dig");
}

fn revive(items: &[String]) {
    for item in items {
        println!("reviving: {}", item);
    }
}

fn delete_file(items: &[String]) {
    if items.is_empty() {
        println!("No files specified to delete.");
    } else {
        for item in items {
            println!("deleting: {}", item);

            let home_dir = env::var("HOME").expect("Home directory not found.");
            let mut trash = PathBuf::from(home_dir);
            trash.push(".local/share/Trash/files");
            trash.push(item);

            match fs::rename(item, &trash) {
                Ok(_) => println!("Deleted {}", item),
                Err(e) => eprintln!("Failed to delete {}: {}", item, e),
            }
        }
    }
}

fn main() {
    let matches = Command::new("ripper")
        .version("0.1")
        .about("A CLI tool to manage Linux trash bin")
        .arg(
            Arg::new("dig")
                .short('d')
                .long("dig")
                .help("Dig out (List) all trashed items")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("revive")
                .short('r')
                .long("revive")
                .help("Revive (Recover) items from trash bin")
                .num_args(1..)
                .action(clap::ArgAction::Set),
        )
        .arg(
            Arg::new("bury")
                .short('b')
                .long("bury")
                .help("Bury (delete) an item")
                .num_args(1..)
                .action(clap::ArgAction::Set),
        )
        .arg(
            Arg::new("files")
                .help("Files to delete")
                .num_args(0..)
                .index(1), // Set the first positional argument to index 1
        )
        .get_matches();

    let mut items_to_delete = Vec::new();

    // Check for files in the "files" argument
    if let Some(files) = matches.get_many::<String>("files") {
        items_to_delete.extend(files.map(|s| s.to_string()));
    }

    // Check if bury flag is set and get the associated items
    if let Some(items) = matches.get_many::<String>("bury") {
        items_to_delete.extend(items.map(|s| s.to_string()));
    }

    // If no flags are set, delete files
    if !matches.get_flag("dig") && !matches.contains_id("revive") && items_to_delete.is_empty() {
        delete_file(&[]);
        return; // End function after files deleted
    }

    // Check if dig flag is set
    if matches.get_flag("dig") {
        dig();
    }

    // Check if revive flag is set and get the associated items
    if let Some(items) = matches.get_many::<String>("revive") {
        let items_vec: Vec<String> = items.map(|s| s.to_string()).collect();
        revive(&items_vec);
    }

    // Finally, delete the collected items
    if !items_to_delete.is_empty() {
        delete_file(&items_to_delete);
    }
}
