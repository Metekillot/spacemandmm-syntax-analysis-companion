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
mod debug_run;
mod dream_analysis;
mod dream_weaving;
mod interface;
mod parsed_dream;

use dream_analysis::analyze_dreams;
use dream_weaving::*;
use interface::{analysis_menu, main_menu, name_dream};
use parsed_dream::ParsedDream;
use rustyline::{Config, Editor, history::MemHistory};
use std::collections::*;
use std::env;

enum MenuChoice {
    AddDream,
    ListDreams,
    ModifyDreams,
    QuitDreaming,
    DreamInterpretation,
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
    let mut arguments = env::args();
    match arguments.nth(1) {
        Some(arg) => {
            if arg == "--debug" {
                use debug_run;
                println!("Debug mode. Automatic choices & limited input.");
                match arguments.next() {
                    Some(debug_type) => match debug_type.as_str() {
                        "proc" => debug_run::debug_run(Some(debug_run::RunType::ProcRun)),
                        "file" => debug_run::debug_run(Some(debug_run::RunType::FileRun)),
                        "type" => debug_run::debug_run(Some(debug_run::RunType::TypeRun)),
                        _ => {
                            println!("\"{}\" wasn't understood, defaulting", debug_type);
                            debug_run::debug_run(Some(debug_run::RunType::ProcRun))
                        }
                    },
                    None => debug_run::debug_run(None),
                }
                std::process::exit(0);
            } else {
                println!(
                    "{} wasn't understood as an argument to the process, so ignoring.",
                    arg
                );
            }
        }
        None => {}
    }
    todo!("Checking arguments");
    let mut dream_space: HashMap<String, Box<ParsedDream>> = HashMap::new();
    let mut rl: Editor<(), MemHistory> =
        Editor::<(), MemHistory>::with_history(Config::default(), MemHistory::new())
            .expect("Failed to create editor");
    println!("Initiating add Dream for your first Dream...");
    loop {
        let dream_name = name_dream(&mut rl);
        let first_dream = add_dream(&mut rl);
        if let None = first_dream {
            continue;
        } else {
            let new_dream = first_dream.unwrap();
            dream_space.insert(dream_name, Box::new(new_dream));
            break;
        }
    }
    loop {
        match main_menu(&mut rl) {
            MenuChoice::AddDream => {
                let new_dream_name = confirm_new_dream_name(&mut rl, &dream_space);
                let new_dream = add_dream(&mut rl);
                if new_dream.is_none() {
                    continue;
                }
                dream_space.insert(new_dream_name, Box::new(new_dream.unwrap()));
            }
            MenuChoice::ListDreams => todo!(),
            MenuChoice::ModifyDreams => todo!(),
            MenuChoice::QuitDreaming => todo!(),
            MenuChoice::DreamInterpretation => break,
        }
    }
    let dream_exploration = analyze_dreams(&mut dream_space);
    analysis_menu(&mut rl, dream_exploration)
}
