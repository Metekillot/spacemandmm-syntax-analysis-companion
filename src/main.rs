use console::Key;
use console::Term;
use dm::annotation::AnnotationTree;
use dm::indents::IndentProcessor;
use dm::objtree::ProcValue;
use dm::objtree::TypeRef;
use dm::parser::Parser;
use dm::preprocessor::Preprocessor;
use dm::Context;
use dmc;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;

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
    match env::var("USERPROFILE") {
        Ok(val) => {
            let mut requiem_path = PathBuf::from(&val);
            let mut relative_path_ = requiem_path.clone();
            relative_path_.push("code/git/requiem");
            let relative_path = relative_path_.to_str().unwrap().to_owned();
            requiem_path.push("code/git/requiem/tgstation.dme");

            let req_context = Context::default();
            let mut req_annotation_tree = AnnotationTree::default();

            let pp = Box::new(Preprocessor::new(&req_context, requiem_path.to_owned()).unwrap());
            let ip = IndentProcessor::new(&req_context, pp);
            let mut req_parser = Parser::new(&req_context, ip);
            let anno_mut = &mut req_annotation_tree;
            req_parser.annotate_to(anno_mut);

            
            let req_obj_tree = req_parser.parse_object_tree();
            
            dmc::run(&req_context, &req_obj_tree);
            let indexed_paths_vector = index_files_by_index(&req_context);
            /*
            let req_errors = req_context.errors().to_owned();
            let types: Vec<&dm::objtree::Type> = req_obj_tree.iter_types().map(|t| t.get()).collect();
            let annotations: Vec<&Annotation> = req_annotation_tree.iter().map(|(_, a)| a).collect();
            */
            let CONVENIENT_TUPLE = &(
                &req_annotation_tree,
                &req_obj_tree,
                &req_context,
                &indexed_paths_vector,
                &relative_path,
            );
            let terminal = Term::stdout();
            let l_i = |prompt: &str| line_input(&terminal, prompt);
            let c_i = |prompt: &str| char_input(&terminal, prompt);
            let input_tuple = &(
                l_i,
                c_i,
            );
                loop {
                let input = c_i("Waiting for input...\n'a'nnotations, 'f'iles, 't'ypes, 'l'exer print, 'e'xit.");
                println!("{}", input);
                match input {
                    'a' => inspect_annotation(input_tuple, CONVENIENT_TUPLE),
                    'f' => todo!(),
                    't' => inspect_type(input_tuple, CONVENIENT_TUPLE),
                    'l' => lexer_print(input_tuple, CONVENIENT_TUPLE),
                    'e' => break,
                    _ => continue,
                }
            }
        }
        Err(e) => {
            println!("USERPROFILE is not set: {}", e)
        }
    }
}

fn inspect_type(
    input_tuple: &(
        impl Fn(&str) -> String,
        impl Fn(&str) -> char,
    ),
    CONVENIENT_TUPLE: &(
        &AnnotationTree,
        &dm::objtree::ObjectTree,
        &Context,
        &Vec<String>,
        &String,
    ),
) {
    let (anno, obj_t, context, file_index, relative_path) = *CONVENIENT_TUPLE;
    let (l_i, c_i) = input_tuple;
    loop {
        let input = &l_i("Enter type path to find, or 'exit'");
        if let Some(type_ref_got) = obj_t.find(input) {
            println!("Found {}", type_ref_got);
            match c_i("Enumerate 'p'rocs or 'v'ars"){
                'p' => inspect_procs(input_tuple, &type_ref_got),
                'v' => inspect_vars(input_tuple, &type_ref_got),
                _ => continue,
            }
        } else {
            if input == "exit" {
                break;
            } else {
                println!("Type not found: {}", input);
                continue;
            }
        }
    }
}

fn inspect_procs(input_tuple: &(
        impl Fn(&str) -> String,
        impl Fn(&str) -> char,
    ),
    type_ref: &TypeRef){
    let (l_i, c_i) = input_tuple;
}
fn inspect_vars(input_tuple: &(
        impl Fn(&str) -> String,
        impl Fn(&str) -> char,
    ),
    type_ref: &TypeRef){
    let (l_i, c_i) = input_tuple;
}


fn inspect_annotation(
    input_tuple: &(
        impl Fn(&str) -> String,
        impl Fn(&str) -> char,
    ),
    CONVENIENT_TUPLE: &(
        &AnnotationTree,
        &dm::objtree::ObjectTree,
        &Context,
        &Vec<String>,
        &String,
    ),
) {
    let (anno, obj_t, context, file_index, relative_path) = *CONVENIENT_TUPLE;
    let (l_i, c_i) = input_tuple;
    let anno_iter = anno.iter();
    let file_num = l_i("Enter File ID:");
    println!(
        "Verifying path: {}",
        file_index[file_num.parse::<usize>().unwrap()]
    );
    if c_i("(Y) to confirm") == 'y' {
        let real_file_id = context
            .get_file(std::path::Path::new(
                &file_index[file_num.parse::<usize>().unwrap()],
            ))
            .unwrap();
        println!("Confirming File ID {:?}", real_file_id);
        println!("pre-trim annotation number: {}", anno_iter.clone().count());
        let mut file_anno_iter = anno_iter.filter(|ele| {
            let given = ele.0.end;
            given.file == real_file_id
        });
        println!(
            "Post-trim annotation number: {}",
            file_anno_iter.clone().count()
        );
        let mut new_line_trip = 0;
        let trip_ref = &mut new_line_trip;
        let mut new_line_signal = |last_line: usize| {
            if *trip_ref != last_line {
                print!("\nline {}   ", last_line);
            }
            *trip_ref = last_line;
        };
        loop {
            if let Some(annotation) = file_anno_iter.next() {
                let last_line = annotation.0.start.line;
                new_line_signal(last_line as usize);
                print!("{:?} ", annotation.1);
            } else {
                break;
            }
        }
    }
}

fn lexer_print(
        input_tuple: &(
        impl Fn(&str) -> String,
        impl Fn(&str) -> char,
    ),
    CONVENIENT_TUPLE: &(
        &AnnotationTree,
        &dm::objtree::ObjectTree,
        &Context,
        &Vec<String>,
        &String,
    ),
) {
    let (anno, obj_t, context, file_index, relative_path) = *CONVENIENT_TUPLE;
    let (l_i, c_i) = input_tuple;
    loop {
        if let Ok(file_id) = l_i("Enter File index").parse::<usize>() {
            println!("Confirm FileID {}: {}", file_id, file_index[file_id]);
            let answer = c_i("Y/N?");
            match answer {
                'y' => {
                    let mut got_path_ = std::path::PathBuf::new();
                    got_path_.push(relative_path);
                    got_path_.push(&file_index[file_id]);
                    let partial_relative = std::path::Path::new(&file_index[file_id]);
                    let got_id_ = context.get_file(partial_relative).unwrap();
                    let lexer_ =
                        dm::lexer::Lexer::from_file(context, got_id_, got_path_.as_path()).unwrap();
                    let lexed_token = lexer_.map(|loc_tok| loc_tok.token);
                    let print_buffer = &mut String::new();
                    dm::pretty_print(print_buffer, lexed_token, true);
                    print!("{}", print_buffer);
                }
                _ => {
                    continue;
                }
            }
        }
    }
}

fn file_experiment(
    CONVENIENT_TUPLE: &(
        &AnnotationTree,
        &dm::objtree::ObjectTree,
        &Context,
        &Vec<String>,
    ),
    terminal: &console::Term,
) {
    let (anno_tree, obj_tree, context, indexed_paths) = *CONVENIENT_TUPLE;
    let terminal_ = terminal;
    let char_input_ = char_input;
    let line_input_ = line_input;
    let c_i = |prompt: &str| char_input_(terminal_, prompt);
    let l_i = |prompt: &str| line_input_(terminal_, prompt);
    let f_list = context.file_list();
    loop {
        match c_i("Search by: (f)ile ID, (p)ath OR (e)xit") {
            'f' => {
                if let Ok(str_id) = l_i("Enter the ID").parse::<u16>() {
                    println!("Filepath of {}: {}", str_id, indexed_paths[str_id as usize]);
                    println!(
                        "Verifying path: {}",
                        f_list
                            .get_path(
                                f_list
                                    .get_id(&std::path::Path::new(&indexed_paths[str_id as usize]))
                                    .unwrap()
                            )
                            .display()
                    );
                } else {
                    println!("Failed to parse that file ID. Please enter a number.");
                }
            }
            'p' => {
                todo!()
            }
            'e' => {
                break;
            }
            invalid => {
                println!("??{}??", invalid);
                continue;
            }
        }
    }
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
