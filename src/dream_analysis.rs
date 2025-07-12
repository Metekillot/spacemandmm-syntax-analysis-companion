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
use std::ops::Range;

fn annotation_structure_inspection(range_inspected: Range<Location>, annotation_tree: &AnnotationTree) {

}


#[derive(Debug)]
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
        let end_statement = code_iter.last().unwrap().to_owned();
        let end_location: Location;
        match end_statement.elem {
            Statement::While { condition, block } => {},
            Statement::DoWhile { block, condition } => {},
            Statement::ForInfinite { block } => {},
            Statement::ForList(list_box_statement) => {},
            Statement::ForLoop { init, test, inc, block } => {},
            Statement::ForRange(range_ox_statement) => {},
            Statement::If { arms, else_arm } => {},
            Statement::Spawn { delay, block } => {},
            Statement::
        }
        let annotation_documentation = annotation_tree
            .get_range(Range {
                start: start_location,
                end: end_location,
            })
            .map(|iteration| {
                println!("{:?}", iteration.1);
                format!("{:?}", iteration.1)
            })
            .collect();
        annotation_structure_inspection(Range{start: start_location, end: end_location}, annotation_tree);
        annotation_documentation
    }

    pub(crate) fn explore_proc(&mut self, rl_m: &mut Editor<(), MemHistory>) {
        todo!()
    }

    pub(crate) fn explore_file(&self, rl_m: &mut Editor<(), MemHistory>) {
        todo!()
    }

    pub(crate) fn explore_type(&self, rl_m: &mut Editor<(), MemHistory>) {
        todo!()
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
