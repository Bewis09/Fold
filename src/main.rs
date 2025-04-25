use std::{
    collections::HashMap,
    env::{args, current_dir},
    fs,
};

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
        eprintln!(
            "Use '{} help' to show correct usage",
            args().collect::<Vec<String>>()[0]
        );
        std::process::exit(1);
    }
}

fn load_file(path: &str) -> Result<Value, String> {
    let fold_file = current_dir()
        .and_then(|p| Ok(p.join(path)))
        .map_err(|_| "Failed to load fold.json")?;

    let fold_content = fs::read_to_string(&fold_file).map_err(|_| "Failed to load fold.json")?;
    let fold_json: Value =
        serde_json::from_str(&fold_content).map_err(|_| "Invalid JSON in fold.json")?;

    Ok(fold_json)
}

fn load_tasks(path: &str, json_path: &Vec<Value>) -> Result<HashMap<String, String>, String> {
    let fold_json: Value = load_file(path)?;

    let mut tasks = fold_json;

    for path in json_path.iter() {
        tasks = tasks
            .get(path.as_str().ok_or("Invalid JSON format")?)
            .ok_or(format!("No tasks found at path: {}", path))?
            .to_owned();
    }

    let array = tasks.as_object_mut().ok_or("Invalid JSON format")?;

    let mut map = HashMap::new();
    for (key, value) in array.iter() {
        let value = value.as_str().ok_or("Invalid task format")?;
        map.insert(key.to_string(), value.to_string());
    }

    Ok(map)
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
        "help" => {
            run_help()?;
        }
        _ => return Err("Invalid arguments".to_string()),
    }

    Ok(())
}

fn run_task(task: Vec<String>) -> Result<(), String> {
    if task.is_empty() {
        return Err("No task specified".to_string());
    }

    let task_name = task
        .get(0)
        .ok_or("No task present")?
        .split("/")
        .collect::<Vec<&str>>();

    if task_name.len() > 2 {
        return Err("Invalid task name".to_string());
    }

    let fold_json: Value = load_file("fold.json")?;
    let binding = json!({});
    let config = fold_json.get("config").unwrap_or(&binding);

    let silent = config
        .get("silent")
        .and_then(|x| x.as_bool())
        .unwrap_or(false);

    let mut tasks: Vec<(String, String)> = vec![];

    fn get_plugin_file(prefix: &str) -> Option<Value> {
        match prefix {
            "deno" => Some(json!({
                "path": "deno.json",
                "json_path": ["tasks"],
                "prefix": "deno"
            })),
            "node" => Some(json!({
                "path": "package.json",
                "json_path": ["scripts"],
                "prefix": "node"
            })),
            "fold" => Some(json!({
                "path": "fold.json",
                "json_path": ["tasks"],
                "prefix": "fold"
            })),
            _ => None,
        }
    }

    if let Some(cf) = config
        .get("configFiles")
        .and_then(|x| x.as_array())
        .and_then(|x| {
            let mut t = x.clone();
            let temp = &vec![];
            let mut enabled_files = config
                .get("enabledFiles")
                .and_then(|x| x.as_array())
                .unwrap_or(temp).clone();

            enabled_files.push(json!("fold"));

            for file in enabled_files {
                if let Some(file) = get_plugin_file(file.as_str().unwrap_or("")) {
                    t.push(file);
                }
            }

            Some(t)
        })
    {
        for file in cf {
            let object = file.as_object().ok_or("Invalid JSON format")?;

            let path = object
                .get("path")
                .ok_or("Invalid JSON format")?
                .as_str()
                .ok_or("Invalid JSON format")?;

            let json_path = object
                .get("json_path")
                .ok_or("Invalid JSON format")?
                .as_array()
                .ok_or("Invalid JSON format")?;

            let prefix = file
                .get("prefix")
                .and_then(|x| x.as_str())
                .unwrap_or(path);

            if task_name.len() > 1 {
                if task_name[0] != prefix {
                    continue;
                }
            }

            let file_tasks = load_tasks(path, json_path)?;
            let task: Option<&String> =
                file_tasks.get(task_name.last().unwrap_or(&&(task[0][..])).to_owned());

            if let Some(t) = task {
                tasks.push((prefix.to_string(), t.to_string()));
            }
        }
    }

    if tasks.is_empty() {
        Err("Task not found".to_string())?;
    }

    if tasks.len() > 1 {
        return Err(
            "Multiple tasks found. Use a prefix to specify which task you meant".to_string(),
        );
    }

    let task_command = tasks.iter().next().unwrap().1.to_string();

    if !silent {
        println!(
            "\x1B[1;34mRunning\x1B[0m: Task '{}' with command '{}'",
            task.get(0).unwrap(),
            task_command
        );
    }

    let output = std::process::Command::new("cmd")
        .arg("/c")
        .arg(task_command)
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .stdin(std::process::Stdio::inherit())
        .spawn()
        .map_err(|x| x.to_string())?
        .wait()
        .map_err(|x| x.to_string())?;

    if output.success() {
        if !silent {
            println!(
                "\x1B[1;32mSuccess\x1B[0m: Task '{}' executed successfully",
                task.get(0).unwrap()
            );
        }
    } else {
        return Err(format!(
            "Task '{}' failed with exit code {}",
            task.get(0).unwrap(),
            output
        ));
    }

    Ok(())
}

fn run_init() -> Result<(), String> {
    fs::write(
        current_dir()
            .and_then(|p| Ok(p.join("fold.json")))
            .map_err(|x| x.to_string())?,
        serde_json::to_string_pretty(&default_json()).unwrap(),
    )
    .map_err(|x| x.to_string())?;

    println!("\x1B[1;32mSuccess\x1B[0m: Created fold.json file");
    println!("You can now edit the file to add your own tasks");

    Ok(())
}

fn run_help() -> Result<(), String> {
    let help = format!(
        r#"
    Usage: {} [command] [args]

    Commands:
        run [task]   Run a task
        init         Initialize a new fold.json file
        help         Show this help message
    "#,
        args().collect::<Vec<String>>()[0]
    );

    println!("{}", help);
    Ok(())
}
