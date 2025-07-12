use std::process::exit;

use crate::dream_analysis::AnalyzedDream;

use super::interface;

pub(crate) enum RunType {
    ProcRun,
    TypeRun,
    FileRun,
}

pub(crate) fn debug_run(run_type: Option<RunType>) {
    match run_type {
        Some(run_type) => match run_type {
            RunType::ProcRun => proc_run(None),
            RunType::TypeRun => type_run(None),
            RunType::FileRun => file_run(None),
        },
        None => inspect_tracker(),
    }
}

fn setup_run() -> AnalyzedDream {
    let dme_dir = std::path::PathBuf::from(std::env::var("SDMM_DEBUG_DATA_DIR").expect("No SDMM_DEBUG_DATA_DIR environment variable set."));
    let dme = interface::dme_from_dir(dme_dir).expect("Found no .dme for debug run");
    let mut debug_dream = crate::parsed_dream::ParsedDream::new(&dme);
    AnalyzedDream::new(
        debug_dream
    )
}

fn inspect_tracker() {

}


fn proc_run<'a, 'b>(run_args: Option<[&'b str; 2]>) {
    let type_for: &'b str;
    let proc_for: &'b str;
    if run_args.is_none() {
        println!("No args, so defaulting to generic type and proc");
        type_for = "/datum";
        proc_for = "Destroy";
    } else {
        let our_args = run_args.unwrap();
        type_for = our_args[0];
        proc_for = our_args[1];
    }
    let debug_analyze = &mut setup_run();
    debug_analyze.analyze_proc(type_for, proc_for);
    exit(0)
}

fn type_run(run_args: Option<[&str; 3]>) {
    let all_types: &str;
    let type_for: &str;
    let all_procs: &str;
    if run_args.is_none() {
        proc_run(None)
    }
    let debug_analyze = setup_run();
}
fn file_run(run_args: Option<[&str; 1]>) {
    let file_for: &str;
    if run_args.is_none() {
        proc_run(None)
    }
    let debug_analyze = setup_run();
}
