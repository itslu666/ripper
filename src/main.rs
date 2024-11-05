use clap::{Arg, Command};

fn dig() {
    println!("dig");
}

fn resurrect() {
    println!("resurrect");
}

fn delete_file() {
    println!("delete files");
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
            Arg::new("resurrect")
                .short('r')
                .long("resurrect")
                .help("Resurrect (Recover) item from trash bin")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    // delete files if no flags are set
    if !matches.get_flag("dig") && !matches.get_flag("resurrect") {
        delete_file();
        return; // End function after files deleted
    }

    // check if dig flag is set
    if matches.get_flag("dig") {
        dig();
    }

    // check if resurrect flag is set
    if matches.get_flag("resurrect") {
        resurrect();
    }
}
