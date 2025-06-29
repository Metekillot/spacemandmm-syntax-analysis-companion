use console::*;
use dm::annotation::*;
use dm::indents::*;
use dm::lexer::*;
use dm::objtree::*;
use dm::parser::*;
use dm::preprocessor::*;
use dm::*;
use dmc::*;
use std::env;
use std::path::*;

// a number-indexed array of the files so we can access them by FileId
// because that's a private constructor and you can't index them with it otherwise...

fn index_files_by_index(context: &Context) -> Vec<String> {
    let mut indexed_paths: Vec<String> = Vec::new();
    indexed_paths.push(String::from("parser file ID list is 1-indexed, the horror"));
    let file_list = context.file_list();
    file_list.for_each(|path| {
        let path_string = path.to_str().unwrap();
        indexed_paths.push(path_string.to_owned());
    });
    indexed_paths
}

fn main() {
    let dme_path: PathBuf = PathBuf::new();
    let context = Context::default();
    let mut annotation_tree = AnnotationTree::default();

    let pre_processor = Preprocessor::new(&context, dme_path.to_owned()).unwrap();
    let indent_processor = IndentProcessor::new(&context, pre_processor);
    let mut parser = Parser::new(&context, indent_processor);
    let annotation_tree_mutable = &mut annotation_tree;
    parser.annotate_to(annotation_tree_mutable);

    let obj_tree = parser.parse_object_tree();

    dmc::run(&context, &obj_tree);
    let indexed_paths_vector = index_files_by_index(&context);
}


fn line_input(terminal: &Term, prompt: &str) -> String {
    loop {
        println!("{}", prompt);
        match terminal.read_line() {
            Ok(line) => {
                if line.trim().is_empty() {
                    println!("Input cannot be empty. Please try again.");
                    continue;
                }
                return line;
            }
            Err(e) => {
                println!("Error reading line: {}", e);
                continue;
            }
        }
    }
}

fn char_input(terminal: &Term, prompt: &str) -> char {
    loop {
        println!("{}", prompt);
        match terminal.read_key() {
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
