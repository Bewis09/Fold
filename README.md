# Fold

Fold is a task runner designed to help you automate and manage your development workflows efficiently.

## Features

- Define and run custom tasks
- Simple and intuitive configuration
- Fast execution and clear output

## Usage

Download the latest exe from the releases tab and add it to the PATH Environment Variable

```$ fold help``` Show help about the commands`

```$ fold init [--force]``` Add a template configuration file

```$ fold run <task> [args]``` Run a predefined task

## Configuration

```
{
    "tasks": {
        "example": "echo hello world"
        // Define tasks here
    },
    "config": {
        "configFiles": [
            {
                "path":"file.json",
                "json_path":["scripts"],
                "prefix":"example"
                // This defines another file from which the tasks will be loaded
            }
        ],
        "enabledFiles": [
            "deno", "node"
            // Those are predefined config files from which fold can load the scripts
            // Currently only deno and node (package.json) aare supported
        ],
        "silent": false
        // Sets if fold should not print its debug output
    }
}
```

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## License

This project is licensed under the CC BY-SA 4.0.