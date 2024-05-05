use owo_colors::OwoColorize;
use std::env;
use std::fs;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};

// Crea una funzione per eseguire il clean della directory ✅
// Crea una funzione per attraversare le directory da pulire ✅
// Implementa il threading ✅
// Implementa l'input da cli ✅
// Implementa output migliorato (usa owo_colors ad esempio) ✅
// Implementa output verboso ✅
//
// Implementa la ricorsione delle cartelle ✅
// Sistema l'error handling di clean in relazione ai file

pub fn clean(path: &Path) -> Result<(), Error> {
    //[ Controllo che la cartella esiste ]
    let flag = if path.exists() { true } else { false };

    if path.is_file() {
        return Ok(());
    }

    let path_name = path
        .to_str()
        .expect("failed to scan path".on_bright_red().to_string().as_str())
        .split("/")
        .collect::<Vec<_>>();

    let folder_name = path_name.get(path_name.len() - 2);

    //[ Rimuovo la cartella ]
    fs::remove_dir_all(path).unwrap_or_else(|err| match Error::kind(&err) {
        ErrorKind::NotFound => {
            println!(
                "Target folder not found: {:?}",
                folder_name
                    .expect(
                        "failed to obtain folder name"
                            .on_bright_red()
                            .to_string()
                            .as_str()
                    )
                    .yellow()
            );
        }
        _ => eprintln!("Another kind of error: {}", Error::kind(&err).on_red()),
    });

    //[ Se la cartella è stata pulita allora stampo ]
    if flag {
        println!(
            "folder {:?} cleaned",
            folder_name.expect(
                "failed to obtain folder name"
                    .on_bright_red()
                    .to_string()
                    .as_str()
            )
        );
    }

    Ok(())
}

fn traverse_directories(path: &PathBuf) -> Result<(), Error> {
    for entry in fs::read_dir(path)? {
        let entry = match entry.as_ref() {
            Ok(entry) => entry,
            Err(_) => continue,
        };

        if entry.path().ends_with(".git") {
            return Ok(());
        }

        if entry.path().ends_with("target") || entry.path().ends_with("release") {
            clean(entry.path().as_path())?;
        }

        if entry.path().is_dir() {
            traverse_directories(&entry.path())?;
        }
    }

    return Ok(());
}

fn help() {
    println!(
        "{}",
        "Hello, to use Mo type: \nm_o <path_to_folder>".green()
    );

    println!(
        "{}",
        "You can also add the --verbose flag or verbose output: \nm_o <path_to_folder> --verbose"
            .green()
    );
}

fn main() {
    let args = env::args().collect::<Vec<_>>();

    if args.len() < 2 {
        help();
        return;
    }

    println!("Mo ready for cleaning");

    let out = traverse_directories(&PathBuf::from(
        args.get(1)
            .expect("failed to read argument".on_red().to_string().as_str()),
    ));

    match out {
        Ok(_) => println!("{}", "Your space is now freed, bye bye".green()),
        Err(err) => {
            println!("{}", err.on_red());
        }
    };
}
