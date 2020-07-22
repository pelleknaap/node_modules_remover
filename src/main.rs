use clap::App;
use std::ffi::{OsStr, OsString};
use std::fs;
use walkdir::WalkDir;

fn main() {
    let matches = App::new("Node Modules Remover")
        .version("1.0")
        .author("Pelle van der Knaap pelle.vd.knaap@xs4all.nl")
        .about("Searches for node_modules folders that haven't been used for X amount of time and deleted them")
        .arg("<path> 'Sets the path'")
        .arg("-a, --age=[AGE] 'Sets minimum age in weeks of the node_modules folders that should be removed")
        .get_matches();

    let output = matches.value_of("path").unwrap();

    let min_age = matches.value_of("age").unwrap_or("10");
    let min_age = min_age
        .parse::<u64>()
        .expect("Please use a valid float as the age");

    println!("Minimum age of node_modules folders: {}", min_age);
    println!("Targeted folder for node_modules deletion: {}", output);
    println!("Looking for node_modules folders... this can take a while for large folders");

    // Using a label for the outer loop so we can call continue on it later on
    'outer: for entry in WalkDir::new(output).into_iter().filter_map(|e| e.ok()) {
        // Trying to get the metadata so we can get the last accessed time
        if let Ok(time) = entry
            .metadata()
            .expect("Couldn't get the metadata of the file")
            .accessed()
        {
            let file_name = entry.file_name();
            let path = entry.path();

            if check_if_path_is_node_modules(file_name) {
                // Getting the elapsed from when the node_modules was used for the last time and converting it to weeks from seconds
                let last_accessed_in_weeks =
                    time.elapsed().expect("couldn't get elapsed time").as_secs() / 3600 / 24;

                if last_accessed_in_weeks > min_age {
                    // Checking if the node_modules folder isn't already inside another one, so we don't double delete stuff
                    for ancestor in path.ancestors().skip(1) {
                        if let Some(file_name) = ancestor.file_name() {
                            if check_if_path_is_node_modules(file_name) {
                                continue 'outer;
                            }
                        }
                    }

                    println!("Found node_modules at: {}", path.display());

                    // Removing the actual folder
                    fs::remove_dir_all(path).expect("Couldn't remove folder");
                }
            }
        } else {
            println!("Not supported on this platform");
        }
    }

    println!("All unused/old node_modules folders have successfully been removed, thank you for using this CLI.")
}

fn check_if_path_is_node_modules(file_name: &OsStr) -> bool {
    file_name == OsString::from("node_modules")
}
