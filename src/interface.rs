use super::dream_weaving::*;
use super::parsed_dream::*;
use crate::MenuChoice;
use crate::dream_analysis::AnalyzedDream;

use rustyline::Cmd;
use rustyline::KeyCode;
use rustyline::{Editor, history::MemHistory};
use std::collections::HashMap;
use std::env::{current_dir, set_current_dir};
use std::ops::DerefMut;
use std::path::{Path, PathBuf};
use std::process::exit;
use std::str::FromStr;

pub(crate) fn space_output<'a>(
    mut to_space: impl Iterator<Item = &'a str>,
    how_many: usize,
) -> String {
    let mut formed_output = String::new();
    let mut line_break_signal: usize = 0;
    loop {
        match to_space.next() {
            Some(value) => {
                &mut formed_output.push_str(value);
            }
            None => {
                &mut formed_output.push_str("\n");
                break;
            }
        }
        line_break_signal += 1;
        if line_break_signal == how_many {
            formed_output.push_str("\n");
            line_break_signal = 0;
        }
    }
    formed_output
}
enum PathNavOperators {
    Here,
    Parent,
    Root,
}

impl PathNavOperators {
    fn op(self) {
        match self {
            PathNavOperators::Here => {
                current_dir()
                    .unwrap()
                    .read_dir()
                    .unwrap()
                    .for_each(|dir_entry| {
                        let this_entry = dir_entry.unwrap();
                        println!(
                            "{} {}",
                            &this_entry.file_name().into_string().unwrap(),
                            (|| {
                                let file_type = &this_entry.file_type().unwrap();
                                if file_type.is_dir() { "<DIR>" } else { "" }
                            })()
                        )
                    })
            }
            PathNavOperators::Parent => {
                let mut to_path = current_dir().unwrap();
                to_path.pop();
                set_current_dir(to_path).expect("Failed to set current path to parent of path");
            }
            PathNavOperators::Root => {
                set_current_dir("\\").expect("Failed to set current path to root");
            }
        }
    }
}

pub(crate) fn main_menu(rl_m: &mut Editor<(), MemHistory>) -> MenuChoice {
    let prompt = "
    1. Add Dream
    2. List Dreams
    3. Modify Dreams
    4. Quit Dreaming
    ";
    loop {
        let response = rl_m.readline(prompt).unwrap();
        match response.as_str() {
            "1" => break MenuChoice::AddDream,
            "2" => break MenuChoice::ListDreams,
            "3" => break MenuChoice::ModifyDreams,
            "4" => break MenuChoice::QuitDreaming,
            "5" => break MenuChoice::DreamInterpretation,
            _ => continue,
        }
    }
}

pub(crate) fn path_nav(rl_m: &mut Editor<(), MemHistory>) -> PathBuf {
    let cwd_call = || current_dir().expect("Failure to resolve current_dir()");
    let cwd_prompt_full = || {
        format!(
            "
        help : Display this message again
        . or null entry : View current directory (does not terminate)
        .. : navigate into parent directory (does not terminate)
        \\ : navigates to the root (does not terminate)
        exit : exit the program (terminates program)
        ! : Return the current directory (terminates input)
        all other entries : navigates to path, relative from CWD (does not terminate)
        entries starting with '\\' navigates relative to root, instead
        
    Current: {}\n",
            cwd_call().display()
        )
    };
    let mut show_full_help: bool = true;
    let flip_handle = &mut show_full_help;

    loop {
        let prompt = if *flip_handle {
            *flip_handle = false;
            cwd_prompt_full()
        } else {
            format!(
                "
    Current: {}\n",
                cwd_call().display()
            )
        };
        let path_result = rl_m
            .readline(&prompt)
            .expect("Failed to parse path_nav() line entry");
        match path_result.as_str() {
            "!" => return cwd_call(),
            "." | "" => {
                PathNavOperators::op(PathNavOperators::Here);
            }
            "\\" => {
                PathNavOperators::op(PathNavOperators::Root);
            }
            ".." => PathNavOperators::op(PathNavOperators::Parent),
            "help" => {
                *flip_handle = true;
            }
            "exit" => exit(0),
            try_path_to => {
                let root_or_relative = try_path_to
                    .chars()
                    .nth(0)
                    .expect("Index of first path character failed");
                let result: Result<(), std::io::Error>;
                if root_or_relative == '\\' {
                    result = set_current_dir(Path::new(format!("\\{}", try_path_to).as_str()));
                } else {
                    let mut current_dir = cwd_call();
                    current_dir.push(try_path_to);
                    result = set_current_dir(&current_dir);
                }
                match result {
                    Ok(_) => {}
                    Err(_) => println!("{} was invalid!", try_path_to),
                }
            }
        }
    }
}

pub(crate) fn name_dream(rl_m: &mut Editor<(), MemHistory>) -> String {
    let mut dream_name = String::new();
    loop {
        let prompt = "Enter a name for the dream: ";
        dream_name = rl_m
            .readline(prompt)
            .expect("Failed to parse name_dream() line entry")
            .trim()
            .to_string();
        if dream_name.is_empty() {
            println!("Dream name cannot be empty. Please try again.");
        } else {
            break dream_name;
        }
    }
}

pub(crate) fn analysis_menu(
    rl_m: &mut Editor<(), MemHistory>,
    mut dream_analyses: HashMap<String, Box<AnalyzedDream>>,
) {
    loop {
        let prompt = "
        1. List analyzed Dreams
        2. Modify analyzed Dreams
        3. Explore analyzed Dreams
        4. Add another Dream to analyze
        0. Quit
        ";
        let response = rl_m
            .readline(prompt)
            .expect("Failed to get analysis choice");
        if response.is_empty() {
            continue;
        }
        match response.chars().nth(0).unwrap() {
            '1' => todo!(),
            '2' => todo!(),
            '3' => explore_dreams(rl_m, &mut dream_analyses),
            '4' => todo!(),
            '0' => todo!(),
            _ => continue,
        }
    }
}

pub(crate) fn explore_dreams(
    rl_m: &mut Editor<(), MemHistory>,
    dream_analyses: &mut HashMap<String, Box<AnalyzedDream>>,
) {
    let analyses_keys = dream_analyses.keys();
    let mut prompt = String::new();
    prompt.push_str("\n");
    prompt.push_str(&space_output(analyses_keys.map(|key| key.as_str()), 3));
    prompt.insert_str(0, &"         Choose the Dream to explore.\n");
    loop {
        let response = rl_m
            .readline(&prompt)
            .expect("Failed to read line for Dream name to retrieve");
        if let Some(dream_to_explore) = dream_analyses.get_mut(&response) {
            exploration_choice_menu(rl_m, dream_to_explore);
        } else {
            println!("That Dream name wasn't found.");
            continue;
        }
    }
}

pub(crate) fn exploration_choice_menu(
    rl_m: &mut Editor<(), MemHistory>,
    dream_to_explore: &mut AnalyzedDream,
) {
    let prompt = "
    1. Explore Procs
    2. Explore Files
    3. Explore Types
    4. View completed explorations
    0. Back to Dream Exploration Menu
    ";
    loop {
        let response = rl_m
            .readline(prompt)
            .expect("Failed to read line for exploration choice");
        match response.as_str() {
            "1" => todo!(),
            "2" => todo!(),
            "3" => todo!(),
            "4" => todo!(),
            "0" => break,
            _ => continue,
        }
        break;
    }
}

pub(crate) fn dme_from_dir(dme_dir: PathBuf) -> Option<PathBuf> {
    let mut dir_contents = dme_dir.read_dir().unwrap().map(|entry| entry.unwrap());
    match dir_contents.find(|entry| match entry.path().extension() {
        Some(extension) => extension == std::ffi::OsStr::new("dme"),
        _ => false,
    }) {
        Some(found_dme) => Some(found_dme.path()),
        None => None,
    }
}