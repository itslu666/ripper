use clap::{Arg, Command};

fn dig() {
    println!("dig")
}

fn resurrect() {
    println!("resurrect")
}

fn main() {
    let matches = Command::new("ripper")
    .version("0.1")
    .about("A CLI tool to manage Linux trash bin")
    .arg(Arg::new("dig")
        .short('d')
        .long("dig")
        .help("Dig out (List) all trashed items"))

    .arg(Arg::new("resurrect")
        .short('r')
        .long("resurrect")
        .help("Resurrect (Recover) item from trash bin"))

    .get_matches();

    if matches.contains_id("dig") {
        dig();
    }

    if matches.contains_id("resurrect") {
        resurrect();
    }
}
