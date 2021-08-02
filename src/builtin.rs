use std::convert::TryInto;
use std::rc::Rc;

use crate::*;

fn jbuiltin_add(args: Vec<JValue>, _env: JEnvRef) -> JResult {
    let mut sum: JTInt = 0;
    for arg in args {
        match arg.into_int() {
            Ok(n) => {
                sum += n;
            }
            Err(e) => return Err(e),
        }
    }
    Ok(JValue::Int(sum))
}

fn jbuiltin_define(args: Vec<JValue>, env: JEnvRef) -> JResult {
    let [jsym, jval] = get_n_args(args)?;
    let sym = match jsym {
        JValue::Symbol(s) => s,
        _ => return Err(JError::new("TypeError", "expected a symbol")),
    };
    let jval = jeval(jval, Rc::clone(&env))?;
    env.define(&sym, jval);
    Ok(JValue::SExpr(vec![]))
}

fn jbuiltin_lambda(args: Vec<JValue>, env: JEnvRef) -> JResult {
    let [pvals, code] = get_n_args(args)?;
    let pvals = match pvals {
        JValue::SExpr(p) => p,
        _ => return Err(JError::new("TypeError", "expected a list of symbols")),
    };
    let mut params = vec![];
    for val in pvals {
        match val {
            JValue::Symbol(s) => params.push(s),
            _ => return Err(JError::new("TypeError", "expected a list of symbols")),
        }
    }
    Ok(JValue::Lambda(Box::new(JLambda {
        closure: env,
        code,
        params,
    })))
}

fn add_builtin<T>(name: &str, f: T, env: &JEnv)
where
    T: 'static + Fn(Vec<JValue>, JEnvRef) -> JResult,
{
    let val = JValue::Builtin(JBuiltin {
        name: name.to_string(),
        f: Rc::new(f),
    });
    env.define(name, val);
}

fn add_builtin_macro<T>(name: &str, f: T, env: &JEnv)
where
    T: 'static + Fn(Vec<JValue>, JEnvRef) -> JResult,
{
    let val = JValue::BuiltinMacro(JBuiltin {
        name: name.to_string(),
        f: Rc::new(f),
    });
    env.define(name, val);
}

pub fn add_builtins(env: &JEnv) {
    add_builtin("+", jbuiltin_add, env);
    add_builtin_macro("def", jbuiltin_define, env);
    add_builtin_macro("fn", jbuiltin_lambda, env);
}

fn get_n_args<const N: usize>(args: Vec<JValue>) -> Result<[JValue; N], JError> {
    if args.len() == N {
        Ok(args.try_into().unwrap())
    } else {
        Err(JError::new(
            "ArgumentError",
            &format!("expected {} arguments", N),
        ))
    }
}