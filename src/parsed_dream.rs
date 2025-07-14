use dm::annotation::*;
use dm::indents::*;
use dm::lexer::*;
use dm::objtree::*;
use dm::parser::*;
use dm::preprocessor::*;
use dm::*;
use dmc::{AnalyzeObjectTree};
use std::cell::RefCell;
use std::collections::HashMap;
use std::path::*;

pub(crate) struct ParsedDream {
    pub(crate) context: Context,
    pub(crate) annotation_tree: AnnotationTree,
    pub(crate) object_tree: ObjectTree,
}

impl ParsedDream {
    pub(crate) fn new(dme_path: &PathBuf) -> ParsedDream {
        let context = Context::default();
        println!("Context templated...");
        let mut annotation_tree = AnnotationTree::default();
        println!("Annotation Tree templated...");
        let mut pre_processor = Preprocessor::new(&context, dme_path.to_owned()).unwrap();
        println!("Preprocessor created...");
        /*pre_processor.enable_annotations();
        println!("Preprocessor annotations enabled...");*/
        let indent_processor = IndentProcessor::new(&context, &mut pre_processor);
        println!("Indent processor created...");
        let mut parser = Parser::new(&context, indent_processor);
        println!("Parser created...");
        let annotation_tree_mutable = &mut annotation_tree;
        parser.annotate_to(annotation_tree_mutable);
        println!("Parser annotations enabled...");
        let object_tree = parser.parse_object_tree();
        println!("Object Tree created...");
        let annotation_refcell = RefCell::new(annotation_tree);
        dmc::run(&context, &object_tree, Some(&annotation_refcell));
        println!("Dreamchecker analysis completed...");
        let annotation_tree = annotation_refcell.take();
        /*annotation_tree_mutable.merge(
            pre_processor
                .take_annotations()
                .expect("Failed to merge macro annotations in"),
        );
        println!("Annotations merged between Parser and Preprocessor...");*/
        ParsedDream {
            context,
            annotation_tree,
            object_tree,
        }
    }
}
