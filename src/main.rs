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
use std::fs::*;
use std::path::*;
use std::process::*;
use std::str::FromStr;
use std::sync::LazyLock;
use std::vec::*;

static TERMINAL: LazyLock<Term> = LazyLock::new(|| console::Term::stdout());

use char_input as c_i;
use line_input as l_i;
use line_input_into as l_i_i;

fn line_input_into<T: FromStr>(prompt: &str) -> T {
    loop {
        let got_input = line_input(prompt);
        match got_input.parse::<T>() {
            Ok(result) => break result,
            Err(error) => {
                println!("{} couldn't be parsed, please try again.", got_input);
                continue;
            }
        }
    }
}

fn line_input(prompt: &str) -> String {
    loop {
        println!("{}", prompt);
        match TERMINAL.read_line() {
            Ok(result) => break result,
            Err(err) => {
                println!("Error reading line");
                continue;
            }
        }
    }
}

fn char_input(prompt: &str) -> char {
    loop {
        println!("{}", prompt);
        match TERMINAL.read_key() {
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

fn get_dme_path_from_dir() -> PathBuf {
    let mut dme_dir: PathBuf = PathBuf::new();
    let init_input = l_i("Please input the .dme directory. '$env_var' to access an environment variable, '!command arg1 arg2 arg3' to invoke a process's output for the value.");
    match *&init_input
        .chars()
        .nth(0)
        .expect("Fatal failure to parse initial input.")
    {
        '$' => {
            let fetched_var_name: &String = &init_input.chars().skip(1).collect();
            let fetched_var_value = env::var(fetched_var_name).expect(&format!(
                "Failed to fatch environmental variable: {}",
                fetched_var_name
            ));
            dme_dir.push(fetched_var_value);
        }
        '!' => {
            let parsed_command: &String = &init_input.chars().skip(1).collect();
            let mut command_parts = parsed_command.split(' ').into_iter();
            let actual_command = command_parts
                .next()
                .expect("Failed to parse a command for a process to initiate.");
            let command_output = Command::new(actual_command)
                .args(command_parts.collect::<Vec<_>>())
                .output()
                .unwrap();
            let trimmed_out = &*String::from_utf8_lossy(command_output.stdout.trim_ascii_end());
            println!("{:?}", &trimmed_out);
            dme_dir.push(&trimmed_out);
        }
        _ => {
            dme_dir.push(init_input);
        }
    }
    let resolved_dir_contents: Vec<DirEntry> = (*dme_dir)
        .read_dir()
        .expect(&format!("Failed to read directory: {}", dme_dir.display()))
        .map(|res| res.expect("Invalid directory entry"))
        .collect();

    for entry in &resolved_dir_contents {
        println!("{:?}", entry);
    }

    resolved_dir_contents
        .iter()
        .filter(|file| file.file_name().to_str().unwrap().ends_with(".dme"))
        .next()
        .unwrap()
        .path()
}

pub(crate) struct ParsedDream<'a> {
    context: Context,
    annotation_tree: AnnotationTree,
    object_tree: ObjectTree,
    indexed_files: Vec<&'a Path>,
}

// a number-indexed array of the files so we can access them by FileId
// because that's a private constructor and you can't index them with it otherwise...

fn index_files_by_index(context: &Context) -> Vec<String> {
    let mut indexed_paths: Vec<String> = Vec::new();
    indexed_paths.push(String::from("DISREGARD_ALIGN_ITEM: Context FileID is 1-indexed"));
    let file_list = context.file_list();
    file_list.for_each(|path| {
        let path_string = path.to_str().unwrap();
        indexed_paths.push(path_string.to_owned());
    });
    indexed_paths
}

fn main() {
    let dme_path = get_dme_path_from_dir();
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
