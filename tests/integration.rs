use yoloxide::environment::{ContextMap, Environment};
use yoloxide::execute_line;

use yolk::ast::YololNode;
use yolk::error::Error;
use yolk::{optimize, parse, transpile};

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

fn yolol_to_env(yolol: &[YololNode]) -> Environment {
    let mut env = Environment::new("");
    execute_line(&mut env, YololNode::format_as_program(&yolol));
    env
}

#[test]
fn test_correctness() -> Result<(), Error> {
    let test_files = find_test_files();
    for file in test_files {
        println!("case: {}", file);
        let source = fs::read_to_string(file).unwrap();
        let yolk = parse(&source)?;
        let (yolol, context) = transpile(&yolk)?;
        let optimized = optimize(&yolol, &context);
        let env = yolol_to_env(&optimized);
        assert_eq!(env.get_val("n").to_string(), env.get_val("e").to_string());
    }
    Ok(())
}

#[test]
fn test_idempotence() -> Result<(), Error> {
    let test_files = find_test_files();
    for file in test_files {
        println!("case: {}", file);
        let source = fs::read_to_string(file).unwrap();
        let yolk = parse(&source)?;
        let (yolol, context) = transpile(&yolk)?;
        let once = optimize(&yolol, &context);
        let twice = optimize(&once, &context);
        assert_eq!(once, twice);
    }
    Ok(())
}