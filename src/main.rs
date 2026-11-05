use std::{env, path::PathBuf, process::ExitCode};

const USAGE: &str =
    "usage: binnacle [--root <Tools dir>] table | doctor | run <tool|step> [args...]";

fn emit<T: serde::Serialize>(v: &T) -> ExitCode {
    match serde_json::to_string_pretty(v) {
        Ok(s) => {
            println!("{s}");
            ExitCode::SUCCESS
        }
        Err(_) => ExitCode::from(2),
    }
}

fn main() -> ExitCode {
    let mut args: Vec<String> = env::args().skip(1).collect();
    let mut root = if std::path::Path::new("reval").is_dir() {
        PathBuf::from(".")
    } else if std::path::Path::new("../reval").is_dir() {
        PathBuf::from("..")
    } else {
        PathBuf::from(".")
    };
    if args.first().map(String::as_str) == Some("--root") && args.len() >= 2 {
        root = PathBuf::from(args.remove(1));
        args.remove(0);
    }
    match args.first().map(String::as_str) {
        Some("table") => emit(
            &binnacle::STEPS
                .iter()
                .map(|s| serde_json::json!({"step": s.step, "name": s.name, "tool": s.tool}))
                .collect::<Vec<_>>(),
        ),
        Some("doctor") => {
            let rows = binnacle::doctor(&root);
            let ok = rows.iter().all(|r| r.responds);
            emit(&rows);
            if ok {
                ExitCode::SUCCESS
            } else {
                ExitCode::from(1)
            }
        }
        Some("run") if args.len() >= 2 => match binnacle::run_tool(&root, &args[1], &args[2..]) {
            Ok(env) => {
                emit(&env);
                if env.passed {
                    ExitCode::SUCCESS
                } else {
                    ExitCode::from(1)
                }
            }
            Err(e) => {
                eprintln!("error: {e}");
                ExitCode::from(2)
            }
        },
        _ => {
            eprintln!("{USAGE}");
            ExitCode::from(2)
        }
    }
}
