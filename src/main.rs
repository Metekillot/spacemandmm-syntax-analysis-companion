/*
    Copyright (C) 2025  Joshua 'Joan Metek Metekillot' Kidder

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/
use console::*;
use dm::annotation::*;
use dm::indents::*;
use dm::lexer::*;
use dm::objtree::*;
use dm::parser::*;
use dm::preprocessor::*;
use dm::*;
use std::collections::*;
use std::env;
use std::path::*;
use std::process::*;
use std::str::FromStr;
use std::sync::LazyLock;
use std::vec::*;

static TERMINAL: LazyLock<Term> = LazyLock::new(|| console::Term::stdout());
use char_input as c_i;
use line_input as l_i;
use line_input_into as l_i_i;

fn line_input_into<T: FromStr>(prompt: &str) -> T {
    loop {
        let got_input = line_input(prompt);
        match got_input.parse::<T>() {
            Ok(result) => break result,
            Err(error) => {
                println!("{} couldn't be parsed, please try again.", got_input);
                continue;
            }
        }
    }
}

fn line_input(prompt: &str) -> String {
    loop {
        println!("{}", prompt);
        match TERMINAL.read_line() {
            Ok(result) => break result,
            Err(err) => {
                println!("Error reading line");
                continue;
            }
        }
    }
}

fn char_input(prompt: &str) -> char {
    loop {
        println!("{}", prompt);
        match TERMINAL.read_key() {
            Ok(Key::Char(c)) => return c,
            Ok(_) => {
                continue;
            }
            Err(e) => {
                println!("Error reading character: {}", e);
                continue;
            }
        }
    }
}

fn get_dme_path_from_dir() -> PathBuf {
    let mut dme_dir: PathBuf = PathBuf::new();
    let init_input = l_i("Please input the .dme directory. '$env_var' to access an environment variable, '!command arg1 arg2 arg3' to invoke a process's output for the value.");
    match *&init_input
        .chars()
        .nth(0)
        .expect("Fatal failure to parse initial input.")
    {
        '$' => {
            let fetched_var_name: &String = &init_input.chars().skip(1).collect();
            let fetched_var_value = env::var(fetched_var_name).expect(&format!(
                "Failed to fatch environmental variable: {}",
                fetched_var_name
            ));
            dme_dir.push(fetched_var_value);
        }
        '!' => {
            let parsed_command: &String = &init_input.chars().skip(1).collect();
            let mut command_parts = parsed_command.split(' ').into_iter();
            let actual_command = command_parts
                .next()
                .expect("Failed to parse a command for a process to initiate.");
            let command_output = Command::new(actual_command)
                .args(command_parts.collect::<Vec<_>>())
                .output()
                .unwrap();
            let trimmed_out = &*String::from_utf8_lossy(command_output.stdout.trim_ascii_end());
            println!("{:?}", &trimmed_out);
            dme_dir.push(&trimmed_out);
        }
        _ => {
            dme_dir.push(init_input);
        }
    }
    (*dme_dir)
        .read_dir()
        .expect(&format!("Failed to read directory: {}", dme_dir.display()))
        .map(|res| res.expect("Invalid directory entry"))
        .filter(|file| file.file_name().to_str().unwrap().ends_with(".dme"))
        .next()
        .unwrap()
        .path()
}

pub(crate) struct ParsedDream {
    context: Context,
    annotation_tree: AnnotationTree,
    object_tree: ObjectTree,
}

impl ParsedDream {
    fn new(dme_path: &PathBuf) -> ParsedDream {
        let context = Context::default();
        let mut annotation_tree = AnnotationTree::default();
        let pre_processor = Preprocessor::new(&context, dme_path.to_owned()).unwrap();
        let indent_processor = IndentProcessor::new(&context, pre_processor);
        let mut parser = Parser::new(&context, indent_processor);
        let annotation_tree_mutable = &mut annotation_tree;
        parser.annotate_to(annotation_tree_mutable);

        let object_tree = parser.parse_object_tree();
        dmc::run(&context, &object_tree);
        ParsedDream {
            context,
            annotation_tree,
            object_tree,
        }
    }
}

fn add_dream(dream_space: &mut HashMap<String, Box<ParsedDream>>) {
    let dme_path = &get_dme_path_from_dir();
    println!("Dream parsing, please wait...");
    let new_dream = ParsedDream::new(dme_path);
    println!("Dream realized from {}", dme_path.display());
    let name = l_i("Please give a unique name for this Dream.");
    dream_space.insert(name, Box::new(new_dream));
}

fn list_dreams(dream_space: &HashMap<String, Box<ParsedDream>>) {
    println!();
    println!("Dreams realized:");
    for dream_name in dream_space.keys() {
        println!("{}", dream_name);
    }
    println!();
}

fn modify_dreams(dream_space: &mut HashMap<String, Box<ParsedDream>>) {
    list_dreams(dream_space);
    loop {
        match c_i("'d'elete a Dream, 'i'nspect a Dream, 'e'xit Dream modification") {
            'd' => delete_dream(dream_space),
            'i' => println!("To inspect Dreams, load all Dreams from the main menu and continue."),
            'e' => break,
            _ => continue,
        }
    }
}

fn delete_dream(dream_space: &mut HashMap<String, Box<ParsedDream>>) {
    let name_set: Vec<String> = dream_space.keys().map(|key| key.to_owned()).collect();
    loop {
        let dream_name_to_delete =
            l_i("Enter the name of the Dream you're deleting, or, 'exit' to cancel");
        if dream_name_to_delete == "exit" {
            break;
        }
        match name_set.contains(&dream_name_to_delete) {
            true => {
                let answer = c_i("Confirm deletion of this Dream? 'Y' to confirm.");
                if ['y', 'Y'].contains(&answer) {
                    dream_space
                        .remove(&dream_name_to_delete)
                        .expect("Failed to give up on a Dream -- inspirational, but not intentional here...");
                    println!("Dream deleted.");
                    break;
                } else {
                    println!("Dream deletion aborted -- inspirational!");
                    break;
                }
            }
            false => println!("Couldn't find that Dream..."),
        }
    }
}

fn hello_world() {
    print!("{}",
    "
    ~~~~~~~~~~~~~~~~~~~~
    SpacemanDMM-Analysis-Companion  Copyright (C) 2025 Joshua 'Joan Metek Metekillot' Kidder
                                    joanmetek@gmail.com
                                    
    This program comes with ABSOLUTELY NO WARRANTY; for details, see included LICENSE
    This is free software, and you are welcome to redistribute it
    under certain conditions; see included LICENSE for details

    This program is only made possible by thousands of hours of volunteer development from SpacemanDMM and BYOND
    See https://github.com/SpaceManiac/SpacemanDMM
    The BYOND software is a copyrighted work, All Rights Reserved, courtesy of https://byond.com
    ~~~~~~~~~~~~~~~~~~~~
    ");
}
fn main() {
    hello_world();
    let mut dream_space: HashMap<String, Box<ParsedDream>> = HashMap::new();
    add_dream(&mut dream_space);
    loop {
        match c_i("'a'dd another dream\n'c'ontinue with your current dreams\n'l'ist your current dreams\n'm'odify your loaded dreams\n'q'uit dreaming"){
            'a' => add_dream(&mut dream_space),
            'c' => todo!(),
            'l' => list_dreams(&dream_space),
            'm' => modify_dreams(&mut dream_space),
            'q' => todo!(),
            _ => continue,
        }
    }
}
