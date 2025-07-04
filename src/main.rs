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
#![feature(lazy_get)]
mod dream_weaving;
mod parsed_dream;
mod interface;

use rustyline::{history::MemHistory, Config, Editor};
use std::collections::*;
use parsed_dream::{ParsedDream};
use interface::{main_menu,path_nav};
use dream_weaving::*;
use std::sync::LazyLock;

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
    let mut rl: Editor<(), MemHistory> = Editor::<(), MemHistory>::with_history(Config::default(), MemHistory::new())
        .expect("Failed to create editor");
    println!("Initiating add Dream for your first Dream...");
    loop {
        let first_dream = add_dream(&mut rl);
        if let None = first_dream {
            continue;
        }
        else {
            dream_space
        }
    }
}
