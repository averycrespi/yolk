use yoloxide::environment::{ContextMap, Environment};
use yoloxide::execute_line;

use yolk::error::YolkError;
use yolk::{YolkProgram, YololProgram};

use std::convert::TryInto;
use std::fs;
use std::path::PathBuf;

fn find_test_files() -> Vec<String> {
    let mut corpus = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    corpus.push("tests/corpus");
    let mut files = Vec::new();
    let entry = fs::read_dir(corpus).unwrap();
    for path in entry {
        //TODO: cleaner extension checking
        let file = path.unwrap().path().to_str().unwrap().to_string();
        if file.ends_with(".yolk") {
            files.push(file);
        }
    }
    files
}

fn yolol_to_env(program: YololProgram) -> Environment {
    let mut env = Environment::new("");
    execute_line(&mut env, program.to_string());
    env
}

#[test]
fn test_correctness() -> Result<(), YolkError> {
    let test_files = find_test_files();
    for file in test_files {
        println!("case: {}", file);
        let source = fs::read_to_string(file).unwrap();
        let yolk: YolkProgram = source.parse()?;
        let yolol: YololProgram = yolk.try_into()?;
        let optimized = yolol.optimize();
        let env = yolol_to_env(optimized);
        assert_eq!(env.get_val("n").to_string(), env.get_val("e").to_string());
    }
    Ok(())
}

#[test]
fn test_idempotence() -> Result<(), YolkError> {
    let test_files = find_test_files();
    for file in test_files {
        println!("case: {}", file);
        let source = fs::read_to_string(file).unwrap();
        let yolk: YolkProgram = source.parse()?;
        let yolol: YololProgram = yolk.try_into()?;
        let once = yolol.optimize();
        let twice = once.clone().optimize();
        assert_eq!(once, twice);
    }
    Ok(())
}
