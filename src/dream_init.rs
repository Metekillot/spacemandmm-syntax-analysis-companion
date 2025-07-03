use super::interface;
use super::parsed_dream::*;
use std::path::*;
use std::collections::HashMap;

pub(crate) fn get_dme_path_from_dir() -> Option<PathBuf> {
    let mut dme_dir: PathBuf = interface::path_nav();
    let mut dir_contents = dme_dir.read_dir().unwrap().map(|entry| entry.unwrap()); 
    match dir_contents.find(|entry|
        entry.path().ends_with("dme")
    ) { 
        Some(found_dme) => Some(found_dme.path()),
        None => None,
    }    
}
