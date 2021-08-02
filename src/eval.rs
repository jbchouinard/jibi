use std::rc::Rc;

use crate::*;

fn apply_builtin(b: JBuiltin, args: Vec<JValue>, env: JEnvRef) -> JResult {
    let mut evaluated = vec![];
    for arg in args {
        evaluated.push(jeval(arg, Rc::clone(&env))?);
    }
    (b.f)(evaluated, env)
}

fn apply_builtin_macro(b: JBuiltin, args: Vec<JValue>, env: JEnvRef) -> JResult {
    (b.f)(args, env)
}

// TODO: currying
fn apply_lambda(lambda: JLambda, args: Vec<JValue>, _env: JEnvRef) -> JResult {
    if args.len() != lambda.params.len() {
        return Err(JError::new(
            "ArgumentError",
            &format!("expected {} arguments", lambda.params.len()),
        ));
    }
    // Create environment to bind the arguments
    let env = JEnv::new(Some(lambda.closure)).into_ref();
    for (name, val) in lambda.params.iter().zip(args.into_iter()) {
        env.define(name, val);
    }
    jeval(lambda.code, env)
}

fn eval_sexpr(mut list: Vec<JValue>, env: JEnvRef) -> JResult {
    if list.is_empty() {
        Ok(JValue::SExpr(vec![]))
    } else {
        let args = list.split_off(1);
        let func = list.pop().unwrap();
        let func = jeval(func, Rc::clone(&env))?;

        match func {
            JValue::Builtin(b) => apply_builtin(b, args, env),
            JValue::BuiltinMacro(b) => apply_builtin_macro(b, args, env),
            JValue::Lambda(l) => apply_lambda(*l, args, env),
            _ => Err(JError::new("TypeError", "expected a callable")),
        }
    }
}

pub fn jeval(expr: JValue, env: JEnvRef) -> JResult {
    match expr {
        JValue::SExpr(list) => eval_sexpr(list, env),
        JValue::Int(_) => Ok(expr),
        JValue::Symbol(sym) => match env.lookup(&sym) {
            Some(val) => Ok(val),
            None => Err(JError::new("Undefined", &sym)),
        },
        JValue::Error(_) => Ok(expr),
        JValue::Builtin(_) => Ok(expr),
        JValue::BuiltinMacro(_) => Ok(expr),
        JValue::Lambda(_) => Ok(expr),
    }
}