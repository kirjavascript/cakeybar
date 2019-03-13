use std::fs;
use std::path::Path;
use std::os::unix::fs::PermissionsExt;
use std::env;
use std::collections::HashSet;

pub fn get_programs_set() -> HashSet<String> {
    let mut programs = HashSet::new();

    let is_exec = |metadata: fs::Metadata| metadata.permissions().mode() & 0o111 != 0;

    env::var("PATH").unwrap_or_else(|_| "/usr/bin".to_string())
        .split(":")
        .for_each(|path| {
            if let Ok(listing) = fs::read_dir(Path::new(&path)) {
                for entry in listing {
                    if let Ok(entry) = entry {
                        if let Ok(metadata) = entry.metadata() {
                            if metadata.is_file() && is_exec(metadata) {
                                if let Ok(filename) = entry.file_name().into_string() {
                                    programs.insert(filename);
                                }
                            }
                        }
                    }
                }
            }
        });

    programs
}

pub fn get_programs_vec() -> Vec<String> {
    let mut programs = get_programs_set()
        .iter().cloned().collect::<Vec<String>>();
    programs.sort();
    programs
}
