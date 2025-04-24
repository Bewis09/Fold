use std::{env::{args, current_dir}, fs};

use serde_json::{json, Value};

fn default_json() -> Value {
    json!({
        "$schema": "https://raw.githubusercontent.com/Bewis09/Fold/refs/heads/master/fold.schema.json",
        "tasks": {
            "test": "echo 'Hello World'",
        }
    })
}

fn main() {
    let result = run();

    if let Err(e) = result {
        eprintln!("\x1B[1;31mError\x1B[0m: {}", e);
        eprintln!("Use '{} help' to show correct usage", args().collect::<Vec<String>>()[0]);
    }
}

fn run() -> Result<(), String> {
    let args: Vec<String> = args().collect();
    let task = args.get(1).ok_or("Invalid arguments")?;

    match task.as_str() {
        "run" => {
            run_task(args[2..].to_vec())?;
        }
        "init" => {
            run_init()?;
        }
        _ => return Err("Invalid arguments".to_string()),
    }

    Ok(())
}

fn run_task(task: Vec<String>) -> Result<(), String> {

    Ok(())
}

fn run_init() -> Result<(), String> {
    fs::write(current_dir().and_then(|p| {
        Ok(p.join("fold.json"))
    }).map_err(|x| x.to_string())?, serde_json::to_string_pretty(&default_json()).unwrap()).map_err(|x| x.to_string())?;

    Ok(())
}