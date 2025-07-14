use crate::interface::space_output;
use crate::parsed_dream;

use super::parsed_dream::ParsedDream;
use dm::annotation;
use dm::annotation::*;
use dm::ast::*;
use dm::objtree::*;
use dm::{Context, FileList, Location};
use rustyline::Editor;
use rustyline::history::MemHistory;
use std::collections::HashMap;
use std::iter::FlatMap;
use std::ops::Range;
use std::path::PathBuf;

pub(crate) struct AnalyzedDream {
    pub(crate) parsed_dream: ParsedDream,
    pub(crate) explored_types: HashMap<String, HashMap<String, Vec<String>>>,
}

impl AnalyzedDream {
    pub(crate) fn new(parsed_dream: ParsedDream) -> AnalyzedDream {
        AnalyzedDream {
            parsed_dream,
            explored_types: HashMap::new(),
        }
    }

    pub(crate) fn analyze_proc(&mut self, type_for: &str, proc_for: &str) {
        let inner_dream = &mut self.parsed_dream;
        let type_got = inner_dream
            .object_tree
            .find(type_for)
            .expect(&format!("type {} not found", type_for));
        let proc_got = type_got
            .get_proc(proc_for)
            .expect(&format!("proc {} not found on {}", proc_for, type_for))
            .get();
        let type_name = type_got.name();
        if self.explored_types.contains_key(type_name) {
            let type_entry = self.explored_types.get_mut(type_name).unwrap();
            if type_entry.contains_key(&proc_for.to_string()) {
                let proc_entry = type_entry.get_mut(proc_for).unwrap();
                proc_entry.iter().for_each(|string| println!("{}", string))
            } else {
                type_entry.insert(
                    proc_for.to_string().to_owned(),
                    Self::proc_exploration(&inner_dream.annotation_tree, proc_got),
                );
            }
        } else {
            self.explored_types.insert(type_name.to_owned(), {
                let mut new_hash: HashMap<String, Vec<String>> = HashMap::new();
                new_hash.insert(proc_for.to_string(), Self::proc_exploration(&inner_dream.annotation_tree, proc_got));
                new_hash
            });
        }
    }
    pub(crate) fn proc_exploration(
        annotation_tree: &AnnotationTree,
        proc_got: &ProcValue,
    ) -> Vec<String> {
        let proc_code = proc_got.clone().code.unwrap();
        let mut code_iter = proc_code.iter();
        let start_location = code_iter.next().unwrap().location;
        let annotations_got = annotation_tree.get_location(start_location);
        let mut cloned_annotations_got = annotations_got.clone();
        let proc_body: (Vec<Ident>, usize) = match cloned_annotations_got.nth(0).unwrap().1 {
            Annotation::ProcBody(body, idx) => {(body.clone(), idx.clone())},
            _ => panic!("It wasn't a proc body."),
        };
        let mut annotation_documentation: Vec<String> = Vec::new();
        for (iter_location, _annotation) in annotation_tree.iter() {
            match _annotation {
                Annotation::ProcBody(matching_body, _) => if matching_body == &proc_body.0 {
                    let line_check: &mut usize = &mut 0;
                    annotation_tree.get_range_raw(iter_location).for_each(|interval_pair|
                        {
                    if interval_pair.0.start.line as usize != *line_check {
                        *line_check = interval_pair.0.start.line as usize;
                        print!("\n-new_line-\nline {}: ", line_check);
                    }
                    print!("{:?} ", interval_pair.1);
                }
                );
                }
                _ => continue,
            }
        }
        println!("Count of matching annotations: {}", annotation_documentation.len());
        annotation_documentation
    }

    pub(crate) fn analyze_file(&mut self, file_for: PathBuf) {
        let context = &self.parsed_dream.context;
        let annotation_tree = &self.parsed_dream.annotation_tree;
        let mut annotation_iter = annotation_tree.iter().filter(|iteration|
        iteration.0.start.file == context.get_file(file_for.as_path()).unwrap());
        let line_track: &mut usize = &mut 0;
        annotation_iter.for_each(|iteration|
        {
            if *line_track != iteration.0.start.line as usize {
                *line_track = iteration.0.start.line as usize;
                print!("\n--new_line--\nline {}: ", line_track);
            }
            print!("{:?} ", iteration.1);
        });
    }

    pub(crate) fn view_explorations(&self, rl_m: &mut Editor<(), MemHistory>) {
        todo!()
    }
}

pub(crate) fn analyze_dreams(
    dream_space: &mut HashMap<String, Box<ParsedDream>>,
) -> HashMap<String, Box<AnalyzedDream>> {
    let dream_walker = dream_space.drain();
    let mut analyzed_dreams: HashMap<String, Box<AnalyzedDream>> = HashMap::new();
    for (dream_name, parsed_dream_box) in dream_walker {
        let parsed_dream = *parsed_dream_box; // Consume the Box to own ParsedDream
        let new_dream_analysis = AnalyzedDream::new(parsed_dream);
        analyzed_dreams.insert(dream_name, Box::new(new_dream_analysis));
    }
    analyzed_dreams
}
