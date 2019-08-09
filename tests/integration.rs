use yoloxide::environment::{ContextMap, Environment};
use yoloxide::execute_line;

use yolk::ast::YololNode;
use yolk::error::YolkError;
use yolk::optimizer::optimize;
use yolk::parser::parse;
use yolk::transpiler::transpile;

fn evaluate(source: &str) -> Result<(Environment, Environment), YolkError> {
    let (original, context) = transpile(&parse(source)?)?;
    let optimized = optimize(&original, &context);
    let mut original_env = Environment::new("");
    execute_line(&mut original_env, YololNode::format_as_program(&original));
    let mut optimized_env = Environment::new("");
    execute_line(&mut optimized_env, YololNode::format_as_program(&optimized));
    Ok((original_env, optimized_env))
}

#[test]
fn test_let_number() -> Result<(), YolkError> {
    let (original_env, optimized_env) = evaluate("let output = 1; export output;")?;
    assert_eq!(
        original_env.get_val("output"),
        optimized_env.get_val("output")
    );
    Ok(())
}

#[test]
fn test_let_array() -> Result<(), YolkError> {
    let (original_env, optimized_env) = evaluate("let output = [1, 2, 3]; export output;")?;
    assert_eq!(
        vec![
            original_env.get_val("output_1"),
            original_env.get_val("output_2"),
            original_env.get_val("output_3")
        ],
        vec![
            optimized_env.get_val("output_1"),
            optimized_env.get_val("output_2"),
            optimized_env.get_val("output_3")
        ]
    );
    Ok(())
}
