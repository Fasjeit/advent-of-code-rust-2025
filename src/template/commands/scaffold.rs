use std::{
    fs,
    fs::{File, OpenOptions},
    io::Write,
    process,
};

use crate::template::Day;

const MODULE_TEMPLATE: &str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/template.txt"));

fn safe_create_file(path: &str, overwrite: bool) -> Result<File, std::io::Error> {
    let mut file = OpenOptions::new();
    if overwrite {
        file.create(true);
    } else {
        file.create_new(true);
    }
    file.truncate(true).write(true).open(path)
}

fn create_file(path: &str) -> Result<File, std::io::Error> {
    OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
}

pub fn handle(day: Day, overwrite: bool) {
    let day_path = format!(day_path!(), day);
    let examples_path = format!(examples_path!(), day);

    let input_path = format!(input_path!(), day);
    let example_path = format!(example_path!(), day);
    let module_path = format!(module_path!(), day);

    match fs::create_dir_all(day_path) {
        Ok(()) => (),
        Err(e) => {
            eprintln!("Failed to create day folder: {e}");
            process::exit(1);
        }
    };

    let mut file = match safe_create_file(&module_path, overwrite) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to create module file: {e}");
            process::exit(1);
        }
    };

    match file.write_all(
        MODULE_TEMPLATE
            .replace("%DAY_NUMBER%", &day.into_inner().to_string())
            .as_bytes(),
    ) {
        Ok(()) => {
            println!("Created module file \"{}\"", &module_path);
        }
        Err(e) => {
            eprintln!("Failed to write module contents: {e}");
            process::exit(1);
        }
    }

    match create_file(&input_path) {
        Ok(_) => {
            println!("Created empty input file \"{}\"", &input_path);
        }
        Err(e) => {
            eprintln!("Failed to create input file: {e}");
            process::exit(1);
        }
    }

    match fs::create_dir_all(examples_path) {
        Ok(()) => (),
        Err(e) => {
            eprintln!("Failed to create examples folder: {e}");
            process::exit(1);
        }
    };

    match create_file(&example_path) {
        Ok(_) => {
            println!("Created empty example file \"{}\"", &example_path);
        }
        Err(e) => {
            eprintln!("Failed to create example file: {e}");
            process::exit(1);
        }
    }

    println!("---");
    println!("🎄 Type `cargo solve {day}` to run your solution.");
}
