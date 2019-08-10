use yoloxide::environment::{ContextMap, Environment};
use yoloxide::execute_line;

use yolk::ast::YololNode;
use yolk::error::Error;
use yolk::{optimize, parse, transpile};

fn source_to_env(source: &str) -> Result<Environment, Error> {
    let yolk = parse(source)?;
    let (yolol, context) = transpile(&yolk)?;
    let optimized = optimize(&yolol, &context);
    let mut env = Environment::new("");
    execute_line(&mut env, YololNode::format_as_program(&optimized));
    Ok(env)
}

fn expect(env: &Environment, ident: &str, value: &str) {
    assert!(value != "0", "expecting default value");
    assert_eq!(env.get_val(ident).to_string(), value.to_string());
}

#[test]
fn test_sum_one_to_five() -> Result<(), Error> {
    let env = source_to_env("let n = sum([1, 2, 3, 4, 5]); export n;")?;
    expect(&env, "n", "15");
    Ok(())
}

#[test]
fn test_dot_product() -> Result<(), Error> {
    let env = source_to_env("let n = sum([1, 2] * [3, 4]); export n;")?;
    expect(&env, "n", "11");
    Ok(())
}

#[test]
fn test_sum_opposites() -> Result<(), Error> {
    let env = source_to_env("let n = sum(0, 1, -1, 1.0, -1.0, 1.23, -1.23) + 1; export n;")?;
    expect(&env, "n", "1");
    Ok(())
}

#[test]
fn test_arithmetic_precedence() -> Result<(), Error> {
    let env = source_to_env("let n = 1 + 2 - 3 * 4 / 5 % 6; export n;")?;
    expect(&env, "n", "0.6");
    Ok(())
}

#[test]
fn test_boolean_precedence() -> Result<(), Error> {
    let env = source_to_env("let n = 1 or 0 and 1; export n;")?;
    expect(&env, "n", "1");
    Ok(())
}

#[test]
fn test_logical_precedence() -> Result<(), Error> {
    let env = source_to_env("let n = 0 > 0 == 0; export n;")?;
    expect(&env, "n", "1");
    Ok(())
}
