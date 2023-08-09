# Tedo.ai

Tedo.ai is a project management tool that lives primarily in your terminal.


## Basic usage

### 1. Create a Project

Creates a new project and optionally switches the context to the new project.

```bash
tedo create project <project_name> [--switch]
```

- `<project_name>`: The name of the project.
- `--switch`: An optional flag to switch to the newly created project.


### 2. Change Context to an Existing Project

Switches the current context to an existing project.

```bash
tedo switch <project_name>
```

- `<project_name>`: The name of the existing project.

### 3. Create a Task in Current Context

Creates a new task in the current context (project).

```bash
tedo create task <task_description> [--due <due_date>]
```
