use super::parsed_dream::ParsedDream;
use dm::annotation::*;
use dm::ast::*;
use dm::objtree::*;
use dm::{Context, FileList, Location};
use rustyline::Editor;
use rustyline::history::MemHistory;
use std::collections::HashMap;
use std::ops::Range;

pub(crate) struct ProcExploration<'a> {
    annotation_range: dm::annotation::Iter<'a>,
}
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
    pub(crate) fn explore_proc(&self, rl_m: &mut Editor<(), MemHistory>) {
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
            let mut line_break_tracker = 0;
            let mut proc_prommpt = String::new();
            proc_prommpt.push_str("\n");
            loop {
                match proc_iter.next() {
                    Some(found_ref) => 
                    None => {
                        proc_prompt.push_str("\n");
                        proc_prompt.push_str("\n")
                        break;
                    }
                }
            }

        } else {
            println!("Empty response received; returning to previous menu")
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
