use std::path::PathBuf;
use std::str::FromStr;

use home::home_dir;
use lazy_static::lazy_static;
use rustyline::Editor;
use structopt::StructOpt;

use jibi::{Interpreter, Token, TokenError, TokenValidator};

const VERSION: &str = env!("CARGO_PKG_VERSION");

lazy_static! {
    static ref HISTORY_FILE: PathBuf = match home_dir() {
        Some(mut p) => {
            p.push(".jibi_history");
            p
        }
        None => {
            eprintln!("could not locate home dir, saving history to current dir");
            PathBuf::from_str(".jibi_history").unwrap()
        }
    };
}

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(parse(from_os_str))]
    files: Vec<PathBuf>,
    #[structopt(short, long)]
    interactive: bool,
    #[structopt(long)]
    stack_size_mb: Option<usize>,
}

fn main() {
    let Opt {
        files,
        interactive,
        stack_size_mb,
    } = Opt::from_args();

    match stack_size_mb {
        // HACK: start new thread with the configured stack size
        // TODO: is there a way to modify stack size of current thread?
        Some(stack_size_mb) => {
            std::thread::Builder::new()
                .stack_size(stack_size_mb * 1024 * 1024)
                .spawn(move || run(files, interactive))
                .unwrap()
                .join()
                .unwrap();
        }
        None => run(files, interactive),
    }
}

fn run(files: Vec<PathBuf>, interactive: bool) {
    let mut interpreter = Interpreter::default();

    for file in &files {
        if let Err(exc) = interpreter.eval_file(&file) {
            Interpreter::print_exc(exc);
            std::process::exit(1);
        }
    }

    if interactive || files.is_empty() {
        repl(interpreter);
    }
}

fn repl(mut interpreter: Interpreter) {
    println!("jibi v{}", VERSION);
    let mut rl = Editor::<()>::new();
    let _ = rl.load_history(&*HISTORY_FILE);

    loop {
        match get_tokens(&mut rl, &mut interpreter) {
            Ok(tokens) => match interpreter.eval_tokens(Box::new(tokens.into_iter())) {
                Ok(Some(val)) => {
                    interpreter.call("display", vec![val]).unwrap();
                }
                Ok(None) => (),
                Err(exc) => Interpreter::print_exc(exc),
            },
            Err(e) => eprintln!("{}", e),
        }
    }
}

/// Get tokens that looks like they form a complete expression (balanced parens)
/// in multiple lines of input if necessary.
fn get_tokens(
    rl: &mut Editor<()>,
    interpreter: &mut Interpreter,
) -> Result<Vec<Token>, TokenError> {
    let mut validator = TokenValidator::new("#STDIN", &mut interpreter.state);
    let mut input = readline(rl, ">>> ");
    loop {
        match validator.input(input.to_string()) {
            Ok(Some(v)) => return Ok(v),
            Ok(None) => (),
            Err(e) => return Err(e),
        }
        input = readline(rl, "... ");
    }
}

fn readline(rl: &mut Editor<()>, prompt: &str) -> String {
    let input = match rl.readline(prompt) {
        Ok(input) => input,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };
    rl.add_history_entry(&input);
    if let Err(e) = rl.save_history(&*HISTORY_FILE) {
        eprintln!("Error saving history file: {}", e)
    }
    input
}
