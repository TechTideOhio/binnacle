use std::{env, path::PathBuf, process::ExitCode};

const USAGE: &str =
    "usage: binnacle [--root <Tools dir>] table | doctor | run [--pointer </json/pointer>] <tool|step> [args...]";

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
    } else if std::path::Path::new("Tools/reval").is_dir() {
        PathBuf::from("Tools")
    } else if std::path::Path::new("../reval").is_dir() {
        PathBuf::from("..")
    } else if std::path::Path::new("../Tools/reval").is_dir() {
        PathBuf::from("../Tools")
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
        Some("run") => {
            let mut rest = &args[1..];
            let mut pointer = None;
            if rest.first().map(String::as_str) == Some("--pointer") {
                let Some(p) = rest.get(1) else {
                    eprintln!("error: --pointer needs a value, e.g. /passed");
                    return ExitCode::from(2);
                };
                pointer = Some(p.clone());
                rest = &rest[2..];
            }
            let Some((tool, tool_args)) = rest.split_first() else {
                eprintln!("{USAGE}");
                return ExitCode::from(2);
            };
            match binnacle::run_tool(&root, tool, tool_args) {
                Ok(env) => {
                    if let Some(p) = pointer {
                        match binnacle::extract(&env, &p) {
                            Ok(Some(s)) => println!("{s}"),
                            Ok(None) => {
                                eprintln!("error: pointer '{p}' not found in envelope");
                                return ExitCode::from(2);
                            }
                            Err(e) => {
                                eprintln!("error: {e}");
                                return ExitCode::from(2);
                            }
                        }
                    } else {
                        emit(&env);
                    }
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
            }
        }
        _ => {
            eprintln!("{USAGE}");
            ExitCode::from(2)
        }
    }
}
