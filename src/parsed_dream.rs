use dm::annotation::*;
use dm::indents::*;
use dm::lexer::*;
use dm::objtree::*;
use dm::parser::*;
use dm::preprocessor::*;
use dm::*;
use std::path::*;

pub(crate) struct ParsedDream {
    context: Context,
    annotation_tree: AnnotationTree,
    object_tree: ObjectTree,
}

impl ParsedDream {
    pub(crate) fn new(dme_path: &PathBuf) -> ParsedDream {
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