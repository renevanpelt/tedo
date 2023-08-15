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
tedo create task <task_description>
```

### 4. Likewise, create a Note Current Context

```bash
tedo create note <note_title>
```

### 5. List notes

```bash
tedo list notes
```
or in table mode
```bash
tedo table notes
```

### 6. Edit a note

```bash
tedo edit note <note_identifier>
```



## Using shorthands

We can concatenate 1-letter shorthands for commands. That way, we only use spaces to separate commands with identifiers.

We can also identify projects with the first letters of their names. You can find the project shorthands by running `tedo l` (or `list`, or `ls`)

For example

```bash
tedo ttp g
```

is equivalent to

```bash
tedo table tasks project general
```

given that `general` is the name of a project, and `g` is the shorthand

```bash

$ td l

# => Current project: (1) general     # projects: 3           # tasks 5       # notes 6
# => +----+--------------+-------+-------+
# => | ID | Project Name | Tasks | Notes |
# => +----+--------------+-------+-------+
# => | 1  | (g) general  | 5     | 5     |
# => +----+--------------+-------+-------+
# => | 2  | (f) foo      | 0     | 1     |
# => +----+--------------+-------+-------+
# => | 3  | (gr) growth  | 0     | 0     |
# => +----+--------------+-------+-------+

```

In this example, we can list the tasks of the project named "growth" with id 3 by running

```bash
tedo ttp gr
```

or 

```bash
tedo ttp 3
```


Where the `<note_identifier>` is the note's id or slug.