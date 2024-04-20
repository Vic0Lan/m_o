use owo_colors::OwoColorize;
use std::env;
use std::fs;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};
use std::thread;

// Crea una funzione per eseguire il clean della directory ✅
// Crea una funzione per attraversare le directory da pulire ✅
// Implementa il threading ✅
// Implementa l'input da cli ✅
// Implementa output migliorato (usa owo_colors ad esempio) ✅
// Implementa output verboso ✅
//
// Progetto finito per il momento!!!

static mut VERBOSE: bool = false;

fn clean(path: &Path) -> Result<(), Error> {
    //[ Controllo che la cartella esiste ]
    let flag = if path.exists() { true } else { false };

    let path_name = path
        .to_str()
        .expect("failed to scan path".on_bright_red().to_string().as_str())
        .split("/")
        .collect::<Vec<_>>();

    let folder_name = path_name.get(path_name.len() - 2);

    //[ Rimuovo la cartella ]
    fs::remove_dir_all(path).unwrap_or_else(|err| match Error::kind(&err) {
        ErrorKind::NotFound => {
            if unsafe { VERBOSE == true } {
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

fn traverse_directories(dir: &str) -> Result<(), Error> {
    let dirs = fs::read_dir(dir)?;
    let mut handlers = Vec::new();

    for entry in dirs {
        let handler = thread::spawn(|| -> Result<(), Error> {
            let mut buffer = PathBuf::new();

            buffer.push(entry?.path());
            buffer.push("target");

            clean(&buffer)?;
            Ok(())
        });

        handlers.push(handler);
    }

    for handler in handlers {
        handler.join().expect(
            "failed to await process"
                .on_bright_red()
                .to_string()
                .as_str(),
        )?;
    }

    Ok(())
}

fn main() {
    println!("Hi i am Mo and im a here to clean ");
    println!("Cleaning...");

    let args = env::args().collect::<Vec<_>>();

    if let Some(str) = args.get(2) {
        if str == "--verbose" {
            unsafe {
                VERBOSE = true;
            }
        }
    }

    let out = traverse_directories(
        args.get(1)
            .expect("failed to read argument".on_red().to_string().as_str()),
    );

    match out {
        Ok(_) => println!("{}", "Your space is now freed, bye bye".green()),
        Err(err) => {
            println!("{}", err.on_red());
        }
    };
}
