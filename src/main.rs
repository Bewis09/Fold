use std::{env::{args, current_dir}, fs};

const DEFAULT: &str = 
"{
    \"&schema\": \"https
    \"tasks\": {
        \"test\": \"echo 'Hello World'\",
    }
}";

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
            run_init(args[2..].to_vec())?;
        }
        _ => return Err("Invalid arguments".to_string()),
    }

    Ok(())
}

fn run_task(task: Vec<String>) -> Result<(), String> {

    Ok(())
}

fn run_init(task: Vec<String>) -> Result<(), String> {
    fs::write(current_dir().and_then(|p| {
        Ok(p.join("fold.json"))
    }).map_err(|x| x.to_string())?, "{\n    \"tasks\": {\n\n    }\n}").map_err(|x| x.to_string())?;

    Ok(())
}