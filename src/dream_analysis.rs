use crate::interface::space_output;

use super::parsed_dream::ParsedDream;
use dm::annotation::*;
use dm::ast::*;
use dm::objtree::*;
use dm::{Context, FileList, Location};
use rustyline::Editor;
use rustyline::history::MemHistory;
use std::collections::HashMap;
use std::ops::Range;

#[derive(Debug)]
pub(crate) struct ProcExploration<'a> {
    annotation_range: dm::annotation::Iter<'a>,
    range_collection: Vec<&'a Annotation>,
}

impl<'a> ProcExploration<'a> {
    fn new(
        proc_value: &ProcValue,
        dream_annotation_tree: &'a AnnotationTree,
    ) -> ProcExploration<'a> {
        let mut annotation_range: dm::annotation::Iter;
        let first_location = proc_value.location;
        println!("First location: {:?}", first_location);
        let last_location = (proc_value.code.as_ref().unwrap()).last().unwrap().location;
        println!("Last location: {:?}", last_location);
        let annotation_range = dream_annotation_tree.get_range(Range {
            start: first_location,
            end: last_location,
        });
        let mut debug_tracker: usize = 0;
        let range_collection = annotation_range
            .clone()
            .map(|range_pair| {
                *&mut debug_tracker += 1;
                println!("Mapped {} annotations.", *&debug_tracker);
                range_pair.1
            })
            .collect();

        ProcExploration {
            annotation_range,
            range_collection,
        }
    }
}

#[derive(Debug)]
pub(crate) struct AnalyzedDream<'a> {
    pub(crate) dream_object_tree: &'a ObjectTree,
    pub(crate) dream_context: &'a Context,
    pub(crate) dream_annotation_tree: &'a AnnotationTree,
    pub(crate) explored_procs: HashMap<ProcRef<'a>, ProcExploration<'a>>,
}

impl<'a> AnalyzedDream<'a> {
    pub(crate) fn new(
        dream_object_tree: &'a ObjectTree,
        dream_context: &'a Context,
        dream_annotation_tree: &'a AnnotationTree,
    ) -> AnalyzedDream<'a> {
        let mut explored_dream = AnalyzedDream {
            dream_object_tree,
            dream_context,
            dream_annotation_tree,
            explored_procs: HashMap::new(),
        };
        explored_dream
    }
    pub(crate) fn explore_proc(&mut self, rl_m: &mut Editor<(), MemHistory>) {
        let response = rl_m
            .readline("Please enter the type path that defines or overrides the proc.")
            .expect("Failed to get type path for explore_proc.");
        if !response.is_empty() {
            let type_find = self.dream_object_tree.find(response.as_str());
            let type_ref: TypeRef;
            match type_find {
                Some(type_unwrapped) => {
                    type_ref = type_unwrapped.clone();
                }
                None => {
                    println!("{} wasn't found!", response);
                    return;
                }
            }
            println!("Found {}!", type_ref);
            let mut proc_iter = type_ref.iter_self_procs();
            let mut proc_prompt = String::new();
            proc_prompt.push_str("\n");
            let proc_names: Vec<String> = proc_iter.map(|p| p.name().to_string()).collect();
            proc_prompt.push_str(&space_output(proc_names.iter().map(|s| s.as_str()), 5));
            proc_prompt
                .push_str("\n    Please enter the name of the proc you would like to explore.");
            let chosen_proc_ref = Self::choose_proc(rl_m, &proc_prompt, type_ref);
            let chosen_proc_value = chosen_proc_ref.clone().get();
            let new_exploration =
                ProcExploration::new(chosen_proc_value, self.dream_annotation_tree);
            self.explored_procs.insert(chosen_proc_ref, new_exploration);
            println!(
                "Length of collected annotations {}",
                self.explored_procs
                    .get(&chosen_proc_ref)
                    .unwrap()
                    .range_collection
                    .len()
            );
            let mut annotation_num: usize = 0;
            for annotation in &self
                .explored_procs
                .get(&chosen_proc_ref)
                .unwrap()
                .range_collection
            {
                println!("{:?}", annotation);
            }
        } else {
            println!("Empty response received; returning to previous menu")
        }
    }
    fn choose_proc(
        rl_m: &mut Editor<(), MemHistory>,
        proc_prompt: &String,
        type_ref: TypeRef<'a>,
    ) -> ProcRef<'a> {
        loop {
            let response = &rl_m.readline(&proc_prompt).unwrap();
            if !response.is_empty() {
                match type_ref.get_proc(response) {
                    Some(found_ref) => break found_ref,
                    None => {
                        println!("Proc not found.\n");
                        continue;
                    }
                }
            } else {
                continue;
            }
        }
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
    let dream_walker = dream_space.iter();
    let mut analyzed_dreams: HashMap<String, Box<AnalyzedDream>> = HashMap::new();
    for (dream_name, analyzed_dream) in dream_walker {
        let dream_object_tree: &ObjectTree = &analyzed_dream.object_tree;
        let dream_context: &Context = &analyzed_dream.context;
        let dream_annotation_tree: &AnnotationTree = &analyzed_dream.annotation_tree;
        let new_dream_analysis =
            AnalyzedDream::new(dream_object_tree, dream_context, dream_annotation_tree);
        (&mut analyzed_dreams).insert(dream_name.to_owned(), Box::new(new_dream_analysis));
    }
    analyzed_dreams
}
