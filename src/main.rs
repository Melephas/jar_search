// std first :)
use std::{
    ffi::OsStr,
    fs::{
        self,
        DirEntry,
        File
    },
    io::Error,
    path::Path,
};

// clap for argument parsing
use clap::{
    self,
    App,
    Arg,
    ErrorKind
};

// zip for checking if an archive contains the requested file
use zip::ZipArchive;

fn main() -> Result<(), String> {
    match App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .about(clap::crate_description!())
        .author(clap::crate_authors!())
        .arg(Arg::with_name("dir")
                        .short("d")
                        .long("directory")
                        .takes_value(true)
                        .default_value(".")
        ).arg(Arg::with_name("class")
                        .short("c")
                        .long("class")
                        .takes_value(true)
                        .required(true)
        ).get_matches_safe() {
            Ok(matches) => {
                // Get the value of the --directory flag
                let path_str = matches.value_of("dir").expect("No directory value specified");

                // Convert to std::path::Path
                let path = Path::new(path_str);

                // Get the value of the --class flag
                let class_str = matches.value_of("class").expect("No class value specified");

                print_entries(&path, class_str);
                Ok(())
            },
            Err(err) if err.kind == ErrorKind::HelpDisplayed || err.kind == ErrorKind::VersionDisplayed => {
                err.exit();
            }, 
            Err(dfault) => Err(String::from(dfault.message))
        }
}

fn search(dir: &Path) -> Vec<DirEntry> {
    let mut final_entries = Vec::new();
    let entries = fs::read_dir(dir).expect("Error reading directory").collect::<Result<Vec<_>, Error>>();
    let entries = entries.expect("Error collecting directory entries");
    for entry in entries {
        if entry.metadata().expect("Failed to get metadata").is_dir() {
            final_entries.append(&mut search(entry.path().as_path()));
        } else {
            final_entries.push(entry);
        }
    }

    return final_entries;
}

fn print_entries(dir: &Path, class: &str) {
    let entries = search(dir);
    let entries = filter_entries_by_extention(entries, OsStr::new("jar"));

    //let class_path = class.replace("/", ".");

    for entry in entries {
        let archive = File::open(entry.path()).expect("Failed to open file");
        let zip_reader = ZipArchive::new(archive).expect("Failed to open file");
        for internal_file in zip_reader.file_names() {
            if String::from(internal_file) == class {
                println!("{}", entry.path().to_string_lossy());
            }
        }
    }
}

fn filter_entries_by_extention(entries: Vec<DirEntry>, extension: &OsStr) -> Vec<DirEntry> {
    let mut ret = Vec::new();
    
    for entry in entries {
        if let Some(ext) = entry.path().extension() {
            if ext == extension {
                ret.push(entry);
            }
        }
    }

    return ret;
}
