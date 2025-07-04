use super::interface;
use super::parsed_dream::*;

use rustyline::{history::MemHistory, Editor};
use std::path::*;

pub(crate) fn get_dme_path_from_dir(rl_m: &mut Editor<(), MemHistory>) -> Option<PathBuf> {
    let dme_dir: PathBuf = interface::path_nav(rl_m);
    let mut dir_contents = dme_dir.read_dir().unwrap().map(|entry| entry.unwrap()); 
    match dir_contents.find(|entry|
        entry.path().ends_with("dme")
    ) { 
        Some(found_dme) => Some(found_dme.path()),
        None => None,
    }
}

pub(crate) fn add_dream(rl_m: &mut Editor<(), MemHistory>) -> Option<ParsedDream> {
    let dme_path = get_dme_path_from_dir(rl_m);
    if let None = dme_path {
        println!("Couldn't find a .dme in that directory.");
        return None
    } else {
        let new_dream = ParsedDream::new(&dme_path.unwrap());
        Some(new_dream)
    }
}