use super::interface::{path_nav,name_dream};
use super::parsed_dream::*;

use rustyline::{history::MemHistory, Editor};
use std::path::*;
use std::collections::HashMap;

pub(crate) fn get_dme_path_from_dir(rl_m: &mut Editor<(), MemHistory>) -> Option<PathBuf> {
    let dme_dir: PathBuf = path_nav(rl_m);
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

pub(crate) fn confirm_new_dream_name(
    rl_m: &mut Editor<(), MemHistory>,
    dream_space: &HashMap<String, Box<ParsedDream>>,
) -> String {
    let mut dream_name = name_dream(rl_m);
    if dream_space.contains_key(&dream_name) {
        println!("Dream with that name already exists. If you want to replace that dream, enter the same name again.");
        let confirmed_name = name_dream(rl_m);
        if confirmed_name == dream_name {
            println!("Replacement confirmed. {} will be replaced by parsing the new Dream.", dream_name);
        } else {
            println!("Replacement aborted. The new Dream will be named {} instead.", confirmed_name);
            dream_name = confirmed_name;
        }
    } 
        println!("Dream '{}' targeted; proceeding!", dream_name);
        dream_name
    }
