use super::parsed_dream::{ParsedDream};
use dm::{Context, FileList};
use dm::ast::*;
use dm::annotation::*;
use dm::objtree::*;
use std::collections::HashMap;

pub(crate) struct AnnotationCorrelation {

}
pub(crate) struct AnalyzedDream<'a> {
    pub(crate) dream_object_tree: &'a ObjectTree,
    pub(crate) dream_context: &'a Context,
    pub(crate) dream_annotation_tree: &'a AnnotationTree,
    pub(crate) proc_annotation_correlation: Option<HashMap<ProcRef<'a>, Box<[AnnotationCorrelation]>>>
}

impl<'a> AnalyzedDream<'a> {
    pub(crate) fn new(dream_object_tree: &'a ObjectTree, dream_context: &'a Context, dream_annotation_tree: &'a AnnotationTree) -> AnalyzedDream<'a> {
        let mut explored_dream = AnalyzedDream {
            dream_object_tree,
            dream_context,
            dream_annotation_tree,
            proc_annotation_correlation: None
        }
        explored_dream.correlate_procs_to_annotations();
        explored_dream
    }
    fn correlate_procs_to_annotations(&mut self) {
        
    }
}

pub(crate) fn analyze_dreams(dream_space: &mut HashMap<String, Box<ParsedDream>>) -> HashMap<String, Box<AnalyzedDream>> {
    let dream_walker = dream_space.iter();
    let mut analyzed_dreams: HashMap<String, Box<AnalyzedDream>> = HashMap::new();
    for (dream_name,analyzed_dream) in dream_walker {
        let dream_object_tree: &ObjectTree = &analyzed_dream.object_tree;
        let dream_context: &Context = &analyzed_dream.context;
        let dream_annotation_tree: &AnnotationTree = &analyzed_dream.annotation_tree;
        let new_dream_analysis = AnalyzedDream::new(dream_object_tree, dream_context, dream_annotation_tree);
        &mut analyzed_dreams.insert(dream_name.to_owned(), Box::new(new_dream_analysis));
    }
    analyzed_dreams
}