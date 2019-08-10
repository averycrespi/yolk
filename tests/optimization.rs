use yoloxide::environment::{ContextMap, Environment};
use yoloxide::execute_line;

use yolk::ast::YololNode;
use yolk::error::YolkError;
use yolk::{optimize, parse, transpile};

fn source_to_envs(source: &str) -> Result<Vec<Environment>, YolkError> {
    let (original, context) = transpile(&parse(source)?)?;
    let optimized = optimize(&original, &context);
    let mut original_env = Environment::new("");
    execute_line(&mut original_env, YololNode::format_as_program(&original));
    let mut optimized_env = Environment::new("");
    execute_line(&mut optimized_env, YololNode::format_as_program(&optimized));
    Ok(vec![original_env, optimized_env])
}

fn expect_same(envs: &[Environment], ident: &str) {
    assert!(envs.len() >= 2, "not enough envs provided");
    let baseline = envs[0].get_val(ident);
    if baseline == envs[0].get_val("undefined") {
        panic!("baseline has default value");
    }
    for env in envs.iter() {
        assert_eq!(baseline, env.get_val(ident));
    }
}

#[test]
fn test_import() -> Result<(), YolkError> {
    let envs = source_to_envs("import i; let n = i + 1; export n;")?;
    expect_same(&envs, "n");
    Ok(())
}

#[test]
fn test_let_number() -> Result<(), YolkError> {
    let envs = source_to_envs("let n = 2^3 + 4; export n;")?;
    expect_same(&envs, "n");
    Ok(())
}

#[test]
fn test_let_array() -> Result<(), YolkError> {
    let envs = source_to_envs("let a = [1, 2, 3] ^ 2; export a;")?;
    expect_same(&envs, "a_0");
    expect_same(&envs, "a_1");
    expect_same(&envs, "a_2");
    Ok(())
}

#[test]
fn test_let_infix() -> Result<(), YolkError> {
    let envs = source_to_envs(
        "let n = (1 + 2 - 3 * 4 / 5 % 6 < 7 <= 8 > 9 >= 10 == 11 != 12 and 13 or 14) + 1; export n;",
    )?;
    expect_same(&envs, "n");
    Ok(())
}
